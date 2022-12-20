use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;
use log::{debug, info};
use regex::Regex;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 19:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Debug, Clone, Copy)]
enum Ressource {
    Ore(usize),
    Clay(usize),
    Obsidian(usize),
    Geode(usize),
}

#[derive(Debug, Clone)]
struct Robot {
    cost: Vec<Ressource>,
    production: Ressource,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    ore_robot: Robot,
    clay_robot: Robot,
    obsidian_robot: Robot,
    geode_robot: Robot,
}

type Input = Vec<Blueprint>;

fn parse(input: &str) -> Input {
    let expression = Regex::new (r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    use Ressource::*;
    expression
        .captures_iter(input)
        .map(|m| {
            let parsed = m
                .iter()
                .skip(1)
                .map(|group| group.unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            let id = parsed[0];
            let ore_robot = Robot {
                cost: vec![Ore(parsed[1])],
                production: Ore(1),
            };
            let clay_robot = Robot {
                cost: vec![Ore(parsed[2])],
                production: Clay(1),
            };
            let obsidian_robot = Robot {
                cost: vec![Ore(parsed[3]), Clay(parsed[4])],
                production: Obsidian(1),
            };
            let geode_robot = Robot {
                cost: vec![Ore(parsed[5]), Obsidian(parsed[6])],
                production: Geode(1),
            };
            Blueprint {
                id,
                ore_robot,
                clay_robot,
                obsidian_robot,
                geode_robot,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }

    fn produce(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }

    fn remove(&mut self, cost: Ressource) -> bool {
        match cost {
            Ressource::Ore(amount) => {
                if self.ore >= amount {
                    self.ore -= amount;
                    return true;
                }
            }
            Ressource::Clay(amount) => {
                if self.clay >= amount {
                    self.clay -= amount;
                    return true;
                }
            }
            Ressource::Obsidian(amount) => {
                if self.obsidian >= amount {
                    self.obsidian -= amount;
                    return true;
                }
            }
            Ressource::Geode(amount) => {
                if self.geode >= amount {
                    self.geode -= amount;
                    return true;
                }
            }
        }
        false
    }

    fn consume_ressources(&mut self, costs: &[Ressource]) -> bool {
        let mut copy = self.clone();
        if costs.iter().all(|cost| copy.remove(*cost)) {
            *self = copy;
            true
        } else {
            false
        }
    }

    fn build_robot_if_possible(&mut self, blueprint: &Blueprint, kind: Ressource) -> bool {
        match kind {
            Ressource::Ore(_) => {
                if self.consume_ressources(&blueprint.ore_robot.cost) {
                    self.ore_robot += 1;
                    return true;
                }
            }
            Ressource::Clay(_) => {
                if self.consume_ressources(&blueprint.clay_robot.cost) {
                    self.clay_robot += 1;
                    return true;
                }
            }
            Ressource::Obsidian(_) => {
                if self.consume_ressources(&blueprint.obsidian_robot.cost) {
                    self.obsidian_robot += 1;
                    return true;
                }
            }
            Ressource::Geode(_) => {
                if self.consume_ressources(&blueprint.geode_robot.cost) {
                    self.geode_robot += 1;
                    return true;
                }
            }
        }
        false
    }
}

type Time = usize;

fn max_geode(blueprint: Blueprint, time: usize) -> usize {
    let mut queue = VecDeque::<(Inventory, Time)>::new();
    queue.push_back((Inventory::new(), time));
    let mut visited = HashSet::<(Inventory, Time)>::new();

    while let Some((mut inventory, time)) = queue.pop_front() {
        info!("{time}: {}", queue.len());
        if time == 0 && visited.contains(&(inventory.clone(), time)) {
            continue;
        }
        visited.insert((inventory.clone(), time));

        inventory.produce();

        queue.push_back((inventory.clone(), time - 1));

        let mut clone = inventory.clone();
        if clone.build_robot_if_possible(&blueprint, Ressource::Ore(0)) {
            queue.push_back((clone, time - 1));
        }

        let mut clone = inventory.clone();
        if clone.build_robot_if_possible(&blueprint, Ressource::Clay(0)) {
            queue.push_back((clone, time - 1));
        }

        let mut clone = inventory.clone();
        if clone.build_robot_if_possible(&blueprint, Ressource::Obsidian(0)) {
            queue.push_back((clone, time - 1));
        }

        let mut clone = inventory.clone();
        if clone.build_robot_if_possible(&blueprint, Ressource::Geode(0)) {
            queue.push_back((clone, time - 1));
        }
    }

    visited
        .into_iter()
        .map(|(inventory, _)| inventory.geode)
        .max()
        .unwrap()
}

fn solve_part1(input: Input) -> usize {
    let time = 24;
    input
        .into_iter()
        .map(|blueprint| blueprint.id * max_geode(blueprint, time))
        .sum()
}

fn solve_part2(input: Input) -> usize {
    0
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
        assert_eq!(solve_part1(input), 33);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
