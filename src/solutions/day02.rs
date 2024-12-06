use crate::aoc::*;
use crate::solution;
use itertools::Itertools;

const DAY: Day = 2;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let levels = get_levels(input);
    levels.iter().map(diff).filter(is_safe).count()
}

fn solve_part_2(input: impl Lines) -> usize {
    let levels = get_levels(input);
    levels
        .iter()
        .filter(|l| is_safe(&diff(l)) || (0..l.len()).any(|i| is_safe(&diff(&exclude(l, i)))))
        .count()
}

fn is_safe(diffs: &Vec<i64>) -> bool {
    diffs.iter().all(|&v| (1..=3).contains(&v)) || diffs.iter().all(|&v| (-3..=-1).contains(&v))
}

fn diff(values: &Vec<i64>) -> Vec<i64> {
    values.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn get_levels(input: impl Lines) -> Vec<Vec<i64>> {
    input
        .get_lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect()
}

fn exclude(values: &[i64], index: usize) -> Vec<i64> {
    values
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, e)| *e)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 2, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 4, TEST_INPUT);
    }
}
