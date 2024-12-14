use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DenseGrid, XY};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::iter;

const DAY: Day = 14;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let w = 101;
    let h = 103;
    let steps = 100;

    solve(input.get_raw(), w, h, steps)
}

fn solve_part_2(input: impl Lines) -> usize {
    let w = 101;
    let h = 103;
    let should_print = false;

    let robots = parse(input.get_raw()).collect_vec();
    let min_inertia = 500000; // LOL, LMAO even

    for steps in 0.. {
        let mut positions = HashSet::new();
        for robot in &robots {
            let pos = evolve_robot(robot, w, h, steps).p;
            positions.insert(pos);
        }

        let current_inertia = inertia(&positions);

        if current_inertia < min_inertia {
            if should_print {
                let mut grid = DenseGrid::from_iter(w, iter::repeat_n('.', w * h));
                for pos in positions {
                    grid.try_set_at(pos.as_tuple(), 'X').unwrap();
                }
                println!("{grid}");
                println!("{steps}, inertia: {current_inertia}");
            }

            return steps;
        }
    }

    unreachable!()
}

fn inertia(positions: &HashSet<XY>) -> i64 {
    let pos_sum = positions.iter().cloned().reduce(|acc, p| acc + p).unwrap();
    let center_of_mass = XY {
        x: pos_sum.x / (positions.len() as i64),
        y: pos_sum.y / (positions.len() as i64),
    };
    positions
        .iter()
        .map(|p| (center_of_mass - *p).length_sq())
        .sum()
}

fn solve(input: &str, w: usize, h: usize, steps: usize) -> usize {
    parse(input)
        .map(|robot| evolve_robot(&robot, w, h, steps))
        .flat_map(|robot| get_quadrant(&robot.p, w, h))
        .zip(iter::repeat(1))
        .into_grouping_map()
        .sum()
        .values()
        .product()
}

fn evolve_robot(robot: &Robot, width: usize, height: usize, steps: usize) -> Robot {
    let new_pos = robot.p + robot.v * (steps as i64);
    Robot {
        p: XY {
            x: wrap(new_pos.x, width as i64),
            y: wrap(new_pos.y, height as i64),
        },
        v: robot.v,
    }
}

fn get_quadrant(p: &XY, width: usize, height: usize) -> Option<usize> {
    assert_eq!(width % 2, 1);
    assert_eq!(height % 2, 1);
    let middle_x = (width / 2) as i64;
    let middle_y = (height / 2) as i64;
    if p.x < middle_x && p.y < middle_y {
        Some(0)
    } else if p.x > middle_x && p.y < middle_y {
        Some(1)
    } else if p.x < middle_x && p.y > middle_y {
        Some(2)
    } else if p.x > middle_x && p.y > middle_y {
        Some(3)
    } else {
        None
    }
}

fn wrap(value: i64, bound: i64) -> i64 {
    if value < 0 {
        (value % bound + bound) % bound
    } else {
        value % bound
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

struct Robot {
    p: XY,
    v: XY,
}

fn parse(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    LINE_REGEX.captures_iter(input).map(|cap| Robot {
        p: XY {
            x: cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            y: cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        },
        v: XY {
            x: cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            y: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_1() {
        assert_eq!(12, solve(TEST_INPUT, 11, 7, 100));
    }
}
