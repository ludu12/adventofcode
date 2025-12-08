#![allow(unused_variables)] // Disables unused_variables warnings for the entire crate
pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;
pub mod eight;

pub fn run_day(day: u32) {
    match day {
        1 => one::run(),
        2 => two::run(),
        3 => three::run(),
        4 => four::run(),
        5 => five::run(),
        6 => six::run(),
        7 => seven::run(),
        8 => eight::run(),
        _ => println!("Invalid advent day! \"{}\"", day),
    }
}
