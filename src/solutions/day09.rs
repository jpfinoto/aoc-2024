use crate::aoc::*;
use crate::solution;
use std::collections::VecDeque;
use std::iter;

const DAY: Day = 9;

solution!(DAY, solve_part_1);

fn solve_part_1(input: impl Lines) -> usize {
    let data = parse(&input);
    let mut files_only: VecDeque<usize> = data.clone().into_iter().flatten().collect();
    let result = data.into_iter().map(|x| match x {
        None => files_only.pop_back(),
        Some(_) => files_only.pop_front(),
    });
    checksum(result.flatten())
}

#[allow(unused)]
fn solve_part_2(input: impl Lines) -> i64 {
    0
}

fn checksum(values: impl Iterator<Item = usize>) -> usize {
    values.enumerate().map(|(i, value)| i * value).sum()
}

fn parse(input: &impl Lines) -> Vec<Option<usize>> {
    input
        .get_raw()
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let length = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                iter::repeat_n(Some(i / 2), length)
            } else {
                iter::repeat_n(None, length)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 1928, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 0, TEST_INPUT);
    }
}
