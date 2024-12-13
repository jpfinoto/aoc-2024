use crate::aoc::*;
use crate::solution;
use crate::utils::grid::XY;
use lazy_static::lazy_static;
use regex::Regex;

const DAY: Day = 13;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    parse(&input)
        .filter_map(get_steps)
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn solve_part_2(input: impl Lines) -> usize {
    parse(&input)
        .map(|machine| ClawMachine {
            prize: machine.prize
                + XY {
                    x: 10000000000000,
                    y: 10000000000000,
                },
            ..machine
        })
        .filter_map(get_steps)
        .map(|(a, b)| a * 3 + b)
        .sum()
}

struct ClawMachine {
    button_a: XY,
    button_b: XY,
    prize: XY,
}

// a * ax + b * bx = px
// a * ay + b * by = py
//
// b = (py - a * ay) / by
// a * ax + bx/by (py - a * ay) = px
// a (ax - ay bx/by) = px - py bx/by
// a = (px - py (bx/by)) / (ax - ay bx/by)
// a = ((by px - py bx) / by) / ((ax by - ay bx) / by)
// a = (by px - py bx) / (ax by - ay bx)

fn get_steps(machine: ClawMachine) -> Option<(usize, usize)> {
    let ax = machine.button_a.x;
    let ay = machine.button_a.y;
    let bx = machine.button_b.x;
    let by = machine.button_b.y;
    let px = machine.prize.x;
    let py = machine.prize.y;

    let a_nom = by * px - py * bx;
    let a_den = ax * by - ay * bx;
    if a_nom % a_den != 0 {
        return None;
    }

    let a = a_nom / a_den;
    let b_nom = py - a * ay;

    if b_nom % by != 0 {
        return None;
    }

    let b = (py - a * ay) / by;

    Some((a as usize, b as usize))
}

lazy_static! {
    static ref CLAW_REGEX: Regex = Regex::new(
        r".*X\+(?<ax>\d+), Y\+(?<ay>\d+)\n.*X\+(?<bx>\d+), Y\+(?<by>\d+)\n.*X=(?<px>\d+), Y=(?<py>\d+)"
    ).unwrap();
}

fn parse<T: Lines>(input: &T) -> impl Iterator<Item = ClawMachine> + use<'_, T> {
    CLAW_REGEX
        .captures_iter(input.get_raw())
        .map(|cap| ClawMachine {
            button_a: XY {
                x: cap.name("ax").unwrap().as_str().parse().unwrap(),
                y: cap.name("ay").unwrap().as_str().parse().unwrap(),
            },
            button_b: XY {
                x: cap.name("bx").unwrap().as_str().parse().unwrap(),
                y: cap.name("by").unwrap().as_str().parse().unwrap(),
            },
            prize: XY {
                x: cap.name("px").unwrap().as_str().parse().unwrap(),
                y: cap.name("py").unwrap().as_str().parse().unwrap(),
            },
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 480, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 875318608908usize, TEST_INPUT);
    }
}
