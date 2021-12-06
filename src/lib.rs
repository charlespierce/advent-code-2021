use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
use std::collections::HashMap;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Segment> {
    input.lines().map(Segment::from).collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &[Segment]) -> usize {
    let mut covered_points: HashMap<Point, usize> = HashMap::new();

    for segment in input
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical())
    {
        for point in segment.covered() {
            *covered_points.entry(point).or_default() += 1;
        }
    }

    covered_points.values().filter(|count| **count > 1).count()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[Segment]) -> usize {
    let mut covered_points: HashMap<Point, usize> = HashMap::new();

    for segment in input {
        for point in segment.covered() {
            *covered_points.entry(point).or_default() += 1;
        }
    }

    covered_points.values().filter(|count| **count > 1).count()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl<'a> From<&'a str> for Point {
    fn from(input: &'a str) -> Self {
        let mut values = input.split(',');

        let x = values.next().unwrap().parse().unwrap();
        let y = values.next().unwrap().parse().unwrap();

        Point { x, y }
    }
}

#[derive(Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn covered(&self) -> Covered {
        let kind = if self.is_horizontal() {
            SegmentKind::Horizontal
        } else if self.is_vertical() {
            SegmentKind::Vertical
        } else {
            SegmentKind::Diagonal
        };
        Covered {
            start: self.start,
            end: self.end,
            step: Some(0),
            kind,
        }
    }
}

impl<'a> From<&'a str> for Segment {
    fn from(input: &'a str) -> Self {
        let mut values = input.split(" -> ");

        let start = values.next().unwrap().into();
        let end = values.next().unwrap().into();

        Segment { start, end }
    }
}

struct Covered {
    start: Point,
    end: Point,
    step: Option<usize>,
    kind: SegmentKind,
}

impl Iterator for Covered {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let step = self.step?;

        let value = match self.kind {
            SegmentKind::Horizontal => {
                let x = if self.end.x > self.start.x {
                    self.start.x + step
                } else {
                    self.start.x - step
                };

                Point { x, y: self.start.y }
            }
            SegmentKind::Vertical => {
                let y = if self.end.y > self.start.y {
                    self.start.y + step
                } else {
                    self.start.y - step
                };
                Point { x: self.start.x, y }
            }
            SegmentKind::Diagonal => {
                let (x, y) = match (self.end.x > self.start.x, self.end.y > self.start.y) {
                    (true, true) => (self.start.x + step, self.start.y + step),
                    (true, false) => (self.start.x + step, self.start.y - step),
                    (false, true) => (self.start.x - step, self.start.y + step),
                    (false, false) => (self.start.x - step, self.start.y - step),
                };
                Point { x, y }
            }
        };

        self.step = (value != self.end).then(|| step + 1);

        Some(value)
    }
}

enum SegmentKind {
    Horizontal,
    Vertical,
    Diagonal,
}

aoc_lib! { year = 2021 }
