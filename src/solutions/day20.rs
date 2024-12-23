#![allow(unused)]
use crate::aoc::*;
use crate::solution;
use crate::solutions::day16;
use crate::solutions::day16::Tile;
use crate::utils::grid::{DenseGrid, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

const DAY: Day = 20;

solution!(DAY, solve_part_1);

fn solve_part_1(input: impl Lines) -> usize {
    let grid = day16::parse(&input);
    let start_pos = grid.find(&Tile::Start).exactly_one().ok().unwrap();
    let end_pos = grid.find(&Tile::End).exactly_one().ok().unwrap();
    let (initial_path, initial_cost) = find_shortest_path(&grid, start_pos, end_pos);
    let cheats = (0..grid.width())
        .cartesian_product(0..grid.height())
        .map(XY::from)
        .flat_map(|p| [DIR_UP, DIR_DOWN, DIR_LEFT, DIR_RIGHT].map(|d| SimpleCheat(p, p + d)))
        .filter_map(|cheat| {
            if initial_path.contains(&cheat.1) && grid.at(cheat.0.as_tuple()) == Some(&Tile::Wall) {
                let mut grid = grid.clone();
                grid.try_set_at(cheat.0.as_tuple(), Tile::Empty).unwrap();
                grid.try_set_at(cheat.1.as_tuple(), Tile::Empty).unwrap();
                Some((grid, cheat))
            } else {
                None
            }
        })
        .unique_by(|(_, cheat)| cheat.0)
        .par_bridge()
        .map(|(grid, cheat)| {
            let (_, cost) = find_shortest_path(&grid, start_pos, end_pos);
            (initial_cost - cost, cheat)
        });

    // cheats
    //     .into_group_map()
    //     .iter()
    //     .sorted_by_key(|(time_save, _)| **time_save)
    //     .for_each(|(time_save, cheats)| {
    //         println!("{} saving {time_save}", cheats.len());
    //     });

    cheats.filter(|(time_save, _)| *time_save >= 100).count()
}

fn solve_part_2(input: impl Lines) -> usize {
    let grid = day16::parse(&input);
    get_shortcuts(&grid, 2).for_each(|(cheat, time_save)| println!("{cheat:?} saves {time_save}"));

    0
}

fn solve<FN, FS, IN>(start: Node, successors: FN, success: FS) -> Vec<(Node, i64)>
where
    FN: Fn(&Node, i64) -> IN,
    IN: IntoIterator<Item = (Node, i64)>,
    FS: Fn(&Node) -> bool,
{
    let mut visited: HashSet<Node> = HashSet::new();
    let mut pending: Vec<(Node, i64)> = vec![(start, 0)];
    let mut goals = vec![];
    while let Some((node, cost)) = pending.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        if visited.len() % 1_000_000 == 0 {
            println!("visited {}, pending {}", visited.len(), pending.len());
        }
        if success(&node) {
            goals.push((node, cost));
            continue;
        }
        for (next_node, next_cost) in successors(&node, cost) {
            pending.push((next_node, next_cost));
        }
    }

    goals
}

fn get_shortcuts(
    grid: &DenseGrid<Tile>,
    max_length: usize,
) -> impl Iterator<Item = (SimpleCheat, i64)> + use<'_> {
    let start_pos = grid.find(&Tile::Start).exactly_one().ok().unwrap();
    let end_pos = grid.find(&Tile::End).exactly_one().ok().unwrap();
    let (path, _) = find_shortest_path(grid, start_pos, end_pos);
    println!("{path:?}");
    path.clone()
        .into_iter()
        .permutations(2)
        .inspect(|p| println!("{p:?}"))
        .filter_map(move |p| {
            let [a, b] = p.try_into().unwrap();
            let (cost_a, _) = path.iter().find_position(|x| **x == a).unwrap();
            let (cost_b, _) = path.iter().find_position(|x| **x == b).unwrap();
            if cost_b <= cost_a {
                println!("invalid order: {cost_a} {cost_b}");
                return None;
            }
            let dt = (cost_b as i64) - (cost_a as i64);
            println!("dt: {dt}");
            pathfinding::directed::dijkstra::dijkstra(
                &a,
                |p| wall_successors(grid, *p, end_pos),
                |p| *p == b,
            )
            .inspect(|p| println!("{p:?}"))
            .and_then(|(_, cost)| {
                if cost > dt {
                    println!("doesn't save time");
                    None
                } else if cost > max_length as i64 {
                    println!("path is too long {cost}");
                    None
                } else {
                    println!("saves {}", dt - cost);
                    Some((SimpleCheat(a, b), dt - cost))
                }
            })
        })
}

fn wall_successors(grid: &DenseGrid<Tile>, pos: XY, target: XY) -> Vec<(XY, i64)> {
    pos.cardinal_neighbours()
        .filter(|p| *p == target || grid.at(p.as_tuple()) == Some(&Tile::Wall))
        .zip(iter::repeat(1))
        .collect()
}

fn find_shortest_path(grid: &DenseGrid<Tile>, start_pos: XY, end_pos: XY) -> (Vec<XY>, i64) {
    pathfinding::directed::dijkstra::dijkstra(
        &start_pos,
        |p| {
            p.cardinal_neighbours()
                .filter(|p| !matches!(grid.at(p.as_tuple()), None | Some(Tile::Wall)))
                .zip(iter::repeat(1))
        },
        |p| *p == end_pos,
    )
    .unwrap()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    pos: XY,
    cheat: Cheat,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct SimpleCheat(XY, XY);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cheat {
    Available,
    Active { start: XY, remaining: usize },
    Used { start: XY, end: XY },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 0, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 0, TEST_INPUT);
    }
}
