use crate::models::solution::run_day;

mod util;
mod days;
mod models;

fn main() {
    run_day::<days::day1::Day1>();
    run_day::<days::day2::Day2>();
}
