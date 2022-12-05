use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day6/input.txt"
    };
}

macro_rules! EXAMPLE_1_PATH {
    () => {
        "day6/example_1.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 6:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<usize>;

fn parse(input: &str) -> Input {
    vec![]
}

fn solve_part1(input: Input) -> usize {
    0
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
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_1_PATH!()))), 0);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_1_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
