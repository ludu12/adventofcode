pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn find_max_index(numbers: &Vec<u32>, left: usize, right: usize) -> usize {
    let mut max = 0 + left;

    let start = 0 + left;
    let end = numbers.len() - right;

    for i in (start..end).rev() {
        if numbers[i] > numbers[max] {
            max = i
        }
    }
    max
}

fn process(input: &str, part2: bool) -> u32  {

    let mut count = 0;
    input.lines().for_each(|line| {
        let s = line.trim();

        let numbers: Vec<u32> = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let first = find_max_index(&numbers, 0, 1);
        let second = find_max_index(&numbers, first + 1, 0);

        count += numbers[first] * 10 + numbers[second];

    });

    count
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(357, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(357, process(input, true));
    }
}
