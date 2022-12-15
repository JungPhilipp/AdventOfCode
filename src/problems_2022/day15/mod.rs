use std::cmp::max;

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
        solve_part2(parsed, 400000),
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

type Interval = (i32, i32);
fn compute_intervals(input: &Input, row: i32) -> Vec<Interval> {
    input
        .iter()
        .filter_map(|(sensor, beacon)| {
            let distance = manhattan_distance(sensor, beacon);
            match distance as i32 - (row - sensor.1).abs() {
                width if width > 0 => Some((sensor.0 - width, sensor.0 + width)),
                _ => None,
            }
        })
        .sorted()
        .rev()
        .collect()
}

fn solve_part1(input: Input, row: i32) -> usize {
    let mut visible_positions = 0;
    let mut intervals = compute_intervals(&input, row);

    let mut interval = intervals.pop().unwrap();
    for next_interval in intervals.into_iter().rev() {
        if interval.1 >= next_interval.0 {
            interval.1 = max(interval.1, next_interval.1)
        } else {
            visible_positions += interval.1 - interval.0;
            interval = next_interval;
        }
    }

    (interval.1 - interval.0 + visible_positions) as usize
}

fn solve_part2(input: Input, length: i32) -> usize {
    for row in 0..=length {
        if let Err(x) = std::iter::once((length + 1, i32::MAX))
            .chain(compute_intervals(&input, row).into_iter())
            .rev()
            .try_fold(0, |x, next_interval| {
                if x + 1 < next_interval.0 {
                    Err(x + 1)
                } else {
                    Ok(max(x, next_interval.1))
                }
            })
        {
            info!("Found: {:?}", (x, row));
            return x as usize * 4000000 + row as usize;
        }
    }
    unreachable!()
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
            solve_part2(parse(include_str!(EXAMPLE_PATH!())), 20),
            56000011
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse(include_str!(INPUT_PATH!())), 4000000),
            10884459367718
        );
    }
}
