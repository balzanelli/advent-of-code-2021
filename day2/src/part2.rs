type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Default for Submarine {
    fn default() -> Submarine {
        Submarine {
            x: 0,
            y: 0,
            aim: 0,
        }
    }
}

impl Submarine {
    fn forward(&mut self, units: i32) {
        self.x += units;
        self.y += self.aim * units;
    }

    fn down(&mut self, units: i32) {
        self.aim += units;
    }

    fn up(&mut self, units: i32) {
        self.aim -= units;
    }

    fn position(&self) -> i32 {
        self.x * self.y
    }
}

pub fn solve(input: &str) -> Result<i32> {
    let mut submarine = Submarine::default();

    let directions: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut split = line
                .trim()
                .split(" ");
            (split.next().unwrap(), split.next().unwrap().parse().unwrap())
        })
        .collect();

    for direction in &directions {
        match direction.0 {
            "forward" => submarine.forward(direction.1),
            "down" => submarine.down(direction.1),
            "up" => submarine.up(direction.1),
            _ => {}
        }
    }
    Ok(submarine.position())
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2";
        assert_eq!(900, super::solve(input).unwrap())
    }
}
