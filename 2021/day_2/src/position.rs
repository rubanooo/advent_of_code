use crate::{direction::Direction, instruction::Instruction};

pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
}

impl Position {
    pub fn move_to(&mut self, i: Instruction) {
        match i.direction {
            Direction::Up => self.depth -= i.distance,
            Direction::Down => self.depth += i.distance,
            Direction::Forward => self.horizontal += i.distance,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
}
