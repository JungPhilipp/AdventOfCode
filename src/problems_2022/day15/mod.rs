use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::Index,
};

use itertools::Itertools;
use log::info;
use regex::Regex;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 15:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone(), 2000000),
        solve_part2(parsed, 0, 400000),
    );
}

type Point = (i32, i32);

type Input = Vec<(Point, Point)>;

fn parse(input: &str) -> Input {
    let expression =
        Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)")
            .unwrap();
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            expression
                .captures_iter(line)
                .map(|matches| {
                    (1..5)
                        .map(|i| {
                            matches
                                .get(i)
                                .expect("Should contain four groups")
                                .as_str()
                                .parse::<i32>()
                                .expect("Should be a number")
                        })
                        .tuples()
                        .map(|(x, y)| (x, y))
                        .collect_tuple::<(Point, Point)>()
                        .unwrap()
                })
                .exactly_one()
                .unwrap()
        })
        .collect()
}

fn manhattan_distance(lhs: &Point, rhs: &Point) -> usize {
    ((lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()) as usize
}

fn visible_for_sensor(sensor: &Point, beacon: &Point, point: &Point) -> bool {
    let sensor_range = manhattan_distance(sensor, beacon);
    let dist_to_point = manhattan_distance(sensor, point);
    dist_to_point <= sensor_range
}

fn solve_part1(input: Input, row: i32) -> usize {
    let max_x = 10_000_000;
    let mut impossible_positions = 0;
    for x in -max_x..=max_x {
        let point = (x, row);
        for (sensor, beacon) in input.iter() {
            if point == *sensor || point == *beacon {
                break;
            }
            if visible_for_sensor(sensor, beacon, &point) {
                info!("{:?}", point);
                impossible_positions += 1;
                break;
            }
        }
    }
    impossible_positions
}

type range = (i32, i32);
fn solve_part2(input: Input, min: i32, max: i32) -> usize {
    let x_ranges = vec![];
    let y_ranges = vec![];
    let squares = input
        .into_iter()
        .map(|(sensor, beacon)| (sensor, manhattan_distance(&sensor, &beacon)))
        .collect_vec();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    macro_rules! EXAMPLE_PATH {
        () => {
            "example_1.txt"
        };
    }

    #[test]
    fn example_1() {
        let input = parse(include_str!(EXAMPLE_PATH!()));
        assert_eq!(solve_part1(input, 10), 26);
    }

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(parse(include_str!(INPUT_PATH!())), 2000000),
            5142231
        );
    }

    #[test]
    fn example_1_2() {
        assert_eq!(
            solve_part2(parse(include_str!(EXAMPLE_PATH!())), 0, 20),
            56000011
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse(include_str!(INPUT_PATH!())), 0, 4000000),
            0
        );
    }
}
