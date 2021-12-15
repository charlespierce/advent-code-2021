use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashMap;

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut poly = Polymerizer::from(input);

    for _ in 0..10 {
        poly.step();
    }

    poly.score()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut poly = Polymerizer::from(input);

    for _ in 0..40 {
        poly.step();
    }

    poly.score()
}

type Pair = (char, char);

struct Polymerizer {
    pairs: HashMap<Pair, usize>,
    overcount: HashMap<char, usize>,
    insertions: HashMap<Pair, (char, Pair, Pair)>,
}

impl Polymerizer {
    fn step(&mut self) {
        let mut pairs = HashMap::with_capacity(self.pairs.len());
        std::mem::swap(&mut self.pairs, &mut pairs);

        for (pair, count) in pairs {
            if let Some((overlap, first, second)) = self.insertions.get(&pair).copied() {
                *self.pairs.entry(first).or_default() += count;
                *self.pairs.entry(second).or_default() += count;
                *self.overcount.entry(overlap).or_default() += count;
            }
        }
    }

    fn score(&self) -> usize {
        let mut counts = HashMap::new();

        for ((first, second), count) in self.pairs.iter() {
            *counts.entry(*first).or_default() += *count;
            *counts.entry(*second).or_default() += *count;
        }

        let (max, min) = counts.into_iter().fold(
            (0, usize::MAX),
            |(max, min), (chr, count): (char, usize)| {
                let normalized_count =
                    count - self.overcount.get(&chr).copied().unwrap_or_default();

                (max.max(normalized_count), min.min(normalized_count))
            },
        );

        max - min
    }
}

impl<'a> From<&'a str> for Polymerizer {
    fn from(input: &'a str) -> Self {
        let mut parts = input.split("\n\n");
        let (pairs, overcount) = to_pairs(parts.next().unwrap());

        let insertions = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut insertion = line.split(" -> ");
                let mut before = insertion.next().unwrap().chars();
                let pair = (before.next().unwrap(), before.next().unwrap());

                let chr = insertion.next().unwrap().chars().next().unwrap();
                let first = (pair.0, chr);
                let second = (chr, pair.1);

                (pair, (chr, first, second))
            })
            .collect();

        Self {
            pairs,
            overcount,
            insertions,
        }
    }
}

fn to_pairs(template: &str) -> (HashMap<Pair, usize>, HashMap<char, usize>) {
    let mut pairs = HashMap::with_capacity(template.len() - 1);
    let mut overcount = HashMap::new();

    for (index, pair) in template.chars().zip(template.chars().skip(1)).enumerate() {
        if index > 0 {
            *overcount.entry(pair.0).or_default() += 1;
        }
        *pairs.entry(pair).or_default() += 1;
    }

    (pairs, overcount)
}

aoc_lib! { year = 2021 }
