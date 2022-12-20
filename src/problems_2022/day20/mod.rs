use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

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
        "Solutions Day 20:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn mix(vec: &mut [i64], mut index: usize, mut dest: i64) {
    while dest != 0 {
        let new_index = {
            match index as i64 + dest.signum() {
                larger if larger >= vec.len() as i64 => larger as usize % vec.len(),
                smaller if smaller < 0 => (vec.len() as i64 + smaller) as usize,
                in_range => in_range as usize,
            }
        };
        vec.swap(index, new_index);

        debug!("{dest}: {index} -> {new_index}");
        index = new_index;
        dest -= dest.signum();
    }
}

fn solve_part1(mut input: Input) -> i64 {
    let count = input.len();
    let mut tracking = (0..count as i64).collect_vec();

    for original_index in 0..count {
        info!("Move {original_index}/{count}");
        let (index, _) = tracking
            .iter()
            .find_position(|i| **i == original_index as i64)
            .unwrap();
        let displacement = input[index];
        mix(&mut input, index, displacement);
        mix(&mut tracking, index, displacement);
    }

    let pos_0 = input.iter().find_position(|&&v| v == 0).unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| (pos_0.0 + offset) % count)
        .map(|index| input[index])
        .inspect(|e| info!("{e}"))
        .sum()
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
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 4267);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
