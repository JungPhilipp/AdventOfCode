use itertools::Itertools;
use log::{debug, info};
use ndarray::Array2;

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

macro_rules! INPUT_PATH {
    () => {
        "day23/input.txt"
    };
}

#[allow(unused)]
pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 23:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Array2<Field>;
type Energy = usize;
type Position = (usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Field {
    Hallway,
    Wall,
    Player(char),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Wall => write!(f, "#"),
            Field::Hallway => write!(f, "."),
            Field::Player(c) => write!(f, "{}", c),
        }
    }
}

fn valid_room(color: char) -> HashSet<Position> {
    match color {
        'A' => [(2, 3), (3, 3)],
        'B' => [(2, 5), (3, 5)],
        'C' => [(2, 7), (3, 7)],
        'D' => [(2, 9), (3, 9)],
        _ => panic!(),
    }
    .into_iter()
    .collect()
}

fn to_energy(color: char) -> usize {
    match color {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

fn is_hallway(pos: &Position) -> bool {
    pos.0 == 1
}

fn parse(input: &str) -> Input {
    Array2::<Field>::from_shape_vec(
        (5, 13),
        input
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Field::Wall,
                        '.' => Field::Hallway,
                        'A' | 'B' | 'C' | 'D' => Field::Player(c),
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    )
    .unwrap()
}

fn finished(board: &Input, end: &Input) -> bool {
    debug!("\n{}", board);
    board.iter().zip(end).all(|(b, e)| *b == *e)
}

fn next_positions(board: &Input, index: Position) -> Vec<(Position, usize)> {
    let mut visited = HashMap::<Position, usize>::new();
    let mut to_visit = vec![(index, 0)];
    while let Some((pos, steps)) = to_visit.pop() {
        if let Some(steps_before) = visited.get(&pos) {
            if *steps_before <= steps {
                continue;
            }
        }
        *visited.entry(pos).or_insert(0) = steps;

        for neighbor_index in [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ] {
            if board[neighbor_index] == Field::Hallway {
                to_visit.push((neighbor_index, steps + 1));
            }
        }
    }

    visited.into_iter().collect_vec()
}
fn valid_move(board: &Input, color: char, current_pos: &Position, new_pos: &Position) -> bool {
    // Currently in hallway
    if is_hallway(current_pos) {
        let room = valid_room(color);
        room.contains(new_pos)
            && room.iter().all(|pos| {
                if let Field::Player(other_color) = board[*pos] {
                    other_color == color
                } else {
                    true
                }
            })
    }
    // Currently in room
    else {
        let outside_room = [(3, 1), (5, 1), (7, 1), (9, 1)];
        let room = valid_room(color);
        (room.contains(new_pos)
            && room.iter().all(|pos| {
                if let Field::Player(other_color) = board[*pos] {
                    other_color == color
                } else {
                    true
                }
            }))
            || (is_hallway(new_pos) && !outside_room.contains(new_pos))
    }
}
fn next_steps(board: &Input, index: Position) -> Vec<(Input, Energy)> {
    match board[index] {
        Field::Player(color) => next_positions(board, index)
            .into_iter()
            .filter_map(|(new_pos, steps)| {
                if valid_move(board, color, &index, &new_pos) {
                    Some((new_pos, steps * to_energy(color)))
                } else {
                    None
                }
            })
            .map(|(next_position, cost)| {
                let mut new_board = board.clone();
                new_board[index] = board[next_position].clone();
                new_board[next_position] = board[index].clone();
                (new_board, cost)
            })
            .collect_vec(),
        _ => vec![],
    }
}

fn walk(board: Input, energy: Energy, end: &Input) -> Option<Energy> {
    if finished(&board, end) {
        return Some(energy);
    }
    board
        .indexed_iter()
        .filter_map(|(index, _)| {
            next_steps(&board, index)
                .into_iter()
                .filter_map(|(new_board, cost)| walk(new_board, energy + cost, end))
                .min()
        })
        .min()
}

pub fn solve_part1(input: Input) -> i64 {
    let end = parse(include_str!("day23/finished.txt"));
    walk(input, 0, &end);
    0
}
pub fn solve_part2(_input: Input) -> i64 {
    0
}

#[cfg(test)]
mod tests {

    use test_log::test;

    #[test]
    fn example1() {
        //assert_eq!(solve_part1(parse(include_str!("day23/example1.txt"))), 65);
    }

    #[test]
    fn part1() {
        //assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        //assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
