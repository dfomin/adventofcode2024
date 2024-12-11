use ahash::AHashMap;

fn count(cache: &mut AHashMap<(i64, i64), i64>, number: i64, blinks: i64) -> i64 {
    if let Some(&cached_result) = cache.get(&(number, blinks)) {
        return cached_result;
    }

    let result = if blinks == 0 {
        1
    } else if number == 0 {
        count(cache, 1, blinks - 1)
    } else {
        let digits = count_digits(number);
        if digits % 2 == 0 {
            let div = 10i64.pow(digits as u32 / 2);
            count(cache, number / div, blinks - 1) + count(cache, number % div, blinks - 1)
        } else {
            count(cache, number * 2024, blinks - 1)
        }
    };
    cache.insert((number, blinks), result);
    result
}

fn count_digits(number: i64) -> i64 {
    let mut digits = 1;
    let mut zeros = 10;
    while zeros <= number {
        digits += 1;
        zeros *= 10;
    }
    digits
}

pub fn part1(input: &str) -> i64 {
    let mut cache: AHashMap<(i64, i64), i64> = AHashMap::new();
    input
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .map(|number| count(&mut cache, number, 25))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut cache: AHashMap<(i64, i64), i64> = AHashMap::new();
    input
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .map(|number| count(&mut cache, number, 75))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_day11_part1() {
        assert_eq!(part1(INPUT), 55312);
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
