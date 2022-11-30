use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

pub static INPUT_PATH: &str = "src/problems_2020/problem4/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<HashMap<String, String>> {
    read_to_string(path_to_input)
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.replace('\n', " ")
                .split(' ')
                .filter_map(|kv| {
                    kv.split(':')
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect_tuple()
                })
                .collect::<HashMap<String, String>>()
        })
        .collect()
}

pub fn contains_keys(passport: &HashMap<String, String>, required_keys: &[&str]) -> bool {
    required_keys.iter().all(|key| passport.contains_key(*key))
}

pub fn valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => match value.parse::<i32>() {
            Ok(y) => (1920..=2002).contains(&y),
            Err(_) => false,
        },
        "iyr" => match value.parse::<i32>() {
            Ok(y) => (2010..=2020).contains(&y),
            Err(_) => false,
        },
        "eyr" => match value.parse::<i32>() {
            Ok(y) => (2020..=2030).contains(&y),
            Err(_) => false,
        },
        "hgt" => {
            let in_re = Regex::new(r"^\d{2}in$").unwrap();
            let cm_re = Regex::new(r"^\d{3}cm$").unwrap();
            if in_re.is_match(value) {
                match value[..2].parse::<i32>() {
                    Ok(y) => (59..=76).contains(&y),
                    Err(_) => false,
                }
            } else if cm_re.is_match(value) {
                match value[..3].parse::<i32>() {
                    Ok(y) => (150..=193).contains(&y),
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        "hcl" => {
            let re = Regex::new(r"^#[0-9|a-f]{6}$").unwrap();
            re.is_match(value)
        }
        "ecl" => {
            let re = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
            re.is_match(value)
        }
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(value)
        }
        "cid" => true,
        _ => false,
    }
}

pub fn solve_part1(input: &[HashMap<String, String>]) -> usize {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .filter(|passport| contains_keys(passport, &required_keys))
        .count()
}

pub fn solve_part2(input: &[HashMap<String, String>]) -> usize {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .filter(|passport| {
            required_keys.iter().all(|key| match passport.get(*key) {
                None => false,
                Some(value) => valid(key, value),
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_valid_key_value() {
        assert!(valid("byr", "1920"));
        assert!(!valid("byr", "1919"));
        assert!(valid("byr", "2002"));
        assert!(!valid("byr", "2003"));
        assert!(valid("iyr", "2010"));
        assert!(!valid("iyr", "2009"));
        assert!(valid("iyr", "2020"));
        assert!(!valid("iyr", "2021"));
        assert!(valid("eyr", "2020"));
        assert!(!valid("eyr", "2019"));
        assert!(valid("eyr", "2030"));
        assert!(!valid("eyr", "210000"));
        assert!(valid("hgt", "190cm"));
        assert!(valid("hgt", "76in"));
        assert!(valid("hgt", "60in"));
        assert!(!valid("hgt", "76inn"));
        assert!(!valid("hgt", "190"));
        assert!(!valid("hgt", "190in"));
        assert!(valid("hcl", "#aaaaaa"));
        assert!(valid("hcl", "#18171d"));
        assert!(valid("hcl", "#123abc"));
        assert!(valid("hcl", "#abc123"));
        assert!(!valid("hcl", "#aaaaaaa"));
        assert!(!valid("hcl", "#123abz"));
        assert!(!valid("hcl", "123abc"));
        assert!(valid("hcl", "#000aaa"));
        assert!(valid("ecl", "amb"));
        assert!(valid("ecl", "brn"));
        assert!(!valid("ecl", "bbb"));
        assert!(!valid("ecl", "wat"));
        assert!(valid("pid", "000000000"));
        assert!(valid("pid", "000000001"));
        assert!(!valid("pid", "00000000a"));
        assert!(!valid("pid", "0000000010"));
        assert!(!valid("pid", "0123456789"));
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 216);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 150);
    }
}
