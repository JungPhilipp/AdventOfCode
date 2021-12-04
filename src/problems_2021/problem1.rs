use crate::util::parse::parse_to;

pub static INPUT_PATH: &str = "src/problems_2021/problem1/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<i32> {
    parse_to::<i32>(path_to_input)
}
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    input
        .windows(2)
        .map(|pair| if pair[0] < pair[1] { 1 } else { 0 })
        .sum()
}

pub fn solve_part2(input: &Vec<i32>) -> i32 {
    input
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|pair| if pair[0] < pair[1] { 1 } else { 0 })
        .sum()
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
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1706);
    }
}
