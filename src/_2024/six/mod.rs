use crate::utils::{get_bounds, grid, Direction, Position};
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn get_start(g: &Vec<Vec<char>>) -> Position {
    for (y, line) in g.into_iter().enumerate() {
        for (x, char) in line.into_iter().enumerate() {
            match char {
                '^' => {
                    return Position {
                        x: x as i32,
                        y: y as i32,
                        dir: Direction::North,
                    }
                }
                _ => {}
            }
        }
    }
    panic!("Could not find starting point ^");
}

fn get_distinct_positions(
    g: &Vec<Vec<char>>,
    start: Position,
) -> Option<HashMap<(i32, i32), HashSet<Direction>>> {
    let (x, y) = get_bounds(&g);
    let mut position = start;

    let mut distinct: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
    while position.is_valid(x, y) {
        let set = distinct.entry((position.x, position.y)).or_default();

        if set.contains(&position.dir) {
            // we are in a loop
            return None;
        }
        set.insert(position.dir);

        let mut dir = position.dir;
        let mut next = position.go(position.dir);
        while next.is_valid(x, y) && g[next.y as usize][next.x as usize] == '#' {
            dir = dir.turn(90);
            next = position.go(dir);
        }
        position = next;
    }
    Some(distinct)
}

fn process(input: &str, part2: bool) -> usize {
    let mut g = grid(input);
    let start = get_start(&g);

    let map = get_distinct_positions(&g, start).unwrap();

    if part2 {
        let mut obstruction = 0;
        for (key, _value) in map.into_iter() {
            let x = key.0 as usize;
            let y = key.1 as usize;
            let temp = g[y][x];
            g[y][x] = '#';
            let check = get_distinct_positions(&g, start);
            match check {
                None => obstruction += 1,
                Some(_) => {}
            }
            g[y][x] = temp;
        }
        obstruction
    } else {
        map.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(6, process(input, true));
    }
}
