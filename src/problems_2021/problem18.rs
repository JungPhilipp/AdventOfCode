use std::{collections::VecDeque, ops::Add, panic};

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "problem18/input.txt"
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Number {
    Pair(Box<Pair>),
    Number(i32),
}

impl Number {
    fn split(self) -> Option<Number> {
        if let Number::Number(number) = self {
            if number >= 10 {
                return Some(Number::Pair(Box::new(Pair::new(
                    Number::Number(number / 2),
                    Number::Number(number / 2),
                ))));
            } else {
                return None;
            }
        }
        None
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Pair {
    left: Number,
    right: Number,
}
impl Add for Pair {
    type Output = Pair;
    fn add(self, other: Pair) -> Self::Output {
        Pair {
            left: Number::Pair(Box::new(self)),
            right: Number::Pair(Box::new(other)),
        }
    }
}

impl Pair {
    fn reduce(mut self, nested: i32) {
        if let Number::Pair(left) = self.left {
            left.reduce(nested + 1);
        }
        if let Number::Pair(right) = self.right {
            right.reduce(nested + 1);
        }
        loop {
            if nested >= 4 {
                //self.explode();
                continue;
            } else if let Some(Number) = self.left.split() {
                self.left = Number;
            } else if let Some(Number) = self.right.split() {
                self.right = Number;
            } else {
                return;
            }
        }
    }

    fn new(left: Number, right: Number) -> Pair {
        Pair { left, right }
    }
}

type Input = Vec<Pair>;

fn parse_pair(input: &mut dyn Iterator<Item = char>) -> Pair {
    let mut pair = Pair::new(Number::Number(0), Number::Number(0));
    let mut active = &mut pair.left;
    let mut open_processed = false;
    while let Some(c) = input.next() {
        match c {
            '[' => {
                if open_processed {
                    *active = Number::Pair(Box::new(parse_pair(input)));
                } else {
                    open_processed = true;
                }
            }
            ']' => break,
            '0'..='9' => {
                *active = Number::Number(c.to_digit(10).unwrap() as i32);
            }
            ',' => {
                active = &mut pair.right;
            }
            _ => {
                panic!("Unknown pattern {}", c)
            }
        }
    }
    pair
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| parse_pair(&mut line.chars()))
        .collect_vec()
}

pub fn solve_part1(input: Input) -> u64 {
    0
}

pub fn solve_part2(input: Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        let first_pair = Pair::new(Number::Number(1), Number::Number(2));
        assert_eq!(parse_pair(&mut "[1,2]".chars()), first_pair);
        let second_pair = Pair::new(Number::Pair(Box::new(first_pair)), Number::Number(3));
        assert_eq!(parse_pair(&mut "[[1,2],3]".chars()), second_pair);
        let third_pair = Pair::new(
            Number::Number(9),
            Number::Pair(Box::new(Pair::new(Number::Number(8), Number::Number(7)))),
        );
        assert_eq!(parse_pair(&mut "[9,[8,7]]".chars()), third_pair);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
