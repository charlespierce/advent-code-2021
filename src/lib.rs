use aoc_runner_derive::{aoc, aoc_lib};
use std::ops::Range;

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> isize {
    let target = Target::from(input);

    let v_y = calc_max_y(target.y.start);
    v_y * (v_y + 1) / 2
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> usize {
    let target = Target::from(input);
    let mut count = 0;

    let x_min = calc_min_x(target.x.start);
    let x_max = target.x.end;

    let y_min = target.y.start;
    let y_max = isize::abs(target.y.start + 1);

    for v_x in x_min..=x_max {
        for v_y in y_min..=y_max {
            if check_arc(v_x, v_y, &target) {
                count += 1;
            }
        }
    }

    count
}

fn calc_min_x(edge: isize) -> isize {
    (f32::sqrt(8. * edge as f32 + 1.) / 2.) as isize
}

fn calc_max_y(edge: isize) -> isize {
    isize::abs(edge + 1)
}

fn check_arc(mut v_x: isize, mut v_y: isize, target: &Target) -> bool {
    let mut x = 0;
    let mut y = 0;

    loop {
        x += v_x;
        y += v_y;
        v_y -= 1;
        if v_x != 0 {
            v_x -= isize::abs(v_x) / v_x;
        }

        match target.check((x, y)) {
            CheckResult::Fail => {
                return false;
            }
            CheckResult::Hit => {
                return true;
            }
            CheckResult::Continue => {
                continue;
            }
        }
    }
}

struct Target {
    x: Range<isize>,
    y: Range<isize>,
}

enum CheckResult {
    Continue,
    Hit,
    Fail,
}

impl Target {
    fn check(&self, position: (isize, isize)) -> CheckResult {
        if self.x.contains(&position.0) && self.y.contains(&position.1) {
            CheckResult::Hit
        } else if position.0 > self.x.end || position.1 < self.y.start {
            // If we are past the right side of the target area or the bottom of the target area
            // Then we'll never hit, since air resistance / gravity can't reverse those locations
            CheckResult::Fail
        } else {
            CheckResult::Continue
        }
    }
}

impl<'a> From<&'a str> for Target {
    fn from(input: &'a str) -> Self {
        let mut ranges = input
            .trim_start_matches("target area: x=")
            .split(", y=")
            .map(|range| range.split("..").map(|val| val.parse::<isize>().unwrap()));

        let mut x_vals = ranges.next().unwrap();
        let mut y_vals = ranges.next().unwrap();

        let x = x_vals.next().unwrap()..x_vals.next().unwrap() + 1;
        let y = y_vals.next().unwrap()..y_vals.next().unwrap() + 1;

        Self { x, y }
    }
}

aoc_lib! { year = 2021 }
