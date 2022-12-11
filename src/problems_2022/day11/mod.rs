use itertools::Itertools;
use log::{info, debug};
use num::BigInt;
use primes::is_prime;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 11:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Debug, Clone)]
struct Item {
    number: BigInt,
}

impl Item {
    fn new(number: usize) -> Item {
        Item {number:  number.into() }
    }

    fn is_divisible_by(&self, divisor: usize) -> bool {
        self.number.clone() % divisor == 0.into()
    }

    fn mult(&mut self, number: usize) {
        self.number *= number;
    }

    fn add(&mut self, number: usize) {
        self.number += number;
    }

    fn square(&mut self) {
        self.number = self.number.pow(2);
    }

    fn div(&mut self, number: usize) {
        self.number /= number;
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    op: (Op, Operant),
    test: usize,
    true_target: usize,
    false_target: usize,
    items_handled: usize,
}

#[derive(Debug, Clone)]
enum Operant {
    Number(usize),
    Old,
}

#[derive(Debug, Clone)]
enum Op {
    Mult,
    Add,
}

impl Op {
    fn is_mult(&self) -> bool {
        match self {
            Op::Mult => true,
            Op::Add => false,
        }
    }
}

impl Monkey {
    fn inspect(&self, mut item: Item, relief: usize) -> Item {
        match self.op.1.clone() {
            Operant::Number(number) => match self.op.0 {
                Op::Mult => item.mult(number),
                Op::Add => item.add(number),
            },
            Operant::Old => {
                assert!(self.op.0.is_mult());
                item.square();
            }
        };
        item.div(relief);
        item
    }

    fn test(&self, item: &Item) -> usize {
        if item.is_divisible_by(self.test) {
            self.true_target
        } else {
            self.false_target
        }
    }

    fn give_items(&mut self) -> Vec<Item> {
        self.items_handled += self.items.len();
        let items = self.items.clone();
        self.items.clear();
        items
    }
}

type Input = Vec<Monkey>;

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .into_iter()
        .filter(|lines| !lines.trim().is_empty())
        .map(|monkey| {
            let mut lines = monkey.split('\n').skip(1);
            let items = lines
                .next()
                .expect("Starting items")
                .trim()
                .strip_prefix("Starting items: ")
                .expect("to exist")
                .split(',')
                .into_iter()
                .map(|item| Item::new(item.trim().parse::<usize>().expect("a number")))
                .collect_vec();
            let op = lines
                .next()
                .expect("Operation")
                .trim()
                .strip_prefix("Operation: new = old")
                .expect("to exist")
                .trim()
                .split(' ')
                .collect_tuple()
                .map(|(first, second)| {
                    let op = match first {
                        "*" => Op::Mult,
                        "+" => Op::Add,
                        _ => unreachable!(),
                    };
                    let operant = match second.parse::<usize>() {
                        Ok(n) => Operant::Number(n),
                        Err(_) => Operant::Old,
                    };
                    (op, operant)
                })
                .expect("");

            let test = lines
                .next()
                .expect("Test")
                .trim()
                .strip_prefix("Test: divisible by")
                .expect("to exist")
                .trim()
                .parse::<usize>()
                .map(|number| {
                    assert!(is_prime(number as u64));
                    number
                })
                .expect("a number");
            let true_target_monkey = lines
                .next()
                .expect("True branch")
                .trim()
                .strip_prefix("If true: throw to monkey")
                .expect("to exist")
                .trim()
                .parse::<usize>()
                .expect("a number");
            let false_target_monkey = lines
                .next()
                .expect("False branch")
                .trim()
                .strip_prefix("If false: throw to monkey")
                .expect("to exist")
                .trim()
                .parse::<usize>()
                .expect("a number");

            Monkey {
                items,
                op,
                test,
                true_target: true_target_monkey,
                false_target: false_target_monkey,
                items_handled: 0,
            }
        })
        .collect_vec()
}

fn solve_part1(mut input: Input) -> usize {
    for round in 0..20 {
        for (index, monkey) in input.iter().enumerate() {
        }
        for monkey_index in 0..input.len() {
            let monkey = input[monkey_index].clone();
            for item in input[monkey_index].give_items() {
                let inspected_item = monkey.inspect(item, 3);
                input[monkey.test(&inspected_item)]
                    .items
                    .push(inspected_item);
            }
        }
    }
    input
        .into_iter()
        .map(|monkey| monkey.items_handled)
        .sorted()
        .rev()
        .take(2)
        .inspect(|handled| println!("{}", handled))
        .product()
}

fn solve_part2(mut input: Input) -> usize {
    for round in 0..10000 {
        info!("Round: {} -------------------", round);
        for (index, monkey) in input.iter().enumerate() {
            debug!("Monkey {}: {:?}", index, monkey.items)
        }
        for monkey_index in 0..input.len() {
            let monkey = input[monkey_index].clone();
            for item in input[monkey_index].give_items() {
                let inspected_item = monkey.inspect(item, 1);
                input[monkey.test(&inspected_item)]
                    .items
                    .push(inspected_item);
            }
        }
    }
    input
        .into_iter()
        .map(|monkey| monkey.items_handled)
        .sorted()
        .rev()
        .take(2)
        .inspect(|handled| println!("{}", handled))
        .product()
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
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_PATH!()))), 10605);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 54253);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(
            solve_part2(parse(include_str!(EXAMPLE_PATH!()))),
            2713310158
        );
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
