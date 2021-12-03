use std::str::FromStr;

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

pub trait Navigation {
    fn forward(&mut self, units: i32);
    fn down(&mut self, units: i32);
    fn up(&mut self, units: i32);
    fn pos(&self) -> i32;

    fn move_to(&mut self, direction: Direction, units: i32) {
        match direction {
            Direction::Forward => self.forward(units),
            Direction::Down => self.down(units),
            Direction::Up => self.up(units),
        }
    }
}
