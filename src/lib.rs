use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref SYNTAX_SCORES: HashMap<char, usize> = {
        let mut map = HashMap::with_capacity(4);
        map.insert(')', 3);
        map.insert(']', 57);
        map.insert('}', 1197);
        map.insert('>', 25137);
        map
    };

    static ref COMPLETER_SCORES: HashMap<char, usize> = {
        let mut map = HashMap::with_capacity(4);
        map.insert(')', 1);
        map.insert(']', 2);
        map.insert('}', 3);
        map.insert('>', 4);
        map
    };
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().map(corrupted_score).sum()
}

fn corrupted_score(line: &str) -> usize {
    let mut closers = Vec::new();

    for chr in line.chars() {
        match chr {
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            '{' => closers.push('}'),
            '<' => closers.push('>'),
            close => match closers.pop() {
                Some(c) if c == close => {}
                _ => {
                    return *SYNTAX_SCORES.get(&close).unwrap();
                }
            },
        }
    }

    0
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> usize {
    let mut scores: Vec<_> = input.lines().filter_map(incomplete_score).collect();
    scores.sort_unstable();

    let mid = scores.len() / 2;
    scores[mid]
}

fn incomplete_score(line: &str) -> Option<usize> {
    let mut closers = Vec::new();

    for chr in line.chars() {
        match chr {
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            '{' => closers.push('}'),
            '<' => closers.push('>'),
            close => match closers.pop() {
                Some(c) if c == close => {}
                _ => {
                    return None;
                }
            },
        }
    }

    let mut score = 0;
    while let Some(close) = closers.pop() {
        score *= 5;
        score += *COMPLETER_SCORES.get(&close).unwrap();
    }

    Some(score)
}

aoc_lib! { year = 2021 }
