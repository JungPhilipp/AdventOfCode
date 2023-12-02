use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day1/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 1:\nPart1 {}\nPart2 {}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = String;

fn parse(input: &str) -> Input {
    input.to_string()
}

fn solve_part1(input: Input) -> usize {
    input
        .lines()
        .map(|line| {
            (10 * line.chars().find_map(|c| c.to_digit(10)).unwrap_or(0)
                + line.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or(0)) as usize
        })
        .sum()
}

fn solve_part2(input: Input) -> usize {
    input
        .lines()
        .map(|mut line| {
            let mut digits = vec![];

            let spelled = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            while !line.is_empty() {
                if let Some(digit) = line.chars().next().and_then(|c| c.to_digit(10)) {
                    digits.push(digit as usize);
                } else if let Some((pos, _)) = spelled
                    .iter()
                    .find_position(|&&digit| line.starts_with(digit))
                {
                    digits.push(pos + 1);
                }
                line = &line[1..];
            }

            10 * digits.first().unwrap_or(&0) + digits.last().unwrap_or(&0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_example_1() {
        assert_eq!(142, solve_part1(parse(include_str!("day1/example_1.txt"))));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(281, solve_part2(parse(include_str!("day1/example_2.txt"))));
    }
}
