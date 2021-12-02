use crate::util::parse::read_lines_with_separator;
use log::debug;

pub static INPUT_PATH: &str = "src/problems_2021/problem2/input.txt";

pub enum Directions {
    Forward(i32),
    Down(i32),
    Up(i32),
}
pub fn parse_input(path_to_input: &str) -> Vec<Directions> {
    read_lines_with_separator(path_to_input, ' ')
        .iter()
        .map(|line| {
            let magnitude = line[1].parse::<i32>().unwrap();
            let direction = match line[0].as_str() {
                "forward" => Directions::Forward(magnitude),
                "down" => Directions::Down(magnitude),
                "up" => Directions::Up(magnitude),
                unknown => panic!("Unknown string encountered {}", unknown),
            };
            direction
        })
        .collect()
}
pub fn solve_part1(input: &Vec<Directions>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for direction in input {
        match direction {
            &Directions::Forward(x) => horizontal_position += x,
            &Directions::Down(x) => depth += x,
            &Directions::Up(x) => depth -= x,
        };
    }
    debug!("{:?}, {:?}", horizontal_position, depth);
    horizontal_position * depth
}

pub fn solve_part2(input: &Vec<Directions>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for direction in input {
        match direction {
            &Directions::Forward(x) => {
                horizontal_position += x;
                depth += aim * x;
            }
            &Directions::Down(x) => {
                aim += x;
            }
            &Directions::Up(x) => {
                aim -= x;
            }
        };
    }
    debug!("{:?}, {:?}", horizontal_position, depth);
    horizontal_position * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 1383564);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1488311643);
    }
}
