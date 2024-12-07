use crate::aoc::*;
use crate::solution;
use itertools::Itertools;

const DAY: Day = 7;

solution!(DAY, solve_part_1, solve_part_2);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operation {
    Sum,
    Multiply,
    Combine,
}

fn solve_part_1(input: impl Lines) -> i64 {
    input
        .get_lines()
        .flat_map(parse_line)
        .filter_map(check_line(&[Operation::Sum, Operation::Multiply]))
        .sum()
}

fn solve_part_2(input: impl Lines) -> i64 {
    input
        .get_lines()
        .flat_map(parse_line)
        .filter_map(check_line(&[
            Operation::Sum,
            Operation::Multiply,
            Operation::Combine,
        ]))
        .sum()
}

fn check_line(options: &[Operation]) -> impl Fn((i64, Vec<i64>)) -> Option<i64> + use<'_> {
    |(result, inputs)| {
        if find_operators(result, &inputs, options).next().is_some() {
            Some(result)
        } else {
            None
        }
    }
}

fn find_operators<'a, 'b>(
    result: i64,
    inputs: &'a [i64],
    options: &'b [Operation],
) -> impl Iterator<Item = Vec<Operation>> + use<'a, 'b> {
    (0..inputs.len() - 1)
        .map(|_| options.iter())
        .multi_cartesian_product()
        .filter_map(move |operations| {
            let output = inputs.iter().skip(1).zip(operations.iter()).fold(
                inputs[0],
                |acc, (n, op)| match op {
                    Operation::Sum => acc + n,
                    Operation::Multiply => acc * n,
                    Operation::Combine => acc * (10i64.pow(n.ilog10() + 1)) + n,
                },
            );
            if output == result {
                Some(operations.into_iter().cloned().collect())
            } else {
                None
            }
        })
}

fn parse_line(line: &str) -> Option<(i64, Vec<i64>)> {
    let (result, inputs) = line.split(':').next_tuple()?;
    Some((
        result.parse::<i64>().ok()?,
        inputs
            .split_whitespace()
            .flat_map(str::parse::<i64>)
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 3749, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 11387, TEST_INPUT);
    }
}
