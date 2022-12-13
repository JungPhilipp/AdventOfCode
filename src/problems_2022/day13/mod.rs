use itertools::Itertools;
use log::{debug, info};
use primes::is_prime;

macro_rules! INPUT_PATH {
    () => {
        "input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 11:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<(String, String)>;

enum Expand {
    Left,
    Right,
}
enum Op {
    Continue,
    Ordered(bool),
    Retry(Expand),
}

fn compare_char(lhs: char, rhs: char) -> Op {
    debug!("Compare: {} {}", lhs, rhs);
    assert!(lhs.is_ascii_digit() || lhs == '[' || lhs == ']' || lhs == ',');
    assert!(rhs.is_ascii_digit() || rhs == '[' || rhs == ']' || rhs == ',');

    match (lhs.to_digit(10), rhs.to_digit(10)) {
        (None, None) => match (lhs, rhs) {
            _ if lhs == rhs => Op::Continue,
            ('[', ']') => Op::Ordered(false),
            (']', '[') => Op::Ordered(true),
            (']', ',') => Op::Ordered(true),
            (',', ']') => Op::Ordered(false),
            _ => unreachable!("lhs:({}), rhs({})", lhs, rhs),
        },
        (None, Some(_)) => {
            if lhs == '[' {
                Op::Retry(Expand::Right)
            } else {
                Op::Ordered(true)
            }
        }
        (Some(_), None) => {
            if rhs == '[' {
                Op::Retry(Expand::Left)
            } else {
                Op::Ordered(false)
            }
        }
        (Some(a), Some(b)) => match a.cmp(&b) {
            std::cmp::Ordering::Less => Op::Ordered(true),
            std::cmp::Ordering::Equal => Op::Continue,
            std::cmp::Ordering::Greater => Op::Ordered(false),
        },
    }
}

fn compare(lhs: &mut String, mut lhs_pos: usize, rhs: &mut String, mut rhs_pos: usize) -> bool {
    loop {
        debug!("Pair:\n{}\n{}", lhs, rhs);
        let a = lhs.chars().nth(lhs_pos).expect("Should have next");
        let b = rhs.chars().nth(rhs_pos).expect("Should have next");

        match compare_char(a, b) {
            Op::Continue => {
                lhs_pos += 1;
                rhs_pos += 1;
            }
            Op::Ordered(result) => return result,
            Op::Retry(side) => match side {
                Expand::Left => {
                    assert!(a.is_ascii_digit());
                    lhs.replace_range(lhs_pos..lhs_pos + 1, format!("[{}]", a).as_str())
                }
                Expand::Right => {
                    assert!(b.is_ascii_digit());
                    rhs.replace_range(rhs_pos..rhs_pos + 1, format!("[{}]", b).as_str())
                }
            },
        };
    }
}

fn compare_packet(lhs: &mut String, rhs: &mut String) -> bool {
    compare(lhs, 0, rhs, 0)
}

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|group| group.trim())
        .filter(|group| !group.is_empty())
        .map(|group| {
            group
                .split('\n')
                .map(|line| line.chars().collect())
                .collect_tuple()
                .expect("Should contain two packets")
        })
        .collect_vec()
}

fn solve_part1(input: Input) -> usize {
    input
        .into_iter()
        .enumerate()
        .map(|(index, (mut lhs, mut rhs))| {
            debug!("Pair: {}", index);
            (index, compare_packet(&mut lhs, &mut rhs))
        })
        .filter_map(|(index, valid)| if valid { Some(index + 1) } else { None })
        .inspect(|index| info!("Valid pair at {}", index))
        .sum()
}

fn solve_part2(input: Input) -> usize {
    0
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
        assert!(compare_packet(&mut "1".to_string(), &mut "2".to_string()));
        assert!(!compare_packet(&mut "2".to_string(), &mut "1".to_string()));
        assert!(compare_packet(
            &mut "[1,1,3,1,1]".to_string(),
            &mut "[1,1,5,1,1]".to_string()
        ));
        assert!(compare_packet(
            &mut "[[4,4],4,4]".to_string(),
            &mut "[[4,4],4,4,4]".to_string()
        ));
        assert!(!compare_packet(
            &mut "[7,7,7,7]".to_string(),
            &mut "[7,7,7]".to_string()
        ));
        assert!(compare_packet(
            &mut "[]".to_string(),
            &mut "[3]".to_string()
        ));
        assert!(!compare_packet(
            &mut "[[[]]]".to_string(),
            &mut "[[]]".to_string()
        ));
        assert!(!compare_packet(
            &mut "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
            &mut "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string()
        ));

        assert!(!compare_packet(
            &mut "[9]".to_string(),
            &mut "[[8,7,6]]".to_string()
        ));
        assert!(compare_packet(
            &mut "[[1],[2,3,4]]".to_string(),
            &mut "[[1],4]".to_string()
        ));
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_PATH!()))), 13);
    }
    #[test]
    fn example_1_custom() {
        assert!(compare_packet(
            &mut "[[1],[2,3,4]]".to_string(),
            &mut "[[1],4,5]".to_string()
        ));
        assert!(!compare_packet(
            &mut "[[8,6,[[9,0,0],2,[6,3,2,0]],[],6],[10,[10,10,8,2],[[4,3,7],4,8],1,7]]"
                .to_string(),
            &mut "[[[],0,[[8,8,10,5,7],1,5,0],[]]]".to_string()
        ));
        assert!(compare_packet(
            &mut "[[[],6]]".to_string(),
            &mut "[[9,[],3,[9]],[],[[1],[10,9,4,[]],[2]],[]]".to_string()
        ));
        assert!(compare_packet(
            &mut "[[],[]]".to_string(),
            &mut "[10]".to_string()
        ));
        assert!(!compare_packet(
            &mut "[[10,1],[]]".to_string(),
            &mut "[10]".to_string()
        ));
        assert!(compare_packet(
            &mut "[8]".to_string(),
            &mut "[[[8,7,6]]]".to_string()
        ));
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 5232);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 0);
    }
}
