use crate::{direction::Direction, instruction::Instruction};

pub struct PositionWithAim {
    pub aim: i32,
    pub horizontal: i32,
    pub depth: i32,
}

impl PositionWithAim {
    pub fn move_to(&mut self, i: Instruction) {
        match i.direction {
            Direction::Up => self.aim -= i.distance,
            Direction::Down => self.aim += i.distance,
            Direction::Forward => {
                self.horizontal += i.distance;
                self.depth += self.aim * i.distance;
            }
        }
    }
}

impl Default for PositionWithAim {
    fn default() -> Self {
        Self {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }
}
