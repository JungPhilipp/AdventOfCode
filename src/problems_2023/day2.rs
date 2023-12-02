use itertools::Itertools;
use lazy_static::lazy_static;
use log::info;
use regex::Regex;

macro_rules! INPUT_PATH {
    () => {
        "day2/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 2:\nPart1 {}\nPart2 {}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<(usize, (Vec<usize>, Vec<usize>, Vec<usize>))>;

fn parse(input: &str) -> Input {
    lazy_static! {
        static ref GAME: Regex = Regex::new(r"^Game (?P<game>\d+).*").unwrap();
    }
    lazy_static! {
        static ref MARBLES: Regex =
            Regex::new(r"(?P<count>\d+)\s(?P<color>red|green|blue)").unwrap();
    }

    input
        .lines()
        .filter_map(|line| {
            GAME.captures(line).map(|game| {
                let id = game["game"].to_string().parse::<usize>().unwrap();
                let marbles = MARBLES.captures_iter(line).fold(
                    (vec![], vec![], vec![]),
                    |mut acc, capture| {
                        let count = capture["count"].to_string().parse::<usize>().unwrap();
                        match &capture["color"] {
                            "red" => acc.0.push(count),
                            "green" => acc.1.push(count),
                            "blue" => acc.2.push(count),
                            _ => {}
                        };
                        acc
                    },
                );
                (id, marbles)
            })
        })
        .collect_vec()
}

fn solve_part1(input: Input) -> usize {
    input
        .iter()
        .filter_map(|(id, (r, g, b))| {
            if r.iter().max() <= Some(&12)
                && g.iter().max() <= Some(&13)
                && b.iter().max() <= Some(&14)
            {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(input: Input) -> usize {
    input
        .iter()
        .map(|(_, (r, g, b))| {
            (
                r.iter().max().unwrap_or(&0),
                g.iter().max().unwrap_or(&0),
                b.iter().max().unwrap_or(&0),
            )
        })
        .map(|(r, g, b)| r * g * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_example_1() {
        assert_eq!(8, solve_part1(parse(include_str!("day2/example_1.txt"))));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(2286, solve_part2(parse(include_str!("day2/example_1.txt"))));
    }
}
