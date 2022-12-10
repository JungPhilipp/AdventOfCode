use std::collections::HashSet;

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day10/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 10:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<String>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}
fn parse_addx(instruction: &str) -> i32 {
    instruction
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .expect("A number")
}

fn solve_part1(input: Input) -> i32 {
    let mut register = 1;
    input
        .into_iter()
        .flat_map(|instruction| {
            let mut register_values = vec![];
            match instruction.as_str() {
                "noop" => register_values.push(register),
                i if i.starts_with("addx") => {
                    register_values.push(register);
                    register += parse_addx(i);
                    register_values.push(register);
                }
                unmatched => panic!("Not a valid instruction {}", unmatched),
            };
            register_values
        })
        .enumerate()
        .filter_map(|(index, register)| {
            let cycle = index as i32 + 2;
            if (cycle - 20) % 40 == 0  && cycle > 0 && cycle <= 220{
                let signal_strength = cycle * register;
                info!("{}, {}, {}", cycle, register, signal_strength);
                Some(signal_strength)
            } else {
                None
            }
        })
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
            "day10/example_1.txt"
        };
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_PATH!()))), 13140);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    // #[test]
    // fn example_1_2() {
    //     assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    // }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
