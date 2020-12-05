use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2020/problem5/input.txt";

fn parse_boarding_pass(pass: &str, low: char, high: char) -> Vec<bool> {
    pass.chars()
        .into_iter()
        .map(|c| {
            if c == low {
                false
            } else if c == high {
                true
            } else {
                panic!()
            }
        })
        .collect::<Vec<bool>>()
}
pub fn parse_input(path_to_input: &str) -> Vec<(Vec<bool>, Vec<bool>)> {
    read_lines(path_to_input)
        .into_iter()
        .map(|line| {
            let first = parse_boarding_pass(&line[..7], 'F', 'B');
            let second = parse_boarding_pass(&line[7..], 'L', 'R');
            (first, second)
        })
        .collect()
}
fn get_rows_cols(input: &Vec<(Vec<bool>, Vec<bool>)>) -> Vec<(i32, i32, i32)> {
    input
        .into_iter()
        .map(|(first, second)| {
            let row = binary_space_partition(first, 0, 127);
            let col = binary_space_partition(second, 0, 7);
            (row, col, row * 8 + col)
        })
        .collect()
}

pub fn solve_part1(input: &Vec<(Vec<bool>, Vec<bool>)>) -> i32 {
    get_rows_cols(input)
        .into_iter()
        .map(|(_, _, id)| id)
        .max()
        .unwrap()
}

pub fn solve_part2(input: &Vec<(Vec<bool>, Vec<bool>)>) -> i32 {
    let mut ids: Vec<i32> = get_rows_cols(input)
        .into_iter()
        .map(|(_, _, id)| id)
        .collect();
    ids.sort();

    for i in *ids.first().unwrap()..*ids.last().unwrap() {
        if ids.contains(&(i + 1)) && ids.contains(&(i - 1)) && !ids.contains(&i) {
            return i;
        }
    }
    panic!();
}

fn binary_space_partition(flags: &Vec<bool>, mut min: i32, mut max: i32) -> i32 {
    for flag in flags {
        let middle = min + (max - min) / 2;
        if *flag {
            min = middle + 1;
        } else {
            max = middle;
        }
    }
    assert_eq!(min, max);
    min
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn binary_partition() {
        assert_eq!(
            binary_space_partition(&vec![false, true, false, true, true, false, false], 0, 127),
            44
        );
        assert_eq!(binary_space_partition(&vec![true, false, true], 0, 7), 5);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 904);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 669);
    }
}
