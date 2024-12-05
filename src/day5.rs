use std::cmp::Ordering;

use ahash::{AHashMap, AHashSet};

pub fn part1(input: &str) -> i32 {
    let mut pairs: AHashMap<i32, AHashSet<i32>> = AHashMap::new();
    let mut read_pairs = true;
    let mut result = 0;
    for line in input.lines().map(|x| x.trim()) {
        if line.is_empty() {
            read_pairs = false;
            continue;
        }

        if read_pairs {
            let pair = line
                .split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            pairs.entry(pair[0]).or_default().insert(pair[1]);
        } else {
            let update = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut ok = true;
            for i in 1..update.len() {
                if let Some(set) = pairs.get(&update[i]) {
                    for j in 0..i {
                        if set.contains(&update[j]) {
                            ok = false;
                            break;
                        }
                    }
                }
            }
            if ok {
                result += update[update.len() / 2];
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i32 {
    let mut pairs: AHashMap<i32, AHashSet<i32>> = AHashMap::new();
    let mut read_pairs = true;
    let mut result = 0;
    for line in input.lines().map(|x| x.trim()) {
        if line.is_empty() {
            read_pairs = false;
            continue;
        }

        if read_pairs {
            let pair = line
                .split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            pairs.entry(pair[0]).or_default().insert(pair[1]);
        } else {
            let mut update = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut ok = true;
            for i in 1..update.len() {
                if let Some(set) = pairs.get(&update[i]) {
                    for j in 0..i {
                        if set.contains(&update[j]) {
                            ok = false;
                            break;
                        }
                    }
                }
            }
            if !ok {
                update.sort_by(|a, b| {
                    if let Some(set) = pairs.get(a) {
                        return if set.contains(&b) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        };
                    }
                    Ordering::Greater
                });
                result += update[update.len() / 2];
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
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

    #[test]
    fn test_day5_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_day5_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
