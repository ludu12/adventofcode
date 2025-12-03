#![allow(unused_variables)] // Disables unused_variables warnings for the entire crate
pub mod one;
pub mod two;
pub mod three;

pub fn run_day(day: u32) {
    match day {
        1 => one::run(),
        2 => two::run(),
        3 => three::run(),
        _ => println!("Invalid advent day! \"{}\"", day),
    }
}
