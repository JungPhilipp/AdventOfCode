use std::collections::HashSet;

use array_tool::vec::Intersect;
use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day4/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 4:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Pair = (usize, usize);
type Input = Vec<(Pair, Pair)>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|pair| {
                    pair.split('-')
                        .map(|c| c.parse::<usize>().expect("Expected a number"))
                        .collect_tuple::<Pair>()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn contains(first: &Pair, second: &Pair) -> bool {
    first.0 <= second.0 && first.1 >= second.1
}

fn solve_part1(input: Input) -> usize {
    input
        .into_iter()
        .filter(|(first, second)| contains(first, second) || contains(second, first))
        .count()
}

fn solve_part2(input: Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse(include_str!("day4/example_1.txt"))), 2);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 477);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!("day4/example_1.txt"))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
