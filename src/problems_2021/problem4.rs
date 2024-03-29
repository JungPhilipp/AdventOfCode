use itertools::Itertools;
use log::info;
use ndarray::Array2;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem4/input.txt";

type Board = Array2<(bool, i32)>;
type Input = (Vec<i32>, Vec<Board>);

pub fn parse_input(path_to_input: &str) -> Input {
    let lines = read_lines(path_to_input);
    let numbers = lines[0]
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect_vec();
    let boards = lines
        .iter()
        .skip(2)
        .flat_map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
        })
        .chunks(25)
        .into_iter()
        .map(|chunk| {
            Array2::from_shape_vec(
                (5, 5),
                chunk
                    .into_iter()
                    .map(|number| (false, number))
                    .collect_vec(),
            )
            .unwrap()
        })
        .collect_vec();
    (numbers, boards)
}
fn bingo(board: &Board) -> bool {
    for row in board.rows() {
        if row.iter().all(|(hit, _)| *hit) {
            return true;
        }
    }
    for col in board.columns() {
        if col.iter().all(|(hit, _)| *hit) {
            return true;
        }
    }
    if [[0, 4], [1, 3], [2, 2], [3, 1], [0, 4]]
        .iter()
        .all(|index| board[*index].0)
    {
        return true;
    }

    false
}

fn set_number(board: &mut Board, new_number: i32) -> bool {
    board.into_iter().for_each(|(hit, number)| {
        if new_number == *number {
            *hit = true;
        }
    });
    bingo(board)
}

fn sum_no_hit(board: &Board) -> i32 {
    board
        .into_iter()
        .filter_map(|(hit, number)| if !*hit { Some(number) } else { None })
        .sum()
}

pub fn solve_part1(input: &Input) -> i32 {
    let (numbers, input_boards) = input;
    let mut boards = input_boards.clone();

    for number in numbers {
        for board in &mut boards {
            if set_number(board, *number) {
                return number * sum_no_hit(board);
            }
        }
    }
    0
}

pub fn solve_part2(input: &Input) -> i32 {
    let (numbers, input_boards) = input;
    let mut boards = input_boards.clone();

    let mut last_board = boards[0].clone();
    let mut last_number = 0;
    for number in numbers {
        boards.retain_mut(|board| {
            if set_number(board, *number) {
                last_board = board.clone();
                last_number = *number;
                false
            } else {
                true
            }
        });
    }
    info!("{:?} : {:?}", last_number, last_board);
    last_number * sum_no_hit(&last_board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 41668);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 10478);
    }
}
