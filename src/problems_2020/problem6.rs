use itertools::Itertools;
use std::fs::read_to_string;

pub static INPUT_PATH: &str = "src/problems_2020/problem6/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<Vec<String>> {
    read_to_string(path_to_input)
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.replace("\n", " ")
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn solve_part1(input: &Vec<Vec<String>>) -> usize {
    input
        .into_iter()
        .map(|group| group.join("").chars().unique().count())
        .sum()
}

pub fn solve_part2(input: &Vec<Vec<String>>) -> usize {
    let chars: Vec<char> = ('a'..='z').collect();
    input
        .into_iter()
        .map(|group| {
            chars
                .iter()
                .map(|c| group.into_iter().all(|s| s.contains(*c)) as usize)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 6911);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 3473);
    }
}
