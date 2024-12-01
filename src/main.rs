pub mod day1;

use std::fs;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day))
        .unwrap()
        .trim()
        .to_string()
}

fn main() {
    let input = read_input(1);

    println!("{}", day1::part1(&input));
    println!("{}", day1::part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day1::part1;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_day1() {
        let result = part1(INPUT);
        assert_eq!(result, 11);
    }
}
