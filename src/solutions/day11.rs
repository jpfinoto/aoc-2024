use crate::aoc::*;
use crate::solution;
use std::str::FromStr;

const DAY: Day = 11;

solution!(DAY, solve_part_1);

fn solve_part_1(input: impl Lines) -> usize {
    let mut stones = parse(input);

    for _ in 0..25 {
        stones = stones.into_iter().flat_map(step).flatten().collect();
    }

    stones.len()
}

#[allow(unused)]
fn solve_part_2(input: impl Lines) -> usize {
    0
}

fn step(n: u64) -> [Option<u64>; 2] {
    if n == 0 {
        return [Some(1), None];
    }

    let num_digits = n.ilog10() + 1;

    if num_digits % 2 == 0 {
        let first_half = n / 10u64.pow(num_digits / 2);
        let second_half = n - first_half * 10u64.pow(num_digits / 2);

        [Some(first_half), Some(second_half)]
    } else {
        [Some(n * 2024), None]
    }
}

fn parse(input: impl Lines + Sized) -> Vec<u64> {
    input
        .get_raw()
        .split_whitespace()
        .flat_map(u64::from_str)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 55312, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 0, TEST_INPUT);
    }
}
