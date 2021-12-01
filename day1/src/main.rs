mod part1;
mod part2;

fn main() {
    println!("advent-of-code-2021: day1");

    let file_path: String = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("./day1/input"));

    let input = std::fs::read_to_string(file_path)
        .unwrap();

    println!("part1: {}", part1::solve(&input).unwrap());
    println!("part2: {}", part2::solve(&input).unwrap());
}
