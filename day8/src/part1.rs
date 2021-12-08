use std::str::FromStr;

use itertools::Itertools;

struct Reading {
    signals: Vec<String>,
    digits: Vec<String>,
}

impl Reading {
    fn new(signals: Vec<String>, digits: Vec<String>) -> Self {
        Self { signals, digits }
    }

    fn digits(&self) -> Vec<i32> {
        self.digits
            .iter()
            .filter_map(|pattern| match pattern.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}

impl FromStr for Reading {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, digits): (&str, &str) = s.trim().split(" | ").collect_tuple().unwrap();

        let signals = signals
            .split_whitespace()
            .map(|pattern| pattern.to_owned())
            .collect::<Vec<_>>();

        let digits = digits
            .split_whitespace()
            .map(|pattern| pattern.to_owned())
            .collect::<Vec<_>>();

        Ok(Self::new(signals, digits))
    }
}

pub fn solve(input: &str) -> usize {
    let readings = input
        .lines()
        .map(|line| Reading::from_str(line).unwrap())
        .collect::<Vec<_>>();

    readings.iter().map(|reading| reading.digits().len()).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(26, super::solve(input))
    }
}
