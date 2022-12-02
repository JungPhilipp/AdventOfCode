use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day2/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 1:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Clone, Copy, Debug)]
enum Symbol {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, Debug)]
enum Game {
    Lost = 0,
    Tie = 3,
    Won = 6,
}

type Input = Vec<(Symbol, Symbol)>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                line.split(' ')
                    .map(|sign| match sign.trim().chars().next().unwrap() {
                        'A' | 'X' => Symbol::Rock,
                        'B' | 'Y' => Symbol::Paper,
                        'C' | 'Z' => Symbol::Scissors,
                        invalid => panic!("{} not a valid symbol", invalid),
                    })
                    .collect_tuple()
            }
        })
        .collect_vec()
}

fn determine_points(elf: Symbol, me: Symbol) -> usize {
    let outcome = match (elf, me) {
        (Symbol::Rock, Symbol::Rock) => Game::Tie,
        (Symbol::Rock, Symbol::Paper) => Game::Won,
        (Symbol::Rock, Symbol::Scissors) => Game::Lost,
        (Symbol::Paper, Symbol::Rock) => Game::Lost,
        (Symbol::Paper, Symbol::Paper) => Game::Tie,
        (Symbol::Paper, Symbol::Scissors) => Game::Won,
        (Symbol::Scissors, Symbol::Rock) => Game::Won,
        (Symbol::Scissors, Symbol::Paper) => Game::Lost,
        (Symbol::Scissors, Symbol::Scissors) => Game::Tie,
    };
    outcome as usize + me as usize
}

fn solve_part1(input: Input) -> usize {
    input
        .into_iter()
        .map(|(elf, me)| determine_points(elf, me))
        .sum()
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
        assert_eq!(solve_part1(parse(include_str!("day2/example_1.txt"))), 15);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
