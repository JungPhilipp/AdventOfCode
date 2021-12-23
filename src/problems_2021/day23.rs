use crate::util::index::*;
use array_tool::vec::Intersect;
use itertools::Itertools;
use log::{debug, info};

use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
    ops::{Range, RangeInclusive},
};

macro_rules! INPUT_PATH {
    () => {
        "day23/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 23:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Energy = u64;
type Position = usize;
type Dimensions = (usize, usize);
type Input = (
    HashSet<Position>,
    Vec<Vec<Position>>,
    Dimensions,
    Vec<Amphi>,
);

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

impl Color {}

#[derive(Clone, Debug)]
pub struct Amphi {
    pos: Position,
    energy: Energy,
    color: Color,
}

impl Amphi {
    fn from_char(pos: Position, input_color: char) -> Amphi {
        let color = match input_color {
            'A' => Color::Amber,
            'B' => Color::Bronze,
            'C' => Color::Copper,
            'D' => Color::Desert,
            _ => panic!("expected color not {}", input_color),
        };
        Amphi {
            pos,
            energy: 0,
            color,
        }
    }
}
fn print_board(board: HashSet<Position>, dimension: (usize, usize)) {
    let mut output = String::new();
    for y in 0..dimension.1 {
        for x in 0..dimension.0 {
            let flat_index = flatten((x as i32, y as i32), &dimension).unwrap();
            output += if board.contains(&flat_index) {
                "."
            } else {
                "#"
            }
        }
        output += "\n";
    }
    print!("{}", output);
}
pub fn parse(input: &str) -> Input {
    let lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let flat_lines = lines.iter().flatten().collect_vec();
    let rows = lines.len();
    let cols = lines[0].len();
    let dimensions = (cols, rows);
    dbg!(dimensions, flat_lines.len());
    let mut neighbors: Vec<Vec<Position>> = vec![vec![]; rows * cols];
    let mut valid_postions = HashSet::new();
    let amphis = flat_lines
        .iter()
        .enumerate()
        .filter_map(|(flat_index, &&c)| match c {
            'A' | 'B' | 'C' | 'D' => Some(Amphi::from_char(flat_index, c)),
            _ => None,
        })
        .collect_vec();
    for (flat_index, x) in flat_lines.iter().enumerate() {
        if ['.', 'A', 'B', 'C', 'D'].contains(x) {
            valid_postions.insert(flat_index);
            let (col_index, row_index) = expand(flat_index as i32, &dimensions);
            neighbors[flat_index] = [
                (col_index - 1, row_index),
                (col_index + 1, row_index),
                (col_index, row_index - 1),
                (col_index, row_index + 1),
            ]
            .into_iter()
            .filter_map(|index| {
                if let Some(flat_index) = flatten(index, &dimensions) {
                    if ['.', 'A', 'B', 'C', 'D'].contains(flat_lines[flat_index]) {
                        Some(flat_index)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect_vec();
        }
    }

    (valid_postions, neighbors, dimensions, amphis)
}

fn walk(neighbors: Vec<Vec<Position>>, amphis: Vec<Amphi>) -> Option<Energy> {
    for amphi in amphis {}
    0
}

pub fn solve_part1(input: Input) -> i64 {
    let (valid_postions, neighbors, dimensions, amphis) = input.clone();
    debug!("{:?}", input.clone());
    print_board(valid_postions, dimensions);
    0
}
pub fn solve_part2(input: Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        assert_eq!(solve_part1(parse(include_str!("day23/example1.txt"))), 65);
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
