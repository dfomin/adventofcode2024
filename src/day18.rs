use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn heuristic(&self, d: i64, width: usize, height: usize) -> i64 {
        -(d + (width - self.x + height - self.y) as i64)
    }

    fn neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        let mut result = Vec::with_capacity(4);
        if self.x > 0 {
            result.push(Position {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            result.push(Position {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width - 1 {
            result.push(Position {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height - 1 {
            result.push(Position {
                x: self.x,
                y: self.y + 1,
            });
        }
        result
    }
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| (x[0], x[1]))
        .collect()
}

fn solve(obstacles: &[(usize, usize)], width: usize, height: usize) -> i64 {
    let mut field = vec![vec![false; width]; height];
    for (i, o) in obstacles.iter().enumerate() {
        field[o.1][o.0] = true;
    }
    let mut visited = vec![vec![i64::MAX; width]; height];
    let mut heap: BinaryHeap<(i64, i64, Position)> = BinaryHeap::with_capacity(1024);
    let start = Position { x: 0, y: 0 };
    heap.push((start.heuristic(0, width, height), 0, start));
    let mut result = i64::MAX;
    while let Some((_, d, position)) = heap.pop() {
        if d >= result {
            continue;
        }

        if position.x == width - 1 && position.y == height - 1 {
            result = result.min(d);
            continue;
        }

        if visited[position.y][position.x] <= d {
            continue;
        }

        visited[position.y][position.x] = d;

        for neighbor in position.neighbors(width, height) {
            if visited[neighbor.y][neighbor.x] > d + 1 && !field[neighbor.y][neighbor.x] {
                heap.push((neighbor.heuristic(d + 1, width, height), d + 1, neighbor));
            }
        }
    }
    result
}

fn find_block(input: &str, width: usize, height: usize) -> String {
    let obstacles = parse(input);
    let (mut i, mut j) = (0, obstacles.len() - 1);
    while i < j - 1 {
        let m = (i + j) / 2;
        if solve(&obstacles[..m], width, height) < i64::MAX {
            i = m;
        } else {
            j = m;
        }
    }
    format!("{},{}", obstacles[j - 1].0, obstacles[j - 1].1).to_string()
}

pub fn part1(input: &str) -> i64 {
    let obstacles = parse(input);
    solve(&obstacles[..1024], 71, 71)
}

pub fn part2(input: &str) -> String {
    find_block(input, 71, 71)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        ";

    #[test]
    fn test_day18_part1() {
        let obstacles = parse(INPUT);
        assert_eq!(solve(&obstacles[..12], 7, 7), 22);
    }

    #[test]
    fn test_day18_part2() {
        assert_eq!(find_block(INPUT, 7, 7), "6,1".to_string());
    }
}
