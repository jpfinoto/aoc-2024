use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use std::collections::HashMap;

const DAY: Day = 1;

solution!(DAY, solve_part_1, solve_part_2);

fn read_columns(input: impl Lines) -> (Vec<i32>, Vec<i32>) {
    let (a, b) = input
        .get_lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .next_tuple()
        })
        .unzip();

    (a, b)
}

fn solve_part_1(_input: impl Lines) -> i32 {
    let (a, b) = read_columns(_input);
    a.iter()
        .sorted()
        .zip_eq(b.iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn solve_part_2(_input: impl Lines) -> i32 {
    let (a, b) = read_columns(_input);
    let counts = b.iter().fold(HashMap::new(), |mut counts, w| {
        counts.entry(*w).and_modify(|v| *v += 1).or_insert(1);
        counts
    });
    a.iter().map(|v| v * counts.get(v).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 11, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 31, TEST_INPUT);
    }
}
