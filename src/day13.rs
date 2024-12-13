use regex::Regex;

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Machine {
    fn price(&self) -> Option<i64> {
        let d = self.ax * self.by - self.ay * self.bx;
        if d == 0 {
            if self.px % self.ax == 0 && self.ay * (self.px / self.ax) == self.py {
                Some((3 * self.px / self.ax).min(self.py / self.ay))
            } else {
                None
            }
        } else {
            let a = (self.px * self.by - self.py * self.bx) / d;
            let b = (self.ax * self.py - self.ay * self.px) / d;
            if self.ax * a + self.bx * b == self.px && self.ay * a + self.by * b == self.py {
                Some(3 * a + b)
            } else {
                None
            }
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    re.captures_iter(input)
        .map(|captures| {
            let values = captures
                .extract::<6>()
                .1
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            Machine {
                ax: values[0],
                ay: values[1],
                bx: values[2],
                by: values[3],
                px: values[4],
                py: values[5],
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|machine| machine.price())
        .filter_map(|price| price)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    parse(input)
        .iter_mut()
        .map(|machine| {
            machine.px += 10_000_000_000_000;
            machine.py += 10_000_000_000_000;
            machine.price()
        })
        .filter_map(|price| price)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279";

    #[test]
    fn test_day13_part1() {
        assert_eq!(part1(INPUT), 480);
    }

    #[test]
    fn test_day13_part2() {
        assert_eq!(part2(INPUT), 875318608908);
    }
}
