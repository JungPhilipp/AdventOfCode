use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2020/problem11/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<Vec<char>> {
    read_lines(path_to_input)
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn in_range(i: i32, j: i32, input: &Vec<Vec<char>>) -> bool {
    j >= 0 && j < input.len() as i32 && i >= 0 && i < input[j as usize].len() as i32
}
fn num_adjacent_seats(input: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut seat_count = 0;
    if in_range(i - 1, j - 1, input) {
        seat_count += (input[(i - 1) as usize][(j - 1) as usize] == '#') as i32;
    }
    if in_range(i, j - 1, input) {
        seat_count += (input[i as usize][(j - 1) as usize] == '#') as i32;
    }
    if in_range(i + 1, j - 1, input) {
        seat_count += (input[(i + 1) as usize][(j - 1) as usize] == '#') as i32;
    }
    if in_range(i - 1, j, input) {
        seat_count += (input[(i - 1) as usize][j as usize] == '#') as i32;
    }
    if in_range(i + 1, j, input) {
        seat_count += (input[(i + 1) as usize][j as usize] == '#') as i32;
    }
    if in_range(i - 1, j + 1, input) {
        seat_count += (input[(i - 1) as usize][(j + 1) as usize] == '#') as i32;
    }
    if in_range(i, j + 1, input) {
        seat_count += (input[i as usize][(j + 1) as usize] == '#') as i32;
    }
    if in_range(i + 1, j + 1, input) {
        seat_count += (input[(i + 1) as usize][(j + 1) as usize] == '#') as i32;
    }
    seat_count
}

pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    let mut changed = true;
    let mut current = input.clone();
    while changed {
        changed = false;
        let mut next = current.clone();
        for j in 0..input.len() {
            for i in 0..input[j as usize].len() {
                let seat_count = num_adjacent_seats(&current, i as i32, j as i32);
                //println!("{}, {}, {}", i, j, seat_count);
                if current[j as usize][i as usize] == 'L' && seat_count == 0 {
                    changed = true;
                    next[j as usize][i as usize] = '#';
                } else if current[j as usize][i as usize] == '#' && seat_count >= 4 {
                    next[j as usize][i as usize] = 'L';
                    changed = true;
                }
            }
        }
        current = next;
    }
    current
        .into_iter()
        .map(|line| line.into_iter().filter(|c| *c == '#').count())
        .sum()
}

pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&parse_input("src/problems_2020/problem11/input_test0.txt")),
            38
        );
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 0);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 0);
    }
}
