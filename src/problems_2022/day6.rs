use std::collections::HashSet;

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day6/input.txt"
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

type Input = Vec<char>;

fn parse(input: &str) -> Input {
    input.trim().chars().collect_vec()
}

fn first_unique_sequence(input: Input, length: usize) -> usize {
    *input
        .into_iter()
        .enumerate()
        .collect_vec()
        .windows(length)
        .find(|window| {
            window
                .iter()
                .map(|(_, x)| x)
                .cloned()
                .collect::<HashSet<char>>()
                .len()
                == length
        })
        .expect("Expected to find at least one")
        .iter()
        .map(|(index, _)| index)
        .max()
        .expect("Expected to find a max index")
        + 1
}

fn solve_part1(input: Input) -> usize {
    first_unique_sequence(input, 4)
}

fn solve_part2(input: Input) -> usize {
    first_unique_sequence(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part1(parse("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_part1(parse("nppdvjthqldpwncqszvftbrmjlhg")), 6);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve_part1(parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
    }

    #[test]
    fn example_5() {
        assert_eq!(solve_part1(parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 1929);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 3298);
    }
}
