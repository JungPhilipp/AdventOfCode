mod problem1;
mod util;

use log::info;

fn main() {
    env_logger::init();
    let input1 = problem1::parse_input(problem1::INPUT_PATH);
    info!("Solution1.1: {:?}", problem1::solve_part1(&input1));
    info!("Solution1.2: {:?}", problem1::solve_part2(&input1));
}
