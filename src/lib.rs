use aoc_runner_derive::{aoc, aoc_lib};

fn parse_input(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut pieces = input.split("\n\n");

    let nums = pieces
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let boards = pieces.map(Board::from).collect();

    (nums, boards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (nums, mut boards) = parse_input(input);

    for num in nums {
        for board in &mut boards {
            board.mark(num);

            if board.is_winning() {
                return board.unmarked_sum() * num;
            }
        }
    }

    unreachable!("Should have won at least one board");
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (nums, mut boards) = parse_input(input);
    let mut last_winning_score = 0;

    for num in nums {
        let mut index = 0;

        while index < boards.len() {
            boards[index].mark(num);

            if boards[index].is_winning() {
                last_winning_score = boards[index].unmarked_sum() * num;

                boards.swap_remove(index);
            } else {
                index += 1;
            }
        }
    }

    last_winning_score
}

struct Board {
    spaces: Vec<Vec<Space>>,
}

impl<'a> From<&'a str> for Board {
    fn from(input: &'a str) -> Self {
        Board {
            spaces: input
                .lines()
                .map(|line| {
                    line.split(char::is_whitespace)
                        .filter(|num| !num.is_empty())
                        .map(|num| Space {
                            value: num.parse().unwrap(),
                            marked: false,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

struct Space {
    value: usize,
    marked: bool,
}

impl Board {
    fn mark(&mut self, value: usize) {
        for row in self.spaces.iter_mut() {
            for col in row.iter_mut() {
                if col.value == value {
                    col.marked = true;
                }
            }
        }
    }

    fn is_winning(&self) -> bool {
        Permuter::new(self).any(|mut line| line.all(|space| space.marked))
    }

    fn unmarked_sum(&self) -> usize {
        self.spaces
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(|space| (!space.marked).then(|| space.value))
            })
            .sum()
    }
}

struct Permuter<'a> {
    current_kind: Option<PermutationKind>,
    board: &'a Board,
}

impl<'a> Permuter<'a> {
    fn new(board: &'a Board) -> Self {
        Permuter {
            current_kind: Some(PermutationKind::Row(0)),
            board,
        }
    }
}

impl<'a> Iterator for Permuter<'a> {
    type Item = BoardSpaces<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_kind.map(|kind| {
            self.current_kind = kind.next();

            BoardSpaces {
                kind,
                board: self.board,
                index: 0,
            }
        })
    }
}

#[derive(Clone, Copy)]
enum PermutationKind {
    Row(usize),
    Column(usize),
    DownDiagonal,
    UpDiagonal,
}

impl PermutationKind {
    fn next(self) -> Option<Self> {
        use PermutationKind::*;

        match self {
            Row(4) => Some(Column(0)),
            Row(index) => Some(Row(index + 1)),
            Column(4) => Some(DownDiagonal),
            Column(index) => Some(Column(index + 1)),
            DownDiagonal => Some(UpDiagonal),
            UpDiagonal => None,
        }
    }
}

struct BoardSpaces<'a> {
    kind: PermutationKind,
    board: &'a Board,
    index: usize,
}

impl<'a> Iterator for BoardSpaces<'a> {
    type Item = &'a Space;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.kind {
            PermutationKind::Row(index) => self
                .board
                .spaces
                .get(index)
                .and_then(|row| row.get(self.index)),
            PermutationKind::Column(index) => self
                .board
                .spaces
                .get(self.index)
                .and_then(|row| row.get(index)),
            PermutationKind::DownDiagonal => self
                .board
                .spaces
                .get(self.index)
                .and_then(|row| row.get(self.index)),
            PermutationKind::UpDiagonal => self
                .board
                .spaces
                .get(4 - self.index)
                .and_then(|row| row.get(self.index)),
        };

        self.index += 1;

        result
    }
}

aoc_lib! { year = 2021 }
