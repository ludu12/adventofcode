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
    let part1 = process_part1(input, 1000);
    println!("Part1: {}", part1.to_string());
    let part2 = process_part2(input);
    println!("Part2: {}", part2.to_string());
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Point {
    const fn dist(&self, other: &Self) -> i64 {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        let z_dist = self.z - other.z;

        x_dist * x_dist + y_dist * y_dist + z_dist * z_dist
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
struct Edge {
    v1: Point,
    v2: Point,
    dist: i64,
}

impl Hash for Edge {
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

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let e1 = calculate_hash(&self);
        let e2 = calculate_hash(&other);
        e1 == e2
    }
}

fn create_boxes(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(",");
            let x = line.next().unwrap().parse::<i64>().unwrap();
            let y = line.next().unwrap().parse::<i64>().unwrap();
            let z = line.next().unwrap().parse::<i64>().unwrap();
            Point { x, y, z }
        })
        .collect::<Vec<Point>>()
}

fn create_edges(boxes: &Vec<Point>) -> Vec<Edge> {
    let mut edges: HashSet<Edge> = HashSet::new();
    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }

            let dist = boxes[i].dist(&boxes[j]);
            edges.insert(Edge {
                v1: boxes[i],
                v2: boxes[j],
                dist,
            });
        }
    }
    let mut sorted_edges = edges.into_iter().collect_vec();
    sorted_edges.sort_by(|a, b| a.dist.cmp(&b.dist));

    sorted_edges
}

fn process_part1(input: &str, stop: i64) -> u32 {
    let boxes = create_boxes(input);
    let edges = create_edges(&boxes);
    let mut circuits: Vec<Vec<Point>> = Vec::new();

    let mut count = 0;
    for edge in edges {
        if stop != -1 && count >= stop {
            break;
        }

        let mut c1_index: Option<usize> = None;
        let mut c2_index: Option<usize> = None;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&edge.v1) {
                c1_index = Some(i);
            }
            if circuit.contains(&edge.v2) {
                c2_index = Some(i);
            }
        }

        match (c1_index, c2_index) {
            (Some(i1), Some(i2)) => {
                // if not the same circuit, merge
                if i1 != i2 {
                    let mut c2 = circuits.remove(i2);

                    let i = if i1 < i2 { i1 } else { i1 - 1 };
                    let c1 = &mut circuits[i];
                    c1.append(&mut c2);
                }
            }
            (Some(i1), None) => {
                // add another
                let c1 = &mut circuits[i1];
                c1.push(edge.v2);
            }
            (None, Some(i2)) => {
                // add another
                let c2 = &mut circuits[i2];
                c2.push(edge.v1);
            }
            (None, None) => {
                // create new circuit
                circuits.push(vec![edge.v1, edge.v2]);
            }
        }
        count += 1;
    }

    let lens = circuits
        .into_iter()
        .map(|c| c.len() as u32)
        .collect_vec()
        .into_iter()
        .sorted()
        .rev()
        .collect_vec();
    lens[0..3].iter().fold(1u32, |acc, x| acc * x)
}

fn process_part2(input: &str) -> i64 {
    let boxes = create_boxes(input);
    let edges = create_edges(&boxes);
    let mut circuits: Vec<Vec<Point>> = Vec::new();

    let mut last_edge = None;

    for i in 0..edges.len() {
        let edge = edges[i];
        let mut c1_index: Option<usize> = None;
        let mut c2_index: Option<usize> = None;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&edge.v1) {
                c1_index = Some(i);
            }
            if circuit.contains(&edge.v2) {
                c2_index = Some(i);
            }
        }

        match (c1_index, c2_index) {
            (Some(i1), Some(i2)) => {
                // if not the same circuit, merge
                if i1 != i2 {
                    let mut c2 = circuits.remove(i2);

                    let i = if i1 < i2 { i1 } else { i1 - 1 };
                    let c1 = &mut circuits[i];
                    c1.append(&mut c2);
                }
            }
            (Some(i1), None) => {
                // add another
                let c1 = &mut circuits[i1];
                c1.push(edge.v2);
            }
            (None, Some(i2)) => {
                // add another
                let c2 = &mut circuits[i2];
                c2.push(edge.v1);
            }
            (None, None) => {
                // create new circuit
                circuits.push(vec![edge.v1, edge.v2]);
            }
        }
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            last_edge = Some(edge);
            break;
        }
    }

    let last = last_edge.unwrap();

    last.v1.x * last.v2.x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(40, process_part1(input, 10));
    }

    #[test]
    fn part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(25272, process_part2(input));
    }
}
