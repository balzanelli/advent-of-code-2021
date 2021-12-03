use std::str::FromStr;

use crate::navigation::Direction;

pub struct Point {
    pub direction: Direction,
    pub units: i32,
}

impl Point {
    fn new(direction: Direction, units: i32) -> Self {
        Self { direction, units }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(" ");
        let direction = Direction::from_str(split.next().unwrap()).unwrap();
        let units = split.next().unwrap().parse().unwrap();
        Ok(Self::new(direction, units))
    }
}

pub fn parse_route(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| Point::from_str(line).unwrap())
        .collect::<Vec<Point>>()
}
