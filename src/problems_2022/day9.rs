use std::collections::HashSet;

use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day9/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 9:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

type Input = Vec<(Direction, i32)>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (s_dir, s_steps) = line.split_whitespace().collect_tuple().expect("Two Parts");
            let direction = match s_dir {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                unmatched => panic!("Unknown direction {} ", unmatched),
            };
            let steps = s_steps.parse::<i32>().expect("a number");
            (direction, steps)
        })
        .collect_vec()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    fn in_range(&self, other: &Point) -> bool {
        let (x_dist, y_dist) = self.distance(other);
        x_dist.abs() <= 1 && y_dist.abs() <= 1
    }

    fn step(&mut self, dir: Direction) {
        use Direction::*;
        match dir {
            Right => self.x += 1,
            Left => self.x -= 1,
            Up => self.y += 1,
            Down => self.y -= 1,
        }
    }

    fn catchup(&mut self, other: &Point) {
        if other.in_range(self) {
            return;
        }

        let (x_dist, y_dist) = other.distance(self);
        self.x += x_dist.signum();
        self.y += y_dist.signum();
    }
}

fn do_step(points: &mut [Point], dir: Direction, steps: i32) -> Vec<Point> {
    (0..steps)
        .map(|_| {
            points[0].step(dir);
            for (i, j) in (0..points.len()).tuple_windows() {
                let head = points[i].clone();
                let tail = &mut points[j];
                tail.catchup(&head);
            }
            points.last().unwrap().clone()
        })
        .collect()
}

fn solve_part1(input: Input) -> usize {
    let mut points = vec![Point::new(0, 0); 2];
    input
        .into_iter()
        .flat_map(|(direction, steps)| do_step(&mut points, direction, steps))
        .collect::<HashSet<Point>>()
        .len()
}

fn solve_part2(input: Input) -> usize {
    let mut points = vec![Point::new(0, 0); 10];
    input
        .into_iter()
        .flat_map(|(direction, steps)| do_step(&mut points, direction, steps))
        .collect::<HashSet<Point>>()
        .into_iter()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    macro_rules! EXAMPLE_PATH {
        () => {
            "day9/example_1.txt"
        };
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_PATH!()))), 13);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 6057);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(parse(include_str!("day9/example_2.txt"))), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 2514);
    }
}
