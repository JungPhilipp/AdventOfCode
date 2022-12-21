use std::collections::HashMap;

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 21:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Debug, Clone)]
enum Instruction {
    Number(i64),
    Op((String, char, String)),
}

type Input = HashMap<String, Instruction>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split(' ').map(|part| part.trim()).collect_vec();
            let name = parts[0].strip_suffix(':').unwrap().to_string();
            let instruction = match parts.len() {
                2 => Instruction::Number(parts[1].parse::<i64>().unwrap()),
                4 => Instruction::Op((
                    parts[1].to_string(),
                    parts[2].chars().exactly_one().unwrap(),
                    parts[3].to_string(),
                )),
                _ => unreachable!(),
            };
            (name, instruction)
        })
        .collect()
}

fn compute(name: &str, monkeys: &Input, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(number) = cache.get(name) {
        return *number;
    }
    let result = match monkeys.get(name).unwrap() {
        Instruction::Number(number) => *number,
        Instruction::Op((lhs, op, rhs)) => {
            let first = compute(lhs.as_str(), monkeys, cache);
            let second = compute(rhs, monkeys, cache);
            match op {
                '+' => first + second,
                '-' => first - second,
                '*' => first * second,
                '/' => first / second,
                _ => unreachable!(),
            }
        }
    };

    cache.insert(name.to_string(), result);
    result
}

fn solve_part1(input: Input) -> i64 {
    compute("root", &input, &mut HashMap::new())
}

fn solve_part2(mut input: Input) -> i64 {
    let root = input.get_mut("root").unwrap();
    let root_instruction = match root {
        Instruction::Op(op) => op.clone(),
        _ => unreachable!(),
    };
    *root = Instruction::Op((root_instruction.0, '-', root_instruction.2));
    for number in 0..i64::MAX {
        if number % 10_000 == 0 {
            info!("{number}");
        }
        *input.get_mut("humn").unwrap() = Instruction::Number(number);
        if compute("root", &input, &mut HashMap::new()) == 0 {
            return number;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    macro_rules! EXAMPLE_PATH {
        () => {
            "example_1.txt"
        };
    }
    #[test]
    fn example_1() {
        let input = parse(include_str!(EXAMPLE_PATH!()));
        assert_eq!(solve_part1(input), 152);
    }

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(parse(include_str!(INPUT_PATH!()))),
            194501589693264
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 301);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
