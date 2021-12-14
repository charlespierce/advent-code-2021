use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashSet;

#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (mut paper, folds) = parse_input(input);

    paper.fold(folds[0]);

    paper.points.len()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> String {
    let (mut paper, folds) = parse_input(input);

    for fold in folds {
        paper.fold(fold);
    }

    let max_point = paper.points.iter().fold(Point::default(), |acc, point| {
        let x = acc.x.max(point.x);
        let y = acc.y.max(point.y);

        Point { x, y }
    });

    let mut output = String::with_capacity((max_point.x + 1) * max_point.y);
    for y in 0..=max_point.y {
        for x in 0..=max_point.x {
            if paper.points.contains(&Point { x, y }) {
                output.push('#');
            } else {
                output.push(' ');
            }
        }
        output.push_str("\n                  ");
    }

    output
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let mut parts = input.split("\n\n");
    let paper = Paper::from(parts.next().unwrap());
    let folds = parts.next().unwrap().lines().map(Fold::from).collect();

    (paper, folds)
}

struct Paper {
    points: HashSet<Point>,
}

impl Paper {
    fn fold(&mut self, axis: Fold) {
        let new_points = self.points.iter().copied().map(|p| p.fold(axis)).collect();

        self.points = new_points;
    }
}

impl<'a> From<&'a str> for Paper {
    fn from(input: &'a str) -> Self {
        let points = input.lines().map(Point::from).collect();

        Paper { points }
    }
}

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

impl<'a> From<&'a str> for Fold {
    fn from(input: &'a str) -> Self {
        let mut parts = input.split('=');

        match (parts.next(), parts.next()) {
            (Some("fold along x"), Some(val)) => Fold::X(val.parse().unwrap()),
            (Some("fold along y"), Some(val)) => Fold::Y(val.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn fold(self, axis: Fold) -> Point {
        match axis {
            Fold::X(value) => {
                let x = if self.x < value {
                    self.x
                } else {
                    2 * value - self.x
                };

                Point { x, y: self.y }
            }
            Fold::Y(value) => {
                let y = if self.y < value {
                    self.y
                } else {
                    2 * value - self.y
                };

                Point { x: self.x, y }
            }
        }
    }
}

impl<'a> From<&'a str> for Point {
    fn from(input: &'a str) -> Self {
        let mut parts = input.split(',').map(|d| d.parse().unwrap());

        Point {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
        }
    }
}

aoc_lib! { year = 2021 }
