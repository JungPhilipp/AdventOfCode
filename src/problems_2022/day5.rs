use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day5/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 5:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}
type Board = Vec<Vec<char>>;
type Move = (usize, usize, usize);
type Moves = Vec<Move>;
type Input = (Board, Moves);

fn parse(input: &str) -> Input {
    let (board_input, moves_input) = input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect_tuple()
        .expect("Expected two parts");

    let board = {
        let mut stacks = Board::new();
        stacks.resize(9, vec![]);
        board_input
            .split('\n')
            .filter(|line| !line.is_empty() && !line.starts_with(" 1"))
            .rev()
            .for_each(|line| {
                (0..9).for_each(|stack| {
                    let pos = stack * 4 + 1;
                    if let Some(c) = line.chars().nth(pos) {
                        if c.is_ascii_uppercase() {
                            stacks[stack].push(c)
                        };
                    }
                });
            });
        stacks
    };

    let moves = {
        moves_input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(' ')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_tuple::<Move>()
                    .expect("Expected three numbers")
            })
            .map(|(rep, from, to)| (rep, from - 1, to - 1))
            .collect_vec()
    };

    (board, moves)
}

fn solve_part1(input: Input) -> String {
    let (mut board, moves) = input;
    for (rep, from, to) in moves {
        let new_len = board[from].len() - rep;
        let cargo = board[from].split_off(new_len);
        board[to].extend(cargo.into_iter().rev());
    }
    board
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>()
}

fn solve_part2(input: Input) -> String {
    let (mut board, moves) = input;
    for (rep, from, to) in moves {
        let new_len = board[from].len() - rep;
        let cargo = board[from].split_off(new_len);
        board[to].extend(cargo);
    }
    board
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_part1(parse(include_str!("day5/example_1.txt"))),
            "CMZ"
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), "RNZLFZSJH");
    }

    #[test]
    fn example_1_2() {
        assert_eq!(
            solve_part2(parse(include_str!("day5/example_1.txt"))),
            "MCD"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), "");
    }
}
