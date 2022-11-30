use core::fmt;
use std::{collections::VecDeque, panic};

use itertools::Itertools;
use log::{debug, info};
use regex::Regex;

macro_rules! INPUT_PATH {
    () => {
        "problem18/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 18:\nPart1{}\nPart2{}",
        solve_part1(&parsed),
        solve_part2(&parsed)
    );
}

type Input = VecDeque<Vec<Token>>;

#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Other(char),
}

impl Token {
    fn number(&self) -> i32 {
        match self {
            Token::Number(n) => *n,
            Token::Other(c) => panic!("Not a number {}", c),
        }
    }
    fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            Token::Other(_) => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print = match self {
            Token::Number(n) => n.to_string(),
            Token::Other(c) => c.to_string(),
        };
        write!(f, "{}", print)
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print = match self {
            Token::Number(n) => n.to_string(),
            Token::Other(c) => c.to_string(),
        };
        write!(f, "{}", print)
    }
}
fn parse_line(input: &str) -> Vec<Token> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map_or_else(|| Token::Other(c), |n| Token::Number(n as i32))
        })
        .collect_vec()
}

pub fn parse(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn to_string(number: &[Token]) -> String {
    number.iter().map(|t| t.to_string()).collect::<String>()
}

fn magnitude(it: &mut dyn Iterator<Item = Token>) -> i32 {
    let mut left = None;
    let mut right = None;
    let mut active = &mut left;
    while let Some(token) = it.next() {
        match token {
            Token::Number(number) => *active = Some(number),
            Token::Other(c) => match c {
                ',' => active = &mut right,
                '[' => *active = Some(magnitude(it)),
                ']' => break,
                _ => panic!(),
            },
        }
    }

    if right.is_none() {
        left.unwrap()
    } else {
        3 * left.unwrap_or(0) + 2 * right.unwrap_or(0)
    }
}

fn try_split(current: &mut Vec<Token>) -> bool {
    if let Some((index, token)) = current.iter().find_position(|token| {
        if let Token::Number(n) = token {
            *n >= 10
        } else {
            false
        }
    }) {
        let number = token.number();
        debug!("Split {} into {}, {}", number, number / 2, (number + 1) / 2);
        let pair = vec![
            Token::Other('['),
            Token::Number(number / 2),
            Token::Other(','),
            Token::Number((number + 1) / 2),
            Token::Other(']'),
        ];
        current.splice(index..index + 1, pair);
        true
    } else {
        false
    }
}

fn try_explode(current: &mut Vec<Token>) -> bool {
    let pair = Regex::new(r"^\[\d+,\d+\]$").unwrap();
    let mut level = 0;
    for index in 0..current.len() {
        if let Token::Other(c) = current[index] {
            match c {
                '[' => level += 1,
                ']' => level -= 1,
                _ => {}
            }
        }
        if level <= 4 || index + 5 >= current.len() {
            continue;
        }
        let pair_range = index..index + 5;
        let pattern = to_string(&current[pair_range.clone()]);
        if !pair.is_match(pattern.as_str()) {
            debug!("Did not match {}", pattern);
            continue;
        }
        debug!("Explode {}", pattern);
        let left = current[pair_range.start + 1].number();
        let right = current[pair_range.start + 3].number();

        if let Some(Token::Number(n)) = current[..pair_range.start]
            .iter_mut()
            .rev()
            .find(|t| t.is_number())
        {
            debug!("found left {}, add {}", n, left);
            *n += left;
        }
        if let Some(Token::Number(n)) = current[pair_range.end..].iter_mut().find(|t| t.is_number())
        {
            debug!("found right {}, add {}", n, right);
            *n += right;
        }
        // add to right and left
        current.splice(index..index + 5, [Token::Number(0)].into_iter());
        return true;
    }
    false
}

fn reduce(current: &mut Vec<Token>) {
    loop {
        if !try_explode(current) && !try_split(current) {
            debug!("Could not reduce {}", to_string(current));
            break;
        }
        debug!("After reduce: {}", to_string(current));
    }
}

fn sum(mut input: Input) -> Vec<Token> {
    let mut current = input.pop_front().unwrap();
    while let Some(number) = input.pop_front() {
        debug!("Before Addition: {}", to_string(&current));
        // Add
        current = vec![
            vec![Token::Other('[')],
            current,
            vec![Token::Other(',')],
            number,
            vec![Token::Other(']')],
        ]
        .into_iter()
        .flatten()
        .collect_vec();
        debug!("After Addition:  {}", to_string(&current));
        reduce(&mut current);
    }
    current
}

pub fn solve_part1(input: &Input) -> i32 {
    let final_sum = sum(input.clone());
    info!("Final Sum {}", to_string(&final_sum));
    magnitude(&mut final_sum.into_iter())
}

pub fn solve_part2(input: &Input) -> i32 {
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            let x = input[i].clone();
            let y = input[j].clone();
            let sum = magnitude(&mut sum([x, y].into()).into_iter());
            if sum > max {
                max = sum;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        assert_eq!(
            to_string(&sum(parse(include_str!("problem18/example1.txt")))),
            to_string(&parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
        );
    }

    #[test]
    fn example1_small() {
        assert_eq!(
            to_string(&sum(parse(
                "[1,1]
                 [2,2]
                 [3,3]
                 [4,4]"
            ))),
            to_string(&parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
        );
        assert_eq!(
            to_string(&sum(parse(
                "[1,1]
                 [2,2]
                 [3,3]
                 [4,4]
                 [5,5]"
            ))),
            to_string(&parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
        );
        assert_eq!(
            to_string(&sum(parse(
                "[1,1]
                 [2,2]
                 [3,3]
                 [4,4]
                 [5,5]
                 [6,6]"
            ))),
            to_string(&parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
        );
    }

    #[test]
    fn example2_small() {
        assert_eq!(
            to_string(&sum(parse(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                 [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
            ))),
            to_string(&parse_line(
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            )),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            to_string(&sum(parse(include_str!("problem18/example2.txt")))),
            to_string(&parse_line(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
        );
    }
    #[test]
    fn example3_part1() {
        assert_eq!(
            solve_part1(&parse(include_str!("problem18/example3.txt"))),
            4140
        );
    }

    #[test]
    fn example3_part2() {
        assert_eq!(
            solve_part2(&parse(include_str!("problem18/example3.txt"))),
            3993
        );
    }

    #[test]
    fn test_magnitude(input: Input) {
        assert_eq!(
            magnitude(&mut parse_line("[[1,2],[[3,4],5]]").into_iter()),
            143
        );
        assert_eq!(
            magnitude(&mut parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").into_iter()),
            1384
        );
        assert_eq!(
            magnitude(&mut parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]").into_iter()),
            445
        );
        assert_eq!(
            magnitude(&mut parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]").into_iter()),
            791
        );
        assert_eq!(
            magnitude(&mut parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]").into_iter()),
            1137
        );
        assert_eq!(
            magnitude(
                &mut parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                    .into_iter()
            ),
            3488
        );
    }

    #[test]
    fn test_reduce() {
        let mut input = parse_line("[[[[[9,8],1],2],3],4]");
        reduce(&mut input);
        assert_eq!(
            to_string(&input),
            to_string(&parse_line("[[[[0,9],2],3],4]"))
        );

        input = parse_line("[7,[6,[5,[4,[3,2]]]]]");
        reduce(&mut input);
        assert_eq!(
            to_string(&input),
            to_string(&parse_line("[7,[6,[5,[7,0]]]]"))
        );

        input = parse_line("[[6,[5,[4,[3,2]]]],1]");
        reduce(&mut input);
        assert_eq!(
            to_string(&input),
            to_string(&parse_line("[[6,[5,[7,0]]],3]"))
        );

        input = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        reduce(&mut input);
        assert_eq!(
            to_string(&input),
            to_string(&parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
        );
    }

    #[test]
    fn test_explode() {
        let mut input = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert!(try_explode(&mut input));
        assert_eq!(
            to_string(&input),
            to_string(&parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse(include_str!(INPUT_PATH!()))), 4480);
    }

    #[test]
    fn part2() {
        //assert_eq!(solve_part2(&parse(include_str!(INPUT_PATH!()))), 4676);
    }
}
