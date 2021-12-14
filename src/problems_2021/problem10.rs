#![allow(clippy::ptr_arg)]
use std::panic;

use itertools::Itertools;
use log::debug;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem10/input.txt";

type Line = Vec<Bracket>;
type Input = Vec<Line>;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

fn bracket_error_score(bracket: &Bracket) -> i32 {
    match bracket {
        Bracket::RoundOpen | Bracket::RoundClose => 3,
        Bracket::SquareOpen | Bracket::SquareClose => 57,
        Bracket::CurlyOpen | Bracket::CurlyClose => 1197,
        Bracket::AngleOpen | Bracket::AngleClose => 25137,
    }
}

fn map_to_other_bracket(bracket: &Bracket) -> Bracket {
    match bracket {
        Bracket::RoundOpen => Bracket::RoundClose,
        Bracket::RoundClose => Bracket::RoundOpen,
        Bracket::SquareOpen => Bracket::SquareClose,
        Bracket::SquareClose => Bracket::SquareOpen,
        Bracket::CurlyOpen => Bracket::CurlyClose,
        Bracket::CurlyClose => Bracket::CurlyOpen,
        Bracket::AngleOpen => Bracket::AngleClose,
        Bracket::AngleClose => Bracket::AngleOpen,
    }
}

pub fn parse_input(path_to_input: &str) -> Input {
    read_lines(path_to_input)
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|char| match char {
                    '(' => Some(Bracket::RoundOpen),
                    ')' => Some(Bracket::RoundClose),
                    '[' => Some(Bracket::SquareOpen),
                    ']' => Some(Bracket::SquareClose),
                    '{' => Some(Bracket::CurlyOpen),
                    '}' => Some(Bracket::CurlyClose),
                    '<' => Some(Bracket::AngleOpen),
                    '>' => Some(Bracket::AngleClose),
                    c if c.is_whitespace() => None,
                    _ => panic!("Not a bracket: {:?}", char),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn find_errors(line: &Line) -> Option<Bracket> {
    let mut bracket_stack = vec![];
    for bracket in line {
        match bracket {
            &Bracket::RoundOpen
            | &Bracket::SquareOpen
            | &Bracket::CurlyOpen
            | &Bracket::AngleOpen => bracket_stack.push(bracket),
            &Bracket::RoundClose
            | &Bracket::SquareClose
            | &Bracket::CurlyClose
            | &Bracket::AngleClose => match bracket_stack.last() {
                None => return Some(Bracket::AngleClose),
                Some(&open) => {
                    let expected_close = map_to_other_bracket(open);
                    if *bracket == expected_close {
                        bracket_stack.pop();
                    } else {
                        return Some(*bracket);
                    }
                }
            },
        }
    }
    None
}

pub fn solve_part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|line| find_errors(line).map_or(0, |bracket| bracket_error_score(&bracket) as i32))
        .sum()
}
fn autocomplete(line: &Line) -> Vec<Bracket> {
    let mut bracket_stack = vec![];
    for bracket in line {
        match bracket {
            &Bracket::RoundOpen
            | &Bracket::SquareOpen
            | &Bracket::CurlyOpen
            | &Bracket::AngleOpen => bracket_stack.push(bracket),
            &Bracket::RoundClose
            | &Bracket::SquareClose
            | &Bracket::CurlyClose
            | &Bracket::AngleClose => match bracket_stack.pop() {
                None => panic!("Expected only incomplete not corrupted lines"),
                Some(&open) => {
                    let expected_close = map_to_other_bracket(&open);
                    assert_eq!(*bracket, expected_close);
                }
            },
        }
    }
    bracket_stack
        .iter()
        .rev()
        .map(|&bracket| map_to_other_bracket(bracket))
        .collect_vec()
}

fn autocomplete_score(line: &Line) -> usize {
    debug!("Incomplete line found {:?}", line);
    let mut score = 0;
    for bracket in line {
        score = score * 5
            + match bracket {
                Bracket::RoundClose => 1,
                Bracket::SquareClose => 2,
                Bracket::CurlyClose => 3,
                Bracket::AngleClose => 4,
                _ => panic!("Illegal suggestion: {:?}", bracket),
            };
    }
    if line.is_empty() {
        assert!(score > 0);
    }
    score
}

pub fn solve_part2(input: &Input) -> usize {
    let scores = input
        .iter()
        .filter(|line| find_errors(line).is_none())
        .map(autocomplete)
        .map(|suggestion| autocomplete_score(&suggestion))
        .sorted()
        .collect_vec();

    assert_eq!(scores.len() % 2, 1);
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example() {
        assert_eq!(
            solve_part1(&parse_input("src/problems_2021/problem10/example.txt")),
            26397
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 462693);
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&parse_input("src/problems_2021/problem10/example.txt")),
            288957
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 3094671161);
    }
}
