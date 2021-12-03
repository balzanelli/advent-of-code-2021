pub fn parse_measurements(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}
