use std::str::FromStr;

use itertools::Itertools;

use crate::point::Point;

pub fn parse_vent_map(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(" -> ")
                .collect::<Vec<_>>()
                .iter()
                .map(|point| Point::from_str(point).unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>()
}
