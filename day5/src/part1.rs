use std::collections::HashMap;

use num::signum;

use crate::vent_map::parse_vent_map;

pub fn solve(input: &str) -> usize {
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

    for line in parse_vent_map(input) {
        if !line.0.is_horizontal(&line.1) && !line.0.is_vertical(&line.1) {
            continue;
        }

        let dx = signum(line.1.x - line.0.x);
        let dy = signum(line.1.y - line.0.y);

        let mut p = line.0.clone();
        loop {
            *visited.entry((p.x, p.y)).or_insert(0) += 1;
            if p == line.1 {
                break;
            }
            p.x += dx;
            p.y += dy;
        }
    }

    visited.iter().filter(|&(_, count)| *count > 1).count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";
        assert_eq!(5, super::solve(input))
    }
}
