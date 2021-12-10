use std::collections::HashSet;

use itertools::Itertools;
use log::debug;

use crate::util::parse::parse_to_vec;

pub static INPUT_PATH: &str = "src/problems_2021/problem9/input.txt";

type Input = (usize, Vec<i32>);

pub fn parse_input(path_to_input: &str) -> Input {
    let input = parse_to_vec(path_to_input);
    (input[0].len(), input.into_iter().flatten().collect_vec())
}

fn low_point(height_map: &[i32], index: i32, line_width: i32) -> bool {
    let neighbors = [index - 1, index + 1, index - line_width, index + line_width];
    neighbors
        .iter()
        .filter_map(|neighbor| {
            if *neighbor >= 0 {
                height_map.get(*neighbor as usize)
            } else {
                None
            }
        })
        .all(|height| *height > height_map[index as usize])
}

pub fn solve_part1(input: &Input) -> i32 {
    let (line_width, height_map) = input;
    height_map
        .iter()
        .enumerate()
        .filter_map(|(index, height)| {
            if low_point(height_map, index as i32, *line_width as i32) {
                Some(height + 1)
            } else {
                None
            }
        })
        .sum()
}

fn basin_size(height_map: &[i32], index_low_point: i32, line_width: i32) -> i32 {
    let mut basin_indices = HashSet::<usize>::new();
    let mut processed = HashSet::<usize>::new();
    let mut candidates = vec![index_low_point];
    while !candidates.is_empty() {
        let candidate = candidates.pop().unwrap();
        let height = height_map[candidate as usize];
        if height == 9 || processed.contains(&(candidate as usize)) {
            processed.insert(candidate as usize);
            continue;
        }
        processed.insert(candidate as usize);
        basin_indices.insert(candidate as usize);

        let mut neighbors = vec![candidate + line_width, candidate - line_width];
        if candidate % line_width != 0 {
            neighbors.push(candidate - 1);
        }
        if candidate % line_width != line_width - 1 {
            neighbors.push(candidate + 1);
        }
        for neighbor in neighbors {
            if neighbor >= 0 && neighbor < height_map.len() as i32 {
                candidates.push(neighbor as i32)
            }
        }
    }
    debug!(
        "Found Basin {:?} with size {:?} : {:?}",
        index_low_point,
        basin_indices.len(),
        basin_indices
    );
    basin_indices.len() as i32
}
pub fn solve_part2(input: &Input) -> i32 {
    let (line_width, height_map) = input;
    let basins = height_map
        .iter()
        .enumerate()
        .filter_map(|(index, _)| {
            if low_point(height_map, index as i32, *line_width as i32) {
                Some(basin_size(height_map, index as i32, *line_width as i32))
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .collect_vec();
    debug!("Basins {:?}", basins);
    basins.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 468);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1280496);
    }
}
