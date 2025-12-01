use std::collections::HashMap;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> i32 {
    let count = input.lines().count();
    let mut left: Vec<i32> = Vec::with_capacity(count);
    let mut right: Vec<i32> = Vec::with_capacity(count);

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    });

    if !part2 {
        left.sort_unstable();
        right.sort_unstable();

        left.iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum()
    } else {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for r in right {
            *counts.entry(r).or_insert(0) += 1;
        }

        left.iter()
            .map(|l| l * counts.get(l).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(31, process(input, true));
    }
}
