use std::{collections::HashSet, panic};

use itertools::Itertools;
use log::info;

use crate::util::bool_helper::vec_to_number;

macro_rules! INPUT_PATH {
    () => {
        "problem20/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 20:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = (HashSet<usize>, HashSet<(i32, i32)>, (i32, i32));

pub fn parse(input: &str) -> Input {
    let lookup = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(index, c)| match c {
            '.' => None,
            '#' => Some(index),
            _ => panic!("Expected pixel"),
        })
        .collect();

    let image = input
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(column, c)| match c {
                    '.' => None,
                    '#' => Some((column as i32, row as i32)),
                    _ => panic!("Expected pixel"),
                })
                .collect_vec()
        })
        .flatten()
        .collect();
    (
        lookup,
        image,
        (
            input.lines().count() as i32 - 3,
            input.lines().nth(2).unwrap().len() as i32,
        ),
    )
}

fn enhance(input: Input, iterations: usize) -> usize {
    let (lookup, mut image, (_max_x, _max_y)) = input;
    let mut padding = false;
    for _ in 0..iterations {
        let mut output = HashSet::new();
        let min_x = *image.iter().map(|(x, _)| x).min().unwrap();
        let max_x = *image.iter().map(|(x, _)| x).max().unwrap();
        let min_y = *image.iter().map(|(_, y)| y).min().unwrap();
        let max_y = *image.iter().map(|(_, y)| y).max().unwrap();

        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                let indices = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];
                let new_index = indices
                    .iter()
                    .map(|(i, j)| {
                        if (min_x..=max_x).contains(i) && (min_y..=max_y).contains(j) {
                            image.contains(&(*i, *j))
                        } else {
                            padding
                        }
                    })
                    .collect_vec();
                if lookup.contains(&(vec_to_number(&new_index) as usize)) {
                    output.insert((x, y));
                }
            }
        }

        padding = if padding {
            lookup.contains(&511)
        } else {
            lookup.contains(&0)
        };
        image = output;
        dbg!(image.len());
    }

    image.len()
}

pub fn solve_part1(input: Input) -> usize {
    enhance(input, 2)
}

pub fn solve_part2(input: Input) -> usize {
    enhance(input, 50)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(parse(include_str!("problem20/example1.txt"))),
            35
        );
    }
    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 5819);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 18516);
    }
}
