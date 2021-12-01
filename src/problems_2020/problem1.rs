use crate::util::multiple_sum_problem::{three_sum, two_sum};
use crate::util::parse::parse_to;

pub static INPUT_PATH: &str = "src/problems_2020/problem1/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<i32> {
    parse_to::<i32>(path_to_input)
}
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    two_sum(&input, &2020)[0].iter().product()
}

pub fn solve_part2(input: &Vec<i32>) -> i32 {
    three_sum(&input, 2020)[0].iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 960075);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 212900130);
    }
}
