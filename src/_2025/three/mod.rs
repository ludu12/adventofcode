pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> i64  {

    

    357
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
