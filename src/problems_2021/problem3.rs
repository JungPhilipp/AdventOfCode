use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem3/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<Vec<bool>> {
    read_lines(path_to_input)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() > 0)
                .collect_vec()
        })
        .collect()
}
fn vec_to_number(binary_number: &[bool]) -> i32 {
    binary_number
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, digit)| 2_i32.pow(pos as u32) * *digit as i32)
        .sum()
}
pub fn solve_part1(input: &[Vec<bool>]) -> i32 {
    let num_bits = input[0].len();
    let mut counts = vec![0; num_bits];
    input.iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(index, digit)| counts[index] += if *digit { 1 } else { -1 });
    });

    let most_common: Vec<bool> = counts
        .iter()
        .map(|&digit| {
            if digit == 0 {
                panic!("0 and 1 count is same");
            }
            digit > 0
        })
        .collect();
    let least_common = most_common.iter().map(|digit| !(*digit)).collect_vec();

    vec_to_number(&most_common) * vec_to_number(&least_common)
}
fn most_common_bit(vector: &[Vec<bool>], pos: usize) -> bool {
    vector
        .iter()
        .map(|number| if number[pos] { 1 } else { -1 })
        .sum::<i32>()
        < 0
}

pub fn solve_part2(input: &[Vec<bool>]) -> i32 {
    let num_bits = input[0].len();
    let mut tmp = input.to_vec();
    for pos in 0..num_bits {
        let most_common = most_common_bit(&tmp, pos);
        tmp = tmp
            .iter()
            .filter_map(|number| {
                if number[pos] == most_common {
                    Some(number.clone())
                } else {
                    None
                }
            })
            .collect();
        if tmp.len() == 1 {
            break;
        }
    }
    assert!(tmp.len() == 1);
    let oxygen_rate = vec_to_number(&tmp[0]);

    tmp = input.to_vec();
    for pos in 0..num_bits {
        let least_common = !most_common_bit(&tmp, pos);
        tmp = tmp
            .iter()
            .filter_map(|number| {
                if number[pos] == least_common {
                    Some(number.clone())
                } else {
                    None
                }
            })
            .collect();
        if tmp.len() == 1 {
            break;
        }
    }
    assert!(tmp.len() == 1);
    let scrubber_rate = vec_to_number(&tmp[0]);
    oxygen_rate * scrubber_rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 841526);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 4790390);
    }
}
