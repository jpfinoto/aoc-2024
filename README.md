# Advent of Code 2024

An overcomplicated setup for getting inputs and benchmarking the solutions!

# Results

<!---BENCH_START--->

Benchmark CPU: **AMD Ryzen 9 7950X3D 16-Core Processor**

`|#########################-------------------------| 25/50 stars`

| Day                          | Part 1              | Part 2              |
|------------------------------|---------------------|---------------------|
| [01](src/solutions/day01.rs) | 42.5µs / 31 KiB     | 66.3µs / 33 KiB     |
| [02](src/solutions/day02.rs) | 175.2µs / 86 KiB    | 319.7µs / 86 KiB    |
| [03](src/solutions/day03.rs) | 94.9µs / 96 bytes   | 145.9µs / 192 bytes |
| [04](src/solutions/day04.rs) | 1.5ms / 109 KiB     | 4.3ms / 628 KiB     |
| [05](src/solutions/day05.rs) | 216.2µs / 17 KiB    | 212.5µs / 17 KiB    |
| [06](src/solutions/day06.rs) | 494.5µs / 1 MiB     | 112.7ms / 1 MiB     |
| [07](src/solutions/day07.rs) | 14.2ms / 696 bytes  | 698.3ms / 696 bytes |
| [08](src/solutions/day08.rs) | 25.1µs / 20 KiB     | 166.9µs / 59 KiB    |
| [09](src/solutions/day09.rs) | 1.1ms / 2 MiB       | 563.7ms / 1 MiB     |
| [10](src/solutions/day10.rs) | 277.1µs / 30 KiB    | 268.9µs / 19 KiB    |
| [11](src/solutions/day11.rs) | 4.0ms / 5 MiB       | -                   |
| [12](src/solutions/day12.rs) | 68.8ms / 961 KiB    | 68.6ms / 961 KiB    |
| [13](src/solutions/day13.rs) | 249.0µs / 224 bytes | 250.3µs / 224 bytes |
| 14                           | -                   | -                   |
| 15                           | -                   | -                   |
| 16                           | -                   | -                   |
| 17                           | -                   | -                   |
| 18                           | -                   | -                   |
| 19                           | -                   | -                   |
| 20                           | -                   | -                   |
| 21                           | -                   | -                   |
| 22                           | -                   | -                   |
| 23                           | -                   | -                   |
| 24                           | -                   | -                   |
| 25                           | -                   | -                   |

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
