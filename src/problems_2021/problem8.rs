use std::collections::HashSet;

use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem8/input.txt";

type Line = Vec<String>;
type Input = Vec<Line>;

pub fn parse_input(path_to_input: &str) -> Input {
    read_lines(path_to_input)
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|group| {
                    if group.contains('|') {
                        None
                    } else {
                        Some(group.to_string())
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn solve_part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|line| {
            line.iter()
                .skip(10)
                .map(|group| match group.len() {
                    2 => 1,
                    3 => 1,
                    4 => 1,
                    7 => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum()
}

type Pattern = HashSet<char>;
fn line_to_digits(input: &Line) -> Vec<i32> {
    let line = input
        .iter()
        .map(|group| group.chars().collect::<HashSet<char>>())
        .collect_vec();
    let (map, displayed_digits) = line.split_at(10);
    let mut digits = vec![Pattern::new(); 10];
    for pattern in map {
        let mapped = match pattern.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        };
        if let Some(index) = mapped {
            digits[index] = pattern.clone();
        }
    }
    let bd = &digits[4] - &digits[1];

    // len 6 && contains 4
    digits[9] = map
        .iter()
        .find(|pattern| {
            pattern.len() == 6 && digits[4].iter().all(|position| pattern.contains(position))
        })
        .unwrap()
        .clone();
    // len 6 &&  != 9 && contains bd
    digits[6] = map
        .iter()
        .find(|pattern| {
            pattern.len() == 6
                && **pattern != digits[9]
                && bd.iter().all(|position| pattern.contains(position))
        })
        .unwrap()
        .clone();
    // len 6 &&  != 9 && !=6
    digits[0] = map
        .iter()
        .find(|pattern| pattern.len() == 6 && **pattern != digits[6] && **pattern != digits[9])
        .unwrap()
        .clone();

    // len 5 && contains 1
    digits[3] = map
        .iter()
        .find(|pattern| {
            pattern.len() == 5 && digits[1].iter().all(|position| pattern.contains(position))
        })
        .unwrap()
        .clone();
    // len 5 && contains bd
    digits[5] = map
        .iter()
        .find(|pattern| pattern.len() == 5 && bd.iter().all(|position| pattern.contains(position)))
        .unwrap()
        .clone();
    // len 5 && !=3 && != 5
    digits[2] = map
        .iter()
        .find(|pattern| pattern.len() == 5 && **pattern != digits[3] && **pattern != digits[5])
        .unwrap()
        .clone();

    displayed_digits
        .iter()
        .map(|pattern| {
            digits
                .iter()
                .enumerate()
                .find_map(|(digit, digit_pattern)| {
                    if pattern == digit_pattern {
                        Some(digit as i32)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .collect()
}
pub fn solve_part2(input: &Input) -> i32 {
    input
        .iter()
        .map(|line| {
            let digits = line_to_digits(line);
            digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 452);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1096964);
    }
}
