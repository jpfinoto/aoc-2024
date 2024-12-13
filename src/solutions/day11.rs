use crate::aoc::*;
use crate::solution;
use std::collections::HashMap;
use std::iter::Flatten;
use std::str::FromStr;

const DAY: Day = 11;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let stones = parse(input);
    let mut corridor = CachedStoneCorridor::default();

    stones.into_iter().map(|s| corridor.step(s, 25)).sum()
}

fn solve_part_2(input: impl Lines) -> usize {
    let stones = parse(input);
    let mut corridor = CachedStoneCorridor::default();

    stones.into_iter().map(|s| corridor.step(s, 75)).sum()
}

#[derive(Default)]
struct CachedStoneCorridor {
    cache: HashMap<(u64, usize), usize>,
}

impl CachedStoneCorridor {
    fn step(&mut self, stone: u64, num_steps: usize) -> usize {
        if num_steps == 0 {
            1
        } else if let Some(&cached_result) = self.cache.get(&(stone, num_steps)) {
            cached_result
        } else {
            let result = step(stone)
                .into_iter()
                .map(|s| self.step(s, num_steps - 1))
                .sum();
            self.cache.insert((stone, num_steps), result);
            result
        }
    }
}

enum StepResult {
    OneStone(u64),
    TwoStones(u64, u64),
}

impl IntoIterator for StepResult {
    type Item = u64;
    type IntoIter = Flatten<std::array::IntoIter<Option<u64>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            StepResult::OneStone(a) => [Some(a), None].into_iter().flatten(),
            StepResult::TwoStones(a, b) => [Some(a), Some(b)].into_iter().flatten(),
        }
    }
}

fn step(n: u64) -> StepResult {
    if n == 0 {
        return StepResult::OneStone(1);
    }

    let num_digits = n.ilog10() + 1;

    if num_digits % 2 == 0 {
        let exp = 10u64.pow(num_digits / 2);
        let first_half = n / exp;
        let second_half = n - first_half * exp;

        StepResult::TwoStones(first_half, second_half)
    } else {
        StepResult::OneStone(n * 2024)
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
        aoc_test!(DAY, 2, 55312, TEST_INPUT);
    }
}
