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
        "Solutions Day 11:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: String,
    test: usize,
    true_target: usize,
    false_target: usize,
    items_handled: usize,
}

impl Monkey {
    fn inspect(&self, mut item: usize) -> usize {
        let (op, rhs) = self.op.split(' ').collect_tuple().expect("two parts");
        let rhs = match rhs.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                assert!(rhs.trim() == "old");
                item
            }
        };
        match op.trim() {
            "*" => item *= rhs,
            "+" => item += rhs,
            _ => unreachable!(),
        };
        item / 3
    }

    fn test(&self, item: usize) -> usize {
        if item % self.test == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }

    fn give_items(&mut self) -> Vec<usize> {
        //info!("Handle items {} + {}", self.items_handled, self.items.len());
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
                .map(|item| item.trim().parse::<usize>().expect("a number"))
                .collect_vec();
            let op = lines
                .next()
                .expect("Operation")
                .trim()
                .strip_prefix("Operation: new = old")
                .expect("to exist")
                .trim()
                .to_string();

            let test = lines
                .next()
                .expect("Test")
                .trim()
                .strip_prefix("Test: divisible by")
                .expect("to exist")
                .trim()
                .parse::<usize>()
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
        info!("Round: {} -------------------", round);
        for (index, monkey) in input.iter().enumerate() {
            info!("Monkey {}: {:?}", index, monkey.items)
        }
        for monkey_index in 0..input.len() {
            let monkey = input[monkey_index].clone();
            for item in input[monkey_index].give_items() {
                let inspected_item = monkey.inspect(item);
                input[monkey.test(inspected_item)]
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
            info!("Monkey {}: {:?}", index, monkey.items)
        }
        for monkey_index in 0..input.len() {
            let monkey = input[monkey_index].clone();
            for item in input[monkey_index].give_items() {
                let inspected_item = monkey.inspect(item);
                input[monkey.test(inspected_item)]
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
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn example_1_2() {
        //assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
