use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashSet;

const DAY: Day = 18;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    solve(71, 71, &parse(input.get_raw())[0..1024]).unwrap().1
}

fn solve_part_2(input: impl Lines) -> String {
    let falling_bytes = parse(input.get_raw());
    if let Some(p) = get_unreachable(71, 71, &falling_bytes) {
        return format!("{},{}", p.x, p.y);
    }
    unreachable!()
}

fn get_unreachable(width: usize, height: usize, falling_bytes: &[XY]) -> Option<XY> {
    let mut lower = 0;
    let mut upper = falling_bytes.len() - 1;
    // binary search
    loop {
        let i = (lower + upper) / 2;
        if solve(width, height, &falling_bytes[0..i]).is_none() {
            upper = i;
        } else {
            lower = i;
        }

        if (lower == upper) || (lower == upper - 1) {
            return Some(falling_bytes[lower]);
        }
    }
}

fn solve(width: usize, height: usize, fallen_bytes: &[XY]) -> Option<(Vec<XY>, i64)> {
    let target_p = XY {
        x: (width - 1) as i64,
        y: (height - 1) as i64,
    };
    let obstacles = HashSet::from_iter(fallen_bytes.iter().copied());
    astar(
        &XY { x: 0, y: 0 },
        |p| successors(*p, width, height, &obstacles),
        |p| (*p - target_p).taxicab_length(),
        |p| *p == target_p,
    )
}

fn successors(p: XY, width: usize, height: usize, obstacles: &HashSet<XY>) -> Vec<(XY, i64)> {
    [DIR_UP, DIR_DOWN, DIR_LEFT, DIR_RIGHT]
        .iter()
        .filter_map(|d| {
            let next_p = p + *d;
            if (0..width as i64).contains(&next_p.x)
                && (0..height as i64).contains(&next_p.y)
                && !obstacles.contains(&next_p)
            {
                Some((next_p, 1))
            } else {
                None
            }
        })
        .collect()
}

fn parse(input: &str) -> Vec<XY> {
    input
        .lines()
        .filter_map(|l| {
            l.trim()
                .split(",")
                .flat_map(|s| s.parse::<i64>())
                .next_tuple()
                .map(|(x, y)| XY { x, y })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "

5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(7, 7, &parse(TEST_INPUT)[0..12]).unwrap().1, 22);
    }

    #[test]
    fn test_part_2() {
        let input = parse(TEST_INPUT);
        assert_eq!(get_unreachable(7, 7, &input), Some(XY { x: 6, y: 1 }));
    }
}
