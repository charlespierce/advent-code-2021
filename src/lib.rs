use aoc_runner_derive::{aoc, aoc_lib};

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> usize {
    count_increases(parse_depths(input))
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    count_increases(SlidingWindow::new(parse_depths(input)).unwrap())
}

fn parse_depths(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

fn count_increases<I>(iter: I) -> usize
where
    I: Iterator<Item = usize>,
{
    iter.fold((usize::MAX, 0), |(prev, count), cur| {
        if cur > prev {
            (cur, count + 1)
        } else {
            (cur, count)
        }
    })
    .1
}

struct SlidingWindow<I> {
    iter: I,
    first: usize,
    second: usize,
}

impl<I> SlidingWindow<I>
where
    I: Iterator<Item = usize>,
{
    fn new(mut iter: I) -> Option<Self> {
        let first = iter.next()?;
        let second = iter.next()?;

        Some(SlidingWindow {
            iter,
            first,
            second,
        })
    }
}

impl<I> Iterator for SlidingWindow<I>
where
    I: Iterator<Item = usize>,
{
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let next = self.iter.next()?;

        let total = self.first + self.second + next;
        self.first = self.second;
        self.second = next;

        Some(total)
    }
}

aoc_lib! { year = 2021 }
