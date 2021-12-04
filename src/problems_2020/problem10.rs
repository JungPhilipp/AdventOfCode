use crate::util::parse::parse_to;
use itertools::Itertools;
use std::collections::HashMap;

pub static INPUT_PATH: &str = "src/problems_2020/problem10/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<i32> {
    let mut parsed = parse_to::<i32>(path_to_input);
    parsed.push(0);
    parsed.push(parsed.iter().max().unwrap() + 3);
    parsed.into_iter().sorted().collect()
}

pub fn solve_part1(input: &Vec<i32>) -> usize {
    let sorted = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    let ones = sorted.iter().filter(|x| *x == &1).count();
    let threes = sorted.iter().filter(|x| *x == &3).count();
    ones * threes
}

fn backtrack(
    input: &Vec<i32>,
    start_index: usize,
    end_index: usize,
    lookup: &mut HashMap<usize, usize>,
) -> usize {
    let current = input[start_index];
    let mut solution_count = 0;
    if start_index == end_index {
        return 1;
    }
    if let Some(v) = lookup.get(&start_index) {
        return *v;
    }
    for next in start_index + 1..=start_index + 3 {
        if next <= end_index && input[next] <= current + 3 {
            solution_count += backtrack(input, next, end_index, lookup);
        }
    }
    lookup.insert(start_index, solution_count);
    solution_count
}
pub fn solve_part2(input: &Vec<i32>) -> usize {
    backtrack(
        input,
        0,
        input.len() - 1,
        &mut HashMap::<usize, usize>::new(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&parse_input("src/problems_2020/problem10/input_test0.txt")),
            35
        );
        assert_eq!(
            solve_part1(&parse_input("src/problems_2020/problem10/input_test1.txt")),
            220
        );
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 2048);
    }
    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&parse_input("src/problems_2020/problem10/input_test0.txt")),
            8
        );
        assert_eq!(
            solve_part2(&parse_input("src/problems_2020/problem10/input_test1.txt")),
            19208
        );
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1322306994176);
    }
}
