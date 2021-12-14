use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::{HashMap, VecDeque};

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> usize {
    let cave = Cave::from(input);

    cave.paths().count()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> usize {
    let cave = Cave::from(input);

    cave.paths2().count()
}

struct Cave<'a> {
    tunnels: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Cave<'a> {
    fn paths(&self) -> Paths<'_> {
        let mut queue = VecDeque::new();
        queue.push_back(("start", "-start".into()));
        Paths { cave: self, queue }
    }

    fn paths2(&self) -> Paths2<'_> {
        let mut queue = VecDeque::new();
        queue.push_back(("start", "start".into()));
        Paths2 { cave: self, queue }
    }
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(input: &'a str) -> Self {
        let mut tunnels: HashMap<_, Vec<_>> = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split('-');
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();

            tunnels.entry(first).or_default().push(second);
            tunnels.entry(second).or_default().push(first);
        }

        Self { tunnels }
    }
}

struct Paths<'a> {
    cave: &'a Cave<'a>,
    queue: VecDeque<(&'a str, String)>,
}

impl<'a> Iterator for Paths<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        while let Some((current, path)) = self.queue.pop_front() {
            if current == "end" {
                return Some(path);
            }

            let adjacent = self
                .cave
                .tunnels
                .get(current)
                .into_iter()
                .flatten()
                .filter(|cavern| {
                    cavern.chars().next().unwrap().is_uppercase()
                        || !path.contains(&format!("-{}", cavern))
                });

            for cav in adjacent {
                self.queue.push_back((cav, format!("{}-{}", path, cav)));
            }
        }

        None
    }
}

struct Paths2<'a> {
    cave: &'a Cave<'a>,
    queue: VecDeque<(&'a str, String)>,
}

impl<'a> Iterator for Paths2<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        while let Some((current, path)) = self.queue.pop_front() {
            if current == "end" {
                return Some(path);
            }

            let adjacent = self
                .cave
                .tunnels
                .get(current)
                .into_iter()
                .flatten()
                .filter(|cavern| **cavern != "start");

            for cav in adjacent {
                match (
                    cav.starts_with(|c: char| c.is_uppercase()),
                    path.split('-').filter(|c| c == cav).count(),
                ) {
                    (true, _) => {
                        self.queue.push_back((cav, format!("{}-{}", path, cav)));
                    }
                    (false, 0) => {
                        self.queue.push_back((cav, format!("{}-{}", path, cav)));
                    }
                    (false, 1) => {
                        if !path.starts_with('!') {
                            self.queue.push_back((cav, format!("!{}-{}", path, cav)));
                        }
                    }
                    (false, _) => {}
                }
            }
        }

        None
    }
}

aoc_lib! { year = 2021 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let cave = Cave::from(
            r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        );

        let mut count = 0;
        for path in cave.paths2() {
            println!("{}", path);
            count += 1;
        }

        println!("Count: {}", count);

        assert_eq!(count, 36);
    }
}
