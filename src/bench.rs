#[cfg(feature = "benchmark_memory")]
use crate::PEAK_ALLOC;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

const TARGET_DURATION_PER_PART: Duration = Duration::from_secs(1);
const MAX_RUNS: usize = 1_000_000;

#[derive(Debug)]
pub(crate) struct BenchmarkResults {
    pub iterations: usize,
    pub average_duration: Duration,
    pub peak_memory: PeakMemoryUsage,
}

pub fn format_duration(duration: Duration) -> String {
    if duration < Duration::from_micros(1) {
        format!("{}ns", duration.subsec_nanos())
    } else if duration < Duration::from_millis(1) {
        format!("{:.1}µs", duration.as_secs_f64() * 1_000_000.0)
    } else if duration < Duration::from_secs(1) {
        format!("{:.1}ms", duration.as_secs_f64() * 1_000.0)
    } else {
        format!("{:.1}s", duration.as_secs_f64())
    }
}

const ONE_KB: usize = 1024;
const ONE_MB: usize = ONE_KB * 1024;

pub fn format_memory(bytes: usize) -> String {
    if bytes < ONE_KB {
        format!("{bytes} bytes")
    } else if bytes < ONE_MB {
        format!("{} KiB", bytes / ONE_KB)
    } else {
        format!("{} MiB", bytes / ONE_MB)
    }
}

impl Display for BenchmarkResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = match self.iterations {
            1 => "1 iteration".to_string(),
            n => format!("{} iterations", n),
        };

        if let PeakMemoryUsage::Bytes(peak_bytes) = self.peak_memory {
            write!(
                f,
                "{} / {} peak ({iter})",
                format_duration(self.average_duration),
                format_memory(peak_bytes),
            )
        } else {
            write!(f, "{} ({iter})", format_duration(self.average_duration),)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum PeakMemoryUsage {
    Bytes(usize),
    Disabled,
}

pub(crate) fn benchmark<T, F: Fn() -> Option<T>>(
    bench_fn: F,
) -> Result<BenchmarkResults, BenchmarkError> {
    let start = Instant::now();

    // prepare stats
    #[cfg(feature = "benchmark_memory")]
    PEAK_ALLOC.reset_peak_usage();
    #[cfg(feature = "benchmark_memory")]
    let initial_mem = PEAK_ALLOC.current_usage();

    // run the function to get an idea of how long it takes
    // and also measure peak memory usage
    let _ = bench_fn().ok_or(BenchmarkError::NotImplemented)?;

    // stop timer and count memory usage
    let first_run_duration = start.elapsed();

    #[cfg(feature = "benchmark_memory")]
    let peak_mem = PEAK_ALLOC.peak_usage();
    #[cfg(feature = "benchmark_memory")]
    let used_mem = PeakMemoryUsage::Bytes(peak_mem - initial_mem);

    #[cfg(not(feature = "benchmark_memory"))]
    let used_mem = PeakMemoryUsage::Disabled;

    if first_run_duration > TARGET_DURATION_PER_PART {
        Ok(BenchmarkResults {
            iterations: 1,
            average_duration: first_run_duration,
            peak_memory: used_mem,
        })
    } else {
        let project_runs = (TARGET_DURATION_PER_PART.as_secs_f64()
            / first_run_duration.as_secs_f64())
        .ceil() as usize;
        let iterations = min(MAX_RUNS, project_runs);

        let start = Instant::now();
        for _ in 0..iterations {
            _ = bench_fn().unwrap();
        }
        let duration = start.elapsed();

        Ok(BenchmarkResults {
            iterations,
            average_duration: duration / (iterations as u32),
            peak_memory: used_mem,
        })
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub(crate) enum BenchmarkError {
    #[error("not implemented")]
    NotImplemented,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    fn very_fast_solver() -> Option<()> {
        sleep(Duration::from_millis(10));
        Some(())
    }

    fn very_slow_solver() -> Option<()> {
        sleep(Duration::from_millis(2000));
        Some(())
    }

    fn not_implemented_solver() -> Option<()> {
        None
    }

    fn memory_intensive_solver() -> Option<f64> {
        let mut fib_numbers = vec![1.0, 1.0];

        for _ in 0..100000 {
            let [a, b] = fib_numbers.last_chunk().unwrap();
            fib_numbers.push(a + b);
        }

        Some(fib_numbers.iter().sum())
    }

    #[test]
    fn test_benchmark_slow_solver() {
        let bench = benchmark(very_slow_solver);
        assert!(bench.is_ok());
        assert_eq!(1, bench.unwrap().iterations);
    }

    #[test]
    fn test_benchmark_fast_solver() {
        let bench = benchmark(very_fast_solver);
        assert!(bench.is_ok());
        assert!(bench.unwrap().iterations > 10);
    }

    #[test]
    fn test_benchmark_fails() {
        let bench = benchmark(not_implemented_solver);
        assert_eq!(Some(BenchmarkError::NotImplemented), bench.err());
    }

    #[cfg(feature = "benchmark_memory")]
    #[test]
    fn test_benchmark_memory_intensive_solver() {
        let bench = benchmark(memory_intensive_solver);
        assert!(bench.is_ok());

        let PeakMemoryUsage::Bytes(used_mem) = bench.unwrap().peak_memory else {
            panic!()
        };

        assert!(used_mem > 10000);
    }
}
