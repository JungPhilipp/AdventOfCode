use core::fmt;
use std::collections::HashMap;

use itertools::Itertools;
use log::{debug, info};
use num_derive::FromPrimitive;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 22:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = (Vec<Vec<char>>, String);

fn parse(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let mut map = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let max_columns = map.iter().map(|line| line.len()).max().unwrap();
    map.iter_mut()
        .for_each(|line| line.resize(max_columns, ' '));

    let path = parts.next().unwrap().trim().to_string();

    (map, path)
}

#[derive(Debug, Clone, Copy, FromPrimitive)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_clockwise(&mut self) {
        let index = (*self as usize + 1) % std::mem::variant_count::<Direction>();
        *self = num::FromPrimitive::from_usize(index).unwrap();
    }
    fn turn_counter_clockwise(&mut self) {
        let index = {
            if *self as usize == 0 {
                std::mem::variant_count::<Direction>() - 1
            } else {
                *self as usize - 1
            }
        };
        *self = num::FromPrimitive::from_usize(index).unwrap();
    }

    fn to_step(self) -> (i64, i64) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }
}

fn wrap_index_if_needed(index: i64, len: usize) -> usize {
    match index {
        negative if index < 0 => (len as i64 + negative) as usize,
        in_range if (0..len as i64).contains(&index) => in_range as usize,
        wrap if index >= len as i64 => wrap as usize % len,
        _ => unreachable!(),
    }
}

fn next_position_flat(
    position: (usize, usize),
    dir: (i64, i64),
    x_len: usize,
    y_len: usize,
) -> (usize, usize) {
    (
        wrap_index_if_needed(position.0 as i64 + dir.0, y_len),
        wrap_index_if_needed(position.1 as i64 + dir.1, x_len),
    )
}

fn next_position_cube(
    position: (usize, usize),
    dir: (i64, i64),
    x_len: usize,
    y_len: usize,
) -> (usize, usize) {
    assert_eq!(x_len, y_len);
    assert_eq!(x_len, 50);
    let side_len = x_len / 3;

    (
        wrap_index_if_needed(position.0 as i64 + dir.0, y_len),
        wrap_index_if_needed(position.1 as i64 + dir.1, x_len),
    )
}

type NextPositionFunction = dyn Fn((usize, usize), (i64, i64), usize, usize) -> (usize, usize);

fn do_steps(
    mut position: (usize, usize),
    steps: usize,
    direction: Direction,
    map: &Vec<Vec<char>>,
    next_position: &NextPositionFunction,
) -> (usize, usize) {
    let dir = direction.to_step();
    let x_len = map[0].len();
    let y_len = map.len();
    'steps: for _ in 0..steps {
        let mut new_postion = next_position(position, dir, x_len, y_len);
        'single_step: loop {
            match map[new_postion.0][new_postion.1] {
                '#' => break 'steps,
                '.' => {
                    position = new_postion;
                    break 'single_step;
                }
                ' ' => {
                    new_postion = next_position(new_postion, dir, x_len, y_len);
                    continue 'single_step;
                }
                _ => unreachable!(),
            }
        }
    }
    position
}

fn password(input: Input, next_position: &NextPositionFunction) -> usize {
    let (mut map, instructions) = input;
    let mut position = {
        let (pos, _) = map[0]
            .iter()
            .find_position(|&&c| c == '.' || c == '#')
            .unwrap();

        (0, pos)
    };

    let mut path = instructions.chars();
    let mut direction = Direction::Right;
    loop {
        let steps = path
            .take_while_ref(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        position = do_steps(position, steps, direction, &map, next_position);

        if let Some(turn) = path.next() {
            match turn {
                'R' => {
                    direction.turn_clockwise();
                }
                'L' => {
                    direction.turn_counter_clockwise();
                }
                unexpected => unreachable!("Unexpected char ({unexpected})"),
            }
        } else {
            break;
        }
    }
    map[position.0][position.1] = 'H';
    let map_string = map
        .iter()
        .flat_map(|line| line.iter().chain(std::iter::once(&'\n')))
        .collect::<String>();
    info!("\n{map_string}");
    info!("Final Position {:?}", position);
    (position.0 + 1) * 1000 + (position.1 + 1) * 4 + direction as usize
}

fn solve_part1(input: Input) -> usize {
    password(input, &next_position_flat)
}

fn solve_part2(input: Input) -> usize {
    password(input, &next_position_cube)
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
        assert_eq!(solve_part1(input), 6032);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 88226);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 5031);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
