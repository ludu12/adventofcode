use itertools::Itertools;

pub fn run() {
    let input = include_str!("input.txt");
    // let part1 = process(input, false);
    // println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn get_mid(rules: &Vec<Vec<usize>>, update: &Vec<usize>) -> usize {
    for i in 0..update.len() - 1 {
        let vec = &rules[update[i]];
        let page_num = &update[i + 1];

        if !vec.contains(page_num) {
            return 0;
        }
    }

    update[(update.len() - 1) / 2]
}

fn process(input: &str, _part2: bool) -> usize {
    let (r, u) = input.split_once("\n\n").unwrap();

    let mut rules = vec![Vec::new(); 100];

    r.lines().into_iter().for_each(|rule| {
        let (k, v) = rule.split_once('|').unwrap();
        let key = k.parse::<usize>().unwrap();
        let value = v.parse::<usize>().unwrap();
        rules[key].push(value);
    });

    let mut sum = 0;

    let mut line = 0;

    u.lines().into_iter().for_each(|x| {
        let updates = x
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if _part2 {
            if get_mid(&rules, &updates) == 0 {
                let mut n = 0;
                let perms = updates.iter().cloned().permutations(updates.len());
                let perm_len = perms.try_len().unwrap();
                for perm in perms {
                    n += 1;
                    let mid = get_mid(&rules, &perm);

                    println!("Line: {} --- permutation: {} / {} --- {}", line, n, perm_len, mid);
                    if mid != 0 {
                        sum += mid;
                        break;
                    }
                }
            }
            line += 1;
        } else {
            let mid = get_mid(&rules, &updates);
            sum += mid;
        }
    });

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(123 , process(input, true));
    }
}
