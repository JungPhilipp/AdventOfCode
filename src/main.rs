#![feature(step_trait)]
#![feature(vec_retain_mut)]
#![feature(destructuring_assignment)]

mod playground;
mod problems_2020;
mod problems_2021;
mod util;

fn main() {
    env_logger::init();
    problems_2020::solve();
    problems_2021::solve();
}
