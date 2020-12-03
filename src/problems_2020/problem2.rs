use crate::util::parse::read_lines_with_separator;

pub static INPUT_PATH: &str = "src/problems_2020/problem2/input.txt.txt";

pub fn parse_input(path_to_input: &str) -> Vec<(usize, usize, char, String)> {
    let split_lines = read_lines_with_separator(path_to_input, ':');
    let mut result = vec![];
    for line in split_lines {
        assert_eq!(line.len(), 2);
        let parts: Vec<&str> = line[0].split(|c| c == ' ' || c == '-').collect();
        let min = parts[0].parse::<usize>().unwrap();
        let max = parts[1].parse::<usize>().unwrap();
        let c = parts[2].parse::<char>().unwrap();
        result.push((min, max, c, line[1].clone()));
    }
    result
}
pub fn solve_part1(input: &Vec<(usize, usize, char, String)>) -> usize {
    input
        .into_iter()
        .filter(|(min, max, c, s)| {
            let count = s.chars().filter(|ch| ch == c).count();
            *min <= count && count <= *max
        })
        .count()
}

pub fn solve_part2(input: &Vec<(usize, usize, char, String)>) -> usize {
    input
        .into_iter()
        .filter(|(first, second, c, s)| {
            (s.chars().nth(*first).unwrap() == *c) ^ (s.chars().nth(*second).unwrap() == *c)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 564);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 325);
    }
}
