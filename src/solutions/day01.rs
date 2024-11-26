use crate::aoc::*;
use crate::solution;

fn fib(n: usize) -> f64 {
    match n {
        0 => 0.0,
        1 => 1.0,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn solve_part_1(_: impl Lines) -> u64 {
    fib(10) as u64
}

fn solve_part_2(_: impl Lines) -> u64 {
    // deliberately dumb implementation!

    let mut fib_numbers = vec![1.0, 1.0];

    for _ in 0..1000000usize {
        let [a, b] = fib_numbers.last_chunk().unwrap();
        fib_numbers.push(a + b);
    }

    fib_numbers.iter().sum::<f64>() as u64
}

solution!(1, solve_part_1, solve_part_2);
