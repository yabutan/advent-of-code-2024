use day_04::InputData;
use glam::IVec2;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let text = include_str!("../../data/input.txt");
    let input = InputData::new(text);
    let count = count_xmas(&input);

    println!("count: {}", count);
    // count: 2551

    Ok(())
}

fn count_xmas(input: &day_04::InputData) -> usize {
    let points: Vec<day_04::Point> = input
        .text
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == 'X' {
                    Some(day_04::Point {
                        pos: IVec2::new(x as i32, y as i32),
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    let directions = [
        IVec2::new(1, 0),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(0, -1),
        IVec2::new(1, 1),
        IVec2::new(-1, 1),
        IVec2::new(1, -1),
        IVec2::new(-1, -1),
    ];

    let mut count = 0;
    points.iter().for_each(|pos| {
        directions.iter().for_each(|d| {
            if let Some(text) = pos.get_text(input, d) {
                if text == "XMAS" {
                    count += 1;
                }
            }
        })
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_04::Point;
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

        let p = Point {
            pos: IVec2::new(5, 0),
        };
        let text = p.get_text(&input, &IVec2::new(1, 0));
        println!("{p:?} {text:?}");

        let p = Point {
            pos: IVec2::new(4, 0),
        };
        let text = p.get_text(&input, &IVec2::new(1, 1));
        println!("{p:?} {text:?}");

        let count = count_xmas(&input);
        println!("count: {count}");
        assert_eq!(count, 18);
    }
}
