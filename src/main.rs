pub mod aoc;
pub(crate) mod bench;
pub mod solutions;

use crate::aoc::{get_days_iter, Day, MockedPuzzleSource, Part, PuzzleSource, SolverMap};
use crate::bench::{benchmark, BenchmarkResults};
use crate::solutions::get_solvers;
use clap::{arg, command, Command};
#[cfg(feature = "benchmark_memory")]
use peak_alloc::PeakAlloc;
use std::collections::HashMap;

#[cfg(feature = "benchmark_memory")]
#[global_allocator]
pub static PEAK_ALLOC: PeakAlloc = PeakAlloc;

type BenchmarkMap = HashMap<(Day, Part), BenchmarkResults>;

fn run_benchmarks(solver_map: &SolverMap, puzzle_source: &impl PuzzleSource) {
    for day in get_days_iter() {
        let mut part_bench: BenchmarkMap = HashMap::new();
        for part in 1..=2 as Part {
            if let Some(solver) = solver_map.get(&(day, part)) {
                let input = puzzle_source.get_input(day, part);
                let bench = benchmark(|| solver(&input));
                if let Ok(result) = bench {
                    part_bench.insert((day, part), result);
                }
            }
        }
        if !part_bench.is_empty() {
            log::info!(
                "Day {day}: \n - part 1: {}\n - part 2: {} ",
                part_bench
                    .get(&(day, 1))
                    .map(|t| t.to_string())
                    .unwrap_or("-".to_string()),
                part_bench
                    .get(&(day, 2))
                    .map(|t| t.to_string())
                    .unwrap_or("-".to_string()),
            );
        }
    }
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let _matches = command!()
        .subcommand(
            Command::new("bench")
                .about("Run the benchmark")
                .arg(arg!(-d [DAY] "which day to run"))
                .arg(arg!(-p [PART] "which part of the day to run")),
        )
        .subcommand(
            Command::new("solve")
                .about("Solve a day")
                .arg(arg!(<DAY> "which day to solve").required(true)),
        )
        .get_matches();

    run_benchmarks(&get_solvers(), &MockedPuzzleSource {});
}
