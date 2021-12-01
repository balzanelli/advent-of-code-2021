use std::fs;

mod part1;
mod part2;

static INPUT_PATH: &str = "./day1/input";

fn main() {
    println!("advent-of-code-2021: day1");

    let input = fs::read_to_string(INPUT_PATH)
        .unwrap();

    println!("part1: {}", part1::solve(&input).unwrap());
}
