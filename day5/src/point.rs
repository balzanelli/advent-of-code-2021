use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> (i32, i32) {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        (dx, dy)
    }

    pub fn is_horizontal(&self, other: &Point) -> bool {
        self.x == other.x
    }

    pub fn is_vertical(&self, other: &Point) -> bool {
        self.y == other.y
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Point::new(x, y))
    }
}
