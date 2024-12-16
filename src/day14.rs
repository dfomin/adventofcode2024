use regex::Regex;
use std::cmp::Ordering;

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn apply_steps(&self, steps: i32, width: i32, height: i32) -> (i32, i32) {
        (
            ((self.x + self.vx * steps) % width + width) % width,
            ((self.y + self.vy * steps) % height + height) % height,
        )
    }

    fn quadrant(x: i32, y: i32, width: i32, height: i32) -> Option<usize> {
        match (x.cmp(&(width / 2)), y.cmp(&(height / 2))) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            _ => None,
        }
    }
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(input)
        .map(|captures| {
            let values = captures
                .extract::<4>()
                .1
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            Robot {
                x: values[0],
                y: values[1],
                vx: values[2],
                vy: values[3],
            }
        })
        .collect()
}

fn simulate(robots: &[Robot], width: i32, height: i32, steps: i32) -> i32 {
    robots
        .iter()
        .fold([0; 4], |mut acc, robot| {
            let pos = robot.apply_steps(steps, width, height);
            if let Some(index) = Robot::quadrant(pos.0, pos.1, width, height) {
                acc[index] += 1;
            }
            acc
        })
        .iter()
        .product()
}

pub fn part1(input: &str) -> i32 {
    let robots = parse(input);
    simulate(&robots, 101, 103, 100)
}

pub fn part2(input: &str) -> i32 {
    let width = 101;
    let height = 103;
    let robots = parse(input);
    let (mut x, mut y) = ((1_000_000.0, 0), (1_000_000.0, 0));
    for i in 0..width.max(height) {
        let sums = robots.iter().fold((0, 0), |acc, robot| {
            let (x, y) = robot.apply_steps(i, width, height);
            (acc.0 + x, acc.1 + y)
        });
        let (mean_x, mean_y) = (
            sums.0 as f64 / robots.len() as f64,
            sums.1 as f64 / robots.len() as f64,
        );
        let vars = robots.iter().fold((0.0, 0.0), |acc, robot| {
            let (x, y) = robot.apply_steps(i, width, height);
            (
                acc.0 + (mean_x - x as f64) * (mean_x - x as f64),
                acc.1 + (mean_y - y as f64) * (mean_y - y as f64),
            )
        });
        if vars.0 < x.0 {
            x.0 = vars.0;
            x.1 = i;
        }
        if vars.1 < y.0 {
            y.0 = vars.1;
            y.1 = i;
        }
    }

    for i in 1..20000 {
        if i % width == x.1 && i % height == y.1 {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3";

    #[test]
    fn test_day14_part1() {
        let mut robots = parse(INPUT);
        assert_eq!(simulate(&mut robots, 11, 7, 100), 12);
    }

    #[test]
    fn test_day14_part2() {}
}
