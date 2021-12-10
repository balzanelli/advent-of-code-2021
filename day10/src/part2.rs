use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

lazy_static! {
    static ref PAIRS: HashMap<char, char> = hashmap! {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
    };
    static ref POINTS: HashMap<char, usize> = hashmap! {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
    };
}

pub fn solve(input: &str) -> usize {
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();

    let mut scores: Vec<usize> = Vec::new();

    for line in lines {
        let mut corrupt: bool = false;
        let mut tokens: Vec<char> = Vec::new();

        for token in line.chars() {
            if PAIRS.contains_key(&token) {
                tokens.push(token);
            } else {
                if token != PAIRS[tokens.last().unwrap()] {
                    corrupt = true;
                } else {
                    tokens.pop();
                }
            }
        }

        if !corrupt {
            let score = tokens
                .into_iter()
                .rev()
                .fold(0, |score, token| score * 5 + POINTS[&token]);
            scores.push(score);
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = "
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(288957, super::solve(input))
    }
}
