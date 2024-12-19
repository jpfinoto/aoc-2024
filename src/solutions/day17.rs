use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

const DAY: Day = 17;

solution!(DAY, solve_part_1);

fn solve_part_1(input: impl Lines) -> String {
    let mut computer = parse(&input);
    let mut results = vec![];

    loop {
        match step(&mut computer) {
            StepResult::None => {}
            StepResult::Output(i) => {
                results.push(i);
            }
            StepResult::Halt => {
                break;
            }
        }
    }

    results.iter().join(",")
}

#[allow(unused)]
fn solve_part_2(input: impl Lines) -> i64 {
    let mut computer = parse(&input);

    for a in 0.. {
        if a % 1_000_000 == 0 {
            println!("a: {a}");
        }

        computer.state = State {
            a,
            b: 0,
            c: 0,
            ip: 0,
        };

        let mut results = vec![];
        let mut terminated = false;
        for _ in 0..100_000_000 {
            match step(&mut computer) {
                StepResult::None => {}
                StepResult::Output(i) => {
                    results.push(i);
                }
                StepResult::Halt => {
                    terminated = true;
                    break;
                }
            }
        }

        if !terminated {
            println!("iteration limit")
        }

        if results == computer.program {
            return a;
        }
    }

    unreachable!()
}

fn step(computer: &mut Computer) -> StepResult {
    if computer.state.ip >= computer.program.len() {
        return StepResult::Halt;
    }

    match computer.program[computer.state.ip] {
        0 => {
            // adv
            let operand = computer.program[computer.state.ip + 1];
            computer.state.a /= 2i64.pow(combo(operand, &computer.state) as u32);
            computer.state.ip += 2;

            StepResult::None
        }
        1 => {
            // bxl
            let operand = computer.program[computer.state.ip + 1];
            computer.state.b ^= operand;
            computer.state.ip += 2;

            StepResult::None
        }
        2 => {
            // bst
            let operand = computer.program[computer.state.ip + 1];
            computer.state.b = combo(operand, &computer.state) % 8;
            computer.state.ip += 2;

            StepResult::None
        }
        3 => {
            // jnz
            let operand = computer.program[computer.state.ip + 1];
            if computer.state.a != 0 {
                computer.state.ip = operand as usize;
            } else {
                computer.state.ip += 2;
            }

            StepResult::None
        }
        4 => {
            // bxc
            computer.state.b ^= computer.state.c;
            computer.state.ip += 2;

            StepResult::None
        }
        5 => {
            // out
            let operand = computer.program[computer.state.ip + 1];
            computer.state.ip += 2;

            StepResult::Output(combo(operand, &computer.state) % 8)
        }
        6 => {
            // bdv
            let operand = computer.program[computer.state.ip + 1];
            computer.state.b = computer.state.a / 2i64.pow(combo(operand, &computer.state) as u32);
            computer.state.ip += 2;

            StepResult::None
        }
        7 => {
            // cdv
            let operand = computer.program[computer.state.ip + 1];
            computer.state.c = computer.state.a / 2i64.pow(combo(operand, &computer.state) as u32);
            computer.state.ip += 2;

            StepResult::None
        }
        _ => panic!("invalid opcode"),
    }
}

enum StepResult {
    None,
    Output(i64),
    Halt,
}

fn combo(op: i64, state: &State) -> i64 {
    match op {
        0..=3 => op,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("invalid combo"),
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct State {
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug)]
struct Computer {
    state: State,
    program: Vec<i64>,
}

lazy_static! {
    static ref INPUT_REGEX_A: Regex = Regex::new(r"A: (\d+)").unwrap();
    static ref INPUT_REGEX_B: Regex = Regex::new(r"B: (\d+)").unwrap();
    static ref INPUT_REGEX_C: Regex = Regex::new(r"C: (\d+)").unwrap();
    static ref INPUT_REGEX_PROG: Regex = Regex::new(r"Program: ([\d,]+)").unwrap();
}

fn parse(input: &impl Lines) -> Computer {
    let a: i64 = INPUT_REGEX_A
        .captures(input.get_raw())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let b: i64 = INPUT_REGEX_B
        .captures(input.get_raw())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let c: i64 = INPUT_REGEX_C
        .captures(input.get_raw())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let program = INPUT_REGEX_PROG
        .captures(input.get_raw())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    Computer {
        state: State { ip: 0, a, b, c },
        program,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, "4,6,3,5,6,3,5,2,1,0", TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 117440, TEST_INPUT_2);
    }
}
