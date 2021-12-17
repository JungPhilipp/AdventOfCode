use std::{collections::VecDeque, panic};

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

type Input = Vec<bool>;

pub fn parse(input: &str) -> Input {
    vec![]
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
    fn example1() {}

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
