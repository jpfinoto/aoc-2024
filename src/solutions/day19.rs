use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

const DAY: Day = 19;

solution!(DAY, solve_part_1);

fn solve_part_1(input: impl Lines) -> usize {
    let (patterns, designs) = parse(&input);
    let regex = Regex::new(&format!("^({})+$", patterns.join("|"))).unwrap();
    designs.filter(|line| regex.is_match(line)).count()
}

fn solve_part_2(input: impl Lines) -> usize {
    let (patterns, designs) = parse(&input);
    let mut combinations = HashMap::new();
    for pattern in patterns.into_iter().sorted_by_key(String::len) {
        let n = combinations_for_string(&pattern, &mut combinations).unwrap_or(0) + 1;
        combinations.insert(pattern.clone(), n);
    }

    println!("{:?}", combinations);

    designs
        .filter_map(|s| combinations_for_string(s, &mut combinations))
        .sum()
}

fn combinations_for_string(s: &str, blocks: &mut HashMap<String, usize>) -> Option<usize> {
    if let Some(count) = blocks.get(s) {
        return Some(*count);
    }

    let mut head = "";
    let mut tail = s;
    let mut count = 1;
    'outer: while !tail.is_empty() {
        if !head.is_empty() {
            blocks.insert(head.to_string(), count);
        }
        let blocks_sorted = blocks.keys().sorted_by_key(|k| -(k.len() as i32));
        for block in blocks_sorted {
            if tail.starts_with(block) {
                count *= blocks[block];
                head = &s[..head.len() + block.len()];
                tail = &tail[block.len()..];
                continue 'outer;
            }
        }
        return None;
    }
    blocks.insert(s.to_string(), count);
    Some(count)
}

fn parse(input: &impl Lines) -> (Vec<String>, impl Iterator<Item = &str>) {
    let mut lines = input.get_lines();
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
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

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 6, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 0, TEST_INPUT);
    }
}
