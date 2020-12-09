use crate::util::parse::read_lines_with_separator;

pub static INPUT_PATH: &str = "src/problems_2020/problem8/input.txt";

pub fn parse_input(path_to_input: &str) -> Vec<(String, i32)> {
    read_lines_with_separator(path_to_input, ' ')
        .into_iter()
        .map(|line| {
            assert_eq!(line.len(), 2);
            (line[0].to_string(), line[1].parse::<i32>().unwrap())
        })
        .collect()
}

fn exec_instruction(instruction: &str, arg: i32, i_pointer: &mut i32, accumulator: &mut i32) {
    match instruction {
        "nop" => *i_pointer += 1,
        "acc" => {
            *accumulator += arg;
            *i_pointer += 1
        }
        "jmp" => *i_pointer += arg,
        _ => panic!("Unknown instruction {}", instruction),
    }
}

pub fn solve_part1(input: &Vec<(String, i32)>) -> i32 {
    let mut accumulator = 0;
    let mut ins_watch = vec![false; input.len()];
    let mut i: i32 = 0;
    while (i as usize) < input.len() {
        let (instruction, arg) = &input[i as usize];
        if ins_watch[i as usize] {
            return accumulator;
        } else {
            ins_watch[i as usize] = true;
        }

        exec_instruction(instruction, *arg, &mut i, &mut accumulator);
    }
    accumulator
}

pub fn solve_part2(input: &Vec<(String, i32)>) -> i32 {
    for j in 0..input.len() {
        let mut input_cpy = input.clone();
        if input_cpy[j].0 == "nop" {
            input_cpy[j].0 = "jmp".to_string();
        } else if input_cpy[j].0 == "jmp" {
            input_cpy[j].0 = "nop".to_string();
        }

        let mut accumulator = 0;
        let mut ins_watch = vec![false; input_cpy.len()];
        let mut i: i32 = 0;
        while (i as usize) < input_cpy.len() {
            let (instruction, arg) = &input_cpy[i as usize];
            if ins_watch[i as usize] {
                break;
            }
            ins_watch[i as usize] = true;

            exec_instruction(instruction, *arg, &mut i, &mut accumulator);
        }
        if (i as usize) == input_cpy.len() {
            return accumulator;
        }
    }
    panic!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&parse_input(INPUT_PATH)), 1614);
    }
    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&parse_input("src/problems_2020/problem8/input_test2.txt")),
            8
        );
        assert_eq!(solve_part2(&parse_input(INPUT_PATH)), 1260);
    }
}
