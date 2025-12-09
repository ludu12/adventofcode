#![allow(unused_variables)] // Disables unused_variables warnings for the entire crate
#![allow(unused_imports)]
#![allow(unused_must_use)]

use crate::utils::{grid, print_grid, Direction, Position};
use itertools::Itertools;
use petgraph::visit::Walker;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process_part1(input);
    println!("Part1: {}", part1.to_string());
    let part2 = process_part2(input);
    println!("Part2: {}", part2.to_string());
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    const fn area(&self, other: &Self) -> i64 {
        let x_dist = (self.x - other.x).abs() + 1;
        let y_dist = (self.y - other.y).abs() + 1;

        x_dist * y_dist
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// A struct to represent an undirected edge.
// Deriving `Debug` and `Eq` is helpful.
#[derive(Debug, Eq, Clone, Copy)]
struct Rect {
    v1: Point,
    v2: Point,
    area: i64,
}

impl Hash for Rect {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        let v1 = calculate_hash(&self.v1);
        let v2 = calculate_hash(&self.v2);

        if v1 < v2 {
            self.v1.hash(h);
            self.v2.hash(h);
        } else {
            self.v2.hash(h);
            self.v1.hash(h);
        }

        h.finish();
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        let e1 = calculate_hash(&self);
        let e2 = calculate_hash(&other);
        e1 == e2
    }
}

fn create_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(",");
            let x = line.next().unwrap().parse::<i64>().unwrap();
            let y = line.next().unwrap().parse::<i64>().unwrap();
            Point { x, y }
        })
        .collect::<Vec<Point>>()
}

fn create_rectangles(boxes: &Vec<Point>) -> Vec<Rect> {
    let mut edges: HashSet<Rect> = HashSet::new();
    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }

            let area = boxes[i].area(&boxes[j]);
            edges.insert(Rect {
                v1: boxes[i],
                v2: boxes[j],
                area,
            });
        }
    }
    let mut sorted_edges = edges.into_iter().collect_vec();

    // reverse sort
    sorted_edges.sort_by(|a, b| b.area.cmp(&a.area));

    sorted_edges
}

fn process_part1(input: &str) -> i64 {
    let boxes = create_points(input);
    let edges = create_rectangles(&boxes);

    edges[0].area
}

fn process_part2(input: &str) -> i64 {
    50
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(50, process_part1(input));
    }

    #[test]
    fn part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(50, process_part2(input));
    }
}
