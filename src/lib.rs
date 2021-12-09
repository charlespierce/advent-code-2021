use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
use std::collections::{BinaryHeap, HashSet, VecDeque};

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> HeightMap {
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    HeightMap { heights }
}

pub struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn low_points(&self) -> Vec<u8> {
        let mut values = Vec::new();
        let rows = self.heights.len();
        let cols = self.heights[0].len();

        for row in 0..rows {
            for col in 0..cols {
                let cur = self.heights[row][col];
                if self.adjacent(row, col).all(|adj| adj > cur) {
                    values.push(cur);
                }
            }
        }

        values
    }

    fn adjacent(&self, row: usize, column: usize) -> impl Iterator<Item = u8> + '_ {
        self.heights
            .get(row)
            .into_iter()
            .flat_map(move |row| {
                row.get(column - 1)
                    .into_iter()
                    .chain(row.get(column + 1).into_iter())
                    .copied()
            })
            .chain(
                self.heights
                    .get(row - 1)
                    .and_then(move |row| row.get(column))
                    .into_iter()
                    .copied(),
            )
            .chain(
                self.heights
                    .get(row + 1)
                    .and_then(move |row| row.get(column))
                    .into_iter()
                    .copied(),
            )
    }

    fn basins(&self) -> Basins<'_> {
        Basins {
            heights: self,
            visited: HashSet::new(),
            start: None,
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &HeightMap) -> usize {
    input
        .low_points()
        .into_iter()
        .map(|height| height as usize + 1)
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &HeightMap) -> usize {
    let mut heap: BinaryHeap<_> = input.basins().collect();

    heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap()
}

type Point = (usize, usize);

struct Basins<'a> {
    heights: &'a HeightMap,
    visited: HashSet<Point>,
    start: Option<Point>,
}

impl<'a> Basins<'a> {
    fn find_basin(&self) -> Option<Point> {
        let (mut row, mut col) = self.start.unwrap_or((0, 0));
        let rows = self.heights.heights.len();
        let cols = self.heights.heights[0].len();

        while row < rows {
            while col < cols {
                if !self.visited.contains(&(row, col)) && self.heights.heights[row][col] < 9 {
                    return Some((row, col));
                }
                col += 1;
            }
            col = 0;
            row += 1;
        }

        None
    }

    fn adjacent_points(&self, point: Point) -> Vec<Point> {
        let mut points = Vec::new();

        if point.0 > 0 {
            points.push((point.0 - 1, point.1));
        }

        if point.0 < self.heights.heights.len() - 1 {
            points.push((point.0 + 1, point.1));
        }

        if point.1 > 0 {
            points.push((point.0, point.1 - 1));
        }

        if point.1 < self.heights.heights[0].len() - 1 {
            points.push((point.0, point.1 + 1));
        }

        points
    }

    fn explore_basin(&mut self, start: Point) -> usize {
        let mut size = 0;
        let mut candidates = VecDeque::new();
        candidates.push_back(start);

        while let Some(point) = candidates.pop_front() {
            if !self.visited.contains(&point) {
                size += 1;
                self.visited.insert(point);

                for adj in self.adjacent_points(point) {
                    if !self.visited.contains(&adj) && self.heights.heights[adj.0][adj.1] < 9 {
                        candidates.push_back(adj);
                    }
                }
            }
        }

        size
    }
}

impl<'a> Iterator for Basins<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let start = self.find_basin()?;

        self.start = Some(start);

        Some(self.explore_basin(start))
    }
}

aoc_lib! { year = 2021 }
