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

        if check_safe(&levels) || check_safe_with_tolerate(&levels) {
            count += 1;
        }
    }

    println!("Safe reports: {}", count);
    Ok(())
}

fn check_safe(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels.windows(2).map(|x| x[1] - x[0]).collect();
    if diffs.iter().all(|&x| (1..=3).contains(&x)) {
        return true;
    }
    if diffs.iter().all(|&x| (-3..=-1).contains(&x)) {
        return true;
    }
    false
}

fn check_safe_with_tolerate(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels.windows(2).map(|x| x[1] - x[0]).collect();

    if let Some(i) = diffs.iter().position(|&x| !(1..=3).contains(&x)) {
        let (a, b) = candidate_levels(levels, i);
        if check_safe(&a) || check_safe(&b) {
            return true;
        }
    }

    if let Some(i) = diffs.iter().position(|&x| !(-3..=-1).contains(&x)) {
        let (a, b) = candidate_levels(levels, i);
        if check_safe(&a) || check_safe(&b) {
            return true;
        }
    }

    false
}

fn candidate_levels(levels: &[i32], i: usize) -> (Vec<i32>, Vec<i32>) {
    let mut a = levels.to_vec();
    let mut b = levels.to_vec();

    a.remove(i);
    b.remove(i + 1);

    (a, b)
}
