use crate::aoc::*;
use crate::solution;
use crate::utils::grid::{DenseGrid, XY};
use rayon::prelude::*;
use std::collections::HashSet;

const DAY: Day = 12;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    find_regions(&parse(&input))
        .map(|(c, area)| (c, area.len() as i64, calc_perimeter(&area)))
        // .inspect(|(id, area, perimeter)| println!("{id}: area {area}, perimeter {perimeter}"))
        .map(|(_, area, perimeter)| area * perimeter)
        .sum()
}

fn solve_part_2(input: impl Lines) -> i64 {
    find_regions(&parse(&input))
        .map(|(c, area)| (c, area.len() as i64, calc_sides(&area)))
        // .inspect(|(id, area, sides)| println!("{id}: area {area}, sides {sides}"))
        .map(|(_, area, sides)| area * sides)
        .sum()
}

fn find_regions(grid: &DenseGrid<char>) -> impl Iterator<Item = (char, HashSet<XY>)> + use<'_> {
    let mut visited: HashSet<XY> = HashSet::new();

    grid.items().filter_map(move |(p, &id)| {
        if visited.contains(&p) {
            None
        } else {
            let area = grid.flood_fill(p);
            visited.extend(area.iter());
            Some((id, area))
        }
    })
}

fn calc_perimeter(area: &HashSet<XY>) -> i64 {
    area.iter()
        .par_bridge()
        .map(|p| {
            p.cardinal_neighbours()
                .filter(|neighbour_pos| !area.contains(neighbour_pos))
                .count() as i64
        })
        .sum()
}

/// Counts how many sides a set of grid cells has.
///
/// This is done by looking at the corner neighbours and checking how they are arranged.
/// These are the only arrangements that are corners:
///
/// ```
/// XO   XO   XX
/// OO   OX   XO
/// ```
///
/// In this example, we're considering the bottom-right neighbour of the top left square.
fn calc_sides(area: &HashSet<XY>) -> i64 {
    area.iter()
        .par_bridge()
        .map(|p| {
            p.corner_neighbours()
                .map(|np| {
                    let d = *p - np;
                    match (
                        area.contains(&XY {
                            x: np.x + d.x,
                            y: np.y,
                        }),
                        area.contains(&np),
                        area.contains(&XY {
                            x: np.x,
                            y: np.y + d.y,
                        }),
                    ) {
                        (true, false, true) => 1,
                        (false, true, false) => 1,
                        (false, false, false) => 1,
                        _ => 0,
                    }
                })
                .sum::<i64>()
        })
        .sum()
}

fn parse(input: &impl Lines) -> DenseGrid<char> {
    let width = input.get_lines().next().unwrap().len();
    DenseGrid::from_iter(width, input.get_lines().flat_map(|line| line.chars()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_INPUT_2: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST_INPUT_3: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 1930, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 236, TEST_INPUT_2);
    }

    #[test]
    fn test_part_2_3() {
        aoc_test!(DAY, 2, 368, TEST_INPUT_3);
    }

    #[test]
    fn test_simple() {
        aoc_test!(DAY, 2, 4, "X");
        aoc_test!(DAY, 2, 8, "XX");
        aoc_test!(DAY, 2, 1 * 4 + 3 * 6, "OX\nXX");
        aoc_test!(DAY, 2, 8 * 8 + 1 * 4, "XXX\nXOX\nXXX");
        aoc_test!(DAY, 2, 2 * 4 + 2 * 4 + 2 * 4, "XOX\nXOX");
    }
}
