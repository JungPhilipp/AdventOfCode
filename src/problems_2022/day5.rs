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
type Moves = Vec<(usize, usize)>;
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
            .flat_map(|line| {
                let (rep, from, to) = line
                    .split(' ')
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_tuple()
                    .expect("Expected three numbers");
                std::iter::repeat((from - 1, to - 1)).take(rep)
            })
            .collect_vec()
    };

    (board, moves)
}

fn solve_part1(input: Input) -> String {
    let (mut board, moves) = input;
    for (from, to) in moves {
        let cargo = board[from].pop().expect("Stack should not be empty");
        board[to].push(cargo);
    }
    board
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>()
}

fn solve_part2(input: Input) -> usize {
    0
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
        assert_eq!(solve_part2(parse(include_str!("day5/example_1.txt"))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
