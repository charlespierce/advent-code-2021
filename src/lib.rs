use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

// From visual inspection of the input
const BITS: usize = 12;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u16]) -> usize {
    let minority_breakpoint = input.len() / 2;
    let (gamma, epsilon) = input
        .iter()
        .fold(vec![0; BITS], |mut acc, num| {
            for (i, count) in acc.iter_mut().enumerate() {
                if num & (1 << i) > 0 {
                    *count += 1;
                }
            }
            acc
        })
        .into_iter()
        .rev()
        .fold((0, 0), |(gamma, epsilon), bit_count| {
            if bit_count > minority_breakpoint {
                ((gamma << 1) | 1, epsilon << 1)
            } else {
                (gamma << 1, (epsilon << 1) | 1)
            }
        });

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[u16]) -> usize {
    let oxygen = find_rating(true, input);
    let co2 = find_rating(false, input);

    oxygen as usize * co2 as usize
}

fn find_rating(matching: bool, nums: &[u16]) -> u16 {
    let mut current = partition_by_bit_index(BITS - 1, matching, nums);

    for i in (0..BITS - 1).rev() {
        current = match current.as_slice() {
            [single] => return *single,
            val => partition_by_bit_index(i, matching, val),
        };
    }

    current[0]
}

fn partition_by_bit_index(index: usize, matching: bool, nums: &[u16]) -> Vec<u16> {
    let bit_mask = 1 << index;
    let one_count = nums.iter().filter(|num| *num & bit_mask > 0).count();

    let one_is_most_frequent = one_count * 2 >= nums.len();

    nums.iter()
        .copied()
        .filter(|num| matching != (one_is_most_frequent ^ (num & bit_mask > 0)))
        .collect()
}

aoc_lib! { year = 2021 }
