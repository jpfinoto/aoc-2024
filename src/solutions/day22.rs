use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter;

const DAY: Day = 22;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    parse(&input)
        .map(|secret| SecretNumber { secret }.nth(2000).unwrap())
        .sum()
}

fn solve_part_2(input: impl Lines) -> i64 {
    let best_values: Vec<_> = parse(&input)
        .map(|n| (n, best_value_by_sequence(n, 2000)))
        .collect();

    sequences()
        .par_bridge()
        .map(|sequence| sum_for_sequence(best_values.iter().map(|x| &x.1), &sequence))
        .max()
        .unwrap()
}

fn sum_for_sequence<'a>(
    best_values: impl Iterator<Item = &'a HashMap<[i8; 4], i64>>,
    sequence: &[i8; 4],
) -> i64 {
    best_values
        .map(|best| best.get(sequence).cloned().unwrap_or(0))
        .sum()
}

fn best_value_by_sequence(secret: i64, input_length: usize) -> HashMap<[i8; 4], i64> {
    let numbers: Vec<_> = SecretNumber { secret }
        .map(|n| n % 10)
        .take(input_length)
        .collect();

    numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .tuple_windows()
        .map(|(a, b, c, d)| [a as i8, b as i8, c as i8, d as i8])
        .zip_eq(numbers.iter().skip(4).cloned())
        .into_grouping_map()
        .reduce(|acc, _, _| acc)
}

fn sequences() -> impl Iterator<Item = [i8; 4]> {
    iter::repeat_n(-9..=9, 4)
        .multi_cartesian_product()
        .map(|v| v.try_into().unwrap())
}

fn next_number(mut secret: i64) -> i64 {
    secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    secret = ((secret * 2048) ^ secret) % 16777216;
    secret
}

struct SecretNumber {
    secret: i64,
}

impl Iterator for SecretNumber {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let old_secret = self.secret;
        self.secret = next_number(self.secret);
        Some(old_secret)
    }
}

fn parse(input: &impl Lines) -> impl Iterator<Item = i64> + '_ {
    input
        .get_lines()
        .filter_map(|line| line.parse::<i64>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "1
10
100
2024";

    const TEST_INPUT_2: &str = "1
2
3
2024";

    #[test]
    fn test_secret() {
        let mut secret = SecretNumber { secret: 123 };
        assert_eq!(secret.next(), Some(15887950));
        assert_eq!(secret.next(), Some(16495136));
        assert_eq!(secret.next(), Some(527345));
        assert_eq!(secret.next(), Some(704524));
        assert_eq!(secret.next(), Some(1553684));
        assert_eq!(secret.next(), Some(12683156));
        assert_eq!(secret.next(), Some(11100544));
        assert_eq!(secret.next(), Some(12249484));
        assert_eq!(secret.next(), Some(7753432));
        assert_eq!(secret.next(), Some(5908254));
    }

    #[test]
    fn test_sequences() {
        let values = best_value_by_sequence(123, 10);
        assert_eq!(values[&[-1, -1, 0, 2]], 6);
    }

    #[test]
    fn test_best_value() {
        let sequence = [-2, 1, -1, 3];
        let best_values = [
            best_value_by_sequence(1, 2000),
            best_value_by_sequence(2, 2000),
            best_value_by_sequence(3, 2000),
            best_value_by_sequence(2024, 2000),
        ];

        best_values
            .iter()
            .for_each(|v| println!("{:?}", v.get(&sequence)));
        assert_eq!(sum_for_sequence(best_values.iter(), &sequence), 23);
    }

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 37327623, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 23, TEST_INPUT_2);
    }
}
