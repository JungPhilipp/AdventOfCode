pub static INPUT_PATH: &str = "src/problems_2020/problem3/input.txt.txt";

pub fn parse_input(path_to_input: &str) -> Vec<i32> {
    vec![]
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
