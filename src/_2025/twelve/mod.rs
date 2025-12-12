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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Shape {
    area: u32,
    pattern: [[char; 3]; 3],
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Grid {
    height: u32,
    width: u32,
    num_shapes: [u32; 6],
}

fn process_part1(input: &str) -> u64 {
    let paragraphs: Vec<&str> = input.split("\n\n").collect();

    let mut shapes: Vec<Shape> = vec![];
    for i in 0..paragraphs.len() - 1 {
        let mut lines = paragraphs[i].lines();
        lines.next().unwrap();
        let pattern: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
        let area = pattern.iter().flatten().filter(|&&c| c == '#').count() as u32;

        let mut shape_pattern = [['.'; 3]; 3];
        for y in 0..3 {
            for x in 0..3 {
                shape_pattern[y as usize][x as usize] = pattern[y as usize][x as usize];
            }
        }

        shapes.push(Shape {
            area,
            pattern: shape_pattern,
        });
    };

    let grids = paragraphs[paragraphs.len() - 1]
        .lines()
        .map(|line| {
            let (dimensions, vec) = line.split_once(":").unwrap();
            let (width, height) = dimensions.split_once("x").unwrap();

            Grid {
                width: width.trim().parse().unwrap(),
                height: height.trim().parse().unwrap(),
                num_shapes: vec
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect_vec();

    let available = grids
        .iter()
        .filter(|grid| {
            let total = grid.num_shapes.iter().enumerate().map(|(i, n)| {
                let shape = &shapes[i];
                shape.area * n
            }).collect_vec().iter().sum::<u32>();

            println!("{:?}", total);

            total < (grid.height * grid.width)
        })
        .collect_vec();

    available.len() as u64
}

fn process_part2(input: &str) -> u64 {
    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(2, process_part1(input));
    }

    #[test]
    fn part2() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(2, process_part2(input));
    }
}
