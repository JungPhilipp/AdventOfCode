use crate::util::parse::parse_to;

pub static INPUT_PATH: &str = "src/problems_2021/problem4/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<i32> {
    parse_to::<i32>(path_to_input)
}
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    0
}

pub fn solve_part2(input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 1676);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 212900130);
    }
}
