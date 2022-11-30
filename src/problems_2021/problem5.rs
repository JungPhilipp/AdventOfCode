#![allow(clippy::ptr_arg)]
use std::collections::HashMap;

use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem5/input.txt";

type Point = (i32, i32);
type Line = (Point, Point);
type Input = Vec<Line>;
type Board = HashMap<Point, i32>;

pub fn parse_input(path_to_input: &str) -> Input {
    read_lines(path_to_input)
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .map(|number| number.parse::<i32>().unwrap())
                        .collect_tuple::<Point>()
                        .unwrap()
                })
                .collect_tuple::<Line>()
                .unwrap()
        })
        .collect_vec()
}

fn horizontal(line: &Line) -> bool {
    line.0 .1 == line.1 .1
}

fn vertical(line: &Line) -> bool {
    line.0 .0 == line.1 .0
}

pub fn create_inclusive_range<T: std::cmp::PartialOrd + std::iter::Step>(
    start: T,
    end: T,
) -> std::ops::RangeInclusive<T> {
    if start <= end {
        start..=end
    } else {
        end..=start
    }
}
fn add_line(board: &mut Board, line: &Line) {
    let x_range = create_inclusive_range(line.0 .0, line.1 .0);
    let y_range = create_inclusive_range(line.0 .1, line.1 .1);
    let mut update = |point| *board.entry(point).or_insert(0) += 1;
    if horizontal(line) {
        let y = line.0 .1;
        for x in x_range {
            update((x, y));
        }
    } else if vertical(line) {
        let x = line.0 .0;
        for y in y_range {
            update((x, y));
        }
    } else {
        let x_step = if line.0 .0 < line.1 .0 { 1 } else { -1 };
        let y_step = if line.0 .1 < line.1 .1 { 1 } else { -1 };

        let mut point = line.0;
        while point != line.1 {
            update(point);
            point.0 += x_step;
            point.1 += y_step;
        }
        update(line.1);
    }
}

pub fn solve_part1(input: &Input) -> i32 {
    let mut board = Board::new();
    for line in input {
        if !(vertical(line) || horizontal(line)) {
            continue;
        }
        add_line(&mut board, line);
    }
    board.into_values().map(|count| i32::from(count >= 2)).sum()
}

pub fn solve_part2(input: &Input) -> i32 {
    let mut board = Board::new();
    for line in input {
        add_line(&mut board, line);
    }
    board.into_values().map(|count| i32::from(count >= 2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 7085);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 20271);
    }
}
