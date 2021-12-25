use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashMap;

#[aoc(day25, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut floor = SeaFloor::from(input);
    let mut step = 0;

    loop {
        step += 1;

        let moved_east = floor.move_east();
        let moved_south = floor.move_south();

        if !moved_east && !moved_south {
            break step;
        }

        if step % 1000 == 0 {
            println!("Step {}", step);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

struct SeaFloor {
    map: HashMap<Point, Cucumber>,
    rows: usize,
    cols: usize,
}

impl SeaFloor {
    fn move_east(&mut self) -> bool {
        let mut updated = HashMap::new();
        let mut moved = false;

        for (&pos, &cucumber) in self.map.iter() {
            match cucumber {
                Cucumber::South => {
                    updated.insert(pos, cucumber);
                }
                Cucumber::East => {
                    let next = self.wrap(pos.row, pos.col + 1);
                    match self.map.get(&next) {
                        Some(_) => {
                            updated.insert(pos, cucumber);
                        }
                        None => {
                            moved = true;
                            updated.insert(next, cucumber);
                        }
                    }
                }
            }
        }

        self.map = updated;
        moved
    }

    fn move_south(&mut self) -> bool {
        let mut updated = HashMap::new();
        let mut moved = false;

        for (&pos, &cucumber) in self.map.iter() {
            match cucumber {
                Cucumber::East => {
                    updated.insert(pos, cucumber);
                }
                Cucumber::South => {
                    let next = self.wrap(pos.row + 1, pos.col);
                    match self.map.get(&next) {
                        Some(_) => {
                            updated.insert(pos, cucumber);
                        }
                        None => {
                            moved = true;
                            updated.insert(next, cucumber);
                        }
                    }
                }
            }
        }

        self.map = updated;
        moved
    }

    fn wrap(&self, row: usize, col: usize) -> Point {
        Point {
            row: row % self.rows,
            col: col % self.cols,
        }
    }
}

impl<'a> From<&'a str> for SeaFloor {
    fn from(input: &'a str) -> Self {
        let mut map = HashMap::new();
        let mut row = 0;
        let mut col = 0;

        for line in input.lines() {
            col = 0;
            for chr in line.chars() {
                match chr {
                    '>' => {
                        map.insert(Point::new(row, col), Cucumber::East);
                    }
                    'v' => {
                        map.insert(Point::new(row, col), Cucumber::South);
                    }
                    _ => {}
                }
                col += 1;
            }
            row += 1;
        }

        Self {
            map,
            rows: row,
            cols: col,
        }
    }
}

#[derive(Clone, Copy)]
enum Cucumber {
    South,
    East,
}

aoc_lib! { year = 2021 }
