use crate::navigation::Navigation;
use crate::route::parse_route;

#[derive(Default)]
struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
    fn new() -> Self {
        Self::default()
    }
}

impl Navigation for Submarine {
    fn forward(&mut self, units: i32) {
        self.x += units;
    }

    fn down(&mut self, units: i32) {
        self.y += units;
    }

    fn up(&mut self, units: i32) {
        self.y -= units;
    }

    fn pos(&self) -> i32 {
        self.x * self.y
    }
}

pub fn solve(input: &str) -> i32 {
    let mut sub = Submarine::new();
    for point in parse_route(input) {
        sub.move_to(point.direction, point.units)
    }
    sub.pos()
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
        assert_eq!(150, super::solve(input))
    }
}
