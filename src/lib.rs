use aoc_runner_derive::{aoc, aoc_lib};

pub mod dijkstra;

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> usize {
    let cave = Cave::from(input);

    dijkstra::Dijkstra::new(Point::new(0, 0), |p| cave.end(*p), |p| cave.neighbors(*p))
        .next()
        .unwrap()
        .1
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut cave = Cave::from(input);
    cave.extend_right(4);
    cave.extend_down(4);

    dijkstra::Dijkstra::new(Point::new(0, 0), |p| cave.end(*p), |p| cave.neighbors(*p))
        .next()
        .unwrap()
        .1
}

struct Cave {
    risks: Vec<Vec<usize>>,
}

impl Cave {
    fn get(&self, point: Point) -> usize {
        self.risks
            .get(point.row)
            .and_then(|row| row.get(point.column).copied())
            .unwrap()
    }

    fn end(&self, point: Point) -> bool {
        point.row == self.risks.len() - 1 && point.column == self.risks[0].len() - 1
    }

    fn neighbors(&self, point: Point) -> Vec<(Point, usize)> {
        let mut neighbors = Vec::with_capacity(4);

        if point.row > 0 {
            let neighbor = Point::new(point.row - 1, point.column);
            neighbors.push((neighbor, self.get(neighbor)));
        }

        if point.column > 0 {
            let neighbor = Point::new(point.row, point.column - 1);
            neighbors.push((neighbor, self.get(neighbor)));
        }

        if point.column < self.risks[0].len() - 1 {
            let neighbor = Point::new(point.row, point.column + 1);
            neighbors.push((neighbor, self.get(neighbor)));
        }

        if point.row < self.risks.len() - 1 {
            let neighbor = Point::new(point.row + 1, point.column);
            neighbors.push((neighbor, self.get(neighbor)));
        }

        neighbors
    }

    fn extend_right(&mut self, times: usize) {
        let base = self.risks.clone();

        for i in 0..times {
            for (index, row) in self.risks.iter_mut().enumerate() {
                row.extend(base[index].iter().map(|risk| (*risk + i) % 9 + 1));
            }
        }
    }

    fn extend_down(&mut self, times: usize) {
        let base = self.risks.clone();

        for i in 0..times {
            self.risks.extend(
                base.iter()
                    .map(|row| row.iter().map(|risk| (*risk + i) % 9 + 1).collect()),
            );
        }
    }
}

impl<'a> From<&'a str> for Cave {
    fn from(input: &'a str) -> Cave {
        let risks = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|chr| chr.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Cave { risks }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl dijkstra::Node for Point {
    type Id = Self;

    fn id(&self) -> Self {
        *self
    }
}

aoc_lib! { year = 2021 }
