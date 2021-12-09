use std::str::FromStr;

struct HeightMap {
    w: i32,
    h: i32,
    height_map: Vec<Vec<u32>>,
}

impl HeightMap {
    fn new(height_map: Vec<Vec<u32>>) -> Self {
        Self {
            w: height_map[0].len() as i32,
            h: height_map.len() as i32,
            height_map,
        }
    }

    fn get_node_value(&self, x: i32, y: i32) -> u32 {
        self.height_map[y as usize][x as usize]
    }

    fn is_node_in_bounds(&self, x: i32, y: i32) -> bool {
        (!x.is_negative() && x < self.w) && (!y.is_negative() && y < self.h)
    }

    fn get_connected_nodes(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let nodes: Vec<(i32, i32)> = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        nodes
            .iter()
            .filter(|node| self.is_node_in_bounds(node.0, node.1))
            .cloned()
            .collect::<Vec<_>>()
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height_map = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|n| n.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(Self::new(height_map))
    }
}

pub fn solve(input: &str) -> u32 {
    let height_map = HeightMap::from_str(input).unwrap();

    let mut low: Vec<(i32, i32)> = Vec::new();

    for y in 0..height_map.h {
        for x in 0..height_map.w {
            let hi = height_map.get_connected_nodes(x, y).iter().any(|node| {
                height_map.get_node_value(x, y) >= height_map.get_node_value(node.0, node.1)
            });
            if !hi {
                low.push((x, y));
            }
        }
    }

    low.iter().fold(0u32, |sum, node| {
        sum + (height_map.get_node_value(node.0, node.1) + 1)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "2199943210
            3987894921
            9856789892
            8767896789
            9899965678";
        assert_eq!(15, super::solve(input))
    }
}
