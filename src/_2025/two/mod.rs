pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn all_elements_equal<T: PartialEq>(vec: &[T]) -> bool {
    if vec.is_empty() {
        return false;
    }

    let first_element = &vec[0];
    vec.iter().all(|element| element == first_element)
}

fn chunks(s: &str, chunk_size: usize) -> Vec<&str> {
    s.as_bytes()
        .chunks(chunk_size)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

fn check_repeats(s: &str, split_size: usize) -> bool {
    if s.len() % split_size != 0 {
        return false;
    }

    let chunk_size = s.len() / split_size;
    let vec = chunks(&s, chunk_size);
    all_elements_equal(&vec)
}

fn process(input: &str, part2: bool) -> i64  {
    input.trim().split(',').fold(0, |acc, pair| {
        let (s, e) = pair.split_once('-').unwrap();
        let n_start: i64 = s.parse().unwrap();
        let n_end: i64 = e.parse().unwrap();

        let mut count = 0;
        for i in n_start..=n_end {
            let s = i.to_string();

            if part2 {
                for x in 2..=s.len() {
                    if check_repeats(&s, x) {
                        count += i;
                        break;
                    }
                }
            } else {
                let len = s.len();
                if check_repeats(&s, 2) {
                    count += i
                }
            }
        }

        acc + count
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(4174379265, process(input, true));
    }
}
