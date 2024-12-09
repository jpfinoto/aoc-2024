use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use std::collections::{LinkedList, VecDeque};
use std::iter;

const DAY: Day = 9;

solution!(DAY, solve_part_1, solve_part_2);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct File {
    id: usize,
    length: usize,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Block {
    Empty(usize),
    File(File),
}

fn solve_part_1(input: impl Lines) -> usize {
    let data = parse(&input);
    let mut files_only: VecDeque<usize> = data.clone().into_iter().flatten().collect();
    let result = data.into_iter().map(|x| match x {
        None => files_only.pop_back(),
        Some(_) => files_only.pop_front(),
    });
    checksum(result)
}

fn solve_part_2(input: impl Lines) -> usize {
    let mut blocks = parse_2(&input);
    let mut files_only: Vec<_> = blocks
        .clone()
        .into_iter()
        .flat_map(|block| match block {
            Block::Empty(_) => None,
            Block::File(file) => Some(file),
        })
        .rev()
        .collect();
    for file in &mut files_only {
        let position = blocks
            .iter()
            .find_position(|x| **x == Block::File(*file))
            .unwrap()
            .0;
        let mut tail = blocks.split_off(position);
        tail.pop_front();
        blocks.extend(iter::once(Block::Empty(file.length)));
        blocks.append(&mut tail);
        let mut insert_pos = None;
        for (i, block) in blocks.iter().enumerate() {
            match block {
                Block::Empty(length) if *length >= file.length => {
                    insert_pos = Some(i);
                    break;
                }
                _ => {}
            }
        }
        let insert_pos = insert_pos.unwrap();
        let mut tail = blocks.split_off(insert_pos);
        let Block::Empty(empty_block_length) = tail.pop_front().unwrap() else {
            panic!("Block isn't empty?")
        };
        blocks.extend(iter::once(Block::File(*file)));
        if empty_block_length > file.length {
            blocks.extend(iter::once(Block::Empty(empty_block_length - file.length)));
        }
        blocks.append(&mut tail);
    }
    checksum(blocks.iter().flat_map(|block| match block {
        Block::Empty(length) => iter::repeat_n(None, *length),
        Block::File(file) => iter::repeat_n(Some(file.id), file.length),
    }))
}

fn checksum(values: impl Iterator<Item = Option<usize>>) -> usize {
    values
        .enumerate()
        .filter_map(|(i, value)| value.map(|v| v * i))
        .sum()
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

fn parse_2(input: &impl Lines) -> LinkedList<Block> {
    input
        .get_raw()
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let length = c.to_digit(10).unwrap() as usize;
            if length > 0 {
                if i % 2 == 0 {
                    Some(Block::File(File { id: i / 2, length }))
                } else {
                    Some(Block::Empty(length))
                }
            } else {
                None
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
        aoc_test!(DAY, 2, 2858, TEST_INPUT);
    }
}
