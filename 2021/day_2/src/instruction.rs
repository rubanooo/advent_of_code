use std::str::FromStr;

use crate::direction::Direction;

pub struct Instruction {
    pub direction: Direction,
    pub distance: i32,
}

impl Instruction {
    pub fn new(direction: Direction, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dis) = s.split_once(" ").ok_or(())?;

        let direction: Direction = dir.parse::<Direction>()?;
        let distance = dis.parse::<i32>().unwrap();

        Ok(Instruction::new(direction, distance))
    }
}
