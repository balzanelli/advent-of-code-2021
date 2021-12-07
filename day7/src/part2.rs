pub fn solve(input: &str) -> i32 {
    let mut crabs = input
        .split(',')
        .map(|item| item.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    (min..max)
        .map(|start| {
            crabs
                .iter()
                .map(|end| {
                    let distance = (start - end).abs();
                    distance * (distance + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(168, super::solve(input))
    }
}
