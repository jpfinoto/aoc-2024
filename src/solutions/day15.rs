use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DenseGrid, Direction, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP, XY};
use std::collections::HashSet;
use std::fmt::Formatter;

const DAY: Day = 15;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    let (mut grid, moves) = parse(&input, false);
    solve(&mut grid, moves);
    score(&grid)
}

fn solve_part_2(input: impl Lines) -> i64 {
    let (mut grid, moves) = parse(&input, true);
    solve(&mut grid, moves);
    score(&grid)
}

fn solve(grid: &mut DenseGrid<Tile>, moves: Vec<Direction>) {
    let mut robot_position = grid.find(&Tile::Robot).next().unwrap();
    for direction in moves {
        if let Some(actions) = plan_moves(grid, robot_position, direction, &mut HashSet::new()) {
            robot_position = robot_position + direction;
            for action in actions {
                action(grid);
            }
            // println!("// {direction:?}");
            // println!("{grid}");
        }
    }
}

fn score(grid: &DenseGrid<Tile>) -> i64 {
    grid.find(&Tile::Box)
        .chain(grid.find(&Tile::LeftBox))
        .map(|pos| pos.x + 100 * pos.y)
        .sum()
}

type Action = Box<dyn Fn(&mut DenseGrid<Tile>)>;

fn plan_moves(
    grid: &DenseGrid<Tile>,
    position: XY,
    direction: Direction,
    active_set: &mut HashSet<XY>,
) -> Option<Vec<Action>> {
    if active_set.contains(&position) {
        return Some(vec![]);
    }

    let &current_tile = grid.at(position.as_tuple())?;

    match current_tile {
        Tile::Empty => Some(vec![]),
        Tile::Wall => None,
        Tile::Box | Tile::Robot => {
            active_set.insert(position);
            let next_position = position + direction;
            if let Some(mut moves) = plan_moves(grid, next_position, direction, active_set) {
                moves.push(Box::new(move |grid| {
                    grid.set_at(next_position.as_tuple(), current_tile);
                    grid.set_at(position.as_tuple(), Tile::Empty);
                }));
                Some(moves)
            } else {
                None
            }
        }
        Tile::LeftBox | Tile::RightBox => {
            let (other_position, other_tile) = match current_tile {
                Tile::LeftBox => (position + DIR_RIGHT, Tile::RightBox),
                Tile::RightBox => (position + DIR_LEFT, Tile::LeftBox),
                _ => unreachable!(),
            };
            active_set.insert(position);
            active_set.insert(other_position);
            let next_position_1 = position + direction;
            let next_position_2 = other_position + direction;
            let moves_1 = plan_moves(grid, next_position_1, direction, active_set);
            let moves_2 = plan_moves(grid, next_position_2, direction, active_set);
            match (moves_1, moves_2) {
                (Some(mut moves), Some(mut moves2)) => {
                    moves.append(&mut moves2);
                    moves.push(Box::new(move |grid| {
                        grid.set_at(position.as_tuple(), Tile::Empty);
                        grid.set_at(other_position.as_tuple(), Tile::Empty);
                        grid.set_at(next_position_1.as_tuple(), current_tile);
                        grid.set_at(next_position_2.as_tuple(), other_tile);
                    }));
                    Some(moves)
                }
                _ => None,
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Box,
    LeftBox,
    RightBox,
    Robot,
    Empty,
}

fn parse(input: &impl Lines, double: bool) -> (DenseGrid<Tile>, Vec<Direction>) {
    let width = input.get_lines().next().unwrap().len() * if double { 2 } else { 1 };
    let mut iter = input.get_lines();
    let tiles = iter
        .by_ref()
        .take_while(|&l| !l.is_empty())
        .flat_map(str::chars)
        .flat_map(Tile::try_from)
        .flat_map(|tile| {
            if double {
                match tile {
                    Tile::Box => [Some(Tile::LeftBox), Some(Tile::RightBox)],
                    Tile::Robot => [Some(Tile::Robot), Some(Tile::Empty)],
                    Tile::Empty => [Some(Tile::Empty), Some(Tile::Empty)],
                    Tile::Wall => [Some(Tile::Wall), Some(Tile::Wall)],
                    Tile::LeftBox => {
                        unreachable!();
                    }
                    Tile::RightBox => {
                        unreachable!();
                    }
                }
            } else {
                [Some(tile), None]
            }
        })
        .flatten();

    let grid = DenseGrid::from_iter(width, tiles);

    let moves = iter
        .flat_map(str::chars)
        .flat_map(|c| match c {
            '^' => Some(DIR_UP),
            '>' => Some(DIR_RIGHT),
            'v' => Some(DIR_DOWN),
            '<' => Some(DIR_LEFT),
            _ => None,
        })
        .collect();

    (grid, moves)
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            'O' => Ok(Tile::Box),
            '@' => Ok(Tile::Robot),
            '.' => Ok(Tile::Empty),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Box => write!(f, "O"),
            Tile::LeftBox => write!(f, "["),
            Tile::RightBox => write!(f, "]"),
            Tile::Robot => write!(f, "@"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 10092, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 9021, TEST_INPUT);
    }
}
