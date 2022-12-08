use std::collections::HashSet;

use itertools::Itertools;
use log::info;
use ndarray::{iter::IterMut, Array, Array2, Axis};

macro_rules! INPUT_PATH {
    () => {
        "day8/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 8:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Array2<i32>;

fn parse(input: &str) -> Input {
    let vec = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Decimal number") as i32)
                .collect_vec()
        })
        .collect_vec();
    let x = vec[0].len();
    let y = vec.len();
    let flat_vec = vec.into_iter().flatten().collect_vec();
    Array2::from_shape_vec((x, y), flat_vec).expect("regular matrix")
}

fn visible_trees<'a>(
    trees: impl Iterator<Item = &'a i32>,
    visibilities: impl Iterator<Item = &'a mut i32>,
) {
    let mut max = -1;
    trees.zip(visibilities).for_each(|(&tree, visible)| {
        if tree > max {
            max = tree;
            *visible += 1;
        }
    })
}

fn solve_part1(input: Input) -> i32 {
    let mut flags: Input = Array::zeros(input.raw_dim());
    input
        .axis_iter(Axis(0))
        .zip(flags.axis_iter_mut(Axis(0)))
        .for_each(|(trees, mut visibilities)| {
            visible_trees(trees.iter(), visibilities.iter_mut());
            visible_trees(trees.iter().rev(), visibilities.iter_mut().rev());
        });

    input
        .axis_iter(Axis(1))
        .zip(flags.axis_iter_mut(Axis(1)))
        .for_each(|(trees, mut visibilities)| {
            visible_trees(trees.iter(), visibilities.iter_mut());
            visible_trees(trees.iter().rev(), visibilities.iter_mut().rev());
        });
    flags.iter().map(|visible| (*visible > 0) as i32).sum()
}

fn solve_part2(input: Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse(include_str!("day8/example_1.txt"))), 21);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))),1818 );
    }

    #[test]
    fn part2() {
        //assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
