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

#[derive(Debug)]
struct InputData<'a> {
    text: &'a str,
    width: usize,
}

impl<'a> InputData<'a> {
    fn new(text: &'a str) -> Self {
        let width = text.lines().next().unwrap().len();
        Self { text, width }
    }
}

#[derive(Debug)]
struct Point {
    pos: IVec2,
}

impl Point {
    fn get_text(&self, input: &InputData, direction: &IVec2) -> Option<String> {
        let mut s = Vec::new();
        for i in 0..4 {
            let next = self.pos + (direction * i);
            if next.x < 0
                || next.y < 0
                || next.x >= input.width as i32
                || next.y >= input.width as i32
            {
                return None;
            }

            // 末尾に改行があるので、W+1している。
            let index = next.x as usize + (next.y as usize * (input.width + 1));
            let c = &input.text[index..index + 1];
            s.push(c);
        }

        Some(s.join(""))
    }
}

fn count_xmas(input: &InputData) -> usize {
    let points: Vec<Point> = input
        .text
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == 'X' {
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
    }
}
