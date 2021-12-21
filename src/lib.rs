use aoc_runner_derive::{aoc, aoc_lib};
use indexmap::IndexSet;
use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Sub};

static ROTATIONS: [Rotation; 24] = [
    Rotation::XYZ,
    Rotation::XUV,
    Rotation::XVY,
    Rotation::XZU,
    Rotation::YTZ,
    Rotation::YVT,
    Rotation::YXV,
    Rotation::YZX,
    Rotation::ZTU,
    Rotation::ZUX,
    Rotation::ZXY,
    Rotation::ZYT,
    Rotation::TUZ,
    Rotation::TVU,
    Rotation::TYV,
    Rotation::TZY,
    Rotation::UTV,
    Rotation::UVX,
    Rotation::UXZ,
    Rotation::UZT,
    Rotation::VTY,
    Rotation::VUT,
    Rotation::VXU,
    Rotation::VYX,
];

#[aoc(day19, part1)]
pub fn solve_part1(input: &str) -> usize {
    let matched = match_sensors(input);
    let mut points = HashSet::new();

    for sensor in matched {
        points.extend(sensor.beacons);
    }

    points.len()
}

#[aoc(day19, part2)]
fn solve_part2(input: &str) -> usize {
    let matched = match_sensors(input);
    let mut max_distance = 0;

    for i in 0..matched.len() {
        for j in 0..matched.len() {
            if i != j {
                let diff = matched[i].origin - matched[j].origin;
                let distance = diff.x.abs() + diff.y.abs() + diff.z.abs();
                max_distance = max_distance.max(distance as usize);
            }
        }
    }

    max_distance
}

fn match_sensors(input: &str) -> Vec<Sensor> {
    let mut unmatched = parse_input(input);
    let mut matched = vec![unmatched.pop_front().unwrap()];

    'outer: while let Some(compare) = unmatched.pop_front() {
        for base in &matched {
            if let Some(translated) = overlapping(base, &compare) {
                matched.push(translated);
                continue 'outer;
            }
        }
        unmatched.push_back(compare);
    }

    matched
}

fn parse_input(input: &str) -> VecDeque<Sensor> {
    input
        .split("\n\n")
        .map(|points| Sensor::new(points.lines().skip(1).map(Point::from)))
        .collect()
}

fn overlapping(base: &Sensor, compare: &Sensor) -> Option<Sensor> {
    for rotation in &ROTATIONS {
        for base_beacon in base.beacons.iter().copied() {
            for compare_beacon in compare.rotate(*rotation) {
                let diff = base_beacon - compare_beacon;

                let overlap_count = compare
                    .rotate_translate(*rotation, diff)
                    .filter(|point| base.beacons.contains(point))
                    .count();

                if overlap_count >= 12 {
                    return Some(compare.to_fixed(*rotation, diff));
                }
            }
        }
    }

    None
}

struct Sensor {
    origin: Point,
    beacons: IndexSet<Point>,
}

impl Sensor {
    fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Point>,
    {
        Self {
            origin: Point::new(0, 0, 0),
            beacons: iter.into_iter().collect(),
        }
    }

    fn rotate(&self, rotation: Rotation) -> impl Iterator<Item = Point> + '_ {
        self.beacons.iter().map(move |point| point.rotate(rotation))
    }

    fn rotate_translate(
        &self,
        rotation: Rotation,
        diff: Point,
    ) -> impl Iterator<Item = Point> + '_ {
        self.beacons
            .iter()
            .map(move |point| point.rotate(rotation) + diff)
    }

    fn to_fixed(&self, rotation: Rotation, diff: Point) -> Self {
        Self {
            origin: self.origin + diff,
            beacons: self.rotate_translate(rotation, diff).collect(),
        }
    }
}

#[derive(Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
enum Rotation {
    XYZ,
    XZU,
    XUV,
    XVY,
    YTZ,
    YZX,
    YXV,
    YVT,
    ZXY,
    ZYT,
    ZTU,
    ZUX,
    TUZ,
    TVU,
    TYV,
    TZY,
    UXZ,
    UVX,
    UTV,
    UZT,
    VXU,
    VYX,
    VTY,
    VUT,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn rotate(self, direction: Rotation) -> Point {
        use Rotation::*;
        match direction {
            XYZ => self,
            XUV => Point::new(self.x, -self.y, -self.z),
            XVY => Point::new(self.x, -self.z, self.y),
            XZU => Point::new(self.x, self.z, -self.y),
            YTZ => Point::new(self.y, -self.x, self.z),
            YVT => Point::new(self.y, -self.z, -self.x),
            YXV => Point::new(self.y, self.x, -self.z),
            YZX => Point::new(self.y, self.z, self.x),
            ZTU => Point::new(self.z, -self.x, -self.y),
            ZUX => Point::new(self.z, -self.y, self.x),
            ZXY => Point::new(self.z, self.x, self.y),
            ZYT => Point::new(self.z, self.y, -self.x),
            TUZ => Point::new(-self.x, -self.y, self.z),
            TVU => Point::new(-self.x, -self.z, -self.y),
            TYV => Point::new(-self.x, self.y, -self.z),
            TZY => Point::new(-self.x, self.z, self.y),
            UTV => Point::new(-self.y, -self.x, -self.z),
            UVX => Point::new(-self.y, -self.z, self.x),
            UXZ => Point::new(-self.y, self.x, self.z),
            UZT => Point::new(-self.y, self.z, -self.x),
            VTY => Point::new(-self.z, -self.x, self.y),
            VUT => Point::new(-self.z, -self.y, -self.x),
            VXU => Point::new(-self.z, self.x, -self.y),
            VYX => Point::new(-self.z, self.y, self.x),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a> From<&'a str> for Point {
    fn from(input: &'a str) -> Self {
        let mut parts = input.split(',').map(|part| part.parse().unwrap());

        Self {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
    }
}

aoc_lib! { year = 2021 }

#[cfg(test)]
mod tests {
    use super::*;

    const SENSORS: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_solution() {
        assert_eq!(solve_part1(SENSORS), 79);
    }
}
