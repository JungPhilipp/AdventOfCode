use std::collections::VecDeque;

use bit_vec::BitVec;
use hex::FromHex;
use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem16/input.txt";

type Input = VecDeque<bool>;

#[derive(Clone)]
pub struct Package {
    version: u8,
    type_id: u8,
    literal: Option<usize>,
    sub_packages: Vec<Package>,
}

impl Package {
    fn operator(&self) -> usize {
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

    fn invoke_operators(&self) -> Vec<usize> {
        self.sub_packages
            .iter()
            .map(|package| package.operator())
            .collect_vec()
    }
    fn sum(&self) -> usize {
        self.invoke_operators().iter().sum::<usize>()
    }
    fn product(&self) -> usize {
        self.invoke_operators().iter().product()
    }
    fn max(&self) -> usize {
        *self.invoke_operators().iter().max().unwrap()
    }
    fn min(&self) -> usize {
        *self.invoke_operators().iter().min().unwrap()
    }
    fn gt(&self) -> usize {
        assert_eq!(self.sub_packages.len(), 2);
        let values = self.invoke_operators();
        (values[0] > values[1]) as usize
    }
    fn lt(&self) -> usize {
        assert_eq!(self.sub_packages.len(), 2);
        let values = self.invoke_operators();
        (values[0] < values[1]) as usize
    }
    fn eq(&self) -> usize {
        assert_eq!(self.sub_packages.len(), 2);
        let values = self.invoke_operators();
        (values[0] == values[1]) as usize
    }
}

fn hex_to_bin(string: &str) -> Vec<bool> {
    BitVec::from_bytes(&Vec::from_hex(string).unwrap())
        .iter()
        .collect_vec()
}

pub fn parse_input(path_to_input: &str) -> Input {
    let raw_input = read_lines(path_to_input)
        .iter()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                Some(hex_to_bin(line))
            }
        })
        .collect_vec();
    assert_eq!(raw_input.len(), 1);
    raw_input[0].clone().into()
}

fn to_number(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(pos, flag)| 2usize.pow(pos as u32) * *flag as usize)
        .sum()
}

pub fn parse_package(bits: &mut VecDeque<bool>) -> Package {
    let version = to_number(&bits.drain(0..3).collect_vec()) as u8;
    let type_id = to_number(&bits.drain(0..3).collect_vec()) as u8;
    let mut literal = None;
    let sub_packages = match type_id {
        4 => {
            let mut literal_bits = vec![];
            loop {
                let chunk = bits.drain(0..5).collect_vec();
                literal_bits.extend(chunk.iter().skip(1));
                if !chunk[0] {
                    break;
                }
            }
            literal = Some(to_number(&literal_bits));
            vec![]
        }
        _ => {
            if bits.pop_front().unwrap() {
                let num_packages = to_number(&bits.drain(0..11).collect_vec());
                (0..num_packages).map(|_| parse_package(bits)).collect_vec()
            } else {
                let num_bits_sub_packages = to_number(&bits.drain(0..15).collect_vec());
                let current_num_bits = bits.len();
                let mut packages = vec![];
                while bits.len() > current_num_bits - num_bits_sub_packages {
                    packages.push(parse_package(bits));
                }
                packages
            }
        }
    };
    Package {
        version,
        type_id,
        literal,
        sub_packages,
    }
}

pub fn solve_part1(input: &Input) -> usize {
    let mut bits = input.clone();
    let root = parse_package(&mut bits);
    let mut packages = vec![];

    let mut to_process = vec![root];
    while let Some(package) = to_process.pop() {
        packages.push(package.clone());
        to_process.extend(package.sub_packages);
    }
    packages
        .iter()
        .map(|package| package.version as usize)
        .sum::<usize>()
}

pub fn solve_part2(input: &Input) -> usize {
    let mut bits = input.clone();
    parse_package(&mut bits).operator()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    pub static EXAMPLE_PATH: &str = "src/problems_2021/problem16/example.txt";

    fn to_bool_vec(string: &str) -> Vec<bool> {
        string
            .chars()
            .into_iter()
            .map(|c| c.to_digit(2).unwrap() != 0)
            .collect_vec()
    }

    #[test]
    fn example1() {
        let mut input = parse_input(EXAMPLE_PATH);
        assert_eq!(input, to_bool_vec("110100101111111000101000"));
        assert_eq!(parse_package(&mut input).literal.unwrap(), 2021);
    }

    #[test]
    fn example2() {
        let mut input = parse_input("src/problems_2021/problem16/example2.txt");
        assert_eq!(
            input,
            to_bool_vec("00111000000000000110111101000101001010010001001000000000")
        );
        let package = parse_package(&mut input);
        assert_eq!(package.version, 1);
        assert_eq!(package.type_id, 6);
        assert_eq!(package.sub_packages.len(), 2);
        assert_eq!(package.sub_packages[0].literal, Some(10));
        assert_eq!(package.sub_packages[1].literal, Some(20));
    }
    #[test]
    fn example3() {
        let mut input = parse_input("src/problems_2021/problem16/example3.txt");
        assert_eq!(
            input,
            to_bool_vec("11101110000000001101010000001100100000100011000001100000")
        );
        let package = parse_package(&mut input);
        assert_eq!(package.version, 7);
        assert_eq!(package.type_id, 3);
        assert_eq!(package.sub_packages.len(), 3);
        assert_eq!(package.sub_packages[0].literal, Some(1));
        assert_eq!(package.sub_packages[1].literal, Some(2));
        assert_eq!(package.sub_packages[2].literal, Some(3));
    }

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&hex_to_bin("8A004A801A8002F478").into()), 16);
        assert_eq!(
            solve_part1(&hex_to_bin("620080001611562C8802118E34").into()),
            12
        );
        assert_eq!(
            solve_part1(&hex_to_bin("C0015000016115A2E0802F182340").into()),
            23
        );
        assert_eq!(
            solve_part1(&hex_to_bin("A0016C880162017C3686B18A3D4780").into()),
            31
        );
    }
    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&hex_to_bin("C200B40A82").into()), 3);
        assert_eq!(solve_part2(&hex_to_bin("04005AC33890").into()), 54);
        assert_eq!(solve_part2(&hex_to_bin("880086C3E88112").into()), 7);
        assert_eq!(solve_part2(&hex_to_bin("CE00C43D881120").into()), 9);
        assert_eq!(solve_part2(&hex_to_bin("D8005AC2A8F0").into()), 1);
        assert_eq!(solve_part2(&hex_to_bin("F600BC2D8F").into()), 0);
        assert_eq!(solve_part2(&hex_to_bin("9C005AC2F8F0").into()), 0);
        assert_eq!(
            solve_part2(&hex_to_bin("9C0141080250320F1802104A08").into()),
            1
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 965);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 116672213160);
    }
}
