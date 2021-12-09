use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::{HashMap, HashSet};

macro_rules! set {
    [$($val:expr),*] => {
        {
            let mut set = HashSet::new();
            $(
                set.insert($val);
            )*
            set
        }
    }
}

lazy_static::lazy_static! {
    static ref DIGITS: HashMap<u8, HashSet<char>> = {
        let mut map = HashMap::with_capacity(10);
        map.insert(0, set!['a', 'b', 'c', 'e', 'f', 'g']);
        map.insert(1, set!['c', 'f']);
        map.insert(2, set!['a', 'c', 'd', 'e', 'g']);
        map.insert(3, set!['a', 'c', 'd', 'f', 'g']);
        map.insert(4, set!['b', 'c', 'd', 'f']);
        map.insert(5, set!['a', 'b', 'd', 'f', 'g']);
        map.insert(6, set!['a', 'b', 'd', 'e', 'f', 'g']);
        map.insert(7, set!['a', 'c', 'f']);
        map.insert(8, set!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        map.insert(9, set!['a', 'b', 'c', 'd', 'f', 'g']);
        map
    };
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(Display::from_line)
        .map(Display::solve)
        .sum()
}

struct Display {
    patterns: Vec<HashSet<char>>,
    display: Vec<HashSet<char>>,
}

impl Display {
    fn from_line(line: &str) -> Self {
        let mut parts = line
            .split(" | ")
            .map(|nums| nums.split(' ').map(|num| num.chars().collect()).collect());

        let patterns = parts.next().unwrap();
        let display = parts.next().unwrap();

        Self { patterns, display }
    }

    fn solve(self) -> usize {
        let mut translator = HashMap::with_capacity(7);
        let six_missing: HashSet<_> = self
            .patterns
            .iter()
            .filter(|pat| pat.len() == 6)
            .map(|pat| *DIGITS.get(&8).unwrap().difference(pat).next().unwrap())
            .collect();
        let two = self.patterns.iter().find(|pat| pat.len() == 2).unwrap();
        let three = self.patterns.iter().find(|pat| pat.len() == 3).unwrap();
        let four = self.patterns.iter().find(|pat| pat.len() == 4).unwrap();

        let a = *three.difference(two).next().unwrap();
        let b = *four
            .difference(two)
            .find(|chr| !six_missing.contains(*chr))
            .unwrap();
        let c = *six_missing.intersection(two).next().unwrap();
        let d = *four.difference(two).find(|chr| **chr != b).unwrap();
        let e = *six_missing
            .iter()
            .find(|chr| **chr != c && **chr != d)
            .unwrap();
        let f = *two.iter().find(|chr| **chr != c).unwrap();
        let g = *DIGITS
            .get(&8)
            .unwrap()
            .iter()
            .find(|chr| ![a, b, c, d, e, f].contains(chr))
            .unwrap();

        translator.insert(a, 'a');
        translator.insert(b, 'b');
        translator.insert(c, 'c');
        translator.insert(d, 'd');
        translator.insert(e, 'e');
        translator.insert(f, 'f');
        translator.insert(g, 'g');

        translate_output(self.display, translator)
    }
}

fn translate_output(digits: Vec<HashSet<char>>, translator: HashMap<char, char>) -> usize {
    digits
        .into_iter()
        .map(|seg| translate(&seg, &translator))
        .fold(0, |acc, digit| acc * 10 + digit as usize)
}

fn translate(coded_segments: &HashSet<char>, translator: &HashMap<char, char>) -> u8 {
    let decoded = coded_segments
        .iter()
        .map(|chr| *translator.get(chr).unwrap())
        .collect();

    digit(&decoded)
}

fn digit(values: &HashSet<char>) -> u8 {
    DIGITS
        .iter()
        .find_map(|(digit, segments)| {
            if segments == values {
                Some(*digit)
            } else {
                None
            }
        })
        .unwrap()
}

aoc_lib! { year = 2021 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solver() {
        let display = Display::from_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        assert_eq!(display.solve(), 5353);
    }
}
