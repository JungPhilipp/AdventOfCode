#![allow(clippy::ptr_arg)]

use itertools::Itertools;
use log::debug;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem11/input.txt";

type Input = Vec<Vec<i32>>;

pub fn parse_input(path_to_input: &str) -> Input {
    read_lines(path_to_input)
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .map(|x| x as i32)
                .collect_vec()
        })
        .collect_vec()
}

fn flash(octopuses: &mut Input, pos: (i32, i32)) {
    octopuses[pos.0 as usize][pos.1 as usize] = 0;
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
    ]
    .iter()
    .for_each(|(y, x)| {
        if *y >= 0 && *x >= 0 {
            if let Some(octopus) = octopuses
                .get_mut(*y as usize)
                .and_then(|line| line.get_mut(*x as usize))
            {
                if *octopus != 0 {
                    *octopus += 1;
                }
            }
        }
    });
}

pub fn solve_part1(input: &Input) -> usize {
    let mut flashes = 0;
    let side = input[0].len();
    let mut octopuses = input.clone();
    for step in 0..100 {
        let display = octopuses
            .iter()
            .map(|line| {
                line.iter()
                    .map(|digit| digit.to_string())
                    .collect::<String>()
            })
            .join("\n");
        debug!("Step {} :\n{}", step, display);
        octopuses.iter_mut().flatten().for_each(|pos| *pos += 1);
        let mut flashed = true;
        while flashed {
            let will_flash = octopuses
                .iter()
                .flatten()
                .enumerate()
                .filter_map(|(index, energy)| {
                    if *energy > 9 {
                        let y = index / side;
                        let x = index % side;
                        Some((y as i32, x as i32))
                    } else {
                        None
                    }
                })
                .collect_vec();
            will_flash
                .iter()
                .for_each(|pos| flash(&mut octopuses, *pos));
            flashed = !will_flash.is_empty();
        }
        flashes += octopuses.iter().flatten().filter(|&&pos| pos == 0).count();
    }
    flashes
}

pub fn solve_part2(input: &Input) -> i32 {
    let side = input[0].len();
    let mut octopuses = input.clone();
    for step in 0..10000 {
        let display = octopuses
            .iter()
            .map(|line| {
                line.iter()
                    .map(|digit| digit.to_string())
                    .collect::<String>()
            })
            .join("\n");
        debug!("Step {} :\n{}", step, display);
        octopuses.iter_mut().flatten().for_each(|pos| *pos += 1);
        let mut flashed = true;
        while flashed {
            let will_flash = octopuses
                .iter()
                .flatten()
                .enumerate()
                .filter_map(|(index, energy)| {
                    if *energy > 9 {
                        let y = index / side;
                        let x = index % side;
                        Some((y as i32, x as i32))
                    } else {
                        None
                    }
                })
                .collect_vec();
            will_flash
                .iter()
                .for_each(|pos| flash(&mut octopuses, *pos));
            flashed = !will_flash.is_empty();
        }
        if 100 == octopuses.iter().flatten().filter(|&&pos| pos == 0).count() {
            return step;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&parse_input("src/problems_2021/problem11/example.txt")),
            1656
        );
    }
    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 1702);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 250);
    }
}
