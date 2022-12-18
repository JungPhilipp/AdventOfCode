use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use log::{debug, info};

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 14:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}
type Point = (i32, i32, i32);
type Input = HashSet<Point>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .split(',')
                .map(|c| c.parse::<i32>().expect("Should be a number"))
                .collect_tuple()
                .expect("Should be three parts")
        })
        .collect()
}

fn cube_sides(point: &Point) -> Vec<Point> {
    [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    .into_iter()
    .map(|offset| (point.0 + offset.0, point.1 + offset.1, point.2 + offset.2))
    .collect()
}

fn solve_part1(input: Input) -> usize {
    input
        .iter()
        .flat_map(cube_sides)
        .filter(|side| !input.contains(side))
        .count()
}

fn is_background(
    cache_background: &mut HashSet<Point>,
    cache_not_background: &mut HashSet<Point>,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
    point: &Point,
) -> bool {
    if cache_background.contains(&point) {
        return true;
    }

    if cache_not_background.contains(&point) {
        return false;
    }

    let mut queue = vec![*point];
    let mut visited = HashSet::<Point>::new();
    while let Some(p) = queue.pop() {
        if cache_background.contains(&p)
            || (p.0 < x_range.0 || p.0 > x_range.1)
            || (p.1 < y_range.0 || p.1 > y_range.1)
            || (p.2 < z_range.0 || p.2 > z_range.1)
        {
            cache_background.insert(p);
            return true;
        }
        if visited.contains(&p) || cache_not_background.contains(&p) {
            continue;
        }
        visited.insert(p);

        for neighbor in cube_sides(&p) {
            queue.push(neighbor);
        }
    }
    cache_not_background.insert(*point);
    false
}

fn solve_part2(input: Input) -> usize {
    let x_range = match input.iter().map(|(x, _, _)| x).minmax() {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => unreachable!(),
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
    };

    let y_range = match input.iter().map(|(_, y, _)| y).minmax() {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => unreachable!(),
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
    };

    let z_range = match input.iter().map(|(_, _, z)| z).minmax() {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => unreachable!(),
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
    };

    let mut cache_background = HashSet::new();
    let mut cache_not_background: HashSet<Point> = input.iter().cloned().collect();

    debug!("{:?}, {:?}, {:?}", x_range, y_range, z_range);

    input
        .iter()
        .flat_map(cube_sides)
        .filter(|side| {
            is_background(
                &mut cache_background,
                &mut cache_not_background,
                x_range,
                y_range,
                z_range,
                side,
            )
        })
        .count()
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
    fn example_0() {
        let input = parse(include_str!("example_0.txt"));
        assert_eq!(solve_part1(input), 10);
    }

    #[test]
    fn example_0_1() {
        assert_eq!(
            solve_part1([(1, 1, 1), (2, 1, 1), (3, 1, 1)].into_iter().collect()),
            14
        );
        assert_eq!(
            solve_part1(
                [(1, 1, 1), (2, 1, 1), (3, 1, 1), (4, 1, 1)]
                    .into_iter()
                    .collect()
            ),
            18
        );
        assert_eq!(
            solve_part1([(1, 1, 1), (2, 1, 1), (2, 2, 1)].into_iter().collect()),
            14
        );
    }

    #[test]
    fn example_1() {
        let input = parse(include_str!(EXAMPLE_PATH!()));
        assert_eq!(solve_part1(input), 64);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 4282);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 58);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 2452);
    }
}
