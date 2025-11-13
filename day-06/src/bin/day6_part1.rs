use glam::{IVec2, USizeVec2};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn as_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    pos: IVec2,
    facing: Direction,
}
#[derive(Debug)]
struct InputData {
    obstructions_x_to_y: HashMap<i32, BTreeSet<i32>>,
    obstructions_y_to_x: HashMap<i32, BTreeSet<i32>>,
    start_pos: Guard,
    size: USizeVec2,
}

impl InputData {
    fn parse(text: &str) -> Self {
        let mut obstructions_x_to_y: HashMap<_, BTreeSet<_>> = HashMap::new();
        let mut obstructions_y_to_x: HashMap<_, BTreeSet<_>> = HashMap::new();
        let mut guard = None;

        let width = text.lines().next().unwrap().len();
        let height = text.lines().count();

        for (y, line) in text.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstructions_x_to_y
                            .entry(x as i32)
                            .or_default()
                            .insert(y as i32);
                        obstructions_y_to_x
                            .entry(y as i32)
                            .or_default()
                            .insert(x as i32);
                    }
                    '^' => {
                        guard = Some(Guard {
                            pos: IVec2::new(x as i32, y as i32),
                            facing: Direction::Up, // up
                        });
                    }
                    _ => {}
                }
            }
        }

        Self {
            obstructions_x_to_y,
            obstructions_y_to_x,
            start_pos: guard.unwrap(),
            size: USizeVec2::new(width, height),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let text = include_str!("../../data/input.txt");
    //let text = include_str!("../../data/sample.txt");
    let input = InputData::parse(text);
    println!("{:?}", input);

    let mut tracker = Tracker::new();
    let mut guard = input.start_pos;
    loop {
        println!("guard: {:?}", guard);
        match move_next(&guard, &input) {
            Next::Next(next) => {
                tracker.track(&guard, next.pos);
                guard = next;
                continue;
            }
            Next::End(next) => {
                tracker.track(&guard, next.pos);
                guard = next;
                println!("guard: {:?}", guard);
                break;
            }
        }
    }

    println!("max: {}", tracker.len());
    // max: 5516

    Ok(())
}

#[derive(Debug)]
enum Next {
    Next(Guard),
    End(Guard),
}

#[derive(Debug)]
struct Tracker(HashSet<IVec2>);

impl Tracker {
    fn new() -> Self {
        Self(HashSet::new())
    }
    fn track(&mut self, guard: &Guard, pos: IVec2) {
        let d = guard.facing.as_ivec2();

        let mut cur = guard.pos;
        loop {
            self.0.insert(cur);
            cur += d;
            if cur == pos {
                self.0.insert(cur);
                break;
            }
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn move_next(guard: &Guard, input: &InputData) -> Next {
    match guard.facing {
        Direction::Up => {
            let facing = guard.facing.rotate_90();
            let next_y = input
                .obstructions_x_to_y
                .get(&guard.pos.x)
                .and_then(|v| v.iter().filter(|&&y| y < guard.pos.y).max())
                .map(|&y| y + 1);

            if let Some(next_y) = next_y {
                Next::Next(Guard {
                    pos: IVec2::new(guard.pos.x, next_y),
                    facing,
                })
            } else {
                Next::End(Guard {
                    pos: IVec2::new(guard.pos.x, 0),
                    facing,
                })
            }
        }
        Direction::Down => {
            let facing = guard.facing.rotate_90();
            let next_y = input
                .obstructions_x_to_y
                .get(&guard.pos.x)
                .and_then(|v| v.iter().filter(|&&y| y > guard.pos.y).min())
                .map(|&y| y - 1);

            if let Some(next_y) = next_y {
                Next::Next(Guard {
                    pos: IVec2::new(guard.pos.x, next_y),
                    facing,
                })
            } else {
                Next::End(Guard {
                    pos: IVec2::new(guard.pos.x, input.size.y as i32 - 1),
                    facing,
                })
            }
        }
        Direction::Left => {
            let facing = guard.facing.rotate_90();
            let next_x = input
                .obstructions_y_to_x
                .get(&guard.pos.y)
                .and_then(|v| v.iter().filter(|&&x| x < guard.pos.x).max())
                .map(|&x| x + 1);

            if let Some(next_x) = next_x {
                Next::Next(Guard {
                    pos: IVec2::new(next_x, guard.pos.y),
                    facing,
                })
            } else {
                Next::End(Guard {
                    pos: IVec2::new(0, guard.pos.x),
                    facing,
                })
            }
        }
        Direction::Right => {
            let facing = guard.facing.rotate_90();
            let next_x = input
                .obstructions_y_to_x
                .get(&guard.pos.y)
                .and_then(|v| v.iter().filter(|&&x| x > guard.pos.x).min())
                .map(|&x| x - 1);

            if let Some(next_x) = next_x {
                Next::Next(Guard {
                    pos: IVec2::new(next_x, guard.pos.y),
                    facing,
                })
            } else {
                Next::End(Guard {
                    pos: IVec2::new(input.size.x as i32 - 1, guard.pos.x),
                    facing,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = IVec2::new(4, 6);
        let b = IVec2::new(4, 1);
        let c = IVec2::new(8, 1);

        println!("1: {}", a.manhattan_distance(b));
        println!("2: {}", b.manhattan_distance(c));
    }
}
