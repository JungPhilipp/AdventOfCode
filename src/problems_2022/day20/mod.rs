use itertools::Itertools;
use log::info;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 20:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn mix(vec: &mut [i64], mut index: usize, mut dest: i64) {
    dest = dest.signum() * (dest.abs() % (vec.len() - 1) as i64);
    while dest != 0 {
        let new_index = {
            match index as i64 + dest.signum() {
                larger if larger >= vec.len() as i64 => larger as usize % vec.len(),
                smaller if smaller < 0 => (vec.len() as i64 + smaller) as usize,
                in_range => in_range as usize,
            }
        };
        vec.swap(index, new_index);
        index = new_index;
        dest -= dest.signum();
    }
}

fn mix_vec(input: &mut Input, tracking: &mut [i64]) {
    let count = input.len();
    for original_index in 0..count {
        let (index, _) = tracking
            .iter()
            .find_position(|i| **i == original_index as i64)
            .unwrap();
        let displacement = input[index];
        mix(input, index, displacement);
        mix(tracking, index, displacement);
    }
}

fn solve_part1(mut input: Input) -> i64 {
    let mut tracking = (0..input.len() as i64).collect_vec();

    mix_vec(&mut input, &mut tracking);

    let pos_0 = input.iter().find_position(|&&v| v == 0).unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| (pos_0.0 + offset) % input.len())
        .map(|index| input[index])
        .inspect(|e| info!("{e}"))
        .sum()
}

fn solve_part2(mut input: Input) -> i64 {
    let key = 811589153;
    input.iter_mut().for_each(|e| *e *= key);
    let mut tracking = (0..input.len() as i64).collect_vec();

    for _ in 0..10 {
        mix_vec(&mut input, &mut tracking);
    }

    let pos_0 = input.iter().find_position(|&&v| v == 0).unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| (pos_0.0 + offset) % input.len())
        .map(|index| input[index])
        .inspect(|e| info!("{e}"))
        .sum()
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
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 4267);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_part2(parse(include_str!(EXAMPLE_PATH!()))),
            1623178306
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse(include_str!(INPUT_PATH!()))),
            6871725358451
        );
    }
}
