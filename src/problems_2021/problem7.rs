#![allow(clippy::ptr_arg)]
use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem7/input.txt";

type Input = Vec<i32>;

pub fn parse_input(path_to_input: &str) -> Input {
    read_lines(path_to_input)[0]
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect_vec()
}

pub fn solve_part1(input: &Input) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..max)
        .map(|position| input.iter().map(|crap| (crap - position).abs()).sum())
        .min()
        .unwrap()
}

pub fn solve_part2(input: &Input) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..max)
        .map(|position| {
            input
                .iter()
                .map(|crap| {
                    let distance = (crap - position).abs();
                    distance * (distance + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 343468);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 96086265);
    }
}
