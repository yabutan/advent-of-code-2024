use glam::{IVec2, USizeVec2};
use std::collections::Bound::{Excluded, Unbounded};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct InputData {
    pub obstructions_x_to_y: HashMap<i32, BTreeSet<i32>>,
    pub obstructions_y_to_x: HashMap<i32, BTreeSet<i32>>,
    pub start_pos: IVec2,
    pub size: USizeVec2,
}

impl InputData {
    pub fn parse(text: &str) -> Self {
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
                        guard = Some(IVec2::new(x as i32, y as i32));
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

    pub fn add_obstruction(mut self, pos: IVec2) -> Self {
        self.obstructions_x_to_y
            .entry(pos.x)
            .or_default()
            .insert(pos.y);

        self.obstructions_y_to_x
            .entry(pos.y)
            .or_default()
            .insert(pos.x);

        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn as_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
}

pub trait NavigableSet<T: Ord> {
    fn higher(&self, x: &T) -> Option<&T>;
    fn lower(&self, x: &T) -> Option<&T>;
}

impl<T: Ord> NavigableSet<T> for BTreeSet<T> {
    fn higher(&self, x: &T) -> Option<&T> {
        self.range((Excluded(x), Unbounded)).next()
    }
    fn lower(&self, x: &T) -> Option<&T> {
        self.range(..x).next_back()
    }
}

#[derive(Debug)]
pub struct Tracker(HashMap<IVec2, HashSet<Direction>>);

impl Tracker {
    pub fn new(guard: &Guard) -> Self {
        Self(HashMap::from([(guard.pos, HashSet::from([guard.facing]))]))
    }

    /// return loop detected or not
    pub fn track(&mut self, guard: &Guard, pos: IVec2) -> bool {
        let d = guard.facing.as_ivec2();

        let mut cur = guard.pos;
        loop {
            cur += d;

            let directions = self.0.entry(cur).or_default();
            if !directions.insert(guard.facing) {
                // loop detected
                return true;
            }

            if cur == pos {
                return false;
            }
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = &IVec2> {
        self.0.keys()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Guard {
    pub pos: IVec2,
    pub facing: Direction,
}

impl Guard {
    /// (is_end, guard)
    pub fn move_next(&self, input: &InputData) -> (bool, Guard) {
        let facing = self.facing.rotate_90();

        // 壁の手前座標を取得、なければ端の座標
        let (is_end, pos) = match self.facing {
            Direction::Up => input
                .obstructions_x_to_y
                .get(&self.pos.x)
                .and_then(|v| v.lower(&self.pos.y))
                .map(|&y| (false, IVec2::new(self.pos.x, y + 1)))
                .unwrap_or((true, IVec2::new(self.pos.x, 0))),
            Direction::Down => input
                .obstructions_x_to_y
                .get(&self.pos.x)
                .and_then(|v| v.higher(&self.pos.y))
                .map(|&y| (false, IVec2::new(self.pos.x, y - 1)))
                .unwrap_or((true, IVec2::new(self.pos.x, input.size.y as i32 - 1))),
            Direction::Left => input
                .obstructions_y_to_x
                .get(&self.pos.y)
                .and_then(|v| v.lower(&self.pos.x))
                .map(|&x| (false, IVec2::new(x + 1, self.pos.y)))
                .unwrap_or((true, IVec2::new(0, self.pos.y))),
            Direction::Right => input
                .obstructions_y_to_x
                .get(&self.pos.y)
                .and_then(|v| v.higher(&self.pos.x))
                .map(|&x| (false, IVec2::new(x - 1, self.pos.y)))
                .unwrap_or((true, IVec2::new(input.size.x as i32 - 1, self.pos.y))),
        };

        (is_end, Guard { pos, facing })
    }
}
