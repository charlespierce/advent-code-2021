use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashMap;

pub mod dijkstra;

#[aoc(day23, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut spots = HashMap::new();

    let mut lines = input.lines().skip(2).take(2);
    let top = lines.next().unwrap();

    spots.insert(Position::A1, top[3..4].into());
    spots.insert(Position::B1, top[5..6].into());
    spots.insert(Position::C1, top[7..8].into());
    spots.insert(Position::D1, top[9..10].into());

    let bottom = lines.next().unwrap();

    spots.insert(Position::A2, bottom[3..4].into());
    spots.insert(Position::B2, bottom[5..6].into());
    spots.insert(Position::C2, bottom[7..8].into());
    spots.insert(Position::D2, bottom[9..10].into());

    spots.insert(Position::A3, PodKind::A);
    spots.insert(Position::A4, PodKind::A);
    spots.insert(Position::B3, PodKind::B);
    spots.insert(Position::B4, PodKind::B);
    spots.insert(Position::C3, PodKind::C);
    spots.insert(Position::C4, PodKind::C);
    spots.insert(Position::D3, PodKind::D);
    spots.insert(Position::D4, PodKind::D);

    let burrow = Burrow { spots };

    dijkstra::Dijkstra::new(burrow, |burrow| burrow.success(), |burrow| burrow.moves())
        .next()
        .unwrap()
        .1
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut spots = HashMap::new();

    let mut lines = input.lines().skip(2).take(2);
    let top = lines.next().unwrap();

    spots.insert(Position::A1, top[3..4].into());
    spots.insert(Position::B1, top[5..6].into());
    spots.insert(Position::C1, top[7..8].into());
    spots.insert(Position::D1, top[9..10].into());

    let bottom = lines.next().unwrap();

    spots.insert(Position::A4, bottom[3..4].into());
    spots.insert(Position::B4, bottom[5..6].into());
    spots.insert(Position::C4, bottom[7..8].into());
    spots.insert(Position::D4, bottom[9..10].into());

    spots.insert(Position::A2, PodKind::D);
    spots.insert(Position::B2, PodKind::C);
    spots.insert(Position::C2, PodKind::B);
    spots.insert(Position::D2, PodKind::A);
    spots.insert(Position::A3, PodKind::D);
    spots.insert(Position::B3, PodKind::B);
    spots.insert(Position::C3, PodKind::A);
    spots.insert(Position::D3, PodKind::C);

    let burrow = Burrow { spots };

    dijkstra::Dijkstra::new(burrow, |burrow| burrow.success(), |burrow| burrow.moves())
        .next()
        .unwrap()
        .1
}

#[derive(Clone)]
struct Burrow {
    spots: HashMap<Position, PodKind>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BurrowId {
    a: (Position, Position, Position, Position),
    b: (Position, Position, Position, Position),
    c: (Position, Position, Position, Position),
    d: (Position, Position, Position, Position),
}

impl dijkstra::Node for Burrow {
    type Id = BurrowId;

    fn id(&self) -> Self::Id {
        let mut sorted: Vec<_> = self.spots.iter().collect();
        sorted.sort_by(|(_, a), (_, b)| a.cmp(b));

        BurrowId {
            a: (*sorted[0].0, *sorted[1].0, *sorted[2].0, *sorted[3].0),
            b: (*sorted[4].0, *sorted[5].0, *sorted[6].0, *sorted[7].0),
            c: (*sorted[8].0, *sorted[9].0, *sorted[10].0, *sorted[11].0),
            d: (*sorted[12].0, *sorted[13].0, *sorted[14].0, *sorted[15].0),
        }
    }
}

impl Burrow {
    fn moves(&self) -> Vec<(Burrow, usize)> {
        self.spots
            .iter()
            .flat_map(|(start, pod)| {
                start
                    .possible_moves(*pod)
                    .into_iter()
                    .map(move |end| (*start, end, *pod))
            })
            .filter_map(move |(start, end, pod)| {
                if end.room().into_iter().any(|spot| {
                    self.spots
                        .get(&spot)
                        .map(|kind| *kind != pod)
                        .unwrap_or_default()
                }) {
                    return None;
                }

                if self.spots.contains_key(&end) {
                    return None;
                }

                let steps = start.steps(end);
                let cost = (steps.len() + 1) * pod.cost();

                if steps.iter().all(|step| !self.spots.contains_key(step)) {
                    let mut new_map = self.clone();
                    new_map.spots.remove(&start);
                    new_map.spots.insert(end, pod);
                    Some((new_map, cost))
                } else {
                    None
                }
            })
            .collect()
    }

