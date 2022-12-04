use std::collections::HashSet;

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day3/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 3:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<Vec<usize>>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.chars()
                        .map(|c| match c {
                            c if c.is_ascii_lowercase() => c as usize - 96,
                            c if c.is_ascii_uppercase() => c as usize - 64 + 26,
                            _ => panic!("Not a valid char"),
                        })
                        .collect_vec(),
                )
            }
        })
        .collect_vec()
}

fn solve_part1(input: Input) -> usize {
    input
        .into_iter()
        .map(|rucksack| {
            rucksack
                .chunks(rucksack.len() / 2)
                .map(|compartment| compartment.iter().cloned().collect::<HashSet<usize>>())
                .reduce(|acc, e| acc.intersection(&e).cloned().collect())
                .unwrap()
                .drain()
                .sum::<usize>()
        })
        .sum()
}

fn solve_part2(input: Input) -> usize {
    input
        .into_iter()
        .map(|rucksack| rucksack.into_iter().collect::<HashSet<usize>>())
        .chunks(3)
        .into_iter()
        .map(|group| {
            group
                .reduce(|acc, e| acc.intersection(&e).cloned().collect())
                .unwrap()
                .drain()
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse(include_str!("day3/example_1.txt"))), 157);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 8072);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!("day3/example_1.txt"))), 70);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 2567);
    }
}
