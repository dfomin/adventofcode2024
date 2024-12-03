pub fn part1(input: &str) -> i32 {
    let vectors = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut result = 0;
    for v in vectors {
        let mut ok = true;
        for i in 1..v.len() {
            if (v[i] - v[i - 1]).abs() < 1
                || (v[i] - v[i - 1]).abs() > 3
                || (v[1] - v[0]) * (v[i] - v[i - 1]) <= 0
            {
                ok = false;
                break;
            }
        }
        if ok {
            result += 1;
        }
    }
    result
}

pub fn part2(input: &str) -> i32 {
    let vectors = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut result = 0;
    for v in vectors {
        let mut ok = true;
        for i in 1..v.len() {
            if (v[i] - v[i - 1]).abs() < 1
                || (v[i] - v[i - 1]).abs() > 3
                || (v[1] - v[0]) * (v[i] - v[i - 1]) <= 0
            {
                ok = false;
                break;
            }
        }
        if ok {
            result += 1;
        } else {
            for j in 0..v.len() {
                let mut new_v = v.clone();
                new_v.remove(j);

                let mut ok = true;
                for i in 1..new_v.len() {
                    if (new_v[i] - new_v[i - 1]).abs() < 1
                        || (new_v[i] - new_v[i - 1]).abs() > 3
                        || (new_v[1] - new_v[0]) * (new_v[i] - new_v[i - 1]) <= 0
                    {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    result += 1;
                    break;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
