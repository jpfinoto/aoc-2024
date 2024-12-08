use crate::aoc::*;
use crate::solution;
use crate::utils::grid::XY;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

const DAY: Day = 8;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    solve(&input, 1, 1)
}

fn solve_part_2(input: impl Lines) -> i64 {
    // this is inefficient, but I don't like the idea of needing a max resonance, 
    // so we'll keep trying until the result doesn't change
    (50..)
        .fold_while(0, |acc, i| {
            let count = solve(&input, 0, i);
            if count > acc {
                Continue(count)
            } else {
                Done(count)
            }
        })
        .into_inner()
}

fn solve(input: &impl Lines, min_resonance: usize, max_resonance: usize) -> i64 {
    let w = input.get_lines().next().unwrap().len() as i64;
    let h = input.get_lines().count() as i64;
    let antennas = parse(input);
    antennas
        .iter()
        .flat_map(|(_, points)| {
            points.iter().tuple_combinations().flat_map(|(a, b)| {
                let dl = *b - *a;
                (min_resonance..=max_resonance)
                    .flat_map(move |i| [*a - dl * i as i64, *b + dl * i as i64])
            })
        })
        .filter(|p| p.x >= 0 && p.x < w && p.y >= 0 && p.y < h)
        .unique()
        .count() as i64
}

fn parse(input: &impl Lines) -> HashMap<char, Vec<XY>> {
    input
        .get_lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '.' => None,
                c => Some((
                    c,
                    XY {
                        x: x as i64,
                        y: y as i64,
                    },
                )),
            })
        })
        .into_grouping_map()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 14, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 34, TEST_INPUT);
    }
}
