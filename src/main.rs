use std::env;

mod utils;
mod _2024;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <year> <day>", args[0]);
        return;
    }

    let year:  u32 = args[1].parse::<u32>().expect("Year input should be a number");;
    let day:  u32 = args[2].parse::<u32>().expect("Day input should be a number");;

    println!("Running Advent of Code {}. Day {}\n", year.to_string(), day.to_string());

    match year {
        2024 => _2024::run_day(day),
        _ => println!("Invalid advent year! \"{}\"", year),
    }
}

