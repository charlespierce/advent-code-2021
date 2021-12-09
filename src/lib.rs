use aoc_runner_derive::{aoc, aoc_lib};

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let crabs = parse_input(input);
    let min_position = crabs.iter().copied().min().unwrap();
    let max_position = crabs.iter().copied().max().unwrap();

    let mut min_cost = usize::MAX;
    for pos in min_position..=max_position {
        let cost = crabs.iter().fold(0, |acc, crab| {
            if pos > *crab {
                acc + pos - *crab
            } else {
                acc + *crab - pos
            }
        });

        min_cost = min_cost.min(cost);
    }

    min_cost
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let crabs = parse_input(input);
    let min_position = crabs.iter().copied().min().unwrap();
    let max_position = crabs.iter().copied().max().unwrap();

    let mut min_cost = usize::MAX;
    for pos in min_position..=max_position {
        let cost = crabs
            .iter()
            .copied()
            .fold(0, |acc, crab| acc + fuel_cost(crab, pos));

        min_cost = min_cost.min(cost);
    }

    min_cost
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn fuel_cost(start: usize, end: usize) -> usize {
    let diff = if start > end {
        start - end
    } else {
        end - start
    };

    diff * (diff + 1) / 2
}

aoc_lib! { year = 2021 }
