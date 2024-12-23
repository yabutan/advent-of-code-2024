use std::fs;
use std::io::{BufRead, BufReader};

/// The levels are either all increasing or all decreasing.
/// Any two adjacent levels differ by at least one and at most three.

fn main() -> anyhow::Result<()> {
    //let r = BufReader::new(fs::File::open("day-02/data/sample.txt")?);
    let r = BufReader::new(fs::File::open("day-02/data/input.txt")?);

    let mut count = 0;
    for line in r.lines() {
        let line = line?;
        let levels: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let res = match check_report(&levels) {
            ReportResult::Safe => ReportResult::Safe,
            ReportResult::Unsafe => check_report2(&levels),
        };

        if let ReportResult::Safe = res {
            count += 1;
        }
    }

    println!("Safe reports: {}", count);
    Ok(())
}

enum ReportResult {
    Safe,
    Unsafe,
}

fn check_report(levels: &[i32]) -> ReportResult {
    let diffs: Vec<_> = levels.windows(2).map(|x| x[1] - x[0]).collect();
    if diffs.iter().all(|&x| (1..=3).contains(&x)) {
        return ReportResult::Safe;
    }
    if diffs.iter().all(|&x| (-3..=-1).contains(&x)) {
        return ReportResult::Safe;
    }

    ReportResult::Unsafe
}

fn check_report2(levels: &[i32]) -> ReportResult {
    let diffs: Vec<_> = levels.windows(2).map(|x| x[1] - x[0]).collect();
    if let Some(i) = diffs.iter().position(|&x| !(1..=3).contains(&x)) {
        let mut a = levels.to_vec();
        let mut b = levels.to_vec();
        a.remove(i);
        b.remove(i + 1);

        if let ReportResult::Safe = check_report(&a) {
            return ReportResult::Safe;
        } else if let ReportResult::Safe = check_report(&b) {
            return ReportResult::Safe;
        }
    }

    if let Some(i) = diffs.iter().position(|&x| !(-3..=-1).contains(&x)) {
        let mut a = levels.to_vec();
        let mut b = levels.to_vec();
        a.remove(i);
        b.remove(i + 1);

        if let ReportResult::Safe = check_report(&a) {
            return ReportResult::Safe;
        } else if let ReportResult::Safe = check_report(&b) {
            return ReportResult::Safe;
        }
    }

    ReportResult::Unsafe
}
