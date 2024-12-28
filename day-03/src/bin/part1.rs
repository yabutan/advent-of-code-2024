use day_03::Token;
use std::fs;
use std::io::{BufReader, Read};

fn main() -> anyhow::Result<()> {
    //let mut r = BufReader::new(fs::File::open("day-03/data/sample.txt")?);
    let mut r = BufReader::new(fs::File::open("day-03/data/input.txt")?);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let tokens = Token::parse_all(&input)?;

    let total = tokens.iter().fold(0, |acc, token| match token {
        Token::Mul(a, b) => acc + (a * b),
        _ => acc,
    });

    println!("Total: {}", total);

    Ok(())
}
