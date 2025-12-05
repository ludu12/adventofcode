use std::cmp;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> u64 {
    let (r, a) = input.split_once("\n\n").unwrap();


    if part2 {
        let mut ranges = r
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                let start: u64 = start.parse().unwrap();
                let end: u64 = end.parse().unwrap();
                (start, end)
            })
            .collect::<Vec<(u64, u64)>>();

        ranges.sort_by(|(s1, e1), (s2, e2)| s1.cmp(s2));

        let mut count = 0;
        while ranges.len() > 0 {
            let mut to_remove = vec![];

            let (min, mut max) = ranges[0];
            for i in 0..ranges.len() {
                let (s2, e2) = ranges[i];
                if s2 >= min && s2 <= max {
                    max = cmp::max(max, e2);
                    println!("Removing {}-{} inside {}-{}", s2, e2, min, max);
                    to_remove.push(i);
                }
                else {
                    break;
                }
            }

            count += max - min + 1;
            for index in to_remove.into_iter().rev() {
                ranges.remove(index);
            }
        }

        count
    }
    else {
        let ranges = r
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                let start: u64 = start.parse().unwrap();
                let end: u64 = end.parse().unwrap();
                (start, end)
            })
            .collect::<Vec<(u64, u64)>>();

        let available = a
            .lines()
            .map(|line| line.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        available.into_iter().fold(0u64, |sum, x| {
            for i in 0..ranges.len() {
                let (s,e) = ranges[i];
                if (s..=e).contains(&x) {
                    return sum + 1;
                }
            }

            sum
        })
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(3, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(14, process(input, true));
    }

    #[test]
    fn part2b() {
        let input = "3-5
10-14
16-20
2-6
12-18

1
5
8
11
17
32";
        assert_eq!(16, process(input, true));
    }
}
