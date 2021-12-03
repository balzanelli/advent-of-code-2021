use crate::measurements::parse_measurements;

pub fn solve(input: &str) -> usize {
    parse_measurements(input)
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "199
            200
            208
            210
            200
            207
            240
            269
            260
            263";
        assert_eq!(5, super::solve(input))
    }
}
