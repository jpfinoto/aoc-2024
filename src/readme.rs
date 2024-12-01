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

fn format_row<const N: usize>(row: &[impl AsRef<str>; N], widths: &[usize; N]) -> String {
    let inner = row
        .iter()
        .zip_eq(widths)
        .map(|(s, len)| pad(s.as_ref(), *len))
        .join(" | ");
    format!("| {inner} |")
}

fn format_table<const N: usize>(
    headers: &[impl AsRef<str>; N],
    lines: &[[impl AsRef<str>; N]],
) -> String {
    let column_widths: [usize; N] = (0..N)
        .map(|i| {
            iter::once(headers[i].as_ref())
                .chain(lines.iter().map(|l| l[i].as_ref()))
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
            column_widths.map(|n| repeat_string("-", n + 2)).join("|")
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
            let r1 = results.get(&(d, 1));
            let r2 = results.get(&(d, 2));
            [
                format!("{d:02}"),
                format_day_bench(r1),
                format_day_bench(r2),
            ]
        })
        .collect_vec();

    let table_entries = format_table(&["Day", "Part 1", "Part 2"], &days);

    let star_road = get_star_road(results.len(), get_days_iter().count() * 2);

    let s = sysinfo::System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );

    let cpu = s.cpus().first().unwrap();

    let proc_info = format!("{} @ {} MHz", cpu.brand().trim(), cpu.frequency());
    let sys_info = format!("Benchmark CPU: **{proc_info}**");
    let bench = format!("{sys_info}\n\n{star_road}\n\n{table_entries}");

    let updated_content =
        format!("{start}{MARKER_START}\n\n{bench}\n\n{MARKER_END}{footer}");

    std::fs::write(&readme_path, &updated_content).unwrap();
}

fn get_star_road(current_stars: usize, total_stars: usize) -> String {
    format!(
        "`|{}{}| {current_stars}/{total_stars} stars`",
        repeat_string("#", current_stars),
        repeat_string("-", total_stars - current_stars)
    )
}

fn repeat_string(s: impl AsRef<str>, n: usize) -> String {
    iter::repeat_n(s.as_ref(), n).join("")
}
