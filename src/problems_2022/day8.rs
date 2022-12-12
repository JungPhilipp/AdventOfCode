use itertools::Itertools;
use log::info;
use ndarray::{Array, Array2, Axis};

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

fn count_visible_trees(house_height: i32, trees: &[i32]) -> usize {
    trees
        .iter()
        .position(|tree| tree >= &house_height)
        .map_or(trees.len(), |pos| pos + 1)
}

fn solve_part2(forest: Input) -> usize {
    let mut scores = vec![];
    let i_max = forest.len();
    let j_max = forest[0].len();
    for i in 0..i_max {
        for j in 0..j_max {
            let house_height = forest[i][j];
            let i_back = (0..i).rev().map(|index| forest[index][j]).collect_vec();
            let j_back = (0..j).rev().map(|index| forest[i][index]).collect_vec();
            let i_forward = (i..i_max)
                .skip(1)
                .map(|index| forest[index][j])
                .collect_vec();
            let j_forward = (j..j_max)
                .skip(1)
                .map(|index| forest[i][index])
                .collect_vec();

            scores.push(
                count_visible_trees(house_height, &i_back)
                    * count_visible_trees(house_height, &j_back)
                    * count_visible_trees(house_height, &i_forward)
                    * count_visible_trees(house_height, &j_forward),
            );
        }
    }
    scores.into_iter().max().expect("To find a spot")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn visible_trees_1() {
        assert_eq!(count_visible_trees(5, &[3]), 1);
        assert_eq!(count_visible_trees(5, &[5, 2]), 1);
        assert_eq!(count_visible_trees(5, &[1, 2]), 2);
        assert_eq!(count_visible_trees(5, &[3, 5, 3]), 2);

        assert_eq!(count_visible_trees(5, &[3, 3]), 2);
        assert_eq!(count_visible_trees(5, &[4, 9]), 2);

        assert_eq!(count_visible_trees(5, &[1, 4, 3, 5, 1, 1, 1, 5, 2, 9]), 4);
    }

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
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
