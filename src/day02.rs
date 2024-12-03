use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_FILE: &str = "input/day02.txt";

fn main() {
    println!("Day 2");

    let reports = read_input(INPUT_FILE).expect("Failed to read input file");

    // Part 1: Safe reports
    let num_safe_reports = count_safe_reports(&reports, false);
    println!("[Part 1] {}", num_safe_reports);

    // Part 2: Safe reports with Problem Dampener
    let num_safe_reports = count_safe_reports(&reports, true);
    println!("[Part 2] {}", num_safe_reports);
}

fn read_input(file_path: &str) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut reports = Vec::with_capacity(1000); // Allocate capacity for efficiency

    for line in reader.lines() {
        let line = line?;
        let values: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| "Cannot convert value to u32"))
            .collect::<Result<_, _>>()?;

        reports.push(values);
    }

    Ok(reports)
}

// Check a report without changing it
fn is_report_safe(report: &[u32]) -> bool {
    let mut current_diff: i32 = 0;

    for pair in report.windows(2) {
        let (ok, diff) = check_levels(pair[0], pair[1], current_diff);
        if !ok {
            return false;
        } else {
            current_diff += diff;
        }
    }

    true
}

// Check a report allowing one element to be removed
fn is_report_safe_with_dampener(report: &[u32]) -> bool {
    for drop_index in 0..=report.len() {
        let report_drop: Vec<u32> = report
            .to_vec()
            .into_iter()
            .enumerate()
            .filter(|&(index, _)| index != drop_index)
            .map(|(_, value)| value)
            .collect();

        if is_report_safe(&report_drop) {
            return true;
        }
    }

    false
}

// Check that two adjacent levels meet the requirements
fn check_levels(l0: u32, l1: u32, current_diff: i32) -> (bool, i32) {
    const MIN_DIST: i32 = 1;
    const MAX_DIST: i32 = 3;

    let dist: i32 = (l1 as i32) - (l0 as i32);
    let dist_abs: i32 = dist.abs();
    let next_diff = current_diff + dist;

    let ok =
        (MIN_DIST <= dist_abs) && (dist_abs <= MAX_DIST) && (next_diff.abs() > current_diff.abs());

    (ok, dist)
}

fn count_safe_reports(reports: &Vec<Vec<u32>>, use_problem_dampener: bool) -> usize {
    if use_problem_dampener {
        reports
            .iter()
            .filter(|report| is_report_safe_with_dampener(report))
            .count()
    } else {
        reports
            .iter()
            .filter(|report| is_report_safe(report))
            .count()
    }
}
