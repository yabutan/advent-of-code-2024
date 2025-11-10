use glam::IVec2;

#[derive(Debug)]
pub struct InputData<'a> {
    pub text: &'a str,
    pub width: usize,
}

impl<'a> InputData<'a> {
    pub fn new(text: &'a str) -> Self {
        let width = text.lines().next().unwrap().len();
        Self { text, width }
    }

    pub fn get(&self, pos: IVec2) -> Option<char> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.width as i32 {
            return None;
        }

        // 末尾に改行があるので、width+1している。
        let index = pos.x as usize + (pos.y as usize * (self.width + 1));
        self.text.chars().nth(index)
    }
}

#[derive(Debug)]
pub struct Point {
    pub pos: IVec2,
}

impl Point {
    pub fn get_text(&self, input: &InputData, direction: &IVec2) -> Option<String> {
        let mut chars = Vec::new();
        for i in 0..4 {
            let next = self.pos + (direction * i);
            let c = input.get(next)?;
            chars.push(c);
        }

        Some(String::from_iter(chars))
    }

    pub fn is_cross(&self, input: &InputData) -> bool {
        let Some(right_top) = input.get(self.pos + IVec2::new(1, -1)) else {
            return false;
        };
        let Some(left_bottom) = input.get(self.pos + IVec2::new(-1, 1)) else {
            return false;
        };
        let Some(left_top) = input.get(self.pos + IVec2::new(-1, -1)) else {
            return false;
        };
        let Some(right_bottom) = input.get(self.pos + IVec2::new(1, 1)) else {
            return false;
        };

        match (right_top, left_bottom) {
            ('M', 'S') | ('S', 'M') => {}
            _ => return false,
        }
        match (left_top, right_bottom) {
            ('M', 'S') | ('S', 'M') => {}
            _ => return false,
        }

        true
    }
}
