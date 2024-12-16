use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DenseGrid, Direction, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
use itertools::Itertools;

const DAY: Day = 16;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    let map = parse(&input);
    let start_position = map.find(&Tile::Start).next().unwrap();
    let end_position = map.find(&Tile::End).next().unwrap();
    let (_, cost) = pathfinding::directed::astar::astar(
        &Node {
            position: start_position,
            facing: DIR_RIGHT,
        },
        |node| successors(node, &map),
        |node| (node.position - end_position).taxicab_length(),
        |node| node.position == end_position,
    )
    .unwrap();

    cost
}

fn solve_part_2(input: impl Lines) -> usize {
    let map = parse(&input);
    let start_position = map.find(&Tile::Start).next().unwrap();
    let end_position = map.find(&Tile::End).next().unwrap();
    let (solution, _) = pathfinding::directed::astar::astar_bag(
        &Node {
            position: start_position,
            facing: DIR_RIGHT,
        },
        |node| successors(node, &map),
        |node| (node.position - end_position).taxicab_length(),
        |node| node.position == end_position,
    )
    .unwrap();

    solution
        .into_iter()
        .flatten()
        .map(|node| node.position)
        .unique()
        .count()
}

fn successors(node: &Node, map: &DenseGrid<Tile>) -> Vec<(Node, i64)> {
    // you can always turn around in place
    let mut result = [DIR_UP, DIR_DOWN, DIR_LEFT, DIR_RIGHT]
        .into_iter()
        .filter(|dir| dir != &node.facing)
        .map(|dir| {
            (
                Node {
                    position: node.position,
                    facing: dir,
                },
                1000,
            )
        })
        .collect_vec();

    // try to move forwards
    if map.at((node.position + node.facing).as_tuple()) != Some(&Tile::Wall) {
        result.push((
            Node {
                position: node.position + node.facing,
                facing: node.facing,
            },
            1, // with a cost of 1
        ))
    };

    result
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    position: XY,
    facing: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Start,
    End,
    Empty,
}

fn parse(input: &impl Lines) -> DenseGrid<Tile> {
    let width = input.get_lines().next().unwrap().len();
    DenseGrid::from_iter(
        width,
        input
            .get_lines()
            .flat_map(str::chars)
            .flat_map(Tile::try_from),
    )
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Empty),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 7036, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 45, TEST_INPUT);
    }
}
