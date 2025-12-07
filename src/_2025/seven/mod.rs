#![allow(unused_variables)] // Disables unused_variables warnings for the entire crate
#![allow(unused_imports)]

use crate::utils::{grid, print_grid, Direction, Position};
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process_part1(input);
    println!("Part1: {}", part1.to_string());
    let part2 = process_part2(input);
    println!("Part2: {}", part2.to_string());
}

fn traverse(tree: &Vec<Vec<char>>, position: Position, visited: &mut Vec<Position>) -> u32 {
    if visited.contains(&position) {
        // if it has been visited already, the beam has merged and therefore we don't want to count it again
        return 0;
    }

    visited.push(position);

    let new_position = position.go(Direction::South);

    let option = new_position.grid_value(&tree);
    match option {
        None => 0,
        Some(value) => {
            if value == '^' {
                let mut right = new_position.go(Direction::East);
                right.dir = Direction::South;
                let mut left = new_position.go(Direction::West);
                left.dir = Direction::South;

                return traverse(&tree, left, visited) + traverse(&tree, right, visited) + 1;
            }
            traverse(&tree, new_position, visited)
        }
    }
}

fn traverse_part2(
    tree: &Vec<Vec<char>>,
    position: Position,
    visited: &mut HashMap<Position, u64>,
) -> u64 {
    match visited.get(&position) {
        None => {
            let new_position = position.go(Direction::South);

            let option = new_position.grid_value(&tree);
            match option {
                None => 1,
                Some(value) => {
                    if value == '^' {
                        let mut right = new_position.go(Direction::East);
                        right.dir = Direction::South;
                        let mut left = new_position.go(Direction::West);
                        left.dir = Direction::South;

                        let total = traverse_part2(&tree, left, visited)
                            + traverse_part2(&tree, right, visited);

                        visited.insert(position, total);
                        return total;
                    }
                    traverse_part2(&tree, new_position, visited)
                }
            }
        }
        Some(v) => {
            *v
        }
    }
}

fn process_part1(input: &str) -> u32 {
    let tree = grid(input);

    let start = Position {
        x: ((tree[0].len() - 1) / 2) as i32,
        y: 0,
        dir: Direction::South,
    };

    assert_eq!('S', start.grid_value(&tree).unwrap());

    traverse(&tree, start, &mut Vec::new())
}

fn process_part2(input: &str) -> u64 {
    let tree = grid(input);

    let start = Position {
        x: ((tree[0].len() - 1) / 2) as i32,
        y: 0,
        dir: Direction::South,
    };

    assert_eq!('S', start.grid_value(&tree).unwrap());

    traverse_part2(&tree, start, &mut HashMap::new())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
";
        assert_eq!(3, process_part1(input));
    }

    #[test]
    fn part1b() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(21, process_part1(input));
    }

    #[test]
    fn part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(40, process_part2(input));
    }
}
