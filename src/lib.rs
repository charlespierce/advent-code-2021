use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_lib};

fn parse_input(input: &str) -> School {
    let mut fish = HashMap::with_capacity(8);

    for age_str in input.split(',') {
        let age = age_str.parse().unwrap();
        *fish.entry(age).or_default() += 1;
    }

    School { fish }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut school = parse_input(input);
    for _ in 0..80 {
        school.step();
    }

    school.count()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut school = parse_input(input);
    for _ in 0..256 {
        school.step();
    }

    school.count()
}

struct School {
    fish: HashMap<usize, usize>,
}

impl School {
    fn step(&mut self) {
        let mut fish = HashMap::with_capacity(8);

        for i in 1..9 {
            fish.insert(i - 1, self.fish.get(&i).copied().unwrap_or_default());
        }

        if let Some(ready) = self.fish.get(&0) {
            fish.insert(8, *ready);
            *fish.entry(6).or_default() += *ready;
        }

        self.fish = fish;
    }

    fn count(&self) -> usize {
        self.fish.values().sum()
    }
}

aoc_lib! { year = 2021 }
