fn evolve(fish: Vec<usize>) -> Vec<usize> {
    let mut cycle: Vec<usize> = Vec::new();
    for timer in fish {
        if timer > 0 {
            cycle.push(timer - 1);
        } else {
            cycle.push(6);
            cycle.push(8);
        }
    }
    cycle
}

pub fn solve(input: &str) -> usize {
    let mut fish = input
        .split(',')
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        fish = evolve(fish);
    }

    fish.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "3,4,3,1,2";
        assert_eq!(5934, super::solve(input))
    }
}
