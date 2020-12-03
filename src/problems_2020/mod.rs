pub mod problem1;
pub mod problem2;
pub mod problem3;

use log::info;
pub fn solve() {
    let input1 = problem1::parse_input(problem1::INPUT_PATH);
    info!("Solution1.1: {:?}", problem1::solve_part1(&input1));
    info!("Solution1.2: {:?}", problem1::solve_part2(&input1));
    let input2 = problem2::parse_input(problem2::INPUT_PATH);
    info!("Solution1.1: {:?}", problem2::solve_part1(&input2));
    info!("Solution1.2: {:?}", problem2::solve_part2(&input2));
}
