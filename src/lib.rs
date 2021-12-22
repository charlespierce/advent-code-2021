use aoc_runner_derive::{aoc, aoc_lib};

#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut reactor = Reactor::default();
    let initialization = input.lines().map(Cuboid::from).filter(|cube| {
        cube.x.1 <= 50
            && cube.x.0 >= -50
            && cube.y.1 <= 50
            && cube.y.0 >= -50
            && cube.z.1 <= 50
            && cube.z.0 >= -50
    });

    for cube in initialization {
        reactor.add_cuboid(cube);
    }

    reactor.count()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut reactor = Reactor::default();

    for cube in input.lines().map(Cuboid::from) {
        reactor.add_cuboid(cube);
    }

    reactor.count()
}

#[derive(Default)]
struct Reactor {
    cubes: Vec<Cuboid>,
}

impl Reactor {
    fn add_cuboid(&mut self, cube: Cuboid) {
        let mut overlaps = Vec::with_capacity(self.cubes.len());

        for compare in &self.cubes {
            if let Some(overlap) = cube.inclusion(compare) {
                overlaps.push(overlap);
            }
        }

        self.cubes.extend(overlaps);

        if cube.action == Action::On {
            self.cubes.push(cube);
        }
    }

    fn count(&self) -> usize {
        let mut total = 0;

        for cube in &self.cubes {
            match cube.action {
                Action::On => {
                    total += cube.count();
                }
                Action::Off => {
                    total -= cube.count();
                }
            }
        }

        total
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Action {
    On,
    Off,
}

impl Action {
    fn not(self) -> Self {
        match self {
            Action::On => Action::Off,
            Action::Off => Action::On,
        }
    }
}

impl<'a> From<&'a str> for Action {
    fn from(input: &'a str) -> Self {
        match input {
            "on" => Action::On,
            "off" => Action::Off,
            _ => panic!("Invalid action"),
        }
    }
}

#[derive(Clone, Copy)]
struct Cuboid {
    action: Action,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    fn inclusion(&self, other: &Cuboid) -> Option<Cuboid> {
        let x_min = self.x.0.max(other.x.0);
        let x_max = self.x.1.min(other.x.1);
        let y_min = self.y.0.max(other.y.0);
        let y_max = self.y.1.min(other.y.1);
        let z_min = self.z.0.max(other.z.0);
        let z_max = self.z.1.min(other.z.1);

        if x_min > x_max || y_min > y_max || z_min > z_max {
            None
        } else {
            Some(Cuboid {
                action: other.action.not(),
                x: (x_min, x_max),
                y: (y_min, y_max),
                z: (z_min, z_max),
            })
        }
    }

    fn count(&self) -> usize {
        let x = self.x.1 - self.x.0 + 1;
        let y = self.y.1 - self.y.0 + 1;
        let z = self.z.1 - self.z.0 + 1;

        x as usize * y as usize * z as usize
    }
}

impl<'a> From<&'a str> for Cuboid {
    fn from(input: &'a str) -> Self {
        let (action, ranges) = input.split_once(' ').unwrap();
        let mut axes = ranges
            .split(',')
            .map(|axis| axis[2..].split_once("..").unwrap());
        let (x_start, x_end) = axes.next().unwrap();
        let (y_start, y_end) = axes.next().unwrap();
        let (z_start, z_end) = axes.next().unwrap();

        Self {
            action: action.into(),
            x: (x_start.parse().unwrap(), x_end.parse().unwrap()),
            y: (y_start.parse().unwrap(), y_end.parse().unwrap()),
            z: (z_start.parse().unwrap(), z_end.parse().unwrap()),
        }
    }
}

aoc_lib! { year = 2021 }
