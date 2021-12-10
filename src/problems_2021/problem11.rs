use std::panic;

use itertools::Itertools;
use log::debug;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem10/input.txt";

type Input = Vec<i32>;

pub fn parse_input(path_to_input: &str) -> Input {
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
    fn example() {
        assert_eq!(
            solve_part1(&parse_input("src/problems_2021/problem10/example.txt")),
            26397
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 462693);
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&parse_input("src/problems_2021/problem10/example.txt")),
            288957
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 0);
    }
}
