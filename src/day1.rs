use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let (mut first, mut second) = input.lines().fold((vec![], vec![]), |mut acc, line| {
        let mut iter = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        acc.0.push(iter.next().unwrap());
        acc.1.push(iter.next().unwrap());
        acc
    });
    first.sort();
    second.sort();
    first
        .into_iter()
        .zip(second)
        .fold(0, |acc, (x, y)| acc + (x - y).abs())
}

pub fn part2(input: &str) -> i32 {
    let (first, second): (Vec<i32>, HashMap<i32, i32>) =
        input
            .lines()
            .fold((vec![], HashMap::new()), |mut acc, line| {
                let mut iter = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                acc.0.push(iter.next().unwrap());
                *acc.1.entry(iter.next().unwrap()).or_default() += 1;
                acc
            });
    first
        .into_iter()
        .fold(0, |acc, x| acc + x * second.get(&x).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(INPUT), 31);
    }
}
