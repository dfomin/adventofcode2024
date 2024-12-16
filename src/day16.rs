use std::{collections::BinaryHeap, i64};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    dir: (i32, i32),
}

impl Position {
    fn step(&self, field: &[Vec<u8>]) -> Vec<(Position, i64)> {
        let new_x = (self.x as i32 + self.dir.0) as usize;
        let new_y = (self.y as i32 + self.dir.1) as usize;
        let mut result = vec![];
        if field[new_y][new_x] != b'#' {
            result.push((
                Position {
                    x: new_x,
                    y: new_y,
                    dir: self.dir,
                },
                1,
            ));
        }
        result.push((
            Position {
                x: self.x,
                y: self.y,
                dir: (-self.dir.1, self.dir.0),
            },
            1000,
        ));
        result.push((
            Position {
                x: self.x,
                y: self.y,
                dir: (self.dir.1, -self.dir.0),
            },
            1000,
        ));
        result
    }
}

fn dir_to_index(dir: (i32, i32)) -> usize {
    match dir {
        (-1, 0) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (0, 1) => 3,
        _ => panic!("Unexpected"),
    }
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Position, Position) {
    let field: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect();
    let (mut start, mut end) = (
        Position {
            x: 0,
            y: 0,
            dir: (1, 0),
        },
        Position {
            x: 0,
            y: 0,
            dir: (0, 0),
        },
    );

    for (y, line) in field.iter().enumerate() {
        if let Some(x) = line.iter().position(|&x| x == b'S') {
            start.x = x;
            start.y = y;
        }

        if let Some(x) = line.iter().position(|&x| x == b'E') {
            end.x = x;
            end.y = y;
        }
    }

    (field, start, end)
}

fn heuristic(position: &Position, end: &Position) -> i64 {
    (end.x.abs_diff(position.x)
        + end.y.abs_diff(position.y)
        + if end.x != position.x && end.y != position.y {
            10000
        } else {
            0
        }) as i64
}

fn find_record(
    field: &[Vec<u8>],
    start: Position,
    end: Position,
    visited: &mut [Vec<[i64; 4]>],
) -> i64 {
    let mut heap: BinaryHeap<(i64, i64, Position)> = BinaryHeap::new();
    let mut result = i64::MAX;
    visited[start.y][start.x][dir_to_index(start.dir)] = 0;
    heap.push((heuristic(&start, &end), 0, start));
    while let Some((_, current_points, position)) = heap.pop() {
        for (next_position, points) in position.step(&field) {
            if points >= result {
                continue;
            }
            let new_points = current_points + points;
            if next_position.x == end.x && next_position.y == end.y {
                result = result.min(new_points);
            }
            let prev_points =
                visited[next_position.y][next_position.x][dir_to_index(next_position.dir)];
            if prev_points > new_points {
                visited[next_position.y][next_position.x][dir_to_index(next_position.dir)] =
                    new_points;
                heap.push((heuristic(&next_position, &end), new_points, next_position));
            }
        }
    }
    result
}

fn find_path(
    field: &[Vec<u8>],
    start: Position,
    end: Position,
    visited: &[Vec<[i64; 4]>],
    record: i64,
) -> i64 {
    let mut heap: BinaryHeap<(i64, i64, Position, Vec<(usize, usize)>)> = BinaryHeap::new();
    let mut paths: Vec<Vec<i64>> = vec![vec![0i64; field[0].len()]; field.len()];
    heap.push((heuristic(&start, &end), 0, start, vec![(start.x, start.y)]));
    while let Some((_, current_points, position, history)) = heap.pop() {
        if position.x == end.x && position.y == end.y && current_points == record {
            paths[end.y][end.x] = 1;
            for &p in &history {
                paths[p.1][p.0] = 1;
            }
            continue;
        }
        for (next_position, points) in position.step(&field) {
            let new_points = current_points + points;
            let prev_points =
                visited[next_position.y][next_position.x][dir_to_index(next_position.dir)];
            if prev_points == new_points {
                let mut new_history = history.clone();
                new_history.push((position.x, position.y));
                heap.push((
                    heuristic(&next_position, &end),
                    new_points,
                    next_position,
                    new_history,
                ));
            }
        }
    }
    paths
        .into_iter()
        .map(|line| line.into_iter().sum::<i64>())
        .sum::<i64>()
}

pub fn part1(input: &str) -> i64 {
    let (field, start, end) = parse(input);
    let mut visited = vec![vec![[i64::MAX; 4]; field[0].len()]; field.len()];
    find_record(&field, start, end, &mut visited)
}

pub fn part2(input: &str) -> i64 {
    let (field, start, end) = parse(input);
    let mut visited = vec![vec![[i64::MAX; 4]; field[0].len()]; field.len()];
    let record = find_record(&field, start, end, &mut visited);
    find_path(&field, start, end, &visited, record)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
        ";

    const INPUT2: &str = "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
        ";

    #[test]
    fn test_day16_part1() {
        assert_eq!(part1(INPUT1), 7036);
        assert_eq!(part1(INPUT2), 11048);
    }

    #[test]
    fn test_day16_part2() {
        assert_eq!(part2(INPUT1), 45);
        assert_eq!(part2(INPUT2), 64);
    }
}
