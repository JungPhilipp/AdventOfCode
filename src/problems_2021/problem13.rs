use std::collections::HashSet;

use itertools::Itertools;

use crate::util::parse::read_lines;

pub static INPUT_PATH: &str = "src/problems_2021/problem13/input.txt";

pub enum Instruction {
    XFold(i32),
    YFold(i32),
}
type Dot = (i32, i32);
type Input = (HashSet<Dot>, Vec<Instruction>);

pub fn parse_input(path_to_input: &str) -> Input {
    let mut dots = HashSet::<Dot>::new();
    let mut instructions = vec![];
    for line in read_lines(path_to_input) {
        match line {
            s if s.trim().is_empty() => {}
            s if s.starts_with("fold along x") => {
                instructions.push(Instruction::XFold(s[13..].parse::<i32>().unwrap()));
            }
            s if s.starts_with("fold along y") => {
                instructions.push(Instruction::YFold(s[13..].parse::<i32>().unwrap()));
            }
            _ => {
                dots.insert(
                    line.split(',')
                        .map(|pos| pos.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
            }
        }
    }

    (dots, instructions)
}
fn fold_x(dot: &Dot, axis: i32) -> Dot {
    assert_ne!(dot.0, axis);
    if dot.0 < axis {
        *dot
    } else {
        (2 * axis - dot.0, dot.1)
    }
}
fn fold_y(dot: &Dot, axis: i32) -> Dot {
    assert_ne!(dot.1, axis);
    if dot.1 < axis {
        *dot
    } else {
        (dot.0, 2 * axis - dot.1)
    }
}

fn fold(mut dots: HashSet<Dot>, instructions: &[Instruction]) -> HashSet<Dot> {
    for fold in instructions {
        dots = dots
            .iter()
            .map(|dot| match fold {
                Instruction::XFold(x) => fold_x(dot, *x),
                Instruction::YFold(y) => fold_y(dot, *y),
            })
            .collect::<HashSet<Dot>>();
    }
    dots
}

pub fn solve_part1(input: &Input) -> i32 {
    let (dots, instructions) = input;
    fold(dots.clone(), &instructions[0..1]).len() as i32
}

pub fn solve_part2(input: &Input) -> i32 {
    let (dots, instructions) = input;
    let pixels = fold(dots.clone(), instructions);
    let mut image = vec![vec!["."]; 0];
    for dot in pixels {
        let x = dot.0 as usize;
        let y = dot.1 as usize;
        if y >= image.len() {
            image.resize(y + 1, vec!["."]);
        }
        let row = &mut image[y];
        if x >= row.len() {
            row.resize(x as usize + 1, ".")
        }
        image[y][x] = "#"
    }
    print!("{}", image.iter().map(|row| row.join(" ")).join("\n"));
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn example1() {
        let input = parse_input("src/problems_2021/problem13/example.txt");
        assert_eq!(solve_part1(&input), 17);
        //assert_eq!(solve_part2(&input), 36);
    }

    #[test]
    fn example2() {
        //let input = parse_input("src/problems_2021/problem12/example2.txt");
        //assert_eq!(solve_part1(&input), 19);
        //assert_eq!(solve_part2(&input), 103);
    }
    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 671);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 0);
    }
}
