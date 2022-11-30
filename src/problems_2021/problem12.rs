#![allow(clippy::ptr_arg)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem12/input.txt";

type Edge = String;
type Input = HashMap<Edge, HashSet<Edge>>;

pub fn parse_input(path_to_input: &str) -> Input {
    let input = read_lines(path_to_input)
        .iter()
        .flat_map(|line| {
            let connection: (String, String) = line
                .split('-')
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            [connection.clone(), (connection.1, connection.0)]
        })
        .collect_vec();

    let mut adjacency_list = HashMap::<Edge, HashSet<Edge>>::new();
    input.iter().for_each(|(start, end)| {
        adjacency_list
            .entry(start.clone())
            .or_insert_with(HashSet::new)
            .insert(end.clone());
        adjacency_list
            .entry(end.clone())
            .or_insert_with(HashSet::new)
            .insert(start.clone());
    });
    adjacency_list
}

fn is_small(edge: &str) -> bool {
    edge.chars().all(char::is_lowercase)
}

fn bfs_rec(
    adjacency_list: &Input,
    current: &str,
    end: &str,
    mut visited: HashMap<Edge, i32>,
    visit: &dyn Fn(&Edge, &HashMap<Edge, i32>) -> bool,
) -> i32 {
    let mut paths = 0;
    if current == end {
        return 1;
    }
    if is_small(current) {
        *visited.entry(current.to_string()).or_insert(0) += 1;
    }
    for neighbor in adjacency_list.get(current).unwrap() {
        if visit(neighbor, &visited) {
            paths += bfs_rec(adjacency_list, neighbor, end, visited.clone(), visit);
        }
    }
    paths
}

pub fn solve_part1(input: &Input) -> i32 {
    bfs_rec(input, "start", "end", HashMap::new(), &|edge, visited| {
        !visited.contains_key(edge)
    })
}

pub fn solve_part2(input: &Input) -> i32 {
    bfs_rec(input, "start", "end", HashMap::new(), &|edge, visited| {
        !visited.contains_key(edge)
            || (visited.values().max().map_or(true, |max| *max == 1)
                && edge != "start"
                && edge != "end")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        let input = parse_input("src/problems_2021/problem12/example.txt");
        assert_eq!(solve_part1(&input), 10);
        assert_eq!(solve_part2(&input), 36);
    }

    #[test]
    fn example2() {
        let input = parse_input("src/problems_2021/problem12/example2.txt");
        assert_eq!(solve_part1(&input), 19);
        assert_eq!(solve_part2(&input), 103);
    }
    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 4186);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 92111);
    }
}
