use crate::aoc::*;
use crate::solution;
use lazy_static::lazy_static;
use regex::Regex;

const DAY: Day = 3;

solution!(DAY, solve_part_1, solve_part_2);

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"(mul)\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)").unwrap();
}

fn solve_part_1(input: impl Lines) -> i64 {
    let input = input.get_raw();
    let caps = MUL_REGEX.captures_iter(input);
    caps.map(|cap| {
        let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        a * b
    })
    .sum()
}

fn solve_part_2(input: impl Lines) -> i64 {
    let input = input.get_raw();
    let caps = INSTRUCTION_REGEX.captures_iter(input);
    let (_, sum) = caps.fold((true, 0), |(enabled, sum), cap| {
        let mut instruction = cap.iter().skip(1).flatten();

        match instruction.next().unwrap().as_str() {
            "do" => (true, sum),
            "don't" => (false, sum),
            "mul" if enabled => {
                let a = instruction.next().unwrap().as_str().parse::<i64>().unwrap();
                let b = instruction.next().unwrap().as_str().parse::<i64>().unwrap();

                (enabled, sum + a * b)
            }
            _ => (enabled, sum),
        }
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 161, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 48, TEST_INPUT_2);
    }
}
