use regex::Regex;

pub fn part1(input: &str) -> i32 {
    // let re = Regex::new(r"mul\((?<f>\d{1,3}),(?<s>\d{1,3}])\)").unwrap();
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            (
                caps.name("a").unwrap().as_str().parse::<i32>().unwrap(),
                caps.name("b").unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .fold(0, |acc, (a, b)| acc + a * b)
}

pub fn part2(input: &str) -> i32 {
    let re =
        Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)|(?<c>do)\(\)|(?<d>don't)\(\)").unwrap();

    let mut result = 0;
    let mut enabled = true;
    for caps in re.captures_iter(input) {
        if caps.name("c").is_some() {
            enabled = true;
        }
        if caps.name("d").is_some() {
            enabled = false;
        }
        if enabled {
            if let Some(a) = caps.name("a") {
                if let Some(b) = caps.name("b") {
                    result +=
                        a.as_str().parse::<i32>().unwrap() * b.as_str().parse::<i32>().unwrap();
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
