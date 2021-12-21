use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashMap;
use std::iter::Cycle;
use std::ops::RangeInclusive;

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (mut player1, mut player2) = parse_input(input);
    let mut die = DeterministicDie::new();

    let loser_score = loop {
        player1 = player1.step(die.game_roll());
        if player1.score >= 1000 {
            break player2.score;
        }

        player2 = player2.step(die.game_roll());
        if player2.score >= 1000 {
            break player1.score;
        }
    };

    loser_score * die.count
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (player1, player2) = parse_input(input);
    let mut game = DiracDice::new(player1, player2);

    while !game.states.is_empty() {
        game.turn_player1();
        game.turn_player2();
    }

    game.player1_wins.max(game.player2_wins)
}

fn parse_input(input: &str) -> (Player, Player) {
    let mut players = input.lines().map(Player::from);
    (players.next().unwrap(), players.next().unwrap())
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn step(self, amount: usize) -> Self {
        let position = (self.position + amount) % 10;

        Self {
            position,
            score: self.score + position + 1,
        }
    }
}

impl<'a> From<&'a str> for Player {
    fn from(input: &'a str) -> Self {
        let position = input[28..].parse::<usize>().unwrap() - 1;

        Player::new(position)
    }
}

struct DeterministicDie {
    iter: Cycle<RangeInclusive<usize>>,
    count: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            iter: (1..=100).cycle(),
            count: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.count += 1;
        self.iter.next().unwrap()
    }

    fn game_roll(&mut self) -> usize {
        self.roll() + self.roll() + self.roll()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct GameState {
    player1: Player,
    player2: Player,
}

impl GameState {
    fn move_player1(self, amount: usize) -> Self {
        Self {
            player1: self.player1.step(amount),
            player2: self.player2,
        }
    }

    fn move_player2(self, amount: usize) -> Self {
        Self {
            player1: self.player1,
            player2: self.player2.step(amount),
        }
    }
}

struct DiracDice {
    states: HashMap<GameState, usize>,
    player1_wins: usize,
    player2_wins: usize,
}

impl DiracDice {
    fn new(player1: Player, player2: Player) -> Self {
        let state = GameState { player1, player2 };
        let mut states = HashMap::new();
        states.insert(state, 1);

        Self {
            states,
            player1_wins: 0,
            player2_wins: 0,
        }
    }

    fn turn_player1(&mut self) {
        let mut states = HashMap::with_capacity(self.states.len());
        let mut player1_wins = self.player1_wins;
        let mut check_or_insert = |state: GameState, count: usize| {
            if state.player1.score >= 21 {
                player1_wins += count;
            } else {
                *states.entry(state).or_default() += count;
            }
        };

        for (state, count) in self.states.iter() {
            // Roll 3 (1 universe)
            check_or_insert(state.move_player1(3), *count);
            // Roll 4 (3 universes)
            check_or_insert(state.move_player1(4), count * 3);
            // Roll 5 (6 universes)
            check_or_insert(state.move_player1(5), count * 6);
            // Roll 6 (7 universes)
            check_or_insert(state.move_player1(6), count * 7);
            // Roll 7 (6 universes)
            check_or_insert(state.move_player1(7), count * 6);
            // Roll 8 (3 universes)
            check_or_insert(state.move_player1(8), count * 3);
            // Roll 9 (1 universe)
            check_or_insert(state.move_player1(9), *count);
        }

        self.player1_wins = player1_wins;
        self.states = states;
    }

    fn turn_player2(&mut self) {
        let mut states = HashMap::with_capacity(self.states.len());
        let mut player2_wins = self.player2_wins;
        let mut check_or_insert = |state: GameState, count: usize| {
            if state.player2.score >= 21 {
                player2_wins += count;
            } else {
                *states.entry(state).or_default() += count;
            }
        };

        for (state, count) in self.states.iter() {
            // Roll 3 (1 universe)
            check_or_insert(state.move_player2(3), *count);
            // Roll 4 (3 universes)
            check_or_insert(state.move_player2(4), count * 3);
            // Roll 5 (6 universes)
            check_or_insert(state.move_player2(5), count * 6);
            // Roll 6 (7 universes)
            check_or_insert(state.move_player2(6), count * 7);
            // Roll 7 (6 universes)
            check_or_insert(state.move_player2(7), count * 6);
            // Roll 8 (3 universes)
            check_or_insert(state.move_player2(8), count * 3);
            // Roll 9 (1 universe)
            check_or_insert(state.move_player2(9), *count);
        }

        self.player2_wins = player2_wins;
        self.states = states;
    }
}

aoc_lib! { year = 2021 }
