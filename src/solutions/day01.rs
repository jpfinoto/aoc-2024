use crate::aoc::*;
use crate::solution;
use itertools::Itertools;

fn get_number_simple(line: &str) -> u32 {
    let digits = line.chars().filter_map(|c| c.to_digit(10)).collect_vec();

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn solve_part_1(input: impl Lines) -> u32 {
    input.get_lines().map(get_number_simple).sum()
}

solution!(1, solve_part_1);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    #[test]
    fn test_part_1() {
        aoc_test!(
            1,
            1,
            142,
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        );
    }
}
