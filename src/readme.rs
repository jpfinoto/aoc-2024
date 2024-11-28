use crate::aoc::get_days_iter;
use crate::bench::{format_duration, format_memory, BenchmarkResults};
use crate::BenchmarkMap;
use itertools::Itertools;
use std::env::current_dir;
use std::fs::read_to_string;
use std::iter;
use sysinfo::{CpuRefreshKind, RefreshKind};

const MARKER_START: &str = "<!---BENCH_START--->";
const MARKER_END: &str = "<!---BENCH_END--->";

fn format_day_bench(result: Option<&BenchmarkResults>) -> String {
    if let Some(result) = result {
        format!(
            "{} / {}",
            format_duration(result.average_duration),
            format_memory(result.peak_memory)
        )
    } else {
        "-".to_owned()
    }
}

fn pad(s: &str, len: usize) -> String {
    format!("{:<len$}", s, len = len)
}

fn format_row<const N: usize>(row: &[String; N], widths: &[usize; N]) -> String {
    let inner = row
        .iter()
        .zip_eq(widths)
        .map(|(s, len)| pad(s, *len))
        .join(" | ");
    format!("| {inner} |")
}

fn format_table<const N: usize>(headers: &[String; N], lines: &[[String; N]]) -> String {
    let column_widths: [usize; N] = (0..N)
        .map(|i| {
            iter::once(&headers[i])
                .chain(lines.iter().map(|l| &l[i]))
                .map(|s| s.chars().count())
                .max()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    iter::once(format_row(headers, &column_widths))
        .chain(iter::once(format!(
            "|{}|",
            column_widths
                .map(|n| iter::repeat_n("-", n + 2).join(""))
                .join("|")
        )))
        .chain(lines.iter().map(|line| format_row(line, &column_widths)))
        .join("\n")
}

pub fn update_readme(results: &BenchmarkMap) {
    let readme_path = current_dir().unwrap().join("README.md");
    let contents = read_to_string(&readme_path).unwrap();
    let (start, middle) = contents.split_once(MARKER_START).unwrap();
    let (_, footer) = middle.split_once(MARKER_END).unwrap();

    let days = get_days_iter()
        .map(|d| {
            [
                format!("{d:02}"),
                format_day_bench(results.get(&(d, 1))),
                format_day_bench(results.get(&(d, 2))),
            ]
        })
        .collect_vec();

    let table_entries = format_table(
        &["Day".to_owned(), "Part 1".to_owned(), "Part 2".to_owned()],
        &days,
    );

    let s = sysinfo::System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );

    let cpu = s.cpus().first().unwrap();

    let proc_info = format!("{} @ {} MHz", cpu.brand().trim(), cpu.frequency());
    let sys_info = format!("Benchmark CPU: **{proc_info}**");
    let bench = format!("{sys_info}\n\n{table_entries}");

    let updated_content = format!("{start}{MARKER_START}\n\n{bench}\n\n{MARKER_END}{footer}");

    std::fs::write(&readme_path, &updated_content).unwrap();
}
