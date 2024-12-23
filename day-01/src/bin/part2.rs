use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

fn main() -> anyhow::Result<()> {
    //let r = BufReader::new(fs::File::open("day-01/data/sample.txt")?);
    let r = BufReader::new(fs::File::open("day-01/data/input.txt")?);

    let mut l_map = HashMap::new();
    let mut r_map = HashMap::new();

    let mut total = 0;
    for line in r.lines() {
        let line = line?;
        let mut s = line.split_whitespace();
        let a = s.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
        let b = s.next().map(|x| x.parse::<i32>().unwrap()).unwrap();

        l_map.entry(a).or_insert(0).add_assign(1);
        r_map.entry(b).or_insert(0).add_assign(1);
    }

    for (a, count) in l_map {
        let mul = r_map.get(&a).cloned().unwrap_or_default();
        total += a * count * mul;
    }

    println!("answer: {}", total);
    Ok(())
}
