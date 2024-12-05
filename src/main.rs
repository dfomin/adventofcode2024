use adventofcode2024::{day1, day2, day3, day4, day5, read_input};

fn main() {
    let input = read_input(5);

    println!("{}", day5::part1(&input));
    println!("{}", day5::part2(&input));
}

#[cfg(test)]
mod tests {
    use adventofcode2024::day1::part1;

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
