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
        "Solutions Day 1:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<Vec<usize>>;

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter_map(|item| {
                    if item.is_empty() {
                        None
                    } else {
                        Some(item.parse::<usize>().expect("Not a nummber"))
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

fn solve_part1(input: Input) -> usize {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap_or(0)
}

fn solve_part2(input: Input) -> usize {
    input.iter().map(|elf| elf.iter().sum::<usize>()).sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_example_1() {
        assert_eq!(
            24000,
            solve_part1(parse(include_str!("day1/example_1.txt")))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 71934);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            45000,
            solve_part2(parse(include_str!("day1/example_1.txt")))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 211447);
    }
}
