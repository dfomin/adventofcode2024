use regex::Regex;
use std::cmp::Ordering;

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn update(&mut self, width: i32, height: i32) {
        self.x = (self.x + self.vx + width) % width;
        self.y = (self.y + self.vy + height) % height;
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<usize> {
        match (self.x.cmp(&(width / 2)), self.y.cmp(&(height / 2))) {
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

fn simulate(robots: &mut Vec<Robot>, width: i32, height: i32, steps: i32) -> i32 {
    _ = (0..steps)
        .map(|_| {
            robots
                .iter_mut()
                .for_each(|robot| robot.update(width, height));
        })
        .collect::<()>();
    robots
        .iter()
        .fold([0; 4], |mut acc, robot| {
            if let Some(index) = robot.quadrant(width, height) {
                acc[index] += 1;
            }
            acc
        })
        .iter()
        .product()
}

fn dfs(field: &mut [Vec<u32>], x: usize, y: usize) -> i32 {
    field[y][x] = 2;
    let mut result = 1;
    for shift in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = x as i32 + shift.0;
        let new_y = y as i32 + shift.1;
        if new_x >= 0
            && new_x < field[0].len() as i32
            && new_y >= 0
            && new_y < field.len() as i32
            && field[new_y as usize][new_x as usize] == 1
        {
            result += dfs(field, new_x as usize, new_y as usize);
        }
    }
    result
}

fn find_clusters(robots: &[Robot], width: i32, height: i32) -> i32 {
    let mut field = vec![vec![0; width as usize]; height as usize];
    for robot in robots {
        field[robot.y as usize][robot.x as usize] = 1;
    }
    let mut result = 0;
    for i in 0..height as usize {
        for j in 0..width as usize {
            if field[i][j] == 1 {
                result = result.max(dfs(&mut field, j, i));
            }
        }
    }
    result
}

pub fn part1(input: &str) -> i32 {
    let mut robots = parse(input);
    simulate(&mut robots, 101, 103, 100)
}

pub fn part2(input: &str) -> i64 {
    let width = 101;
    let height = 103;
    let mut robots = parse(input);
    for i in 1..20000 {
        simulate(&mut robots, width, height, 1);
        let cluster = find_clusters(&robots, width, height);
        if cluster == 229 {
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
