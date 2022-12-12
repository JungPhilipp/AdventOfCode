use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use log::info;
use ndarray::Array2;
use num::ToPrimitive;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 12:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Point = (i32, i32);

type Input = (Array2<i32>, Point, Point);

fn parse(input: &str) -> Input {
    let mut start = None;
    let mut end = None;
    let data = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(x, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(y, c)| match c {
                    'a'..='z' => c as i32 - 97,
                    'S' => {
                        start = Some((x as i32, y as i32));
                        0
                    }
                    'E' => {
                        end = Some((x as i32, y as i32));
                        25
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let x_len = data.len();
    let y_len = data[0].len();
    (
        Array2::from_shape_vec((x_len, y_len), data.into_iter().flatten().collect_vec()).unwrap(),
        start.expect("Should contain start"),
        end.expect("Should contain end"),
    )
}

fn get_neighbors(point: &Point, map: &Array2<i32>) -> Vec<Point> {
    let x = point.0;
    let y = point.1;
    let height = map[(x as usize, y as usize)];

    let (x_len, y_len) = map.shape().iter().collect_tuple().expect("two dimensions");
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|&(x_new, y_new)| {
            if x_new >= 0 && x_new < *x_len as i32 && y_new >= 0 && y_new < *y_len as i32 {
                let height_new = map[(x_new as usize, y_new as usize)];
                (height - height_new).abs() <= 1
            } else {
                false
            }
        })
        .collect_vec()
}

fn solve_part1(input: Input) -> usize {
    let (map, start, end) = input;
    info!("\n{:?}, \n{:?}, \n{:?}", map, start, end);
    let mut queue = VecDeque::<(Point, Point, usize)>::new();
    let mut visited = HashMap::<Point, (Point, usize)>::new();
    queue.push_back((start, start, 0));
    while let Some((current, current_parent, current_distance)) = queue.pop_front() {
        if visited.contains_key(&current) {
            continue;
        }
        visited.insert(current, (current_parent, current_distance));

        let neighbors = get_neighbors(&current, &map);
        let new_distance = current_distance + 1;

        for neighbor in neighbors {
            if let Some((parent, distance)) = visited.get_mut(&neighbor) {
                if new_distance < *distance {
                    *parent = current;
                    *distance = new_distance;
                }
            } else {
                queue.push_back((neighbor, current, new_distance))
            }
        }
        queue = queue
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.2, &b.2))
            .collect();
    }
    let mut visited_map: Array2<i32> = Array2::zeros(map.raw_dim());
    for point in visited.keys() {
        visited_map[(point.0 as usize, point.1 as usize)] = 1;
    }
    info!("\n{:?}", visited_map);

    visited.get(&end).expect("path should be found").1
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
        assert_eq!(get_neighbors(&(0, 0), &input.0), vec![(1, 0), (0, 1)]);
        assert_eq!(solve_part1(input), 31);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
