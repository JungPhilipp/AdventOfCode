use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2020/problem3/input.txt";

#[derive(PartialEq)]
pub enum Item {
    Tree,
    Snow,
    Invalid,
}
pub fn parse_input(path_to_input: &str) -> Vec<Vec<Item>> {
    read_lines(path_to_input)
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| {
                    if c == '#' {
                        Item::Tree
                    } else if c == '.' {
                        Item::Snow
                    } else {
                        Item::Invalid
                    }
                })
                .collect()
        })
        .collect()
}
fn trees_hit(input: &[Vec<Item>], dx: usize, dy: usize) -> usize {
    let mut num_trees = 0;
    let mut x = 0;
    let mut y = 0;
    let x_max = input[0].len();
    let y_max = input.len();
    while y < y_max {
        num_trees += (input[y][x] == Item::Tree) as usize;
        x = (x + dx) % x_max;
        y += dy;
    }
    num_trees
}
pub fn solve_part1(input: &[Vec<Item>]) -> usize {
    trees_hit(input, 3, 1)
}

pub fn solve_part2(input: &[Vec<Item>]) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .into_iter()
        .map(|pos| trees_hit(input, pos.0, pos.1))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 240);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 2832009600);
    }
}
