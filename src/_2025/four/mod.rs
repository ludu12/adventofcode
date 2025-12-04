#![allow(unused_imports)]
use crate::utils::{get_bounds, get_neighbors, grid, print_grid, print_neighbors};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn find_locations(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let (x_bound, y_bound) = get_bounds(&map);

    let mut locations: Vec<(usize, usize)> = Vec::new();
    for x in 0..x_bound {
        for y in 0..y_bound {
            let c = map[x][y];
            if c == '@' {
                let n = get_neighbors(&map, x, y, ' ');
                let num = n.into_iter().filter(|n| *n == c).count();
                if num < 4 {
                    locations.push((x, y));
                }
            }
        }
    }

    locations
}

fn replace_locations(map: &mut Vec<Vec<char>>, locations: &Vec<(usize, usize)>, replacement: char) {
    for (x, y) in locations {
        map[*x][*y] = replacement;
    }
}

fn process(input: &str, part2: bool) -> usize {
    let mut map = grid(input);

    if part2 {
        let mut count = 0;

        loop {
            let locations = find_locations(&map);
            if locations.len() == 0 {
                break;
            }
            count += locations.len();
            replace_locations(&mut map, &locations, '.');
        }
        return count;
    }


    let locations = find_locations(&map);

    locations.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(13, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(43, process(input, true));
    }
}
