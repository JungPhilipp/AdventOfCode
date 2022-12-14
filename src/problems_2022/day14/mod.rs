use std::{
    cmp::{max, min},
    collections::HashSet,
    iter,
};

use itertools::Itertools;
use log::{debug, info};
use primes::is_prime;

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
type Point = (i32, i32);
type Input = HashSet<Point>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            line.split("->")
                .map(|points| {
                    points
                        .split(',')
                        .map(|point| point.trim().parse::<i32>().expect("Should be a number"))
                        .collect_tuple::<Point>()
                        .expect("Should have x and y coordinates")
                })
                .tuple_windows()
                .flat_map(|(a, b)| {
                    assert!(a.0 == b.0 || a.1 == b.1);
                    (min(a.0, b.0)..=max(a.0, b.0)).flat_map(move |i| {
                        (min(a.1, b.1)..=max(a.1, b.1)).map(move |j| (i, j))
                    })
                })
        })
        .collect()
}

fn drop_sand(start: Point, rock: &HashSet<Point>, abyss: i32) -> Option<Point> {
    let mut sand = start;
    while sand.1 < abyss {
        if !rock.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
        } else if !rock.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
        } else if !rock.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
        } else {
            return Some(sand);
        }
    }
    None
}

fn drop_sand_floor(start: Point, filled: &HashSet<Point>, floor_y: i32) -> Point {
    let mut sand = start;
    loop {
        if sand.1 + 1 == floor_y {
            break;
        } else if !filled.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
        } else if !filled.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
        } else if !filled.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
        } else {
            break;
        }
    }
    sand
}

fn solve_part1(input: Input) -> usize {
    let mut filled = input;
    let abyss_y = filled.iter().map(|(_, y)| y).max().unwrap() + 1;
    let sand_source = (500, 0);
    let mut sand_grains_at_rest = 0;
    while let Some(sand_at_rest) = drop_sand(sand_source, &filled, abyss_y) {
        filled.insert(sand_at_rest);
        sand_grains_at_rest += 1
    }
    sand_grains_at_rest
}

fn solve_part2(input: Input) -> usize {
    let mut filled = input;
    let floor_y = filled.iter().map(|(_, y)| y).max().unwrap() + 2;
    let sand_source = (500, 0);
    let mut sand_grains_at_rest = 0;
    loop {
        let sand_at_rest = drop_sand_floor(sand_source, &filled, floor_y);
        filled.insert(sand_at_rest);
        sand_grains_at_rest += 1;
        if sand_at_rest == sand_source {
            break;
        }
    }
    sand_grains_at_rest
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
        assert_eq!(
            input.iter().cloned().sorted().collect_vec(),
            vec![
                (498, 4),
                (498, 5),
                (498, 6),
                (497, 6),
                (496, 6),
                (502, 4),
                (503, 4),
                (502, 5),
                (502, 6),
                (502, 7),
                (502, 8),
                (502, 9),
                (501, 9),
                (500, 9),
                (499, 9),
                (498, 9),
                (497, 9),
                (496, 9),
                (495, 9),
                (494, 9)
            ]
            .into_iter()
            .sorted()
            .collect_vec()
        );
        assert_eq!(solve_part1(input), 24);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 696);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 93);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 23610);
    }
}
