use crate::utils::{grid};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

#[derive(Debug)]
enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}
impl Direction {
    fn value(&self) -> (i8, i8) {
        match *self {
            Direction::NorthWest => (-1, -1),
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
        }
    }
    const VALUES: [Self; 8] = [
        Self::NorthWest,
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
    ];
}

fn add(u: usize, i: i8) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u8 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn found(map: &Vec<Vec<char>>, x: Option<usize>, y: Option<usize>, c: char) -> bool {
    match (x, y) {
        (Some(x), Some(y)) => {
            let i = Some(x).unwrap();
            let j = Some(y).unwrap();

            if i < map.len() && j < map[0].len() {
                let item = map[i][j];
                return item == c;
            }

            false
        }
        (_, _) => false,
    }
}

fn check_diagonal(
    map: &Vec<Vec<char>>,
    position: (usize, usize),
    diagonal: (Direction, Direction),
) -> bool {
    let (i, j) = position;
    let (from, to) = diagonal;

    let (dx_from, dy_from) = from.value();
    let (dx_to, dy_to) = to.value();

    // M - A - S
    let a = found(&map, add(i, dx_from), add(j, dy_from), 'M')
        && found(&map, add(i, dx_to), add(j, dy_to), 'S');

    // S - A - M
    let b = found(&map, add(i, dx_from), add(j, dy_from), 'S')
        && found(&map, add(i, dx_to), add(j, dy_to), 'M');

    a || b
}

fn process(input: &str, _part2: bool) -> i32 {
    let map = grid(input);

    let mut result = 0;

    if !_part2 {
        map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, char)| {
                // Depth First Search
                if char == &'X' {
                    for direction in Direction::VALUES {
                        let (d_x, d_y) = direction.value();
                        if found(&map, add(i, d_x), add(j, d_y), 'M')
                            && found(&map, add(i, d_x * 2), add(j, d_y * 2), 'A')
                            && found(&map, add(i, d_x * 3), add(j, d_y * 3), 'S')
                        {
                            result = result + 1;
                        }
                    }
                }
            })
        });
    } else {
        map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, char)| {
                // Depth First Search
                if char == &'A' {
                    // nw <-> se diagonal
                    let nw_se = check_diagonal(
                        &map,
                        (i, j),
                        (Direction::NorthWest, Direction::SouthEast)
                    );
                    // ne <-> sw diagonal
                    let ne_sw = check_diagonal(
                        &map,
                        (i, j),
                        (Direction::NorthEast, Direction::SouthWest)
                    );

                    if nw_se && ne_sw {
                        result = result + 1;
                    }
                }
            })
        });
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, process(input, true));
    }
}
