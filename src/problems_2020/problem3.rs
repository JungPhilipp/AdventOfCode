use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2020/problem3/input.txt.txt";

enum ITEM {
    TREE,
    SNOW,
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
                    }
                })
                .collect()
        })
        .collect()
}
pub fn solve_part1(input: &Vec<Vec<ITEM>>) -> i32 {
    0
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
