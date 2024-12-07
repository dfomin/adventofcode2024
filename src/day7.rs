use rayon::prelude::*;

struct Equation {
    result: i64,
    numbers: Vec<(i64, i64)>,
}

impl Equation {
    fn solvable(&self, concat: bool) -> bool {
        self.backtrack(self.numbers[0].0, 1, concat)
    }

    fn backtrack(&self, cur: i64, index: usize, concat: bool) -> bool {
        if index == self.numbers.len() {
            return cur == self.result;
        }
        if self.backtrack(cur + self.numbers[index].0, index + 1, concat) {
            true
        } else if self.backtrack(cur * self.numbers[index].0, index + 1, concat) {
            true
        } else if concat {
            self.backtrack(
                cur * self.numbers[index].1 + self.numbers[index].0,
                index + 1,
                concat,
            )
        } else {
            false
        }
    }
}

fn parse_line(line: &str) -> Equation {
    let mut iter = line.split(": ");
    let result = iter.next().unwrap().parse().unwrap();
    let numbers = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| (x.parse().unwrap(), 10i64.pow(x.len() as u32) as i64))
        .collect();
    Equation { result, numbers }
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .par_bridge()
        .map(|x| x.trim())
        .map(|x| parse_line(x))
        .filter(|x| x.solvable(false))
        .map(|x| x.result)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .par_bridge()
        .map(|x| x.trim())
        .map(|x| parse_line(x))
        .filter(|x| x.solvable(true))
        .map(|x| x.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_day7_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(part2(INPUT), 11387);
    }
}
