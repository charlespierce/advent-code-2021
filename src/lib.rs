use std::array::IntoIter;
use std::cmp::Ordering;
use std::str::Chars;

use aoc_runner_derive::{aoc, aoc_lib};

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> usize {
    Parser::new(input).parse_packet().version_sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &str) -> usize {
    Parser::new(input).parse_packet().value()
}

struct Parser<'a> {
    bits: Bits<'a>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bits: Bits::new(input),
            pos: 0,
        }
    }

    fn next(&mut self) -> Option<u8> {
        let bit = self.bits.next()?;
        self.pos += 1;

        Some(bit)
    }

    fn take(&mut self, count: u8) -> u16 {
        let mut result = 0;

        for _ in 0..count {
            result <<= 1;
            result |= self.next().unwrap() as u16;
        }

        result
    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.take(3);
        match self.take(3) {
            0 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::Sum(children),
                }
            }
            1 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::Product(children),
                }
            }
            2 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::Minimum(children),
                }
            }
            3 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::Maximum(children),
                }
            }
            4 => {
                let value = self.parse_literal();
                Packet {
                    version,
                    kind: PacketKind::Literal(value),
                }
            }
            5 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::GreaterThan(children),
                }
            }
            6 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::LessThan(children),
                }
            }
            7 => {
                let children = self.parse_operator();
                Packet {
                    version,
                    kind: PacketKind::Equal(children),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_literal(&mut self) -> usize {
        let mut result = 0;

        loop {
            let segment = self.take(5);
            result <<= 4;
            result |= (segment & 15) as usize;

            if segment & 16 == 0 {
                break;
            }
        }

        result
    }

    fn parse_operator(&mut self) -> Vec<Packet> {
        match self.next().unwrap() {
            0 => self.parse_by_length(),
            1 => self.parse_by_count(),
            _ => unreachable!(),
        }
    }

    fn parse_by_length(&mut self) -> Vec<Packet> {
        let length = self.take(15);
        let pos = self.pos;
        let mut result = Vec::new();

        loop {
            result.push(self.parse_packet());

            let diff = (self.pos - pos) as u16;
            match diff.cmp(&length) {
                Ordering::Equal => break,
                Ordering::Greater => panic!("Incorrect number of bytes found!"),
                _ => {}
            }
        }

        result
    }

    fn parse_by_count(&mut self) -> Vec<Packet> {
        let count = self.take(11);
        let mut result = Vec::with_capacity(count as usize);

        for _ in 0..count {
            result.push(self.parse_packet());
        }

        result
    }
}

struct Packet {
    version: u16,
    kind: PacketKind,
}

impl Packet {
    fn version_sum(&self) -> usize {
        use PacketKind::*;

        match &self.kind {
            Literal(_) => self.version as usize,
            Sum(children)
            | Product(children)
            | Minimum(children)
            | Maximum(children)
            | GreaterThan(children)
            | LessThan(children)
            | Equal(children) => {
                self.version as usize + children.iter().map(Packet::version_sum).sum::<usize>()
            }
        }
    }

    fn value(&self) -> usize {
        use PacketKind::*;

        match &self.kind {
            Literal(value) => *value as usize,
            Sum(children) => children.iter().map(Packet::value).sum(),
            Product(children) => children.iter().map(Packet::value).product(),
            Minimum(children) => children.iter().map(Packet::value).min().unwrap(),
            Maximum(children) => children.iter().map(Packet::value).max().unwrap(),
            GreaterThan(children) => {
                if children[0].value() > children[1].value() {
                    1
                } else {
                    0
                }
            }
            LessThan(children) => {
                if children[0].value() < children[1].value() {
                    1
                } else {
                    0
                }
            }
            Equal(children) => {
                if children[0].value() == children[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

enum PacketKind {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equal(Vec<Packet>),
}

struct Bits<'a> {
    hex_chars: Chars<'a>,
    bits: IntoIter<u8, 4>,
}

impl<'a> Bits<'a> {
    fn new(input: &'a str) -> Self {
        let mut hex_chars = input.chars();
        let bits = IntoIterator::into_iter(hex_to_bits(hex_chars.next().unwrap()));

        Self { hex_chars, bits }
    }
}

impl<'a> Iterator for Bits<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.bits.next() {
            Some(bit) => Some(bit),
            None => {
                self.bits = IntoIterator::into_iter(hex_to_bits(self.hex_chars.next()?));
                self.bits.next()
            }
        }
    }
}

fn hex_to_bits(hex: char) -> [u8; 4] {
    match hex {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => unreachable!(),
    }
}

aoc_lib! { year = 2021 }
