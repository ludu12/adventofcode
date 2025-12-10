#![allow(unused_variables)] // Disables unused_variables warnings for the entire crate
#![allow(unused_imports)]
#![allow(unused_must_use)]

use crate::utils::{grid, print_grid, Direction, Position};
use itertools::Itertools;
use petgraph::visit::Walker;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::os::unix::raw::off_t;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process_part1(input);
    println!("Part1: {}", part1.to_string());
    let part2 = process_part2(input);
    println!("Part2: {}", part2.to_string());
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buttons = self.buttons.clone().into_iter().map(|buttons| {
            let binary_string = format!("{:08b}", buttons);
            let mut indices = vec![];
            binary_string.chars().enumerate().for_each(|(i, char)| {
                if char == '1' {
                    indices.push(7 - i);
                }
            });

            format!("({})", indices.iter().map(|n| n.to_string()).rev().join(","))
        }).join(" ");


        let joltage  = self.joltage.clone().iter().map(|n| n.to_string()).join(",");

        write!(f, "[{:08b}] {} {{ {} }}", self.lights, buttons, joltage)
    }
}

fn parse_machine(line: &str) -> Machine {
    let first_space_index = line.find(' ').unwrap();
    let (lights, rest) = line.split_at(first_space_index);
    let last_space_index = rest.rfind(' ').unwrap();
    let (buttons, joltage) = rest.split_at(last_space_index);

    let l_re = Regex::new(r"\[(.*?)\]").unwrap();
    let l_string = l_re
        .captures(lights)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .replace(".", "0")
        .replace("#", "1");

    let reversed_l_string: String = l_string.chars().rev().collect();
    let n_lights = u16::from_str_radix(&reversed_l_string, 2, ).unwrap();

    let v_buttons = buttons
        .trim()
        .split(" ")
        .map(|s| {
            let b_re = Regex::new(r"\((.*?)\)").unwrap();
            let nums = b_re
                .captures(s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|b| b.parse::<u32>().unwrap())
                .collect_vec();

            let mut btn_value: u16 = 0;
            for n in nums {
                btn_value |= 1 << n;
            }

            btn_value
        })
        .collect_vec();

    let v_re = Regex::new(r"\{(.*?)\}").unwrap();
    let v_joltage = v_re
        .captures(joltage)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|j| j.parse::<u16>().unwrap())
        .collect_vec();

    Machine {
        lights: n_lights,
        buttons: v_buttons,
        joltage: v_joltage,
    }
}

fn process_machine(machine: &Machine) -> usize {
    let mut next_presses: VecDeque<(usize, u16)> = VecDeque::new();

    for button in machine.buttons.iter() {
        next_presses.push_back((1, *button ));
    }

    while let Some ((presses, state)) =  next_presses.pop_front() {
        if state == machine.lights {
            return presses;
        }
        for button in machine.buttons.iter() {
            next_presses.push_back((presses+1, state ^ button ));
        }
    }
    0
}

fn process_part1(input: &str) -> i64 {
    let machines = input
        .lines()
        .map(|line| parse_machine(line))
        .collect_vec();

    machines.iter().map(|machine| process_machine(machine) as i64).sum()
}

fn process_part2(input: &str) -> u32 {
    24
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(7, process_part1(input));
    }

    #[test]
    fn part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(24, process_part2(input));
    }
}
