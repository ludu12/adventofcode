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

    for i in start..end {
        if numbers[i] > numbers[max] {
            max = i
        }
    }
    max
}

fn process(input: &str, part2: bool) -> u64  {

    let mut count = 0;
    input.lines().for_each(|line| {
        let s = line.trim();

        let numbers: Vec<u32> = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let total = if part2 { 12 } else { 2 };

        let mut leftmost = 0;
        for i in (0..total).rev() {
            let max = find_max_index(&numbers, leftmost, total - (total - i));
            leftmost = max + 1;
            let pow = 10_u64.pow(i as u32);
            count += (numbers[max] as u64) * pow;
        }
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
    fn part1b() {
        let input = "3722443164324852429541739322454443622742537425744313396455466849784737627295682866595242427454396354
2322233122227224822212323658212222255122122243112292222221242252322222312243522322124211222122322225";
        assert_eq!(194, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(3121910778619, process(input, true));
    }
}
