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
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    };
}

pub fn solve(input: &str) -> usize {
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();

    let mut score: usize = 0;

    for line in lines {
        let mut tokens: Vec<char> = Vec::new();

        for token in line.chars() {
            if PAIRS.contains_key(&token) {
                tokens.push(token);
            } else {
                if token != PAIRS[tokens.last().unwrap()] {
                    score += POINTS[&token];
                    break;
                } else {
                    tokens.pop();
                }
            }
        }
    }

    score
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
        assert_eq!(26397, super::solve(input))
    }
}
