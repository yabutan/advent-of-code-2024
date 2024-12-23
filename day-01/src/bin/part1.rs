use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    //let r = BufReader::new(fs::File::open("day-01/data/sample.txt")?);
    let r = BufReader::new(fs::File::open("day-01/data/input.txt")?);

    let mut l_stack = BinaryHeap::new();
    let mut r_stack = BinaryHeap::new();

    let mut total = 0;
    for line in r.lines() {
        let line = line?;
        let mut s = line.split_whitespace();
        let a = s.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
        let b = s.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
        l_stack.push(Reverse(a));
        r_stack.push(Reverse(b));
    }

    loop {
        let left = l_stack.pop();
        let right = r_stack.pop();
        if left.is_none() || right.is_none() {
            break;
        }

        let left = left.unwrap().0;
        let right = right.unwrap().0;

        total += (left - right).abs()
    }

    println!("answer: {}", total);
    Ok(())
}
