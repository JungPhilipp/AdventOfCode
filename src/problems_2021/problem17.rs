use std::ops::{self, Range};

use log::info;
macro_rules! _INPUT_PATH {
    () => {
        "problem17/input.txt"
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Target {
    x_range: Range<i32>,
    y_range: Range<i32>,
}

type Position = Vec2;
type Velocity = Vec2;
type Input = (Position, Target);
const INPUT: Input = (
    Vec2 { x: 0, y: 0 },
    Target {
        x_range: 70..96 + 1,
        y_range: -179..-124 + 1,
    },
);

pub fn solve() {
    let input = INPUT;
    info!(
        "Solutions Day 17:\nPart1{}\nPart2{}",
        solve_part1(input.clone()),
        solve_part2(input)
    );
}

fn in_target(pos: &Position, target: &Target) -> bool {
    target.x_range.contains(&pos.x) && target.y_range.contains(&pos.y)
}

fn step(pos: &Position, v: &Velocity) -> (Position, Velocity) {
    let new_pos = *pos + *v;
    let new_v = Velocity {
        x: match v.x {
            1..=i32::MAX => v.x - 1,
            i32::MIN..=-1 => v.x + 1,
            0 => 0,
        },
        y: v.y - 1,
    };
    (new_pos, new_v)
}

fn cant_reach_target(pos: &Position, v: &Velocity, target: &Target) -> bool {
    (pos.x > target.x_range.end && v.x >= 0)
        || (pos.x < target.x_range.start && v.x <= 0)
        || (pos.y < target.y_range.start && v.y <= 0)
}

fn shoot(mut pos: Position, mut v: Velocity, target: &Target) -> Option<(Velocity, Position)> {
    let mut max_height_pos = pos;
    loop {
        if in_target(&pos, target) {
            return Some((v, max_height_pos));
        }
        if cant_reach_target(&pos, &v, target) {
            break;
        }

        (pos, v) = step(&pos, &v);
        if pos.y > max_height_pos.y {
            max_height_pos = pos;
        }
    }
    None
}

fn initial_velocity_ranges(start: &Position, target: &Target) -> (Range<i32>, Range<i32>) {
    let _v_x_range = {
        if target.x_range.contains(&start.x) {
            target.x_range.start - start.x..target.x_range.end - start.x
        } else if start.x < target.x_range.start {
            1..target.x_range.end + 1
        } else {
            target.x_range.end..0
        }
    };
    let _v_y_range = {
        let max_y_step = (target.y_range.len() * 1000) as i32 + 1;
        if target.y_range.contains(&start.y) {
            target.y_range.start - start.y..max_y_step
        } else if start.y < target.y_range.start {
            1..max_y_step
        } else {
            -1..max_y_step
        }
    };
    //(v_x_range, v_y_range)
    (-1000..1000, -1000..1000)
}
pub fn solve_part1(input: Input) -> i32 {
    let (start, target) = input;

    let mut highest_position = start;
    let mut _target_v = start;

    let (v_x_range, v_y_range) = initial_velocity_ranges(&start, &target);
    for x in v_x_range {
        for y in v_y_range.clone() {
            let v = Velocity { x, y };
            if let Some((v, max_pos_current)) = shoot(start, v, &target) {
                if max_pos_current.y > highest_position.y {
                    _target_v = v;
                    highest_position = max_pos_current
                }
            }
        }
    }
    dbg!(_target_v, highest_position);
    highest_position.y
}

pub fn solve_part2(input: Input) -> usize {
    let (start, target) = input;

    let (v_x_range, v_y_range) = initial_velocity_ranges(&start, &target);
    let mut solutions = vec![];
    for x in v_x_range {
        for y in v_y_range.clone() {
            let v = Velocity { x, y };
            if let Some((v, _)) = shoot(start, v, &target) {
                solutions.push((v.x, v.y));
            }
        }
    }
    //dbg!(solutions.clone());
    solutions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const EXAMPLE_INPUT: Input = (
        Vec2 { x: 0, y: 0 },
        Target {
            x_range: 20..30 + 1,
            y_range: -10..-5 + 1,
        },
    );
    #[test]
    fn example1() {
        let input = EXAMPLE_INPUT;
        assert_eq!(solve_part1(input.clone()), 45);
        assert_eq!(solve_part2(input), 112);
    }

    #[test]
    fn test_shoot() {
        let (start, target) = EXAMPLE_INPUT;

        let velocities = vec![Velocity { x: 23, y: -10 }, Velocity { x: 20, y: -7 }];
        for v in velocities {
            assert!(shoot(start, v, &target).is_some());
        }
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 15931);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 2555);
    }
}
