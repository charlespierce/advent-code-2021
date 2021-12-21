use aoc_runner_derive::{aoc, aoc_lib};
use std::fmt;

#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut pairs = input.lines().map(|line| Pair::parse(line).0);
    let first = pairs.next().unwrap();

    let reduced = pairs.fold(first, |acc, pair| acc.add(pair));

    reduced.magnitude()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &str) -> usize {
    let pairs: Vec<_> = input.lines().map(|line| Pair::parse(line).0).collect();

    let mut max_magnitude = 0;

    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i != j {
                let sum = pairs[i].clone().add(pairs[j].clone());
                let magnitude = sum.magnitude();

                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                }
            }
        }
    }

    max_magnitude
}

#[derive(Clone)]
struct Pair {
    lhs: Expression,
    rhs: Expression,
}

impl Pair {
    fn parse(input: &str) -> (Pair, &str) {
        let left = input.strip_prefix('[').unwrap();
        let (lhs, left) = Expression::parse(left);
        let left = left.strip_prefix(',').unwrap();
        let (rhs, left) = Expression::parse(left);
        let left = left.strip_prefix(']').unwrap();

        (Pair { lhs, rhs }, left)
    }

    fn add(self, other: Self) -> Self {
        let mut sum = Self {
            lhs: Expression::Pair(Box::new(self)),
            rhs: Expression::Pair(Box::new(other)),
        };

        loop {
            if sum.explode() {
                continue;
            } else if !sum.split() {
                break;
            }
        }

        sum
    }

    fn explode(&mut self) -> bool {
        let mut last: Option<&mut u8> = None;
        let mut next: Option<u8> = None;
        let mut found = false;

        let mut q = vec![(0, &mut self.rhs), (0, &mut self.lhs)];

        while let Some((depth, expr)) = q.pop() {
            match (next, expr) {
                (Some(add), Expression::Literal(value)) => {
                    *value += add;
                    break;
                }
                (Some(_), Expression::Pair(pair)) => {
                    q.push((depth + 1, &mut pair.rhs));
                    q.push((depth + 1, &mut pair.lhs));
                }
                (None, Expression::Literal(value)) => {
                    last = Some(value);
                }
                (None, pair) => {
                    if depth == 3 {
                        if let Some(value) = &mut last {
                            **value += pair.pair_value().lhs.literal_value();
                        }
                        next = Some(pair.pair_value().rhs.literal_value());
                        *pair = Expression::Literal(0);
                        found = true;
                    } else {
                        let pair = pair.pair_value();
                        q.push((depth + 1, &mut pair.rhs));
                        q.push((depth + 1, &mut pair.lhs));
                    }
                }
            }
        }

        found
    }

    fn split(&mut self) -> bool {
        let mut q = vec![&mut self.rhs, &mut self.lhs];

        while let Some(curr) = q.pop() {
            match curr {
                Expression::Pair(pair) => {
                    q.push(&mut pair.rhs);
                    q.push(&mut pair.lhs);
                }
                literal => {
                    let value = literal.literal_value();
                    if value > 9 {
                        let lhs = value / 2;
                        let rhs = value - lhs;

                        *literal = Expression::Pair(Box::new(Pair {
                            lhs: Expression::Literal(lhs),
                            rhs: Expression::Literal(rhs),
                        }));
                        return true;
                    }
                }
            }
        }

        false
    }

    fn magnitude(&self) -> usize {
        3 * self.lhs.magnitude() + 2 * self.rhs.magnitude()
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.lhs, self.rhs)
    }
}

#[derive(Clone)]
enum Expression {
    Literal(u8),
    Pair(Box<Pair>),
}

impl Expression {
    fn parse(input: &str) -> (Self, &str) {
        match input.chars().next().unwrap() {
            '[' => {
                let (pair, left) = Pair::parse(input);
                (Expression::Pair(Box::new(pair)), left)
            }
            digit => {
                let value = digit.to_digit(10).unwrap() as u8;
                let left = &input[1..];

                (Expression::Literal(value), left)
            }
        }
    }

    fn literal_value(&self) -> u8 {
        match self {
            Expression::Literal(value) => *value,
            Expression::Pair(_) => panic!("Expected literal value"),
        }
    }

    fn pair_value(&mut self) -> &mut Pair {
        match self {
            Expression::Literal(_) => panic!("Expected pair value"),
            Expression::Pair(pair) => pair,
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Expression::Literal(value) => *value as usize,
            Expression::Pair(pair) => pair.magnitude(),
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Pair(pair) => write!(f, "{:?}", pair),
        }
    }
}

aoc_lib! { year = 2021 }
