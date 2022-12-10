use log::info;

macro_rules! INPUT_PATH {
    () => {
        "day10/input.txt"
    };
}

pub fn solve() {
    let input = include_str!(INPUT_PATH!());
    let parsed = parse(input);
    info!(
        "Solutions Day 10:\nPart1{}\nPart2{}",
        solve_part1(parsed.clone()),
        solve_part2(parsed)
    );
}

type Input = Vec<String>;

fn parse(input: &str) -> Input {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}
fn parse_addx(instruction: &str) -> i32 {
    instruction
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .expect("A number")
}

fn compute_register_values(input: Input) -> Vec<i32> {
    let mut register = 1;
    let mut values = vec![1, 1];
    values.extend(input.into_iter().flat_map(|instruction| {
        let mut register_values = vec![];
        match instruction.as_str() {
            "noop" => register_values.push(register),
            i if i.starts_with("addx") => {
                register_values.push(register);
                register += parse_addx(i);
                register_values.push(register);
            }
            unmatched => panic!("Not a valid instruction {}", unmatched),
        };
        register_values
    }));

    values
}

fn solve_part1(input: Input) -> i32 {
    compute_register_values(input)
        .into_iter()
        .enumerate()
        .filter_map(|(index, register)| {
            let cycle = index as i32;
            if (cycle - 20) % 40 == 0 && cycle > 0 && cycle <= 220 {
                let signal_strength = cycle * register;
                Some(signal_strength)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(input: Input) -> String {
    let screen = compute_register_values(input)
        .into_iter()
        .skip(1)
        .take(240)
        .enumerate()
        .map(|(cycle, value)| {
            let screen_vertical_pos = cycle as i32 % 40;
            let sprite_middle = value;
            let visible = (screen_vertical_pos - sprite_middle).abs() <= 1;

            let mut pixel = if screen_vertical_pos == 0 && cycle != 0 {
                String::from('\n')
            } else {
                String::new()
            };
            pixel.push(if visible { '#' } else { '.' });
            pixel
        })
        .collect::<String>();
    info!("\n{}", screen);
    screen
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    macro_rules! EXAMPLE_PATH {
        () => {
            "day10/example_1.txt"
        };
    }

    #[test]
    fn example_1() {
        assert_eq!(solve_part1(parse(include_str!(EXAMPLE_PATH!()))), 13140);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(parse(include_str!(INPUT_PATH!()))), 17840);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(
            solve_part2(parse(include_str!(EXAMPLE_PATH!()))),
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######....."
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(parse(include_str!(INPUT_PATH!()))),
            "####..##..#.....##..#..#.#....###...##..\n#....#..#.#....#..#.#..#.#....#..#.#..#.\n###..#..#.#....#....#..#.#....#..#.#....\n#....####.#....#.##.#..#.#....###..#.##.\n#....#..#.#....#..#.#..#.#....#....#..#.\n####.#..#.####..###..##..####.#.....###."
        );
    }
}
