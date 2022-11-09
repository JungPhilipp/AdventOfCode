use std::{
    collections::{HashMap, HashSet},
    panic,
};

use itertools::Itertools;
use log::{debug, info};

use crate::util::shortest_path::{shortest_path, Edge};

macro_rules! INPUT_PATH {
    () => {
        "day24/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Day24:\nPart1{:?}\nPart2{:?}",
        solve_part1(&parsed),
        solve_part2(&parsed)
    );
}

type Input = Vec<Vec<Instruction>>;
type Register = usize;
type Number = i64;

#[derive(Debug, Clone)]
pub enum Source {
    Register(Register),
    Number(Number),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Inp(Register),
    Add((Register, Register)),
    AddScalar((Register, Number)),
    Mul((Register, Register)),
    MulScalar((Register, Number)),
    Div((Register, Register)),
    DivScalar((Register, Number)),
    Mod((Register, Register)),
    ModScalar((Register, Number)),
    Eql((Register, Register)),
    EqlScalar((Register, Number)),
}

fn parse_register_or_number(input: &str) -> Source {
    match input {
        "w" => Source::Register(0),
        "x" => Source::Register(1),
        "y" => Source::Register(2),
        "z" => Source::Register(3),
        _ => Source::Number(input.parse::<Number>().unwrap()),
    }
}

fn parse_register(input: &str) -> Register {
    match parse_register_or_number(input) {
        Source::Register(register) => register,
        Source::Number(_) => panic!("Expected register not number {}", input),
    }
}

pub fn parse(input: &str) -> Input {
    let result = input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            match parts[0] {
                "inp" => {
                    assert_eq!(parts.len(), 2);
                    Instruction::Inp(parse_register(parts[1]))
                }
                "add" => {
                    assert_eq!(parts.len(), 3);
                    let dest = parse_register(parts[1]);
                    match parse_register_or_number(parts[2]) {
                        Source::Register(source) => Instruction::Add((dest, source)),
                        Source::Number(source) => Instruction::AddScalar((dest, source)),
                    }
                }
                "mul" => {
                    assert_eq!(parts.len(), 3);
                    let dest = parse_register(parts[1]);
                    match parse_register_or_number(parts[2]) {
                        Source::Register(source) => Instruction::Mul((dest, source)),
                        Source::Number(source) => Instruction::MulScalar((dest, source)),
                    }
                }
                "div" => {
                    assert_eq!(parts.len(), 3);
                    let dest = parse_register(parts[1]);
                    match parse_register_or_number(parts[2]) {
                        Source::Register(source) => Instruction::Div((dest, source)),
                        Source::Number(source) => Instruction::DivScalar((dest, source)),
                    }
                }
                "mod" => {
                    assert_eq!(parts.len(), 3);
                    let dest = parse_register(parts[1]);
                    match parse_register_or_number(parts[2]) {
                        Source::Register(source) => Instruction::Mod((dest, source)),
                        Source::Number(source) => Instruction::ModScalar((dest, source)),
                    }
                }
                "eql" => {
                    assert_eq!(parts.len(), 3);
                    let dest = parse_register(parts[1]);
                    match parse_register_or_number(parts[2]) {
                        Source::Register(source) => Instruction::Eql((dest, source)),
                        Source::Number(source) => Instruction::EqlScalar((dest, source)),
                    }
                }
                _ => panic!("Unexpected line {:?}", parts),
            }
        })
        .collect_vec()
        .chunks_exact(18)
        .map(|chunk| {
            assert_eq!(chunk[0], Instruction::Inp(0));
            chunk.to_vec()
        })
        .collect_vec();
    debug!("{:?}", result);
    result
}

fn divide(a: Number, b: Number) -> Result<Number, String> {
    if b == 0 {
        Err("Cannot divide by 0".to_owned())
    } else {
        Ok(a / b)
    }
}

fn modulo(a: Number, b: Number) -> Result<Number, String> {
    if a < 0 || b <= 0 {
        Err("Mod failure".to_owned())
    } else {
        Ok(a % b)
    }
}

fn eq(a: Number, b: Number) -> Number {
    (a == b) as Number
}

fn compute_next_state(
    instructions: &[Instruction],
    old_z_state: Number,
    next_input: Number,
) -> Result<Number, String> {
    let mut registers = [0, 0, 0, old_z_state];
    for instruction in instructions {
        match instruction {
            Instruction::Inp(dest) => registers[*dest] = next_input,
            Instruction::Add((dest, src)) => registers[*dest] += registers[*src],
            Instruction::AddScalar((dest, scalar)) => registers[*dest] += scalar,
            Instruction::Mul((dest, src)) => registers[*dest] *= registers[*src],
            Instruction::MulScalar((dest, scalar)) => registers[*dest] *= scalar,
            Instruction::Div((dest, src)) => {
                registers[*dest] = divide(registers[*dest], registers[*src])?;
            }
            Instruction::DivScalar((dest, scalar)) => {
                registers[*dest] = divide(registers[*dest], *scalar)?;
            }
            Instruction::Mod((dest, src)) => {
                registers[*dest] = modulo(registers[*dest], registers[*src])?;
            }
            Instruction::ModScalar((dest, scalar)) => {
                registers[*dest] = modulo(registers[*dest], *scalar)?;
            }
            Instruction::Eql((dest, src)) => {
                registers[*dest] = eq(registers[*dest], registers[*src]);
            }
            Instruction::EqlScalar((dest, scalar)) => {
                registers[*dest] = eq(registers[*dest], *scalar);
            }
        }
    }

    Ok(registers[3])
}

type ZState = Number;
pub fn solve_part1(input: &Input) -> (Number, Number) {
    let mut states = vec![HashSet::<ZState>::new(); 15];
    type Node = (ZState, usize);
    let mut map = HashMap::<Node, Vec<Edge<Node>>>::new();

    states[0].insert(0);
    assert_eq!(input.len(), 14);

    for position in 0..14 {
        info!("Position {}", position);
        debug!("Old States {:?}", states);
        if position == 1 {
            info!("{:?} :::: {:?}", map.keys(), map);
        }
        let mut new_states = HashSet::<ZState>::new();
        for old_z_state in &states[position] {
            for next_digit in 1..=9 {
                if let Ok(new_z_state) =
                    compute_next_state(&input[position], *old_z_state, next_digit)
                {
                    new_states.insert(new_z_state);
                    let old_node = (*old_z_state, position);
                    let new_node = (new_z_state, position + 1);

                    map.entry(old_node).or_default().push(Edge {
                        node: new_node,
                        cost: next_digit * 10_i64.pow(position as u32),
                    });
                }
            }
        }
        states[position + 1] = new_states;
    }

    let low = shortest_path(&map, &(0, 0), &(0, 14));
    info!("Found Low {:?}", low);
    map.iter_mut()
        .for_each(|(_, value)| value.iter_mut().for_each(|edge| edge.cost *= -1));
    let high = shortest_path(&map, &(0, 0), &(0, 14));
    info!("Found high {:?}", high);
    (low.unwrap(), high.unwrap())
}

pub fn solve_part2(input: &Input) -> (Number, Number) {
    solve_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    #[test]
    fn part12() {
        assert_eq!(solve_part1(&parse(include_str!(INPUT_PATH!()))), (0, 0));
    }
}
