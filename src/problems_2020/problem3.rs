use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2020/problem3/input.txt";

#[derive(PartialEq)]
pub enum ITEM {
    TREE,
    SNOW,
    INVALID,
}
pub fn parse_input(path_to_input: &str) -> Vec<Vec<ITEM>> {
    read_lines(path_to_input)
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| {
                    if c == '#' {
                        ITEM::TREE
                    } else if c == '.' {
                        ITEM::SNOW
                    } else {
                        ITEM::INVALID
                    }
                })
                .collect()
        })
        .collect()
}
pub fn solve_part1(input: &Vec<Vec<ITEM>>) -> i32 {
    let mut num_trees = 0;
    let mut x = 0;
    let mut y = 0;
    let dx = 3;
    let dy = 1;
    let x_max = input[0].len();
    let y_max = input.len();
    while y < y_max {
        num_trees += (input[y][x] == ITEM::TREE) as i32;
        x = (x + dx) % x_max;
        y += dy;
    }
    num_trees
}

pub fn solve_part2(input: &Vec<Vec<ITEM>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 0);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 0);
    }
}
