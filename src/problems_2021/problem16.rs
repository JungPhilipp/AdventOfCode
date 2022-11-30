use std::{collections::VecDeque, panic};

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "problem16/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 16:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = VecDeque<bool>;

pub fn parse(input: &str) -> Input {
    input
        .trim()
        .chars()
        .filter_map(|b| match b {
            w if w.is_whitespace() => None,
            d if d.is_ascii_hexdigit() => {
                let digit = d.to_digit(16).unwrap();
                Some((0..4).rev().map(move |i| (digit & (1 << i)) != 0))
            }
            _ => panic!("Expected hex not {}", b),
        })
        .flatten()
        .collect()
}

#[derive(Clone)]
pub struct Package {
    version: u64,
    type_id: u64,
    literal: Option<u64>,
    sub_packages: Vec<Package>,
}

impl Package {
    fn operator(&self) -> u64 {
        match self.type_id {
            0 => self.sum(),
            1 => self.product(),
            2 => self.min(),
            3 => self.max(),
            4 => self.literal.unwrap(),
            5 => self.gt(),
            6 => self.lt(),
            7 => self.eq(),
            _ => panic!("Not implemented {}", self.type_id),
        }
    }

    fn invoke_operators(&self) -> Vec<u64> {
        self.sub_packages
            .iter()
            .map(|package| package.operator())
            .collect_vec()
    }
    fn sum(&self) -> u64 {
        self.invoke_operators().iter().sum::<u64>()
    }
    fn product(&self) -> u64 {
        self.invoke_operators().iter().product()
    }
    fn max(&self) -> u64 {
        *self.invoke_operators().iter().max().unwrap()
    }
    fn min(&self) -> u64 {
        *self.invoke_operators().iter().min().unwrap()
    }
    fn gt(&self) -> u64 {
        assert_eq!(self.sub_packages.len(), 2);
        let values = self.invoke_operators();
        (values[0] > values[1]) as u64
    }
    fn lt(&self) -> u64 {
        assert_eq!(self.sub_packages.len(), 2);
        let values = self.invoke_operators();
        (values[0] < values[1]) as u64
    }
    fn eq(&self) -> u64 {
        assert_eq!(self.sub_packages.len(), 2);
        let values = self.invoke_operators();
        (values[0] == values[1]) as u64
    }
}

fn read_bits(bits: &mut dyn Iterator<Item = bool>, num_bits: usize) -> u64 {
    bits.take(num_bits).fold(0, |a, b| a << 1 | b as u64)
}

pub fn parse_package(bits: &mut dyn Iterator<Item = bool>) -> Option<Package> {
    let version = read_bits(bits, 3);
    let type_id = read_bits(bits, 3);
    let mut literal = None;
    let sub_packages = match type_id {
        4 => {
            let mut literal_bits = vec![];
            loop {
                let chunk = bits.take(5).collect_vec();
                literal_bits.extend(chunk.iter().skip(1));
                if !chunk[0] {
                    break;
                }
            }
            literal = Some(read_bits(&mut literal_bits.into_iter(), usize::MAX));
            vec![]
        }
        _ => match bits.next() {
            None => return None,
            Some(true) => {
                let num_packages = read_bits(bits, 11);
                (0..num_packages)
                    .map(|_| parse_package(bits).unwrap())
                    .collect_vec()
            }
            Some(false) => {
                let num_bits_sub_packages = read_bits(bits, 15) as usize;
                let mut sub_bits = bits.take(num_bits_sub_packages).collect_vec().into_iter();
                let mut packages = vec![];
                while let Some(package) = parse_package(&mut sub_bits) {
                    packages.push(package)
                }
                packages
            }
        },
    };

    Some(Package {
        version,
        type_id,
        literal,
        sub_packages,
    })
}

pub fn solve_part1(bits: Input) -> u64 {
    let root = parse_package(&mut bits.into_iter()).unwrap();
    let mut packages = vec![];

    let mut to_process = vec![root];
    while let Some(package) = to_process.pop() {
        packages.push(package.clone());
        to_process.extend(package.sub_packages);
    }
    packages.iter().map(|package| package.version).sum::<u64>()
}

pub fn solve_part2(bits: Input) -> u64 {
    parse_package(&mut bits.into_iter()).unwrap().operator()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    fn to_bool_vec(string: &str) -> Vec<bool> {
        string
            .chars()
            .into_iter()
            .map(|c| c.to_digit(2).unwrap() != 0)
            .collect_vec()
    }

    #[test]
    fn example1() {
        let input = parse("D2FE28");
        assert_eq!(input, to_bool_vec("110100101111111000101000"));
        assert_eq!(
            parse_package(&mut input.into_iter())
                .unwrap()
                .literal
                .unwrap(),
            2021
        );
    }

    #[test]
    fn example2() {
        let input = parse("38006F45291200");
        assert_eq!(
            input,
            to_bool_vec("00111000000000000110111101000101001010010001001000000000")
        );
        let package = parse_package(&mut input.into_iter()).unwrap();
        assert_eq!(package.version, 1);
        assert_eq!(package.type_id, 6);
        assert_eq!(package.sub_packages.len(), 2);
        assert_eq!(package.sub_packages[0].literal, Some(10));
        assert_eq!(package.sub_packages[1].literal, Some(20));
    }
    #[test]
    fn example3() {
        let input = parse("EE00D40C823060");
        assert_eq!(
            input,
            to_bool_vec("11101110000000001101010000001100100000100011000001100000")
        );
        let package = parse_package(&mut input.into_iter()).unwrap();
        assert_eq!(package.version, 7);
        assert_eq!(package.type_id, 3);
        assert_eq!(package.sub_packages.len(), 3);
        assert_eq!(package.sub_packages[0].literal, Some(1));
        assert_eq!(package.sub_packages[1].literal, Some(2));
        assert_eq!(package.sub_packages[2].literal, Some(3));
    }

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(parse("8A004A801A8002F478")), 16);
        assert_eq!(solve_part1(parse("620080001611562C8802118E34")), 12);
        assert_eq!(solve_part1(parse("C0015000016115A2E0802F182340")), 23);
        assert_eq!(solve_part1(parse("A0016C880162017C3686B18A3D4780")), 31);
    }
    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(parse("C200B40A82")), 3);
        assert_eq!(solve_part2(parse("04005AC33890")), 54);
        assert_eq!(solve_part2(parse("880086C3E88112")), 7);
        assert_eq!(solve_part2(parse("CE00C43D881120")), 9);
        assert_eq!(solve_part2(parse("D8005AC2A8F0")), 1);
        assert_eq!(solve_part2(parse("F600BC2D8F")), 0);
        assert_eq!(solve_part2(parse("9C005AC2F8F0")), 0);
        assert_eq!(solve_part2(parse("9C0141080250320F1802104A08")), 1);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 965);
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse(include_str!(INPUT_PATH!()))),
            116672213160
        );
    }
}
