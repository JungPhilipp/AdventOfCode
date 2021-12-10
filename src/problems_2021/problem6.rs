use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem6/input.txt";

type Input = Vec<i32>;

pub fn parse_input(path_to_input: &str) -> Input {
    read_lines(path_to_input)[0]
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect_vec()
}

pub fn solve_part1(input: &Input) -> i32 {
    let mut input_copy = input.clone();
    let days = 80;
    for _ in 0..days {
        for index in 0..input_copy.len() {
            let fish = &mut input_copy[index];
            *fish -= 1;
            if *fish == -1 {
                *fish = 6;
                input_copy.push(8);
            }
        }
    }

    input_copy.len() as i32
}

pub fn solve_part2(input: &Input) -> u64 {
    let mut population = [0_u64; 9];
    for fish in input {
        population[*fish as usize] += 1;
    }
    for _ in 0..256 {
        population.rotate_left(1);
        population[6] += population[8];
    }
    population.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 383160);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1721148811504);
    }
}
