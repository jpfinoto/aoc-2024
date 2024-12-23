# Advent of Code 2024

An overcomplicated setup for getting inputs and benchmarking the solutions!

# Results

<!---BENCH_START--->

Benchmark CPU: **AMD Ryzen 9 7950X3D 16-Core Processor**

`|##########################################--------| 42/50 stars`

| Day                          | Part 1              | Part 2              |
|------------------------------|---------------------|---------------------|
| [01](src/solutions/day01.rs) | 42.9µs / 31 KiB     | 68.7µs / 33 KiB     |
| [02](src/solutions/day02.rs) | 134.0µs / 86 KiB    | 302.8µs / 86 KiB    |
| [03](src/solutions/day03.rs) | 89.3µs / 96 bytes   | 140.1µs / 192 bytes |
| [04](src/solutions/day04.rs) | 1.5ms / 109 KiB     | 4.2ms / 628 KiB     |
| [05](src/solutions/day05.rs) | 205.3µs / 17 KiB    | 203.4µs / 17 KiB    |
| [06](src/solutions/day06.rs) | 317.5µs / 1 MiB     | 113.3ms / 1 MiB     |
| [07](src/solutions/day07.rs) | 13.4ms / 696 bytes  | 691.0ms / 696 bytes |
| [08](src/solutions/day08.rs) | 21.7µs / 20 KiB     | 164.8µs / 59 KiB    |
| [09](src/solutions/day09.rs) | 928.4µs / 2 MiB     | 572.3ms / 1 MiB     |
| [10](src/solutions/day10.rs) | 245.1µs / 32 KiB    | 234.0µs / 16 KiB    |
| [11](src/solutions/day11.rs) | 159.6µs / 150 KiB   | 11.4ms / 9 MiB      |
| [12](src/solutions/day12.rs) | 61.5ms / 961 KiB    | 62.4ms / 961 KiB    |
| [13](src/solutions/day13.rs) | 245.7µs / 224 bytes | 244.0µs / 224 bytes |
| [14](src/solutions/day14.rs) | 109.9µs / 316 bytes | 137.5ms / 41 KiB    |
| [15](src/solutions/day15.rs) | 2.8ms / 772 KiB     | 3.0ms / 776 KiB     |
| [16](src/solutions/day16.rs) | 3.9ms / 2 MiB       | 6.9ms / 5 MiB       |
| [17](src/solutions/day17.rs) | 2.3µs / 320 bytes   | -                   |
| [18](src/solutions/day18.rs) | 1.2ms / 614 KiB     | 2.7ms / 350 KiB     |
| [19](src/solutions/day19.rs) | 1.2ms / 504 KiB     | 22.9ms / 1 MiB      |
| [20](src/solutions/day20.rs) | 683.4ms / 22 MiB    | -                   |
| 21                           | -                   | -                   |
| [22](src/solutions/day22.rs) | 15.3ms / 11 bytes   | 578.5ms / 133 MiB   |
| [23](src/solutions/day23.rs) | 962.6ms / 763 KiB   | 2.6ms / 476 KiB     |
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
