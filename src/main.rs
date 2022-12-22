#![feature(step_trait)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(pin_macro)]
#![feature(result_option_inspect)]
#![feature(hash_drain_filter)]
#![feature(test)]
#![feature(variant_count)]


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
