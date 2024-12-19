use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use regex::Regex;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

const DAY: Day = 19;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let (patterns, designs) = parse(&input);
    let regex = build_regex(&patterns);
    designs.filter(|line| regex.is_match(line)).count()
}

fn solve_part_2(input: impl Lines) -> usize {
    let (patterns, designs) = parse(&input);
    let regex = build_regex(&patterns);
    let mut cache = HashMap::new();

    designs
        .filter(|line| regex.is_match(line))
        .map(|line| combinations(line, &patterns, &mut cache))
        .sum::<usize>()
}

fn combinations(s: &str, options: &HashSet<SmolStr>, cache: &mut HashMap<SmolStr, usize>) -> usize {
    if s.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(s) {
        return count;
    }

    let total = options
        .iter()
        .map(|part| {
            if s.starts_with(part.as_str()) {
                combinations(&s[part.len()..], options, cache)
            } else {
                0
            }
        })
        .sum();

    if total != 0 {
        cache.insert(s.into(), total);
    }

    total
}

fn build_regex(patterns: &HashSet<SmolStr>) -> Regex {
    Regex::new(&format!("^({})+$", patterns.iter().sorted().join("|"))).unwrap()
}

fn parse(input: &impl Lines) -> (HashSet<SmolStr>, impl Iterator<Item = &str>) {
    let mut lines = input.get_lines();
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(SmolStr::from)
        .collect();

    assert!(lines.next().unwrap().trim().is_empty());

    (patterns, lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    const TEST_INPUT_2: &str = "r, b, br

brrrrrrrrrrrrbr";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 6, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 16, TEST_INPUT);
    }

    #[test]
    fn test_part_2b() {
        aoc_test!(DAY, 2, 16, TEST_INPUT_2);
    }
}
