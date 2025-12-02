pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> i32 {
    let mut lands_on_zero = 0;
    let mut clicks_on_zero = 0;
    let mut position = 50;

    input.lines().for_each(|line| {
        let mut s = line.chars();
        let direction = s.next().unwrap();
        let distance: i32 = s.collect::<String>().parse().unwrap();

        clicks_on_zero += distance / 100;

        let real_distance = distance % 100;
        let sign = if direction == 'L' { -1 } else { 1 };
        let mut new_position = position + sign * real_distance;

        let remainder = new_position % 100;
        new_position = if new_position < 0 { 100 + remainder } else { remainder };

        match direction {
            'L' => {
                if position < new_position && position != 0 {
                    clicks_on_zero += 1;
                }
            }
            'R' => {
                if position > new_position && new_position != 0 {
                    clicks_on_zero += 1;
                }
            }
            _ => panic!("Invalid direction"),
        }

        position = new_position;
        if position == 0 {
            clicks_on_zero += 1;
            lands_on_zero += 1;
        }
    });

    if part2 { clicks_on_zero } else {  lands_on_zero }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(3, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(6, process(input, true));
    }
}
