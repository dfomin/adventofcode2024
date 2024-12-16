fn main() {
    let mut day = 1;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day1::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day1::part2(&input)
    );

    day = 2;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day2::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day2::part2(&input)
    );

    day = 3;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day3::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day3::part2(&input)
    );

    day = 4;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day4::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day4::part2(&input)
    );

    day = 5;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day5::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day5::part2(&input)
    );

    day = 6;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day6::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day6::part2(&input)
    );

    day = 7;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day7::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day7::part2(&input)
    );

    day = 8;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day8::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day8::part2(&input)
    );

    day = 9;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day9::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day9::part2(&input)
    );

    day = 10;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day10::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day10::part2(&input)
    );

    day = 11;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day11::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day11::part2(&input)
    );

    day = 12;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day12::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day12::part2(&input)
    );

    day = 13;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day13::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day13::part2(&input)
    );

    day = 14;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day14::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day14::part2(&input)
    );

    day = 15;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day15::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day15::part2(&input)
    );

    day = 16;
    let input = adventofcode2024::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2024::day16::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2024::day16::part2(&input)
    );
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
