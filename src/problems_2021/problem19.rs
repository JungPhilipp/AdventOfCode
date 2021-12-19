use core::fmt;
use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    ops::{Add, Index, IndexMut, Neg, Sub},
    panic,
};

use itertools::Itertools;
use log::{debug, info};
use regex::Regex;

macro_rules! INPUT_PATH {
    () => {
        "problem19/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 16:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn normalized(&self) -> Point {
        self.clone() - Point::from(self.x)
    }

    fn flipped(&self, axis: usize) -> Point {
        let mut flipped = self.clone();
        flipped[axis] = -flipped[axis];
        flipped
    }

    fn transformed(&self, mode: i32) -> Point {
        match mode {
            0 => self.clone(),
            1 => self.transformed(0).flipped(0),
            2 => self.transformed(0).flipped(0).flipped(1),
            3 => self.transformed(0).flipped(0).flipped(1).flipped(2),
            4 => self.transformed(0).flipped(0).flipped(2),
            5 => self.transformed(0).flipped(1),
            6 => self.transformed(0).flipped(1).flipped(2),
            7 => self.transformed(0).flipped(2),
            8 => Point::new(self.z, self.x, self.y),
            9 => self.transformed(8).flipped(0),
            10 => self.transformed(8).flipped(0).flipped(1),
            11 => self.transformed(8).flipped(0).flipped(1).flipped(2),
            12 => self.transformed(8).flipped(0).flipped(2),
            13 => self.transformed(8).flipped(1),
            14 => self.transformed(8).flipped(1).flipped(2),
            15 => self.transformed(8).flipped(2),
            16 => Point::new(self.y, self.z, self.x),
            17 => self.transformed(16).flipped(0),
            18 => self.transformed(16).flipped(0).flipped(1),
            19 => self.transformed(16).flipped(0).flipped(1).flipped(2),
            20 => self.transformed(16).flipped(0).flipped(2),
            21 => self.transformed(16).flipped(1),
            22 => self.transformed(16).flipped(1).flipped(2),
            23 => self.transformed(16).flipped(2),
            24 => Point::new(self.x, self.z, self.y),
            25 => self.transformed(24).flipped(0),
            26 => self.transformed(24).flipped(0).flipped(1),
            27 => self.transformed(24).flipped(0).flipped(1).flipped(2),
            28 => self.transformed(24).flipped(0).flipped(2),
            29 => self.transformed(24).flipped(1),
            30 => self.transformed(24).flipped(1).flipped(2),
            31 => self.transformed(24).flipped(2),
            32 => Point::new(self.z, self.y, self.x),
            33 => self.transformed(32).flipped(0),
            34 => self.transformed(32).flipped(0).flipped(1),
            35 => self.transformed(32).flipped(0).flipped(1).flipped(2),
            36 => self.transformed(32).flipped(0).flipped(2),
            37 => self.transformed(32).flipped(1),
            38 => self.transformed(32).flipped(1).flipped(2),
            39 => self.transformed(32).flipped(2),
            40 => Point::new(self.y, self.x, self.z),
            41 => self.transformed(40).flipped(0),
            42 => self.transformed(40).flipped(0).flipped(1),
            43 => self.transformed(40).flipped(0).flipped(1).flipped(2),
            44 => self.transformed(40).flipped(0).flipped(2),
            45 => self.transformed(40).flipped(1),
            46 => self.transformed(40).flipped(1).flipped(2),
            47 => self.transformed(40).flipped(2),

            _ => panic!(),
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Point {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of range"),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of range"),
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl From<i32> for Point {
    fn from(val: i32) -> Self {
        Point::new(val, val, val)
    }
}

#[derive(Debug, Clone)]
pub struct Scanner {
    pos: Option<Point>,
    transform: Option<i32>,
    points: Vec<Point>,
}

impl Scanner {
    fn new(pos: Option<Point>, points: Vec<Point>) -> Scanner {
        Scanner {
            pos: pos,
            transform: None,
            points: points,
        }
    }
}

type Input = Vec<Scanner>;

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|scanner_input| {
            scanner_input
                .lines()
                .filter_map(|line| {
                    if line.starts_with("---") || line.trim().is_empty() {
                        None
                    } else {
                        let (x, y, z) = line
                            .split(',')
                            .map(|number| number.parse::<i32>().expect("Not a number"))
                            .collect_tuple()
                            .unwrap();
                        Some(Point::new(x, y, z))
                    }
                })
                .collect_vec()
        })
        .map(|points| Scanner::new(None, points))
        .collect_vec()
}

fn common_points(s0: &mut Scanner, s1: &mut Scanner) -> (usize, Option<Point>) {
    let mut s1_to_s0 = None;
    let mut common_points = 0;
    let mut intersection = HashSet::<Point>::new();
    let s0_points: HashSet<_> = s0.points.iter().cloned().collect();

    for trans in 0..48 {
        let s1_points = s1.points.iter().map(|p| p.transformed(trans)).collect_vec();
        for p0 in &s0.points {
            for center in s1_points.iter().map(|p1| p0.clone() - p1.clone()) {
                //let center = Point::new(0, 0, 0);
                let s1_points_translated: HashSet<Point> = s1_points
                    .iter()
                    .cloned()
                    .map(|p| center.clone() + p)
                    .collect();
                let common: HashSet<_> = s0_points.intersection(&s1_points_translated).collect();
                if common.len() >= common_points {
                    common_points = common.len();
                    s1_to_s0 = Some(center.clone());
                    intersection = common.iter().map(|&p| p.clone()).collect();
                }
                if let Some(pos) = s0.pos.clone() {
                    s1.pos = Some(pos + -center);
                }
            }
        }
    }
    (common_points, s1_to_s0)
}

pub fn solve_part1(mut scanners: Input) -> usize {
    scanners[0].pos = Some(Point::new(0, 0, 0));
    scanners[0].transform = Some(0);
    let mut relative_locations = vec![vec![None; scanners.len()]; scanners.len()];
    for i0 in 0..scanners.len() {
        for i1 in 0..scanners.len() {
            let mut s0 = scanners[i0].clone();
            let mut s1 = scanners[i1].clone();
            if let (common_points, Some(s1_to_s0)) = common_points(&mut s0, &mut s1) {
                scanners[i0] = s0;
                scanners[i1] = s1;
                if common_points < 12 {
                    continue;
                }
                relative_locations[i1][i0] = Some(s1_to_s0.clone());
                relative_locations[i0][i1] = Some(-s1_to_s0);
            } else {
                panic!()
            }
        }
    }
    let mut points = scanners[0]
        .points
        .iter()
        .cloned()
        .collect::<HashSet<Point>>();
    dbg!(scanners.iter().filter(|s| s.pos.is_none()).count());
    for scanner in scanners {
        let pos = scanner.pos.unwrap();
        for point in scanner.points {
            points.insert(pos.clone() + point.transformed(scanner.transform.unwrap()));
        }
    }

    points.len()
}

pub fn solve_part2(mut scanners: Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_equal() {
        let points = vec![Point::new(0, 0, 0), Point::new(-2, 5, 3)];
        let mut s0 = Scanner::new(None, points.clone());
        let mut s1 = s0.clone();
        let (count, _) = common_points(&mut s0, &mut s1);
        assert_eq!(count, points.len());
    }

    #[test]
    fn test_rotation1() {
        let scanners = parse(include_str!("problem19/example3.txt"));
        for i in 0..scanners.len() {
            for j in 0..scanners.len() {
                if i == j {
                    continue;
                }
                let mut s0 = scanners[i].clone();
                let mut s1 = scanners[j].clone();
                let (count, _) = common_points(&mut s0, &mut s1);
                assert_eq!(count, 6, "{}, {}", i, j);
            }
        }
    }

    #[test]
    fn test_translation1() {
        let scanners = parse(include_str!("problem19/example1.txt"));
        let mut s0 = scanners[0].clone();
        let mut s1 = scanners[1].clone();
        let (count, _) = common_points(&mut s0, &mut s1);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_rotation2() {
        let p0 = vec![Point::new(-1, -1, 1), Point::new(-2, -3, 1)];
        let p1 = vec![Point::new(1, 1, 1), Point::new(3, 1, 2)];

        let (count, _) = common_points(&mut Scanner::new(None, p0), &mut Scanner::new(None, p1));
        assert_eq!(count, 2);
    }

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(parse(include_str!("problem19/example2.txt"))),
            79
        );
        //assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        //assert_eq!(solve_part2(&parse(include_str!(INPUT_PATH!()))), 0);
    }
}