    fn success(&self) -> bool {
        use Position::*;

        self.spots.get(&A1) == Some(&PodKind::A)
            && self.spots.get(&A2) == Some(&PodKind::A)
            && self.spots.get(&A3) == Some(&PodKind::A)
            && self.spots.get(&A4) == Some(&PodKind::A)
            && self.spots.get(&B1) == Some(&PodKind::B)
            && self.spots.get(&B2) == Some(&PodKind::B)
            && self.spots.get(&B3) == Some(&PodKind::B)
            && self.spots.get(&B4) == Some(&PodKind::B)
            && self.spots.get(&C1) == Some(&PodKind::C)
            && self.spots.get(&C2) == Some(&PodKind::C)
            && self.spots.get(&C3) == Some(&PodKind::C)
            && self.spots.get(&C4) == Some(&PodKind::C)
            && self.spots.get(&D1) == Some(&PodKind::D)
            && self.spots.get(&D2) == Some(&PodKind::D)
            && self.spots.get(&D3) == Some(&PodKind::D)
            && self.spots.get(&D4) == Some(&PodKind::D)
    }
}

impl Print for Burrow {
    fn print(&self) {
        println!("#############");

        print!("#");
        self.spots.get(&Position::U1).print();
        self.spots.get(&Position::U2).print();
        print!(".");
        self.spots.get(&Position::U3).print();
        print!(".");
        self.spots.get(&Position::U4).print();
        print!(".");
        self.spots.get(&Position::U5).print();
        print!(".");
        self.spots.get(&Position::U6).print();
        self.spots.get(&Position::U7).print();
        println!("#");

        print!("###");
        self.spots.get(&Position::A1).print();
        print!("#");
        self.spots.get(&Position::B1).print();
        print!("#");
        self.spots.get(&Position::C1).print();
        print!("#");
        self.spots.get(&Position::D1).print();
        println!("###");

        print!("  #");
        self.spots.get(&Position::A2).print();
        print!("#");
        self.spots.get(&Position::B2).print();
        print!("#");
        self.spots.get(&Position::C2).print();
        print!("#");
        self.spots.get(&Position::D2).print();
        println!("#");

        print!("  #");
        self.spots.get(&Position::A3).print();
        print!("#");
        self.spots.get(&Position::B3).print();
        print!("#");
        self.spots.get(&Position::C3).print();
        print!("#");
        self.spots.get(&Position::D3).print();
        println!("#");

        print!("  #");
        self.spots.get(&Position::A4).print();
        print!("#");
        self.spots.get(&Position::B4).print();
        print!("#");
        self.spots.get(&Position::C4).print();
        print!("#");
        self.spots.get(&Position::D4).print();
        println!("#");

        println!("  #########");
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PodKind {
    A,
    B,
    C,
    D,
}

impl PodKind {
    fn cost(self) -> usize {
        match self {
            PodKind::A => 1,
            PodKind::B => 10,
            PodKind::C => 100,
            PodKind::D => 1000,
        }
    }
}

trait Print {
    fn print(&self);
}

impl<'a> Print for Option<&'a PodKind> {
    fn print(&self) {
        match self {
            Some(PodKind::A) => print!("A"),
            Some(PodKind::B) => print!("B"),
            Some(PodKind::C) => print!("C"),
            Some(PodKind::D) => print!("D"),
            None => print!("."),
        }
    }
}

impl<'a> From<&'a str> for PodKind {
    fn from(input: &'a str) -> Self {
        match input {
            "A" => PodKind::A,
            "B" => PodKind::B,
            "C" => PodKind::C,
            "D" => PodKind::D,
            _ => panic!("Unexpected pod"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Position {
    U1,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    A1,
    A2,
    A3,
    A4,
    B1,
    B2,
    B3,
    B4,
    C1,
    C2,
    C3,
    C4,
    D1,
    D2,
    D3,
    D4,
    AS,
    BS,
    CS,
    DS,
}

impl Position {
    fn possible_moves(self, pod: PodKind) -> Vec<Position> {
        use Position::*;
        match self {
            U1 | U2 | U3 | U4 | U5 | U6 | U7 => match pod {
                PodKind::A => vec![A1, A2, A3, A4],
                PodKind::B => vec![B1, B2, B3, B4],
                PodKind::C => vec![C1, C2, C3, C4],
                PodKind::D => vec![D1, D2, D3, D4],
            },
            A1 | A2 | A3 | A4 | B1 | B2 | B3 | B4 | C1 | C2 | C3 | C4 | D1 | D2 | D3 | D4 => {
                vec![U1, U2, U3, U4, U5, U6, U7]
            }
            _ => panic!("Can't stop on the spacers"),
        }
    }

    fn steps(self, destination: Self) -> Vec<Position> {
        use Position::*;
        match (self, destination) {
            (U1, A1) | (A1, U1) => vec![U2, AS],
            (U1, A2) | (A2, U1) => vec![U2, AS, A1],
            (U1, A3) | (A3, U1) => vec![U2, AS, A1, A2],
            (U1, A4) | (A4, U1) => vec![U2, AS, A1, A2, A3],
            (U1, B1) | (B1, U1) => vec![U2, AS, U3, BS],
            (U1, B2) | (B2, U1) => vec![U2, AS, U3, BS, B1],
            (U1, B3) | (B3, U1) => vec![U2, AS, U3, BS, B1, B2],
            (U1, B4) | (B4, U1) => vec![U2, AS, U3, BS, B1, B2, B3],
            (U1, C1) | (C1, U1) => vec![U2, AS, U3, BS, U4, CS],
            (U1, C2) | (C2, U1) => vec![U2, AS, U3, BS, U4, CS, C1],
            (U1, C3) | (C3, U1) => vec![U2, AS, U3, BS, U4, CS, C1, C2],
            (U1, C4) | (C4, U1) => vec![U2, AS, U3, BS, U4, CS, C1, C2, C3],
            (U1, D1) | (D1, U1) => vec![U2, AS, U3, BS, U4, CS, U5, DS],
            (U1, D2) | (D2, U1) => vec![U2, AS, U3, BS, U4, CS, U5, DS, D1],
            (U1, D3) | (D3, U1) => vec![U2, AS, U3, BS, U4, CS, U5, DS, D1, D2],
            (U1, D4) | (D4, U1) => vec![U2, AS, U3, BS, U4, CS, U5, DS, D1, D2, D3],
            (U2, A1) | (A1, U2) => vec![AS],
            (U2, A2) | (A2, U2) => vec![AS, A1],
            (U2, A3) | (A3, U2) => vec![AS, A1, A2],
            (U2, A4) | (A4, U2) => vec![AS, A1, A2, A3],
            (U2, B1) | (B1, U2) => vec![AS, U3, BS],
            (U2, B2) | (B2, U2) => vec![AS, U3, BS, B1],
            (U2, B3) | (B3, U2) => vec![AS, U3, BS, B1, B2],
            (U2, B4) | (B4, U2) => vec![AS, U3, BS, B1, B2, B3],
            (U2, C1) | (C1, U2) => vec![AS, U3, BS, U4, CS],
            (U2, C2) | (C2, U2) => vec![AS, U3, BS, U4, CS, C1],
            (U2, C3) | (C3, U2) => vec![AS, U3, BS, U4, CS, C1, C2],
            (U2, C4) | (C4, U2) => vec![AS, U3, BS, U4, CS, C1, C2, C3],
            (U2, D1) | (D1, U2) => vec![AS, U3, BS, U4, CS, U5, DS],
            (U2, D2) | (D2, U2) => vec![AS, U3, BS, U4, CS, U5, DS, D1],
            (U2, D3) | (D3, U2) => vec![AS, U3, BS, U4, CS, U5, DS, D1, D2],
            (U2, D4) | (D4, U2) => vec![AS, U3, BS, U4, CS, U5, DS, D1, D2, D3],
            (U3, A1) | (A1, U3) => vec![AS],
            (U3, A2) | (A2, U3) => vec![AS, A1],
            (U3, A3) | (A3, U3) => vec![AS, A1, A2],
            (U3, A4) | (A4, U3) => vec![AS, A1, A2, A3],
            (U3, B1) | (B1, U3) => vec![BS],
            (U3, B2) | (B2, U3) => vec![BS, B1],
            (U3, B3) | (B3, U3) => vec![BS, B1, B2],
            (U3, B4) | (B4, U3) => vec![BS, B1, B2, B3],
            (U3, C1) | (C1, U3) => vec![BS, U4, CS],
            (U3, C2) | (C2, U3) => vec![BS, U4, CS, C1],
            (U3, C3) | (C3, U3) => vec![BS, U4, CS, C1, C2],
            (U3, C4) | (C4, U3) => vec![BS, U4, CS, C1, C2, C3],
            (U3, D1) | (D1, U3) => vec![BS, U4, CS, U5, DS],
            (U3, D2) | (D2, U3) => vec![BS, U4, CS, U5, DS, D1],
            (U3, D3) | (D3, U3) => vec![BS, U4, CS, U5, DS, D1, D2],
            (U3, D4) | (D4, U3) => vec![BS, U4, CS, U5, DS, D1, D2, D3],
            (U4, A1) | (A1, U4) => vec![BS, U3, AS],
            (U4, A2) | (A2, U4) => vec![BS, U3, AS, A1],
            (U4, A3) | (A3, U4) => vec![BS, U3, AS, A1, A2],
            (U4, A4) | (A4, U4) => vec![BS, U3, AS, A1, A2, A3],
            (U4, B1) | (B1, U4) => vec![BS],
            (U4, B2) | (B2, U4) => vec![BS, B1],
            (U4, B3) | (B3, U4) => vec![BS, B1, B2],
            (U4, B4) | (B4, U4) => vec![BS, B1, B2, B3],
            (U4, C1) | (C1, U4) => vec![CS],
            (U4, C2) | (C2, U4) => vec![CS, C1],
            (U4, C3) | (C3, U4) => vec![CS, C1, C2],
            (U4, C4) | (C4, U4) => vec![CS, C1, C2, C3],
            (U4, D1) | (D1, U4) => vec![CS, U5, DS],
            (U4, D2) | (D2, U4) => vec![CS, U5, DS, D1],
            (U4, D3) | (D3, U4) => vec![CS, U5, DS, D1, D2],
            (U4, D4) | (D4, U4) => vec![CS, U5, DS, D1, D2, D3],
            (U5, A1) | (A1, U5) => vec![CS, U4, BS, U3, AS],
            (U5, A2) | (A2, U5) => vec![CS, U4, BS, U3, AS, A1],
            (U5, A3) | (A3, U5) => vec![CS, U4, BS, U3, AS, A1, A2],
            (U5, A4) | (A4, U5) => vec![CS, U4, BS, U3, AS, A1, A2, A3],
            (U5, B1) | (B1, U5) => vec![CS, U4, BS],
            (U5, B2) | (B2, U5) => vec![CS, U4, BS, B1],
            (U5, B3) | (B3, U5) => vec![CS, U4, BS, B1, B2],
            (U5, B4) | (B4, U5) => vec![CS, U4, BS, B1, B2, B3],
            (U5, C1) | (C1, U5) => vec![CS],
            (U5, C2) | (C2, U5) => vec![CS, C1],
            (U5, C3) | (C3, U5) => vec![CS, C1, C2],
            (U5, C4) | (C4, U5) => vec![CS, C1, C2, C3],
            (U5, D1) | (D1, U5) => vec![DS],
            (U5, D2) | (D2, U5) => vec![DS, D1],
            (U5, D3) | (D3, U5) => vec![DS, D1, D2],
            (U5, D4) | (D4, U5) => vec![DS, D1, D2, D3],
            (U6, A1) | (A1, U6) => vec![DS, U5, CS, U4, BS, U3, AS],
            (U6, A2) | (A2, U6) => vec![DS, U5, CS, U4, BS, U3, AS, A1],
            (U6, A3) | (A3, U6) => vec![DS, U5, CS, U4, BS, U3, AS, A1, A2],
            (U6, A4) | (A4, U6) => vec![DS, U5, CS, U4, BS, U3, AS, A1, A2, A3],
            (U6, B1) | (B1, U6) => vec![DS, U5, CS, U4, BS],
            (U6, B2) | (B2, U6) => vec![DS, U5, CS, U4, BS, B1],
            (U6, B3) | (B3, U6) => vec![DS, U5, CS, U4, BS, B1, B2],
            (U6, B4) | (B4, U6) => vec![DS, U5, CS, U4, BS, B1, B2, B3],
            (U6, C1) | (C1, U6) => vec![DS, U5, CS],
            (U6, C2) | (C2, U6) => vec![DS, U5, CS, C1],
            (U6, C3) | (C3, U6) => vec![DS, U5, CS, C1, C2],
            (U6, C4) | (C4, U6) => vec![DS, U5, CS, C1, C2, C3],
            (U6, D1) | (D1, U6) => vec![DS],
            (U6, D2) | (D2, U6) => vec![DS, D1],
            (U6, D3) | (D3, U6) => vec![DS, D1, D2],
            (U6, D4) | (D4, U6) => vec![DS, D1, D2, D3],
            (U7, A1) | (A1, U7) => vec![U6, DS, U5, CS, U4, BS, U3, AS],
            (U7, A2) | (A2, U7) => vec![U6, DS, U5, CS, U4, BS, U3, AS, A1],
            (U7, A3) | (A3, U7) => vec![U6, DS, U5, CS, U4, BS, U3, AS, A1, A2],
            (U7, A4) | (A4, U7) => vec![U6, DS, U5, CS, U4, BS, U3, AS, A1, A2, A3],
            (U7, B1) | (B1, U7) => vec![U6, DS, U5, CS, U4, BS],
            (U7, B2) | (B2, U7) => vec![U6, DS, U5, CS, U4, BS, B1],
            (U7, B3) | (B3, U7) => vec![U6, DS, U5, CS, U4, BS, B1, B2],
            (U7, B4) | (B4, U7) => vec![U6, DS, U5, CS, U4, BS, B1, B2, B3],
            (U7, C1) | (C1, U7) => vec![U6, DS, U5, CS],
            (U7, C2) | (C2, U7) => vec![U6, DS, U5, CS, C1],
            (U7, C3) | (C3, U7) => vec![U6, DS, U5, CS, C1, C2],
            (U7, C4) | (C4, U7) => vec![U6, DS, U5, CS, C1, C2, C3],
            (U7, D1) | (D1, U7) => vec![U6, DS],
            (U7, D2) | (D2, U7) => vec![U6, DS, D1],
            (U7, D3) | (D3, U7) => vec![U6, DS, D1, D2],
            (U7, D4) | (D4, U7) => vec![U6, DS, D1, D2, D3],
            _ => Vec::new(),
        }
    }

    fn room(self) -> Vec<Position> {
        use Position::*;
        match self {
            A1 => vec![A2, A3, A4],
            A2 => vec![A1, A3, A4],
            A3 => vec![A1, A2, A4],
            A4 => vec![A1, A2, A3],
            B1 => vec![B2, B3, B4],
            B2 => vec![B1, B3, B4],
            B3 => vec![B1, B2, B4],
            B4 => vec![B1, B2, B3],
            C1 => vec![C2, C3, C4],
            C2 => vec![C1, C3, C4],
            C3 => vec![C1, C2, C4],
            C4 => vec![C1, C2, C3],
            D1 => vec![D2, D3, D4],
            D2 => vec![D1, D3, D4],
            D3 => vec![D1, D2, D4],
            D4 => vec![D1, D2, D3],
            _ => Vec::new(),
        }
    }
}

aoc_lib! { year = 2021 }
