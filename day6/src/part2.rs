use std::collections::HashMap;

fn evolve(fish: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut cycle: HashMap<usize, usize> = HashMap::new();
    for (timer, num_fish) in fish.into_iter() {
        if timer > 0 {
            *cycle.entry(timer - 1).or_insert(0) += num_fish;
        } else {
            *cycle.entry(6).or_insert(0) += num_fish;
            *cycle.entry(8).or_insert(0) += num_fish;
        }
    }
    cycle
}

pub fn solve(input: &str) -> usize {
    let mut fish = input
        .split(',')
        .map(|fish| fish.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut fish, timer| -> HashMap<usize, usize> {
            *fish.entry(timer).or_insert(0) += 1;
            fish
        });

    for _ in 0..256 {
        fish = evolve(fish);
    }

    fish.iter()
        .fold(0usize, |sum, (_, num_fish)| sum + num_fish)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "3,4,3,1,2";
        assert_eq!(26984457539, super::solve(input))
    }
}
