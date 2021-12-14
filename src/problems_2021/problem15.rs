use std::collections::HashMap;

use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem15/input.txt";

type Input = Vec<i32>;

pub fn parse_input(path_to_input: &str) -> Input {
    vec![]
}

pub fn solve_part1(input: &Input) -> i32 {
    0
}

pub fn solve_part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    pub static EXAMPLE_PATH: &str = "src/problems_2021/problem15/example.txt";

    #[test]
    fn example1() {
        let input = parse_input(EXAMPLE_PATH);
        assert_eq!(solve_part1(&input), 0);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 0);
    }

    #[test]
    fn example2() {
        let input = parse_input(EXAMPLE_PATH);
        assert_eq!(solve_part2(&input), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 0);
    }
}
