pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn is_valid(vec: &Vec<i32>) -> bool {
    let asc = vec.windows(2).all(|w| w[0] <= w[1]);
    let desc = vec.windows(2).all(|w| w[0] >= w[1]);
    if !asc && !desc {
        return false;
    }

    vec.windows(2)
        .map(|w| w[1] - w[0])
        .all(|x| x.abs() >= 1 && x.abs() <= 3)
}

fn process(input: &str, part2: bool) -> i32 {
    input
        .lines()
        .map(|line| {
            let vec = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let valid = is_valid(&vec);
            match valid {
                true => 1,
                false => {
                    return if part2 {
                        match (0..vec.len()).any(|i| {
                            let mut tmp = vec.clone();
                            tmp.remove(i);
                            is_valid(&tmp)
                        }) {
                            true => 1,
                            false => 0,
                        }
                    } else {
                        0
                    }
                },
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(2, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(4, process(input, true));
    }
}
