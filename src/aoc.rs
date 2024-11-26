use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub type Day = usize;
pub type Part = usize;
pub type SolverMap = HashMap<(Day, Part), Box<dyn Fn(&PuzzleInput<'_>) -> Option<String>>>;

#[macro_export]
macro_rules! solution {
    ($day:literal) => {
        use crate::aoc::{Lines, PuzzleInput};
        use std::fmt::{Debug, Display};

        impl<'a> Solver<$day, 1> for PuzzleInput<'a> {
            fn solve(&self) -> Option<impl Display + Debug> {
                None as Option<String>
            }
        }
        impl<'a> Solver<$day, 2> for PuzzleInput<'a> {
            fn solve(&self) -> Option<impl Display + Debug> {
                None as Option<String>
            }
        }
    };
    ($day:literal, $part_1_solver:ident) => {
        use std::fmt::{Debug, Display};

        impl<'a> Solver<$day, 1> for PuzzleInput<'a> {
            fn solve(&self) -> Option<impl Display + Debug> {
                Some($part_1_solver(self))
            }
        }
        impl<'a> Solver<$day, 2> for PuzzleInput<'a> {
            fn solve(&self) -> Option<impl Display + Debug> {
                None as Option<String>
            }
        }
    };
    ($day:literal, $part_1_solver:ident, $part_2_solver:ident) => {
        use std::fmt::{Debug, Display};

        impl<'a> Solver<$day, 1> for PuzzleInput<'a> {
            fn solve(&self) -> Option<impl Display + Debug> {
                Some($part_1_solver(self))
            }
        }
        impl<'a> Solver<$day, 2> for PuzzleInput<'a> {
            fn solve(&self) -> Option<impl Display + Debug> {
                Some($part_2_solver(self))
            }
        }
    };
}

#[macro_export]
macro_rules! aoc_test {
    ($day:literal, $part:literal, $expected:expr, $content:expr) => {
        let input: PuzzleInput = $content.into();
        let result = <PuzzleInput<'_> as Solver<$day, $part>>::solve(&input)
            .expect("no result")
            .to_string();
        assert_eq!(result, $expected.to_string());
    };
}

pub struct PuzzleInput<'a> {
    lines: Vec<&'a str>,
}

pub trait Lines {
    fn get_lines(&self) -> impl Iterator<Item=&str>;
}

impl<'a> Lines for PuzzleInput<'a> {
    fn get_lines(&self) -> impl Iterator<Item=&str> {
        self.lines.iter().copied()
    }
}

impl<'a> Lines for &PuzzleInput<'a> {
    fn get_lines(&self) -> impl Iterator<Item=&str> {
        self.lines.iter().copied()
    }
}

impl<'a> From<&'a str> for PuzzleInput<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            lines: value.lines().map(|s| s.trim()).collect(),
        }
    }
}

pub trait Solver<const D: usize, const P: usize> {
    fn solve(&self) -> Option<impl Display + Debug>;
}

pub trait PuzzleSource {
    fn get_input(&self, day: usize, part: usize) -> PuzzleInput;
}

pub struct MockedPuzzleSource {}

impl PuzzleSource for MockedPuzzleSource {
    fn get_input(&self, _day: usize, _part: usize) -> PuzzleInput {
        PuzzleInput { lines: vec![] }
    }
}

pub fn get_days_iter() -> impl Iterator<Item=Day> {
    1..=25
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_lines(input: impl Lines) -> u64 {
        input.get_lines().map(|l| l.parse::<u64>().unwrap()).sum()
    }

    solution!(10, sum_lines);

    #[test]
    fn test_implemented_solver() {
        aoc_test!(10, 1, 6, "1\n2\n3");
    }
}
