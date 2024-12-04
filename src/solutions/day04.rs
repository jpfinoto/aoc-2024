use crate::aoc::*;
use crate::solution;
use itertools::Itertools;
use std::collections::HashMap;

const DAY: Day = 4;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> usize {
    let lines = get_lines(&input);
    lines
        .iter()
        .map(|l| l.to_string())
        .chain(transpose(&lines))
        .chain(diagonals(&lines))
        .map(|l| count_with_overlap("XMAS", l.as_str()) + count_with_overlap("SAMX", l.as_str()))
        .sum::<usize>()
}

fn solve_part_2(input: impl Lines) -> usize {
    let grid = Grid::from_lines(&get_lines(&input));
    let masks = [
        Mask::from_pattern("M.S\n.A.\nM.S", 3, 3),
        Mask::from_pattern("S.M\n.A.\nS.M", 3, 3),
        Mask::from_pattern("M.M\n.A.\nS.S", 3, 3),
        Mask::from_pattern("S.S\n.A.\nM.M", 3, 3),
    ];

    masks
        .iter()
        .map(|mask| {
            (0..grid.width)
                .cartesian_product(0..grid.height)
                .filter(|p| grid.matches(mask, (*p).into()))
                .count()
        })
        .sum()
}

fn get_lines(input: &impl Lines) -> Vec<&str> {
    input.get_lines().filter(|l| !l.is_empty()).collect()
}

fn diagonals(lines: &[&str]) -> Vec<String> {
    let width = lines[0].len() as i32;

    assert_eq!(width, lines.len() as i32);

    (-(width - 1)..width)
        .map(move |start_x| {
            (0..width)
                .filter(move |t| start_x + t >= 0 && start_x + t < width)
                .map(|t| {
                    lines[t as usize]
                        .chars()
                        .nth((start_x + t) as usize)
                        .unwrap()
                })
                .join("")
        })
        .chain((0..(width * 2 - 1)).map(move |start_y| {
            (0..width)
                .filter(move |t| start_y - t >= 0 && start_y - t < width)
                .map(|t| {
                    lines[(start_y - t) as usize]
                        .chars()
                        .nth(t as usize)
                        .unwrap()
                })
                .join("")
        }))
        .collect()
}

fn transpose(lines: &[&str]) -> Vec<String> {
    let width = lines[0].len() as i32;

    assert_eq!(width, lines.len() as i32);

    (0..width)
        .map(|x| {
            (0..width)
                .map(|y| lines[y as usize].chars().nth(x as usize).unwrap())
                .join("")
        })
        .collect()
}

fn count_with_overlap(needle: &str, haystack: &str) -> usize {
    (0..haystack.len())
        .filter(|&i| haystack[i..].starts_with(needle))
        .count()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct XY {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for XY {
    fn from((x, y): (i32, i32)) -> Self {
        XY { x, y }
    }
}

impl From<(usize, usize)> for XY {
    fn from((x, y): (usize, usize)) -> Self {
        XY {
            x: x as i32,
            y: y as i32,
        }
    }
}

/// A rectangle where 0,0 is on the top left.
/// The cells are all relative to that coordinate
#[derive(Debug, Clone, Eq, PartialEq)]
struct Mask {
    width: usize,
    height: usize,
    cells: HashMap<XY, char>,
}

impl Mask {
    fn from_pattern(pattern: &str, width: usize, height: usize) -> Mask {
        let cells = pattern
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(y, c)| {
                        (
                            XY {
                                x: x as i32,
                                y: y as i32,
                            },
                            c,
                        )
                    })
            })
            .collect();

        Mask {
            width,
            height,
            cells,
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: HashMap<XY, char>,
}

impl Grid {
    fn matches(&self, mask: &Mask, top_left: XY) -> bool {
        (0..(mask.width as i32))
            .cartesian_product(0..(mask.height as i32))
            .all(|(x, y)| {
                let grid_pos = XY {
                    x: x + top_left.x,
                    y: y + top_left.y,
                };
                match (mask.cells.get(&(x, y).into()), self.grid.get(&grid_pos)) {
                    (Some(mask), Some(value)) => mask == value,
                    (None, _) => true,
                    _ => false,
                }
            })
    }

    fn from_lines(lines: &[&str]) -> Grid {
        Grid {
            width: lines.iter().map(|l| l.len()).max().unwrap_or(0),
            height: lines.len(),
            grid: lines
                .iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().map(move |(x, c)| {
                        (
                            XY {
                                x: x as i32,
                                y: y as i32,
                            },
                            c,
                        )
                    })
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;
    use itertools::Itertools;

    const TEST_INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    const TEST_INPUT_DIAGONALS: &str = "123\n456\n789";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 18, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 9, TEST_INPUT);
    }

    #[test]
    fn test_diagonals() {
        let mut diags =
            diagonals(&TEST_INPUT_DIAGONALS.lines().map(|s| s.trim()).collect_vec()).into_iter();
        assert_eq!(diags.next(), Some("7".to_owned()));
        assert_eq!(diags.next(), Some("48".to_owned()));
        assert_eq!(diags.next(), Some("159".to_owned()));
        assert_eq!(diags.next(), Some("26".to_owned()));
        assert_eq!(diags.next(), Some("3".to_owned()));
        assert_eq!(diags.next(), Some("1".to_owned()));
        assert_eq!(diags.next(), Some("42".to_owned()));
        assert_eq!(diags.next(), Some("753".to_owned()));
        assert_eq!(diags.next(), Some("86".to_owned()));
        assert_eq!(diags.next(), Some("9".to_owned()));
        assert_eq!(diags.next(), None);
    }
}
