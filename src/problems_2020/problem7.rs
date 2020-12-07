use crate::util::parse::read_lines;
use std::collections::{HashMap, HashSet};
pub static INPUT_PATH: &str = "src/problems_2020/problem7/input.txt";

pub fn parse_line(line: &str) -> (String, Vec<(i32, String)>) {
    let words = line.split(" ").collect::<Vec<&str>>();
    let color = words[0..2].join(" ");
    let contents = words[4..]
        .join(" ")
        .split(", ")
        .map(|s| {
            let parts = s.split(" ").collect::<Vec<&str>>();
            let count = parts[0].parse::<i32>();
            let bag_color = parts[1..3].join(" ");
            match count {
                Err(_) => Err(""),
                Ok(count) => Ok((count, bag_color)),
            }
        })
        .filter_map(Result::ok)
        .collect::<Vec<(i32, String)>>();
    (color, contents)
}

pub fn parse_input(path_to_input: &str) -> HashMap<String, Vec<(i32, String)>> {
    read_lines(path_to_input)
        .into_iter()
        .map(|line| parse_line(&line))
        .collect()
}

pub fn find_bag(
    input: &HashMap<String, Vec<(i32, String)>>,
    key: &String,
    target: &str,
) -> Option<String> {
    if key == target {
        return None;
    }
    let mut stack = HashSet::new();
    stack.insert(key);
    while !stack.is_empty() {
        let current = stack.iter().next().unwrap().clone();
        stack.remove(current);
        if current == target {
            return Some(key.clone());
        }
        for (_, value) in input.get(current).unwrap() {
            stack.insert(value);
        }
    }
    None
}

pub fn solve_part1(input: &HashMap<String, Vec<(i32, String)>>) -> usize {
    input
        .into_iter()
        .filter_map(|(key, _)| find_bag(input, &key, "shiny gold"))
        .count()
}

pub fn sum_bags(input: &HashMap<String, Vec<(i32, String)>>, key: &str) -> i32 {
    let v = input.get(key).unwrap();
    let mut sum = 0;
    for (c, k) in v {
        sum += c * sum_bags(input, k);
    }
    sum + 1
}

pub fn solve_part2(input: &HashMap<String, Vec<(i32, String)>>) -> usize {
    (sum_bags(input, "shiny gold") - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn test_parse_line() {
        parse_line("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        parse_line("posh maroon bags contain no other bags.");
    }
    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&parse_input("src/problems_2020/problem7/input_test.txt")),
            4
        );
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 177);
    }
    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&parse_input("src/problems_2020/problem7/input_test2.txt")),
            126
        );
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 34988);
    }
}
