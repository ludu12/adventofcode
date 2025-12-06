use crate::utils::{transpose};
use itertools::{max};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process_part1(input);
    println!("Part1: {}", part1.to_string());
    let part2 = process_part2(input);
    println!("Part2: {}", part2.to_string());
}

fn process_part1(input: &str) -> u64 {
    let mut values = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let operators = values.pop().unwrap();
    let numbers = transpose(
        values
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>(),
    );

    assert_eq!(numbers.len(), operators.len());

    let result = operators.iter().enumerate().fold(0, |acc, (i, op)| {
        let ints = &numbers[i];
        let res = match *op {
            "*" => ints.iter().product::<u64>(),
            "+" => ints.iter().sum::<u64>(),
            _ => panic!("Unknown operator {}", op),
        };

        acc + res
    });

    result
}

fn process_part2(input: &str) -> u64 {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let line_length = max(lines.iter().map(|line| line.len()).collect::<Vec<usize>>()).unwrap() + 1;

    let operators = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let line_chars: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut vec: Vec<char> = vec![' '; line_length];

            let chars = line.chars().collect::<Vec<char>>();
            vec.splice(0..chars.len(), chars);
            vec
        })
        .collect();

    let mut equations: Vec<Vec<u64>> = vec![vec![]];
    let mut equation_index = 0;

    for j in 0..line_length {
        let numbers = (0..line_chars.len())
            .map(|i| line_chars[i][j])
            .collect::<Vec<char>>();
        let n = String::from_iter(numbers)
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        if n == 0 {
            equation_index += 1;
            equations.push(vec![]);
        } else {
            equations[equation_index].push(n);
        }
    }

    let result = operators.iter().enumerate().fold(0, |acc, (i, op)| {
        let ints = &equations[i];
        let res = match *op {
            "*" => ints.iter().product::<u64>(),
            "+" => ints.iter().sum::<u64>(),
            _ => panic!("Unknown operator {}", op),
        };

        acc + res
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(4277556, process_part1(input));
    }

    #[test]
    fn part2() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(3263827, process_part2(input));
    }
}
