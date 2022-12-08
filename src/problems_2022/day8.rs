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

type Input = Vec<Vec<i32>>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Decimal number") as i32)
                .collect_vec()
        })
        .collect_vec()
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

fn solve_part1(input: Input) -> usize {
    let x = input[0].len();
    let y = input.len();
    let flat_vec = input.into_iter().flatten().collect_vec();
    let vec = Array2::from_shape_vec((x, y), flat_vec).expect("regular matrix");

    let mut flags: Array2<i32> = Array::zeros(vec.raw_dim());
    vec.axis_iter(Axis(0))
        .zip(flags.axis_iter_mut(Axis(0)))
        .for_each(|(trees, mut visibilities)| {
            visible_trees(trees.iter(), visibilities.iter_mut());
            visible_trees(trees.iter().rev(), visibilities.iter_mut().rev());
        });

    vec.axis_iter(Axis(1))
        .zip(flags.axis_iter_mut(Axis(1)))
        .for_each(|(trees, mut visibilities)| {
            visible_trees(trees.iter(), visibilities.iter_mut());
            visible_trees(trees.iter().rev(), visibilities.iter_mut().rev());
        });
    flags.iter().filter(|&&visible| (visible > 0)).count()
}

fn solve_part2(forest: Input) -> usize {
    let mut scores = vec![];
    let i_max = forest.len();
    let j_max = forest[0].len();
    for i in 0..i_max {
        for j in 0..j_max {
            let house_height = forest[i][j];

            let mut flag = true;
            let i_back = for tree in (0..i).rev(){}
                .filter(|&tree| {
                    let tree_height = forest[tree][j];
                    if tree_height >= house_height {
                        flag = false;

                    } else {
                        false
                    }
                })
                .count();
            let mut max = -1;
            let j_back = (0..j)
                .rev()
                .filter(|&tree| {
                    let tree_height = forest[i][tree];
                    if tree_height >= max {
                        max = tree_height;
                        if max >= house_height {
                            max = 10;
                        }
                        true
                    } else {
                        false
                    }
                })
                .count();
            let mut max = -1;
            let i_forward = (i..i_max)
                .skip(1)
                .filter(|&tree| {
                    let tree_height = forest[tree][j];
                    if tree_height >= max {
                        max = tree_height;
                        if max >= house_height {
                            max = 10;
                        }
                        true
                    } else {
                        false
                    }
                })
                .count();
            let mut max = -1;
            let j_forward = (j..j_max)
                .skip(1)
                .filter(|&tree| {
                    let tree_height = forest[i][tree];
                    if tree_height >= max {
                        max = tree_height;
                        if max >= house_height {
                            max = 10;
                        }
                        true
                    } else {
                        false
                    }
                })
                .count();
            scores.push(i_back * i_forward * j_back * j_forward);
        }
    }
    let flat_forest = forest.iter().flatten().collect_vec();
    dbg!(Array2::from_shape_vec((i_max, j_max), flat_forest.clone()).expect("regular matrix"));
    dbg!(Array2::from_shape_vec((i_max, j_max), scores.clone()).expect("regular matrix"));
    scores.into_iter().max().expect("To find a spot")
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
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 1818);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!("day8/example_1.txt"))), 8);
    }

    #[test]
    fn part2() {
        //assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
