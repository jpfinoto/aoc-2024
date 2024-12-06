# Advent of Code 2024

An overcomplicated setup for getting inputs and benchmarking the solutions!

# Results

<!---BENCH_START--->

Benchmark CPU: **AMD Ryzen 9 7950X3D 16-Core Processor**

`|############--------------------------------------| 12/50 stars`

| Day | Part 1            | Part 2              |
|-----|-------------------|---------------------|
| 01  | 46.0µs / 31 KiB   | 72.4µs / 33 KiB     |
| 02  | 145.4µs / 86 KiB  | 318.2µs / 86 KiB    |
| 03  | 94.6µs / 96 bytes | 146.7µs / 192 bytes |
| 04  | 1.6ms / 109 KiB   | 4.3ms / 628 KiB     |
| 05  | 206.9µs / 17 KiB  | 205.4µs / 17 KiB    |
| 06  | 431.0µs / 1 MiB   | 114.6ms / 1 MiB     |
| 07  | -                 | -                   |
| 08  | -                 | -                   |
| 09  | -                 | -                   |
| 10  | -                 | -                   |
| 11  | -                 | -                   |
| 12  | -                 | -                   |
| 13  | -                 | -                   |
| 14  | -                 | -                   |
| 15  | -                 | -                   |
| 16  | -                 | -                   |
| 17  | -                 | -                   |
| 18  | -                 | -                   |
| 19  | -                 | -                   |
| 20  | -                 | -                   |
| 21  | -                 | -                   |
| 22  | -                 | -                   |
| 23  | -                 | -                   |
| 24  | -                 | -                   |
| 25  | -                 | -                   |

<!---BENCH_END--->

# Setup

You need to create a file called `token.txt` in the root of this repo with your API key to be able to download
puzzle inputs. You can get this from the session token while logged in on the website.

# Solving

- Create a solution with the format `day{n:02}.rs` in the `src/solutions` folder
- Use the `solution!()` macro to declare solutions. See the template at the end of the readme.
- Use `cargo run` to solve the latest solved day
- Alternatively, use `cargo run solve <day>` to solve a specific day.

# Benchmarks

To update the benchmark, run `cargo run --release bench`.

The benchmark runs the solution function for each part of each day, one after the other, and measures the average
execution time and the peak heap usage.

Heap usage is measured on the second call to each solver, so if you have some kind of `lazy_static` that gets allocated
on the first run it will NOT be measured! Stack usage is also not measured.

# Day Template

```rust
use crate::aoc::*;
use crate::solution;

const DAY: Day = 1;

solution!(DAY, solve_part_1, solve_part_2);

fn solve_part_1(input: impl Lines) -> i64 {
    0
}

fn solve_part_2(input: impl Lines) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_test;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part_1() {
        aoc_test!(DAY, 1, 0, TEST_INPUT);
    }

    #[test]
    fn test_part_2() {
        aoc_test!(DAY, 2, 0, TEST_INPUT);
    }
}

```

# Inner Workings

The `solution!` macro expands to something like this:

```rust
impl Solver<DAY, 1> for PuzzleInput {
    fn solve(&self) -> Option<impl Display + Debug> {
        Some(solve_part_1(self))
    }
}
```

Then, the build script detects all the solved days and wraps them all into a map of solver functions.

Yes, it's pretty weird, but I'm too far into this rabbit hole to change how it works now ;)
