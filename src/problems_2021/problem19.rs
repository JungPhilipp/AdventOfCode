use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Index, IndexMut, Neg, Sub},
    panic,
};

use itertools::Itertools;
use log::{debug, info};

macro_rules! INPUT_PATH {
    () => {
        "problem19/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 20:\nPart1{}\nPart2{}",
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

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
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
    _pos: Option<Point>,
    points: Vec<Point>,
}

impl Scanner {
    fn new(pos: Option<Point>, points: Vec<Point>) -> Scanner {
        Scanner { _pos: pos, points }
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

fn fold(
    known_points: &HashSet<Point>,
    scanner: &Scanner,
    min_hits: usize,
) -> Option<(HashSet<Point>, Point)> {
    let mut max = HashSet::<Point>::new();
    for known_point in known_points {
        for rotation in 0..48 {
            let unknown_points_rotated = scanner
                .points
                .iter()
                .cloned()
                .map(|p| p.transformed(rotation))
                .collect_vec();
            for translation in unknown_points_rotated
                .iter()
                .map(|unknown_point| known_point.clone() - unknown_point.clone())
            {
                let unknown_points_transformed: HashSet<Point> = unknown_points_rotated
                    .iter()
                    .cloned()
                    .map(|p| p + translation.clone())
                    .collect();
                let intersection = unknown_points_transformed
                    .intersection(known_points)
                    .collect::<HashSet<_>>();
                if intersection.len() >= max.len() {
                    //dbg!(intersection.len());
                    max = intersection.iter().map(|&i| i.clone()).collect();
                }
                if intersection.len() >= min_hits {
                    return Some((unknown_points_transformed, translation));
                }
            }
        }
    }
    //dbg!(max);
    None
}

pub fn solve_part1(input: Input) -> usize {
    let mut scanners = VecDeque::from(input);
    let mut known_points = scanners
        .pop_front()
        .unwrap()
        .points
        .into_iter()
        .collect::<HashSet<_>>();
    let mut index = 0;
    while !scanners.is_empty() {
        index %= scanners.len();
        //dbg!(known_points.len(), index, scanners.len());
        let scanner = &scanners[index];
        if let Some((new_known_points, _)) = fold(&known_points, scanner, 12) {
            known_points.extend(new_known_points);
            scanners.remove(index);
            debug!("Remove {}, {} are left", index, scanners.len());
        }
        index += 1;
    }

    known_points.len()
}

pub fn solve_part2(input: Input) -> i32 {
    let mut positions = vec![];
    let mut scanners = VecDeque::from(input);
    let mut known_points = scanners
        .pop_front()
        .unwrap()
        .points
        .into_iter()
        .collect::<HashSet<_>>();
    let mut index = 0;
    while !scanners.is_empty() {
        index %= scanners.len();
        //dbg!(known_points.len(), index, scanners.len());
        let scanner = &scanners[index];
        if let Some((new_known_points, pos)) = fold(&known_points, scanner, 12) {
            known_points.extend(new_known_points);
            scanners.remove(index);
            positions.push(pos);
            debug!("Remove {}, {} are left", index, scanners.len());
        }
        index += 1;
    }

    let mut max = 0;
    for pos0 in &positions {
        for pos1 in &positions {
            let sum = (pos0.clone() - pos1.clone()).manhattan();
            if sum > max {
                max = sum;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_equal_fold() {
        let points = vec![Point::new(0, 0, 0), Point::new(-2, 5, 3)];
        let known_points = points.iter().cloned().collect::<HashSet<_>>();
        let s0 = Scanner::new(None, points);
        let transformed_points = fold(&known_points, &s0, 2);
        assert_eq!(transformed_points.unwrap().0, known_points);
    }

    #[test]
    fn example1() {
        let scanners = parse(include_str!("problem19/example1.txt"));
        let known_points = scanners[0].points.iter().cloned().collect::<HashSet<_>>();
        let transformed_points = fold(&known_points, &scanners[1], 3);
        assert_eq!(transformed_points.unwrap().0, known_points);
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(parse(include_str!("problem19/example2.txt"))),
            79
        );
    }

    #[test]
    fn part1() {
        //assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 383);
    }

    #[test]
    fn part2() {
        //assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 9854);
    }
}
