use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{self, Hash, Hasher},
};

use itertools::Itertools;
use log::debug;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem14/input.txt";

type Input = (Vec<char>, HashMap<Pair, char>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pair(char, char);

pub fn parse_input(path_to_input: &str) -> Input {
    let lines = read_lines(path_to_input);
    let template = lines[0].chars().collect_vec();
    let instructions = lines
        .iter()
        .skip(1)
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                let parts = line
                    .trim()
                    .split("->")
                    .map(|part| part.trim())
                    .collect_vec();
                let pair = Pair(
                    parts[0].chars().next().unwrap(),
                    parts[0].chars().next_back().unwrap(),
                );
                Some((pair, parts[1].chars().next().unwrap()))
            }
        })
        .collect();

    (template, instructions)
}

fn iterate_polymer(input: &Input, steps: usize) -> Vec<char> {
    let (mut polymer, instructions) = input.clone();
    for _ in 0..steps {
        let mut next = polymer.clone();
        let mut index = 1;
        for window in polymer.windows(2) {
            let pair = Pair(window[0], window[1]);
            if let Some(mapped) = instructions.get(&pair) {
                next.insert(index, *mapped);
                index += 1;
            }
            index += 1;
        }
        polymer = next;
    }
    polymer
}

pub fn solve_part1(input: &Input) -> i32 {
    let polymer = iterate_polymer(input, 10);
    let mut counts = HashMap::new();
    // TODO find a more idomatic way
    for c in polymer {
        *counts.entry(c).or_insert(0) += 1;
    }
    let values = counts.values().cloned().collect_vec();
    values.iter().max().unwrap() - values.iter().min().unwrap()
}

pub fn solve_part2(input: &Input) -> usize {
    let (template, instructions) = input;
    let mut counts = HashMap::new();
    for c in template {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut pairs = HashMap::new();

    for pair in template.windows(2) {
        *pairs.entry(Pair(pair[0], pair[1])).or_insert(0) += 1
    }
    for _ in 0..40 {
        let mut next = pairs.clone();
        for (key, count) in pairs {
            if let Some(mapped) = instructions.get(&key) {
                let first = Pair(key.0, *mapped);
                let second = Pair(*mapped, key.1);
                *next.get_mut(&key).unwrap() -= count;
                *next.entry(first).or_insert(0) += count;
                *next.entry(second).or_insert(0) += count;
                *counts.entry(mapped).or_insert(0) += count;
            }
        }
        pairs = next;
    }
    let values = counts.values().cloned().collect_vec();
    values.iter().max().unwrap() - values.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        let input = parse_input("src/problems_2021/problem14/example.txt");
        assert_eq!(solve_part1(&input), 1588);
    }

    #[test]
    fn example2() {
        let input = parse_input("src/problems_2021/problem14/example.txt");
        assert_eq!(solve_part2(&input), 2188189693529);
    }
    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 3342);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 3776553567525);
    }
}
