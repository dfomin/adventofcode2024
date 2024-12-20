use ahash::AHashMap;

fn check(pattern: &str, parts: &[&str]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for part in parts {
        if part.len() <= pattern.len()
            && *part == &pattern[..part.len()]
            && check(&pattern[part.len()..], parts)
        {
            return true;
        }
    }
    false
}

fn count(pattern: &str, parts: &[&str], cache: &mut AHashMap<String, i64>) -> i64 {
    if pattern.is_empty() {
        return 1;
    }
    parts
        .iter()
        .filter(|&part| part.len() <= pattern.len() && *part == &pattern[..part.len()])
        .map(|part| {
            if let Some(&result) = cache.get(&pattern[part.len()..]) {
                return result;
            }
            let c = count(&pattern[part.len()..], parts, cache);
            cache.insert(pattern[part.len()..].to_string(), c);
            c
        })
        .sum()
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let parts = iter
        .next()
        .unwrap()
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.trim())
        .next()
        .unwrap()
        .split(", ")
        .collect::<Vec<_>>();
    let patterns = iter
        .next()
        .unwrap()
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| line.trim())
        .collect();
    (parts, patterns)
}

pub fn part1(input: &str) -> i64 {
    let (parts, patterns) = parse(input);
    patterns
        .into_iter()
        .filter(|&pattern| check(pattern, &parts))
        .count() as i64
}

pub fn part2(input: &str) -> i64 {
    let (parts, patterns) = parse(input);
    let mut cache: AHashMap<String, i64> = AHashMap::new();
    patterns
        .iter()
        .map(|&pattern| count(pattern, &parts, &mut cache))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        ";

    #[test]
    fn test_day19_part1() {
        assert_eq!(part1(INPUT), 6);
    }

    #[test]
    fn test_day19_part2() {
        assert_eq!(part2(INPUT), 16);
    }
}
