use day_04::{InputData, Point};
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let text = include_str!("../../data/input.txt");
    let input = InputData::new(text);
    let count = count_xmas_cross(&input);

    println!("count: {}", count);
    // count: 1985

    Ok(())
}

fn count_xmas_cross(input: &InputData) -> usize {
    let points: Vec<Point> = input
        .text
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == 'A' {
                    Some(Point {
                        pos: IVec2::new(x as i32, y as i32),
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    let mut count = 0;
    points.iter().for_each(|pos| {
        if pos.is_cross(input) {
            count += 1;
        }
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_sample() {
        let text = indoc! {r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#};

        let input = InputData::new(text);
        let count = count_xmas_cross(&input);
        println!("count: {count}");
        assert_eq!(count, 9);
    }
}
