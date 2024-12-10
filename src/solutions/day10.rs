use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DenseGrid, XY};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

const DAY: Day = 10;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let map = parse(&input);
    map.find(&0)
        .par_bridge()
        .map(|pos| find_trails(pos, &map, true))
        .sum()
}

fn solve_part_2(input: impl Lines) -> usize {
    let map = parse(&input);
    map.find(&0)
        .par_bridge()
        .map(|pos| find_trails(pos, &map, false))
        .sum()
}

fn find_trails(start: XY, map: &DenseGrid<u8>, remove_duplicates: bool) -> usize {
    let mut visited = HashSet::new();
    let mut pending = VecDeque::new();
    pending.push_back(start);

    let mut score = 0;

    while !pending.is_empty() {
        let current_pos = pending.pop_front().unwrap();
        if remove_duplicates && visited.contains(&current_pos) {
            continue;
        }
        let current_value = *map.at(current_pos.as_tuple()).unwrap();
        visited.insert(current_pos);
        for p in current_pos.cardinal_neighbours() {
            let neighbour_value = map.at(p.as_tuple()).copied();
            if neighbour_value == Some(current_value + 1) && !visited.contains(&p) {
                pending.push_back(p);
            }
        }
        if current_value == 9 {
            score += 1;
        }
    }

    score
}

fn parse(input: &impl Lines) -> DenseGrid<u8> {
    DenseGrid::from_rows(
        input
            .get_lines()
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 36, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 81, TEST_INPUT);
    }
}
