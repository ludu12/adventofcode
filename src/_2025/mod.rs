pub mod one;

pub fn run_day(day: u32) {
    match day {
        1 => one::run(),
        _ => println!("Invalid advent day! \"{}\"", day),
    }
}
