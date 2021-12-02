use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

#[derive(Clone, Copy)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

trait Submarine: Default {
    fn execute_command(&mut self, command: Command);
}

#[derive(Default)]
struct Part1 {
    horizontal: usize,
    depth: usize,
}

impl Submarine for Part1 {
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Forward(dist) => self.horizontal += dist,
            Command::Down(dist) => self.depth += dist,
            Command::Up(dist) => self.depth -= dist,
        }
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            match (parts.next(), parts.next()) {
                (Some("forward"), Some(amount)) => Command::Forward(amount.parse().unwrap()),
                (Some("down"), Some(amount)) => Command::Down(amount.parse().unwrap()),
                (Some("up"), Some(amount)) => Command::Up(amount.parse().unwrap()),
                _ => panic!("Invalid input"),
            }
        })
        .collect()
}

fn run_submarine<S>(commands: &[Command]) -> S
where
    S: Submarine,
{
    commands
        .iter()
        .copied()
        .fold(S::default(), |mut sub, command| {
            sub.execute_command(command);
            sub
        })
}

#[aoc(day2, part1)]
pub fn solve_par1t(input: &[Command]) -> usize {
    let position = run_submarine::<Part1>(input);

    position.horizontal * position.depth
}

#[derive(Default)]
struct Part2 {
    horizontal: usize,
    aim: usize,
    depth: usize,
}

impl Submarine for Part2 {
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Forward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
            Command::Down(amount) => {
                self.aim += amount;
            }
            Command::Up(amount) => {
                self.aim -= amount;
            }
        }
    }
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> usize {
    let position = run_submarine::<Part2>(input);

    position.horizontal * position.depth
}

aoc_lib! { year = 2021 }
