#![allow(warnings)]
mod problem1;
mod problem10;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;

use log::info;

pub fn solve() {
    let input1 = problem1::parse_input(problem1::INPUT_PATH);
    info!("Solution1.1: {:?}", problem1::solve_part1(&input1));
    info!("Solution1.2: {:?}", problem1::solve_part2(&input1));
    let input2 = problem2::parse_input(problem2::INPUT_PATH);
    info!("Solution2.1: {:?}", problem2::solve_part1(&input2));
    info!("Solution2.2: {:?}", problem2::solve_part2(&input2));
    let input3 = problem3::parse_input(problem3::INPUT_PATH);
    info!("Solution3.1: {:?}", problem3::solve_part1(&input3));
    info!("Solution3.2: {:?}", problem3::solve_part2(&input3));
    let input4 = problem4::parse_input(problem4::INPUT_PATH);
    info!("Solution4.1: {:?}", problem4::solve_part1(&input4));
    info!("Solution4.2: {:?}", problem4::solve_part2(&input4));
    let input5 = problem5::parse_input(problem5::INPUT_PATH);
    info!("Solution5.1: {:?}", problem5::solve_part1(&input5));
    info!("Solution5.2: {:?}", problem5::solve_part2(&input5));
    let input6 = problem6::parse_input(problem6::INPUT_PATH);
    info!("Solution6.1: {:?}", problem6::solve_part1(&input6));
    info!("Solution6.2: {:?}", problem6::solve_part2(&input6));
    let input7 = problem7::parse_input(problem7::INPUT_PATH);
    info!("Solution7.1: {:?}", problem7::solve_part1(&input7));
    info!("Solution7.2: {:?}", problem7::solve_part2(&input7));
    let input8 = problem8::parse_input(problem8::INPUT_PATH);
    info!("Solution8.1: {:?}", problem8::solve_part1(&input8));
    info!("Solution8.2: {:?}", problem8::solve_part2(&input8));
    let input9 = problem9::parse_input(problem9::INPUT_PATH);
    info!("Solution9.1: {:?}", problem9::solve_part1(&input9));
    info!("Solution9.2: {:?}", problem9::solve_part2(&input9));
    let input10 = problem10::parse_input(problem10::INPUT_PATH);
    info!("Solution10.1: {:?}", problem10::solve_part1(&input10));
    info!("Solution10.2: {:?}", problem10::solve_part2(&input10));
}
