use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use itertools::Itertools;
use log::{debug, info};

use crate::util::draw::grid_to_string;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 17:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed),
    );
}

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 1,
    Left = -1,
}
type Input = Vec<Direction>;

fn parse(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            u => unreachable!("Unexpected char {u}"),
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Row,
    Cross,
    Angle,
    Column,
    Cube,
}

const ROW: [Point; 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const CROSS: [Point; 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
const ANGLE: [Point; 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const COLUMN: [Point; 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const CUBE: [Point; 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

impl Kind {
    fn all_kinds() -> Vec<Kind> {
        use Kind::*;
        vec![Row, Cross, Angle, Column, Cube]
    }

    fn points(&self) -> &'static [Point] {
        match self {
            Kind::Row => &ROW,
            Kind::Cross => &CROSS,
            Kind::Angle => &ANGLE,
            Kind::Column => &COLUMN,
            Kind::Cube => &CUBE,
        }
    }

    fn min_x(&self) -> i64 {
        0
    }

    fn max_x(&self) -> i64 {
        match self {
            Kind::Row => 3,
            Kind::Cross => 2,
            Kind::Angle => 2,
            Kind::Column => 0,
            Kind::Cube => 1,
        }
    }

    fn max_y(&self) -> i64 {
        match self {
            Kind::Row => 0,
            Kind::Cross => 2,
            Kind::Angle => 2,
            Kind::Column => 3,
            Kind::Cube => 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    origin: Point,
    kind: Kind,
}

impl Rock {
    fn compute_points(&self) -> Vec<Point> {
        self.kind
            .points()
            .iter()
            .map(|point| (self.origin.0 + point.0, self.origin.1 + point.1))
            .collect()
    }

    fn repositioned(self, translation: Point) -> Rock {
        Rock {
            origin: (self.origin.0 + translation.0, self.origin.1 + translation.1),
            kind: self.kind,
        }
    }

    fn pushed(self, dir: Direction) -> Rock {
        self.repositioned((dir as i64, 0))
    }
    fn intersects(&self, chamber: &HashSet<Point>) -> bool {
        self.compute_points()
            .iter()
            .any(|point| chamber.contains(point))
    }

    fn push_unchecked(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                if self.min_x() > 0 {
                    self.origin.0 -= 1;
                }
            }
            Direction::Right => {
                if self.max_x() < 6 {
                    self.origin.0 += 1;
                }
            }
        }
    }

    fn push(&mut self, dir: Direction, chamber: &HashSet<Point>) -> &mut Rock {
        match dir {
            Direction::Left => {
                if self.min_x() <= 0 {
                    debug!("Can't push {:?}: Already on wall", dir);
                    return self;
                }
            }
            Direction::Right => {
                if self.max_x() >= 6 {
                    debug!("Can't push {:?}: Already on wall", dir);
                    return self;
                }
            }
        }

        let simulated_push = self.pushed(dir);
        if !simulated_push.intersects(chamber) {
            debug!("Pushed {:?}", dir);
            *self = simulated_push;
        } else {
            debug!("Can't push {:?}: Next to rock", dir)
        }
        self
    }

    fn dropped(&self) -> Rock {
        self.repositioned((0, -1))
    }

    fn fall(&mut self, chamber: &HashSet<Point>) -> bool {
        if self.max_y() <= 0 {
            debug!("Can't drop: Already on floor");
            return false;
        }
        let simulated_drop = self.dropped();
        if !simulated_drop.intersects(chamber) {
            debug!("Dropped");
            *self = simulated_drop;
            true
        } else {
            debug!("Can't drop: On top of rock");
            false
        }
    }

    fn min_x(&self) -> i64 {
        self.kind.min_x() + self.origin.0
    }

    fn max_x(&self) -> i64 {
        self.kind.max_x() + self.origin.0
    }

    fn max_y(&self) -> i64 {
        self.kind.max_y() + self.origin.1
    }

    fn all_kinds() -> Vec<Rock> {
        Kind::all_kinds()
            .into_iter()
            .map(|kind| Rock {
                origin: (0, 0),
                kind,
            })
            .collect()
    }
}

fn remove_old_rocks(chamber: &mut HashSet<Point>, cutoff: i64, rock_index: usize) {
    let _ = chamber.drain_filter(|point| point.1 < cutoff).count();
    if rock_index % 10_000_000 == 0 {
        info!("Rock: {rock_index}: {:?}", SystemTime::now());
    }
}

fn print_chamber(chamber: &HashSet<Point>, rock: Option<Rock>) {
    let mut rocks = chamber.clone();
    if let Some(rock) = rock {
        rock.compute_points().into_iter().for_each(|point| {
            rocks.insert(point);
        });
    }

    info!("\n{}", grid_to_string(&rocks.into_iter().collect_vec()));
}

fn update_height_profile(
    height_profile: &[i64],
    highest: i64,
    points: &[Point],
) -> (Vec<i64>, i64) {
    let mut translated_profile = height_profile.iter().map(|h| highest + h).collect_vec();
    for (x, y) in points.iter() {
        if *y > translated_profile[*x as usize] {
            translated_profile[*x as usize] = *y;
        }
    }
    let highest = *translated_profile.iter().max().unwrap();
    (
        translated_profile
            .into_iter()
            .map(|y| y - highest)
            .collect_vec(),
        highest,
    )
}

fn simulate_rocks(input: Input, max_rock: usize) -> i64 {
    type CacheKey = (Vec<i64>, usize, Kind);
    let mut cache = HashMap::<CacheKey, Rock>::new();
    let mut chamber = HashSet::<Point>::new();
    let mut height_profile = vec![0; 7];
    let mut highest = -1;
    for floor in 0..7 {
        chamber.insert((floor, -1));
    }

    let mut jets = input.iter().enumerate().cycle();

    for (rock_number, mut rock) in Rock::all_kinds()
        .iter()
        .copied()
        .cycle()
        .enumerate()
        .take(max_rock)
    {
        let jet_index = jets.clone().peekable().peek().unwrap().0;

        let cache_key = (height_profile.clone(), jet_index, rock.kind);
        if let Some(cached_rock) = cache.get(&cache_key) {
            rock = cached_rock.repositioned((0, highest));
        } else {
            rock = rock.repositioned((2, highest + 4));
            loop {
                if !rock.push(*jets.next().unwrap().1, &chamber).fall(&chamber) {
                    break;
                }
            }
            cache.insert(cache_key, rock.repositioned((0, -highest)));
        }

        let points = rock.compute_points();
        (height_profile, highest) = update_height_profile(&height_profile, highest, &points);
        chamber.extend(points.into_iter());

        if rock_number % 100 == 0 {
            remove_old_rocks(&mut chamber, highest - 100, rock_number);
        }
    }

    print_chamber(&chamber, None);
    highest + 1
}

fn solve_part1(input: Input) -> usize {
    simulate_rocks(input, 2022) as usize
}

fn solve_part2(input: Input) -> usize {
    simulate_rocks(input, 1_000_000_000_000) as usize
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;
    use test_log::test;

    macro_rules! EXAMPLE_PATH {
        () => {
            "example_1.txt"
        };
    }

    #[test]
    fn example_1() {
        let input = parse(include_str!(EXAMPLE_PATH!()));
        assert_eq!(solve_part1(input), 3068);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 3232);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 3232));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_part2(parse(include_str!(EXAMPLE_PATH!()))),
            1514285714288
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
