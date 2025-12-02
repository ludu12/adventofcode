#![allow(unused_variables)] // Disables unused_variables warnings for the entire crate
pub mod one;
pub mod two;

pub fn run_day(day: u32) {
    match day {
        1 => one::run(),
        2 => two::run(),
        _ => println!("Invalid advent day! \"{}\"", day),
    }
}
