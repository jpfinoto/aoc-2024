use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

const DAY: Day = 5;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    let (priority, updates) = parse(input);

    updates
        .iter()
        .filter_map(|v| {
            let sorted = sort(v, &priority);
            if *v == sorted {
                Some(sorted)
            } else {
                None
            }
        })
        .map(|v| v[v.len() / 2] as i64)
        .sum()
}

fn solve_part_2(input: impl Lines) -> i64 {
    let (priority, updates) = parse(input);

    updates
        .iter()
        .filter_map(|v| {
            let sorted = sort(v, &priority);
            if *v != sorted {
                Some(sorted)
            } else {
                None
            }
        })
        .map(|v| v[v.len() / 2] as i64)
        .sum()
}

fn sort(line: &[u8], priority: &HashSet<(u8, u8)>) -> Vec<u8> {
    let mut line = line.to_vec();
    line.sort_by(|a, b| {
        if priority.contains(&(*a, *b)) {
            Ordering::Less
        } else if priority.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    line
}

fn parse(input: impl Lines) -> (HashSet<(u8, u8)>, Vec<Vec<u8>>) {
    let mut lines = input.get_lines();
    let priority = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.split('|')
                .map(|s| s.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let updates = lines
        .map(|s| s.split(',').map(|s| s.parse::<u8>().unwrap()).collect())
        .collect();

    (priority, updates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 143, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 123, TEST_INPUT);
    }
}
