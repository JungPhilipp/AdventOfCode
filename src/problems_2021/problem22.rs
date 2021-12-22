use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
    ops::{Range, RangeInclusive},
};

use array_tool::vec::Intersect;
use itertools::Itertools;
use log::{debug, info};
use range_union_find::IntRangeUnionFind;

macro_rules! INPUT_PATH {
    () => {
        "problem22/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 22:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}
type MyRange = Range<i32>;

type Input = Vec<(bool, MyRange, MyRange, MyRange)>;

fn next_number(iter: &mut dyn Iterator<Item = char>) -> Result<i32, ParseIntError> {
    iter.skip_while(|c| !c.is_digit(10) && *c != '-')
        .take_while(|c| c.is_digit(10) || *c == '-')
        .collect::<String>()
        .parse::<i32>()
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let on = if l.starts_with("on") {
                true
            } else if l.starts_with("off") {
                false
            } else {
                panic!("Expected on or off {}", l)
            };
            let mut line = l.chars();
            let x_min = next_number(&mut line).unwrap();
            let x_max = next_number(&mut line).unwrap();
            let y_min = next_number(&mut line).unwrap();
            let y_max = next_number(&mut line).unwrap();
            let z_min = next_number(&mut line).unwrap();
            let z_max = next_number(&mut line).unwrap();

            (on, x_min..x_max + 1, y_min..y_max + 1, z_min..z_max + 1)
        })
        .collect_vec()
}

pub fn solve_part1(input: Input) -> usize {
    let valid_x_range = -50..50 + 1;
    let valid_y_range = -50..50 + 1;
    let valid_z_range = -50..50 + 1;
    let mut points = HashSet::<(i32, i32, i32)>::new();

    let total_lines = input.len();
    for (line, (on, x_range, y_range, z_range)) in input.into_iter().enumerate() {
        info!("Line {}/{}", line, total_lines);
        for x in x_range {
            if !valid_x_range.contains(&x) {
                continue;
            }
            for y in y_range.clone() {
                if !valid_y_range.contains(&y) {
                    continue;
                }
                for z in z_range.clone() {
                    if !valid_z_range.contains(&z) {
                        continue;
                    }
                    if on {
                        points.insert((x, y, z));
                    } else {
                        points.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    points.len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube {
    x_range: MyRange,
    y_range: MyRange,
    z_range: MyRange,
}

impl Cube {
    fn area(&self) -> usize {
        self.x_range.len() * self.y_range.len() * self.z_range.len()
    }

    fn range_intersection(lhs: &MyRange, rhs: &MyRange) -> Option<MyRange> {
        if (lhs.end.max(rhs.end) - lhs.start.min(rhs.start)) as usize >= (lhs.len() + rhs.len()) {
            None
        } else {
            Some(lhs.start.max(rhs.start)..lhs.end.min(rhs.end))
        }
    }

    fn intersect(&self, other: &Cube) -> Option<Cube> {
        if let Some(x_intersection) = Self::range_intersection(&self.x_range, &other.x_range) {
            if let Some(y_intersection) = Self::range_intersection(&self.y_range, &other.y_range) {
                if let Some(z_intersection) =
                    Self::range_intersection(&self.z_range, &other.z_range)
                {
                    return Some(Cube {
                        x_range: x_intersection,
                        y_range: y_intersection,
                        z_range: z_intersection,
                    });
                }
            }
        }
        None
    }
}

pub fn solve_part2(input: Input) -> i64 {
    let mut cubes: Vec<Cube> = vec![];
    let mut negative_cubes: Vec<Cube> = vec![];
    let mut sum = 0;
    let total_lines = input.len();
    for (line, (on, x_range, y_range, z_range)) in input.into_iter().enumerate() {
        let new_cube = Cube {
            x_range,
            y_range,
            z_range,
        };
        let mut intersections = vec![];
        for old_cube in &cubes {
            if let Some(intersection) = old_cube.intersect(&new_cube) {
                intersections.push(intersection);
            }
        }
        let mut negative_intersections = vec![];
        for old_cube in &negative_cubes {
            if let Some(intersection) = old_cube.intersect(&new_cube) {
                negative_intersections.push(intersection);
            }
        }
        if on {
            sum += new_cube.area() as i64;
            cubes.push(new_cube);
        } else {
            negative_cubes.push(new_cube);
        }
        info!("Line {}/{}, Current sum {}", line + 1, total_lines, sum);

        let mut add = -1;
        while !intersections.is_empty() {
            debug!("Intersections {}, sum {}", intersections.len(), sum);
            dbg!(intersections.clone());
            sum += add * intersections.iter().map(|cube| cube.area()).sum::<usize>() as i64;
            debug!("sum {}", sum);
            let mut next_intersections = HashSet::new();
            for i in 0..intersections.len() {
                for j in i + 1..intersections.len() {
                    if let Some(intersection) = intersections[i].intersect(&intersections[j]) {
                        next_intersections.insert(intersection);
                    }
                }
            }

            intersections = next_intersections.into_iter().collect_vec();
            add *= -1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(parse(include_str!("problem22/example1.txt"))),
            39
        );
    }

    #[test]
    fn example1_2() {
        assert_eq!(
            solve_part2(parse(include_str!("problem22/example1.txt"))),
            39
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(parse(include_str!("problem22/example2.txt"))),
            590784
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part2(parse(include_str!("problem22/example3.txt"))),
            2758514936282235
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            solve_part2(parse(include_str!("problem22/example4.txt"))),
            65
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            solve_part2(parse(include_str!("problem22/example5.txt"))),
            38
        );
    }

    #[test]
    fn part1_2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 596989);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 596989);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
