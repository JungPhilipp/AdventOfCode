use core::fmt;
use std::{
    collections::{LinkedList, VecDeque},
    ops::Add,
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
        solve_part1(&parsed),
        solve_part2(&parsed)
    );
}

type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    vec![]
}

pub fn solve_part1(input: &Input) -> i32 {
    0
}

pub fn solve_part2(input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse(include_str!(INPUT_PATH!()))), 4480);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse(include_str!(INPUT_PATH!()))), 4676);
    }
}
