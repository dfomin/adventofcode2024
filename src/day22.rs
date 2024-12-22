use ahash::AHashMap;

fn process_secret(
    mut secret: i64,
    steps: usize,
    use_map: bool,
) -> (i64, AHashMap<(i64, i64, i64, i64), i64>) {
    let mut hash_map = AHashMap::new();
    let mut ones = [secret % 10; 4];
    for i in 0..steps {
        secret ^= secret * 64;
        secret %= 16777216;
        secret ^= secret / 32;
        secret %= 16777216;
        secret ^= secret * 2048;
        secret %= 16777216;

        let cur_ones = secret % 10;

        let tuple = (
            ones[(i + 2) % 4] - ones[(i + 1) % 4],
            ones[(i + 3) % 4] - ones[(i + 2) % 4],
            ones[i % 4] - ones[(i + 3) % 4],
            cur_ones - ones[i % 4],
        );

        ones[(i + 1) % 4] = cur_ones;

        if use_map && i >= 3 {
            if !hash_map.contains_key(&tuple) {
                hash_map.insert(tuple, cur_ones);
            }
        }
    }

    (secret, hash_map)
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(process_secret(line.trim().parse().unwrap(), 2000, false).0)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let maps = input
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(process_secret(line.trim().parse().unwrap(), 2000, true).1)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut result = AHashMap::new();
    let mut record = 0;
    for map in maps {
        for (key, value) in map {
            let entry = result.entry(key).or_default();
            *entry += value;
            record = record.max(*entry);
        }
    }
    record
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        1
        10
        100
        2024
        ";

    const INPUT2: &str = "
        1
        2
        3
        2024
    ";

    #[test]
    fn test_day21_part1() {
        assert_eq!(part1(INPUT1), 37327623);
    }

    #[test]
    fn test_day21_part2() {
        assert_eq!(part2(INPUT2), 23);
    }
}
