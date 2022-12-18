use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 14:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}
type Point = (i32, i32, i32);
type Input = HashSet<Point>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .split(',')
                .map(|c| c.parse::<i32>().expect("Should be a number"))
                .collect_tuple()
                .expect("Should be three parts")
        })
        .collect()
}

fn cube_sides(point: &Point) -> Vec<Point> {
    [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    .into_iter()
    .map(|offset| (point.0 + offset.0, point.1 + offset.1, point.2 + offset.2))
    .collect()
}

fn solve_part1(input: Input) -> usize {
    input
        .iter()
        .flat_map(cube_sides)
        .filter(|side| !input.contains(side))
        .count()
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
    fn example_0() {
        let input = parse(include_str!("example_0.txt"));
        assert_eq!(solve_part1(input), 10);
    }

    #[test]
    fn example_0_1() {
        assert_eq!(
            solve_part1([(1, 1, 1), (2, 1, 1), (3, 1, 1)].into_iter().collect()),
            14
        );
        assert_eq!(
            solve_part1(
                [(1, 1, 1), (2, 1, 1), (3, 1, 1), (4, 1, 1)]
                    .into_iter()
                    .collect()
            ),
            18
        );
        assert_eq!(
            solve_part1([(1, 1, 1), (2, 1, 1), (2, 2, 1)].into_iter().collect()),
            14
        );
    }

    #[test]
    fn example_1() {
        let input = parse(include_str!(EXAMPLE_PATH!()));
        assert_eq!(solve_part1(input), 64);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
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
