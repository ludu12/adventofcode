use itertools::Itertools;

#[allow(dead_code)]
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[allow(dead_code)]
pub fn transpose_clone<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let num_cols = v[0].len();
    (0..num_cols)
        .map(|col| v.iter().map(|row| row[col].clone()).collect())
        .collect()
}

pub fn grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

#[allow(dead_code)]
pub fn get_bounds(g: &Vec<Vec<char>>) -> (usize, usize) {
    (g[0].len(), g.len())
}

#[allow(dead_code)]
pub fn get_neighbor(x: usize, y: usize, dx: isize, dy: isize, width: usize, height: usize) -> Option<(usize, usize)> {
    let nx = x.wrapping_add_signed(dx);
    let ny = y.wrapping_add_signed(dy);

    if nx < width && ny < height {
        Some((nx, ny))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn get_neighbors(g: &Vec<Vec<char>>, x: usize, y: usize, empty: char) -> Vec<char> {
    let (width, height) = get_bounds(g);

    let mut my_vec: Vec<char> = Vec::with_capacity(8);
    for dx in -1isize..=1 {
        for dy in -1isize..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            match get_neighbor(x, y, dx, dy, width, height) {
                None => { my_vec.push(empty) },
                Some((nx, ny)) => { my_vec.push(g[nx][ny])}
            }
        }
    }
    my_vec
}

#[allow(dead_code)]
pub fn print_neighbors(n: &Vec<char>) {
    println!("{}{}{}", n[0], n[1], n[2]);
    println!("{}x{}", n[3], n[4]);
    println!("{}{}{}", n[5], n[6], n[7]);
}

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>())
    }
}

#[allow(dead_code)]
pub fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

#[allow(dead_code)]
pub fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn index(&self) -> usize {
        match *self {
            Direction::NorthWest => 0,
            Direction::North => 1,
            Direction::NorthEast => 2,
            Direction::East => 3,
            Direction::SouthEast => 4,
            Direction::South => 5,
            Direction::SouthWest => 6,
            Direction::West => 7,
        }
    }
    #[allow(dead_code)]
    pub fn value(&self) -> (i8, i8) {
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
    #[allow(dead_code)]
    pub fn turn(&self, angle: usize) -> Direction {
        let (quot, rem) = (angle / 45, angle % 45);
        if rem != 0 {
            panic!("Invalid angle: {}", angle);
        }
        let new_index = (self.index() + quot) % 8;
        Direction::VALUES[new_index]
    }
}


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub dir: Direction
}

impl Position {
    #[allow(dead_code)]
    pub fn is_valid(&self, max_x: usize, max_y: usize) -> bool {
        let p = self;
        p.x >= 0 && p.y >= 0 && p.x < max_x as i32 && p.y < max_y as i32
    }

    #[allow(dead_code)]
    pub fn go(&self, d: Direction) -> Position {
        let (d_y, d_x) = d.value();

        Position { x: self.x + d_x as i32, y: self.y + d_y as i32, dir: d }
    }
}
