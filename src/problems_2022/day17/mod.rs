use std::{cmp::max, collections::HashSet};

use crate::util::draw::grid_to_string;
use itertools::Itertools;
use log::{debug, info};
use regex::Regex;

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

type Point = (i32, i32);

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

fn rocks() -> Vec<Rock> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ]
    .into_iter()
    .map(|points| Rock::new(&points))
    .collect()
}
#[derive(Debug, Clone)]
struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn new(points: &[Point]) -> Rock {
        Rock {
            points: points.into(),
        }
    }

    fn repositioned(self, x: i32, y: i32) -> Rock {
        Rock {
            points: self
                .points
                .into_iter()
                .map(|point| (point.0 + x, point.1 + y))
                .collect(),
        }
    }

    fn pushed(self, dir: Direction) -> Rock {
        Rock {
            points: self
                .points
                .into_iter()
                .map(|(x, y)| (x + dir as i32, y))
                .collect(),
        }
    }
    fn intersects(&self, chamber: &HashSet<Point>) -> bool {
        self.points.iter().any(|point| chamber.contains(point))
    }

    fn push(&mut self, dir: Direction, chamber: &HashSet<Point>) -> &mut Rock {
        match dir {
            Direction::Left => {
                if self.min_x() <= 0 {
                    debug!("Can't push {:?}: Already on wall", dir);
                    return self
                }
            }
            Direction::Right => {
                if self.max_x() >= 6 {
                    debug!("Can't push {:?}: Already on wall", dir);
                     return self;
                }
            }
        }

        let simulated_push = self.clone().pushed(dir);
        if !simulated_push.intersects(chamber) {
            debug!("Pushed {:?}", dir);
            *self = simulated_push;
        } else {
            debug!("Can't push {:?}: Next to rock", dir)
        }
        self
    }

    fn dropped(&self) -> Rock {
        Rock {
            points: self
                .points
                .iter()
                .cloned()
                .map(|point| (point.0, point.1 - 1))
                .collect_vec(),
        }
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

    fn min_x(&self) -> i32 {
        *self.points.iter().map(|(x, _)| x).min().unwrap()
    }

    fn max_x(&self) -> i32 {
        *self.points.iter().map(|(x, _)| x).max().unwrap()
    }

    fn max_y(&self) -> i32 {
        *self.points.iter().map(|(_, y)| y).max().unwrap()
    }
}

fn solve_part1(input: Input) -> usize {
    let mut chamber = HashSet::<Point>::new();
    for floor in 0..7 {
        chamber.insert((floor, -1));
    }

    let mut jet_index = 0;
    for (index, mut rock) in rocks().iter().cycle().cloned().enumerate() {
        if index >= 2022 {
            break;
        }
        info!("Rock: {index}");
        let highest_point = chamber.iter().max_by(|a, b| Ord::cmp(&a.1, &b.1)).unwrap();
        rock = rock.repositioned(2, highest_point.1 + 4);
        loop {
            debug!(
                "\n{}",
                grid_to_string(
                    &chamber
                        .union(&rock.points.iter().cloned().collect())
                        .cloned()
                        .collect_vec()
                )
            );
            let current_index = jet_index;
            jet_index += 1;
            jet_index %= input.len();
            if !rock.push(input[current_index], &chamber).fall(&chamber) {
                break;
            }
        }
        chamber.extend(rock.points.into_iter());
        info!(
            "\n{}",
            grid_to_string(&chamber.iter().cloned().collect_vec())
        );
    }

    chamber
        .into_iter()
        .map(|(_, y)| y)
        .max()
        .expect("Should have top") as usize + 1
}

fn solve_part2(input: Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
