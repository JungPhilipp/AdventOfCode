use crate::util::multiple_sum_problem::two_sum;
use crate::util::parse::parse_to;
use itertools::Itertools;

pub static INPUT_PATH: &str = "src/problems_2020/problem9/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<i64> {
    parse_to::<i64>(path_to_input)
}

pub fn sliding_sum(input: &[i64], window_size: usize) -> i64 {
    for window in input.windows(window_size + 1) {
        let pairs = two_sum(&window[0..window_size], window.last().unwrap());
        if pairs.is_empty() {
            return *window.last().unwrap();
        }
    }
    panic!("No found");
}
pub fn solve_part1(input: &[i64]) -> i64 {
    sliding_sum(input, 25)
}

pub fn solve_part2(input: &[i64]) -> i64 {
    let sum = 85848519;
    for i in 0..input.len() {
        let mut current_sum = 0;
        for j in i..input.len() {
            current_sum += input[j];
            if current_sum == sum && i != j {
                let set = &input[i..=j];
                let minmax = set.iter().minmax().into_option().unwrap();
                return minmax.0 + minmax.1;
            } else if current_sum > sum {
                break;
            }
        }
    }
    panic!("Not found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 85848519);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 13414198);
    }
}
