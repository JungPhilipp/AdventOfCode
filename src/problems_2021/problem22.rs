use std::{collections::HashSet, num::ParseIntError, ops::RangeInclusive};

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "problem22/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 22:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<(
    bool,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
)>;

fn next_number(iter: &mut dyn Iterator<Item = char>) -> Result<i32, ParseIntError> {
    iter.skip_while(|c| !c.is_digit(10) && *c != '-')
        .take_while(|c| c.is_digit(10) || *c == '-')
        .collect::<String>()
        .parse::<i32>()
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let on = if l.starts_with("on") {
                true
            } else if l.starts_with("off") {
                false
            } else {
                panic!("Expected on or off {}", l)
            };
            let mut line = l.chars();
            let x_min = next_number(&mut line).unwrap();
            let x_max = next_number(&mut line).unwrap();
            let y_min = next_number(&mut line).unwrap();
            let y_max = next_number(&mut line).unwrap();
            let z_min = next_number(&mut line).unwrap();
            let z_max = next_number(&mut line).unwrap();

            (on, x_min..=x_max, y_min..=y_max, z_min..=z_max)
        })
        .collect_vec()
}

fn reboot(
    input: Input,
    valid_x_range: RangeInclusive<i32>,
    valid_y_range: RangeInclusive<i32>,
    valid_z_range: RangeInclusive<i32>,
) -> usize {
    let mut points = HashSet::<(i32, i32, i32)>::new();

    for (on, x_range, y_range, z_range) in input {
        for x in x_range {
            if !valid_x_range.contains(&x) {
                continue;
            }
            for y in y_range.clone() {
                if !valid_y_range.contains(&y) {
                    continue;
                }
                for z in z_range.clone() {
                    if !valid_z_range.contains(&z) {
                        continue;
                    }
                    if on {
                        points.insert((x, y, z));
                    } else {
                        points.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    points.len()
}

pub fn solve_part1(input: Input) -> usize {
    let valid_x_range = -50..=50;
    let valid_y_range = -50..=50;
    let valid_z_range = -50..=50;
    reboot(input, valid_x_range, valid_y_range, valid_z_range)
}

pub fn solve_part2(input: Input) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(parse(include_str!("problem22/example1.txt"))),
            39
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(parse(include_str!("problem22/example2.txt"))),
            590784
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part2(parse(include_str!("problem22/example3.txt"))),
            2758514936282235
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
