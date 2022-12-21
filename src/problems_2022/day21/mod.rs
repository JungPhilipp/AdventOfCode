use core::fmt;
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
    Op(([char; 4], Op, [char; 4])),
}

#[derive(Debug, Clone)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
}

impl Op {
    fn new(c: char) -> Op {
        match c {
            '+' => Op::Plus,
            '-' => Op::Minus,
            '*' => Op::Mult,
            '/' => Op::Div,
            _ => unreachable!(),
        }
    }
    fn compute(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Plus => lhs + rhs,
            Op::Minus => lhs - rhs,
            Op::Mult => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Plus => '+',
                Op::Minus => '-',
                Op::Mult => '*',
                Op::Div => '/',
            }
        )
    }
}

type Input = HashMap<[char; 4], Instruction>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split(' ').map(|part| part.trim()).collect_vec();
            let name = parts[0]
                .strip_suffix(':')
                .unwrap()
                .chars()
                .collect_vec()
                .try_into()
                .unwrap();
            let instruction = match parts.len() {
                2 => Instruction::Number(parts[1].parse::<i64>().unwrap()),
                4 => Instruction::Op((
                    parts[1].chars().collect_vec().try_into().unwrap(),
                    Op::new(parts[2].chars().exactly_one().unwrap()),
                    parts[3].chars().collect_vec().try_into().unwrap(),
                )),
                _ => unreachable!(),
            };
            (name, instruction)
        })
        .collect()
}

fn compute(
    name: &[char; 4],
    monkeys: &Input,
    cache: &mut HashMap<[char; 4], i64>,
) -> Result<i64, &'static str> {
    if let Some(number) = cache.get(name) {
        return Ok(*number);
    }
    let result = match monkeys.get(name).ok_or("Monkey not found")? {
        Instruction::Number(number) => *number,
        Instruction::Op((lhs, op, rhs)) => {
            let first = compute(lhs, monkeys, cache)?;
            let second = compute(rhs, monkeys, cache)?;
            op.compute(first, second)
        }
    };

    cache.insert(*name, result);
    Ok(result)
}

fn solve_part1(input: Input) -> i64 {
    compute(&['r', 'o', 'o', 't'], &input, &mut HashMap::new()).unwrap()
}

fn precompute_cache(mut monkeys: Input) -> HashMap<[char; 4], i64> {
    let mut cache = HashMap::new();

    for (name, _) in monkeys.iter() {
        let _ = compute(name, &monkeys, &mut cache);
    }
    info!("{}/{} monkeys cached", cache.len(), monkeys.len());
    cache
}

fn build_equation(name: [char; 4], monkeys: &Input, cache: &HashMap<[char; 4], i64>) -> String {
    if let Some(number) = cache.get(&name) {
        return number.to_string();
    }
    if let Some(monkey) = monkeys.get(&name) {
        match monkey {
            Instruction::Number(number) => number.to_string(),
            Instruction::Op((lhs, op, rhs)) => format!(
                "({} {} {})",
                build_equation(*lhs, monkeys, cache),
                op,
                build_equation(*rhs, monkeys, cache)
            ),
        }
    } else {
        name.into_iter().collect()
    }
}

fn solve_part2(mut input: Input) -> i64 {
    let root_key = ['r', 'o', 'o', 't'];
    let root = input.get_mut(&root_key).unwrap();
    let root_instruction = match root {
        Instruction::Op(op) => op.clone(),
        _ => unreachable!(),
    };
    *root = Instruction::Op((root_instruction.0, Op::Minus, root_instruction.2));
    input.remove(&['h', 'u', 'm', 'n']);

    let cache = precompute_cache(input.clone());

    info!("Equation: {}", build_equation(root_key, &input, &cache));

    for number in 0..i64::MAX {
        if number % 100_000_000 == 0 {
            info!("{number}");
        }
        *input
            .entry(['h', 'u', 'm', 'n'])
            .or_insert(Instruction::Number(0)) = Instruction::Number(number);
        if compute(&['r', 'o', 'o', 't'], &input, &mut cache.clone()).unwrap() == 0 {
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
        //assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 3887609741189);
    }
}
