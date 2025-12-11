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
use z3::ast::Int;
use z3::{Optimize, SatResult};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process_part1(input);
    println!("Part1: {}", part1.to_string());
    let part2 = process_part2(input);
    println!("Part2: {}", part2.to_string());
}


type Graph<'a>= HashMap<&'a str, Vec<&'a str>>;

fn parse_graph(input: &str) -> Graph<'_>{
    let mut grid: Graph = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let key = parts[0];
        let neighbors: Vec<&str> = parts[1].split_whitespace().collect();
        grid.entry(key).or_default().extend(neighbors);
    }
    grid
}

fn paths<'a>(graph: &Graph<'a>, start:  &'a str, end: &'a str, visted: &mut HashMap<(&'a str, &'a str), u64>) -> u64 {
    if start == end {
        return 1;
    }
    let key = (start, end);
    if visted.contains_key(&key) {
        return visted[&key];
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            total += paths(graph, neighbor, end, visted);
        }
    }

    visted.insert(key, total);

    total
}

fn process_part1(input: &str) -> u64 {
    let graph = parse_graph(input);

    paths(&graph, "you", "out", &mut HashMap::new())
}

fn process_part2(input: &str) -> u64 {
    let graph = parse_graph(input);

    let mut visted = HashMap::new();
    // here's one way to get to out:
    let svr_dac= paths(&graph, "svr", "dac", &mut visted);
    let dac_fft= paths(&graph, "dac", "fft", &mut visted);
    let fft_out= paths(&graph, "fft", "out", &mut visted);

    // Here's another
    let svr_fft= paths(&graph, "svr", "fft", &mut visted);
    let fft_dac= paths(&graph, "fft", "dac", &mut visted);
    let dac_out= paths(&graph, "dac", "out", &mut visted);


    // total combinations
    (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(5, process_part1(input));
    }

    #[test]
    fn part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(2, process_part2(input));
    }
}
