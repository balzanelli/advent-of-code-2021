use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    units: i32,
}

impl Instruction {
    fn new(direction: Direction, units: i32) -> Instruction {
        Instruction { direction, units }
    }
}

#[derive(Default)]
struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine::default()
    }

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

    fn follow(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Forward => self.forward(instruction.units),
            Direction::Down => self.down(instruction.units),
            Direction::Up => self.up(instruction.units),
        }
    }

    fn position(&self) -> i32 {
        self.x * self.y
    }
}

pub fn solve(input: &str) -> i32 {
    let mut submarine = Submarine::new();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let mut split = line
                .trim()
                .split(" ");
            let direction = Direction::from_str(split.next().unwrap())
                .unwrap();
            let units = split.next().unwrap()
                .parse().unwrap();
            Instruction::new(direction, units)
        })
        .collect();

    for instruction in &instructions {
        submarine.follow(instruction)
    }
    submarine.position()
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
        assert_eq!(900, super::solve(input))
    }
}
