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
        "Solutions Day 2:\nPart1{}\nPart2{}",
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

impl Symbol {
    fn from(c: char) -> Symbol {
        match c {
            'A' | 'X' => Symbol::Rock,
            'B' | 'Y' => Symbol::Paper,
            'C' | 'Z' => Symbol::Scissors,
            invalid => panic!("{} not a valid symbol", invalid),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Game {
    Lost = 0,
    Tie = 3,
    Won = 6,
}

impl Game {
    fn from(c: char) -> Game {
        match c {
            'X' => Game::Lost,
            'Y' => Game::Tie,
            'Z' => Game::Won,
            invalid => panic!("{} not a valid game", invalid),
        }
    }
}

type Input = Vec<(char, char)>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                line.split(' ')
                    .map(|sign| sign.trim().chars().next().unwrap())
                    .collect_tuple()
            }
        })
        .collect_vec()
}

fn determine_points(elf: Symbol, me: Symbol) -> usize {
    use Game::*;
    use Symbol::*;
    let outcome = match (elf, me) {
        (Rock, Rock) => Tie,
        (Rock, Paper) => Won,
        (Rock, Scissors) => Lost,
        (Paper, Rock) => Lost,
        (Paper, Paper) => Tie,
        (Paper, Scissors) => Won,
        (Scissors, Rock) => Won,
        (Scissors, Paper) => Lost,
        (Scissors, Scissors) => Tie,
    };
    outcome as usize + me as usize
}

fn solve_part1(input: Input) -> usize {
    input
        .into_iter()
        .map(|(elf, me)| (Symbol::from(elf), Symbol::from(me)))
        .map(|(elf, me)| determine_points(elf, me))
        .sum()
}

fn choose_symbol(elf_symbol: Symbol, outcome: Game) -> Symbol {
    use Game::*;
    use Symbol::*;
    match (elf_symbol, outcome) {
        (Rock, Tie) => Rock,
        (Rock, Won) => Paper,
        (Rock, Lost) => Scissors,
        (Paper, Lost) => Rock,
        (Paper, Tie) => Paper,
        (Paper, Won) => Scissors,
        (Scissors, Won) => Rock,
        (Scissors, Lost) => Paper,
        (Scissors, Tie) => Scissors,
    }
}

fn solve_part2(input: Input) -> usize {
    input
        .into_iter()
        .map(|(elf, outcome)| {
            let elf_symbol = Symbol::from(elf);
            (elf_symbol, choose_symbol(elf_symbol, Game::from(outcome)))
        })
        .map(|(elf, me)| determine_points(elf, me))
        .sum()
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
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 13682);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!("day2/example_1.txt"))), 12);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 12881);
    }
}
