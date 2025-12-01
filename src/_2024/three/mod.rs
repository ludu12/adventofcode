use regex::Regex;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> i32 {
    let mul_re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut doit = true;

    mul_re
        .captures_iter(input.trim())
        .map(|captures| {
            let instr = captures.get(0).unwrap().as_str();

            match instr {
                "do()" => doit = true,
                "don't()" => doit = false,
                _ => {
                    if !part2 || doit {
                        let l = captures[1].parse::<i32>().unwrap();
                        let r = captures[2].parse::<i32>().unwrap();
                        return l * r;
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(input, true));
    }
}
