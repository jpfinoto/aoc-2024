use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DenseGrid, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Formatter, Write};

const DAY: Day = 6;

solution!(DAY, solve_part_1, solve_part_2);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Obstacle,
    Guard { facing: XY },
    Visited,
}

fn solve_part_1(input: impl Lines) -> i64 {
    let mut grid = parse(input);
    step_until_outside(&mut grid);
    grid.find(&Cell::Visited).count() as i64
}

fn step_until_outside(grid: &mut DenseGrid<Cell>) {
    let mut pos = grid.find(&Cell::Guard { facing: DIR_UP }).next().unwrap();
    while let Some(new_pos) = step_guard(grid, pos) {
        pos = new_pos;
    }
}

fn solve_part_2(input: impl Lines) -> i64 {
    let grid = parse(input);
    let visited_tiles: HashSet<XY> = {
        let mut grid = grid.clone();
        step_until_outside(&mut grid);
        grid.find(&Cell::Visited).collect()
    };
    (0..grid.width())
        .cartesian_product(0..grid.height())
        .filter(|&(x, y)| visited_tiles.contains(&XY { x, y }))
        .filter_map(|pos| {
            if let Some(Cell::Empty) = grid.at(pos) {
                let mut grid_mod = grid.clone();
                grid_mod.try_set_at(pos, Cell::Obstacle).unwrap();
                Some(grid_mod)
            } else {
                None
            }
        })
        .filter_map(|mut grid| {
            let mut pos = grid.find(&Cell::Guard { facing: DIR_UP }).next().unwrap();
            let mut total_steps = 0;
            while let Some(new_pos) = step_guard(&mut grid, pos) {
                pos = new_pos;
                total_steps += 1;
                // this magic number seems to do the trick, but it's a terrible solution
                if total_steps > 10000 {
                    return Some(());
                }
            }
            None
        })
        .count() as i64
}

fn step_guard(grid: &mut DenseGrid<Cell>, current_guard_pos: XY) -> Option<XY> {
    let Some(Cell::Guard { facing }) = grid.at(current_guard_pos.as_tuple()) else {
        return None;
    };

    let next_pos = current_guard_pos + *facing;
    match grid.at(next_pos.as_tuple()) {
        Some(Cell::Obstacle) => {
            grid.try_set_at(
                current_guard_pos.as_tuple(),
                Cell::Guard {
                    facing: facing.rotate_90_cw(),
                },
            )
            .unwrap();
            Some(current_guard_pos)
        }
        _ => {
            grid.try_set_at(next_pos.as_tuple(), Cell::Guard { facing: *facing });
            grid.try_set_at(current_guard_pos.as_tuple(), Cell::Visited)
                .unwrap();
            Some(next_pos)
        }
    }
}

fn parse(input: impl Lines) -> DenseGrid<Cell> {
    DenseGrid::from_rows(
        input
            .get_lines()
            .map(|l| l.chars().flat_map(Cell::try_from).collect())
            .collect(),
    )
}

impl TryFrom<char> for Cell {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Obstacle),
            '^' => Ok(Cell::Guard { facing: DIR_UP }),
            _ => Err("Invalid character"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Empty => '.',
            Cell::Obstacle => '#',
            Cell::Visited => 'X',
            Cell::Guard { facing } if facing == &DIR_UP => '^',
            Cell::Guard { facing } if facing == &DIR_RIGHT => '>',
            Cell::Guard { facing } if facing == &DIR_DOWN => 'v',
            Cell::Guard { facing } if facing == &DIR_LEFT => '<',
            _ => '?',
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 41, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 6, TEST_INPUT);
    }
}
