use std::io::Read;

mod measurements;
mod part1;
mod part2;

fn main() {
    println!("advent-of-code-2021: day1");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("part1: {}", part1::solve(&input));
    println!("part2: {}", part2::solve(&input));
}
