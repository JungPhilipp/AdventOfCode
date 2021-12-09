use std::collections::{HashMap, HashSet};

use array_tool::vec::Intersect;
use itertools::Itertools;
use log::debug;

use crate::util::parse::{parse_to, parse_to_vec, read_lines};

pub static INPUT_PATH: &str = "src/problems_2021/problem10/input.txt";

type Input = (usize, Vec<i32>);

pub fn parse_input(path_to_input: &str) -> Input {
    let input = parse_to_vec(path_to_input);
    (input[0].len(), input.into_iter().flatten().collect_vec())
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
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 468);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 35287552);
    }
}
