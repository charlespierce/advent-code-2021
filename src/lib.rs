use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::{HashSet, VecDeque};

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut field = OctopusField::from(input);

    (0..100).fold(0, |acc, _| acc + field.step())
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut field = OctopusField::from(input);

    (0..)
        .find(|_| {
            let flashes = field.step();

            flashes == 100
        })
        .unwrap()
        + 1
}

struct OctopusField {
    octopi: Vec<u8>,
}

impl<'a> From<&'a str> for OctopusField {
    fn from(input: &'a str) -> Self {
        Self {
            octopi: input
                .lines()
                .flat_map(|line| line.chars().map(|chr| chr.to_digit(10).unwrap() as u8))
                .collect(),
        }
    }
}

impl OctopusField {
    fn get_mut(&mut self, point: Point) -> &mut u8 {
        &mut self.octopi[point.row * 10 + point.column]
    }

    fn step(&mut self) -> usize {
        let mut flashes = VecDeque::new();

        for (i, power) in self.octopi.iter_mut().enumerate() {
            *power += 1;

            if *power > 9 {
                flashes.push_back(Point::from_index(i));
            }
        }

        let mut visited = HashSet::new();

        while let Some(point) = flashes.pop_front() {
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            for neighbor in point.neighbors() {
                let power = self.get_mut(neighbor);
                *power += 1;

                if *power > 9 && !visited.contains(&neighbor) {
                    flashes.push_back(neighbor);
                }
            }
        }

        let mut flash_count = 0;
        for power in self.octopi.iter_mut() {
            if *power > 9 {
                *power = 0;
                flash_count += 1;
            }
        }

        flash_count
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Debug))]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn from_index(index: usize) -> Self {
        Point {
            row: index / 10,
            column: index % 10,
        }
    }

    fn neighbors(self) -> impl Iterator<Item = Point> {
        let column_lower = self.column.saturating_sub(1);
        let column_upper = if self.column < 9 {
            self.column + 1
        } else {
            self.column
        };

        let row_lower = self.row.saturating_sub(1);
        let row_upper = if self.row < 9 { self.row + 1 } else { self.row };

        (row_lower..=row_upper).flat_map(move |row| {
            (column_lower..=column_upper).filter_map(move |column| {
                let point = Point { row, column };
                if point != self {
                    Some(point)
                } else {
                    None
                }
            })
        })
    }
}

aoc_lib! { year = 2021 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let neighbors1 = Point::default().neighbors().collect::<Vec<_>>();
        assert_eq!(
            neighbors1,
            [
                Point { row: 0, column: 1 },
                Point { row: 1, column: 0 },
                Point { row: 1, column: 1 },
            ]
        );

        let neighbors2 = Point { row: 9, column: 9 }.neighbors().collect::<Vec<_>>();
        assert_eq!(
            neighbors2,
            [
                Point { row: 8, column: 8 },
                Point { row: 8, column: 9 },
                Point { row: 9, column: 8 },
            ]
        );

        let neighbors3 = Point { row: 5, column: 5 }.neighbors().collect::<Vec<_>>();
        assert_eq!(
            neighbors3,
            [
                Point { row: 4, column: 4 },
                Point { row: 4, column: 5 },
                Point { row: 4, column: 6 },
                Point { row: 5, column: 4 },
                Point { row: 5, column: 6 },
                Point { row: 6, column: 4 },
                Point { row: 6, column: 5 },
                Point { row: 6, column: 6 },
            ]
        );
    }
}
