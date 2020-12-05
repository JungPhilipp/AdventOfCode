mod problem1;
mod problem2;
mod problem3;
mod problem4;

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
}
