#![feature(step_trait)]

mod playground;
mod problems_2020;
mod problems_2021;
mod problems_2022;
mod util;

fn main() {
    env_logger::init();
    problems_2020::solve();
    problems_2021::solve();
    problems_2022::solve();
}
