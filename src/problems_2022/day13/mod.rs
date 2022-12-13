use itertools::Itertools;
use log::{debug, info};

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

fn comp(lhs: &str, rhs: &str) -> Op {
    debug!("Compare: {} {}", lhs, rhs);

    match (lhs.parse::<usize>().ok(), rhs.parse::<usize>().ok()) {
        (None, None) => match (lhs, rhs) {
            _ if lhs == rhs => Op::Continue,
            ("[", "]") => Op::Ordered(false),
            ("]", "[") => Op::Ordered(true),
            ("]", ",") => Op::Ordered(true),
            (",", "]") => Op::Ordered(false),
            _ => unreachable!("lhs:({}), rhs({})", lhs, rhs),
        },
        (None, Some(_)) => {
            if lhs == "[" {
                Op::Retry(Expand::Right)
            } else {
                Op::Ordered(true)
            }
        }
        (Some(_), None) => {
            if rhs == "[" {
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
        let mut take_at_least_one = false;
        let mut last_was_digit = false;
        let a: String = lhs
            .chars()
            .skip(lhs_pos)
            .take_while(|&c| {
                let is_digit = c.is_ascii_digit();
                let result = !take_at_least_one || (last_was_digit && is_digit);
                last_was_digit = is_digit;
                take_at_least_one = true;
                result
            })
            .collect();
        take_at_least_one = false;
        last_was_digit = false;
        let b: String = rhs
            .chars()
            .skip(rhs_pos)
            .take_while(|&c| {
                let is_digit = c.is_ascii_digit();
                let result = !take_at_least_one || (last_was_digit && is_digit);
                last_was_digit = is_digit;
                take_at_least_one = true;
                result
            })
            .collect();

        match comp(a.as_str(), b.as_str()) {
            Op::Continue => {
                lhs_pos += 1;
                rhs_pos += 1;
            }
            Op::Ordered(result) => return result,
            Op::Retry(side) => match side {
                Expand::Left => {
                    lhs.replace_range(lhs_pos..lhs_pos + 1, format!("[{}]", a).as_str())
                }
                Expand::Right => {
                    rhs.replace_range(rhs_pos..rhs_pos + 1, format!("[{}]", b).as_str())
                }
            },
        };
    }
}

fn compare_packet(mut lhs: String, mut rhs: String) -> bool {
    compare(&mut lhs, 0, &mut rhs, 0)
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
        .map(|(index, (lhs, rhs))| {
            debug!("Pair: {}", index + 1);
            (index, compare_packet(lhs, rhs))
        })
        .filter_map(|(index, valid)| if valid { Some(index + 1) } else { None })
        .inspect(|index| info!("Valid pair at {}", index))
        .sum()
}

type Input2 = Vec<String>;
const DIVIDERS: [&str; 2] = ["[[2]]", "[[6]]"];

fn parse_part2(input: Input) -> Input2 {
    input
        .into_iter()
        .flat_map(|(first, second)| [first, second])
        .chain(DIVIDERS.into_iter().map(|i| i.to_string()))
        .collect_vec()
}
fn solve_part2(input: Input) -> usize {
    parse_part2(input)
        .into_iter()
        .sorted_by(|lhs, rhs| match compare_packet(lhs.clone(), rhs.clone()) {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        })
        .enumerate()
        .filter_map(|(index, e)| {
            if DIVIDERS.contains(&e.as_str()) {
                Some(index + 1)
            } else {
                None
            }
        })
        .product()
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
        assert!(compare_packet("1".to_string(), "2".to_string()));
        assert!(!compare_packet("2".to_string(), "1".to_string()));
        assert!(compare_packet(
            "[1,1,3,1,1]".to_string(),
            "[1,1,5,1,1]".to_string()
        ));
        assert!(compare_packet(
            "[[4,4],4,4]".to_string(),
            "[[4,4],4,4,4]".to_string()
        ));
        assert!(!compare_packet(
            "[7,7,7,7]".to_string(),
            "[7,7,7]".to_string()
        ));
        assert!(compare_packet("[]".to_string(), "[3]".to_string()));
        assert!(!compare_packet("[[[]]]".to_string(), "[[]]".to_string()));
        assert!(!compare_packet(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string()
        ));

        assert!(!compare_packet("[9]".to_string(), "[[8,7,6]]".to_string()));
        assert!(compare_packet(
            "[[1],[2,3,4]]".to_string(),
            "[[1],4]".to_string()
        ));
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_PATH!()))), 13);
    }
    #[test]
    fn example_1_custom() {
        assert!(compare_packet(
            "[[1],[2,3,4]]".to_string(),
            "[[1],4,5]".to_string()
        ));
        assert!(!compare_packet(
            "[[8,6,[[9,0,0],2,[6,3,2,0]],[],6],[10,[10,10,8,2],[[4,3,7],4,8],1,7]]".to_string(),
            "[[[],0,[[8,8,10,5,7],1,5,0],[]]]".to_string()
        ));
        assert!(compare_packet(
            "[[[],6]]".to_string(),
            "[[9,[],3,[9]],[],[[1],[10,9,4,[]],[2]],[]]".to_string()
        ));
        assert!(compare_packet("[[],[]]".to_string(), "[10]".to_string()));
        assert!(!compare_packet(
            "[[10,1],[]]".to_string(),
            "[10]".to_string()
        ));
        assert!(compare_packet("[8]".to_string(), "[[[8,7,6]]]".to_string()));
        assert!(!compare_packet(
            "[[10,3],[3,1,[3],8,5],[],[6,2,7,[10],[[0,8,9,10],5]]]".to_string(),
            "[[4],[0,1,[10]],[],[],[0,[[3],[7,7],8,[9,0,0],3],3,8]]".to_string()
        ));
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 5252);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(solve_part2(parse(include_str!(EXAMPLE_PATH!()))), 140);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(parse(include_str!(INPUT_PATH!()))), 20592);
    }
}
