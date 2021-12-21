use aoc_runner_derive::{aoc, aoc_lib};

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (mask, mut image) = parse_input(input);

    image.enhance(&mask);
    image.enhance(&mask);

    image.lit_pixels()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (mask, mut image) = parse_input(input);

    for _ in 0..50 {
        image.enhance(&mask);
    }

    image.lit_pixels()
}

fn parse_input(input: &str) -> (Vec<u8>, Image) {
    let mut parts = input.split("\n\n");
    let mask = parts
        .next()
        .unwrap()
        .chars()
        .map(|chr| if chr == '.' { 0 } else { 1 })
        .collect();
    let image = parts.next().unwrap().into();

    (mask, image)
}

struct Image {
    outside: u8,
    data: Vec<Vec<u8>>,
}

impl Image {
    fn enhance(&mut self, mask: &[u8]) {
        let mut data = Vec::with_capacity(self.data.len() + 2);
        for i in 0..self.data.len() + 2 {
            let mut line = Vec::with_capacity(self.data[0].len() + 2);
            for j in 0..self.data[0].len() + 2 {
                let index = self.window(i as isize - 1, j as isize - 1);
                line.push(mask[index]);
            }
            data.push(line);
        }

        self.data = data;
        self.outside = if self.outside == 0 {
            mask[0]
        } else {
            mask[511]
        };
    }

    fn lit_pixels(&self) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter().filter(|pixel| **pixel == 1))
            .count()
    }

    fn window(&self, row: isize, col: isize) -> usize {
        let mut value = 0;
        for i in row - 1..=row + 1 {
            for j in col - 1..=col + 1 {
                value <<= 1;
                value |= self.get(i, j) as usize;
            }
        }
        value
    }

    fn get(&self, row: isize, col: isize) -> u8 {
        if row < 0 || col < 0 {
            return self.outside;
        }
        let row = row as usize;
        let col = col as usize;

        if row >= self.data.len() || col >= self.data[0].len() {
            return self.outside;
        }

        self.data[row][col]
    }
}

impl<'a> From<&'a str> for Image {
    fn from(input: &'a str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|chr| if chr == '.' { 0 } else { 1 })
                    .collect()
            })
            .collect();

        Self { outside: 0, data }
    }
}

aoc_lib! { year = 2021 }
