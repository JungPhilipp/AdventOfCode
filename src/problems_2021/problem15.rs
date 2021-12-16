use itertools::Itertools;

use crate::util::{
    index::{expand, flatten},
    parse::read_lines,
    shortest_path::{shortest_path, Edge},
};

pub static INPUT_PATH: &str = "src/problems_2021/problem15/input.txt";

type Input = ((usize, usize), Vec<i32>);

pub fn parse_input(path_to_input: &str) -> Input {
    let risk_map = read_lines(path_to_input)
        .iter()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                Some(
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as i32)
                        .collect_vec(),
                )
            }
        })
        .collect_vec();
    let dimensions = (risk_map[0].len(), risk_map.len());
    (dimensions, risk_map.into_iter().flatten().collect_vec())
}

fn adj_list(grid: &[i32], dimensions: &(usize, usize)) -> Vec<Vec<Edge>> {
    grid.iter()
        .enumerate()
        .map(|(index, risk)| {
            let (x, y) = expand(index as i32, dimensions);
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .filter_map(|pos| {
                    flatten(*pos, dimensions).map(|flat_index| Edge {
                        node: flat_index as usize,
                        cost: *risk as usize,
                    })
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn solve_part1(input: &Input) -> i32 {
    let (dimensions, risk_map) = input;
    let adj = adj_list(risk_map, dimensions);

    shortest_path(&adj, 0, adj.len() - 1).unwrap() as i32
}

pub fn solve_part2(input: &Input) -> i32 {
    let (dimensions, risk_map) = input;

    let large_dimensions = (dimensions.0 * 5, dimensions.1 * 5);
    let large_map = (0..5 * 5 * risk_map.len())
        .map(|index| {
            let (x_large, y_large) = expand(index as i32, &large_dimensions);
            let x_tile = x_large / dimensions.0 as i32;
            let y_tile = y_large / dimensions.1 as i32;
            let x = x_large % dimensions.0 as i32;
            let y = y_large % dimensions.1 as i32;
            let risk = risk_map[flatten((x, y), dimensions).unwrap() as usize];
            let new_risk = risk + x_tile + y_tile;
            (new_risk - 1) % 9 + 1
        })
        .collect_vec();

    let adj = adj_list(&large_map, &large_dimensions);

    shortest_path(&adj, 0, adj.len() - 1).unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    pub static EXAMPLE_PATH: &str = "src/problems_2021/problem15/example.txt";

    #[test]
    fn example1() {
        let input = parse_input(EXAMPLE_PATH);
        assert_eq!(solve_part1(&input), 40);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 589);
    }

    #[test]
    fn example2() {
        let input = parse_input(EXAMPLE_PATH);
        assert_eq!(solve_part2(&input), 315);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 2885);
    }
}
