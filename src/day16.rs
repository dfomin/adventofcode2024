use std::{collections::BinaryHeap, i64};

use ahash::{AHashMap, AHashSet};

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
    (end.x.abs_diff(position.x) + end.y.abs_diff(position.y)) as i64
}

fn find_record(
    field: &[Vec<u8>],
    start: Position,
    end: Position,
    visited: &mut AHashMap<Position, i64>,
) -> i64 {
    let mut heap: BinaryHeap<(i64, i64, Position)> = BinaryHeap::new();
    let mut result = i64::MAX;
    visited.insert(start, 0);
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
            let mut add = false;
            if let Some(&prev_points) = visited.get(&next_position) {
                if prev_points > new_points {
                    add = true;
                }
            } else {
                add = true;
            }
            if add {
                visited.insert(next_position, new_points);
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
    visited: &AHashMap<Position, i64>,
    record: i64,
) -> i64 {
    let mut heap: BinaryHeap<(i64, i64, Position, Vec<Position>)> = BinaryHeap::new();
    let mut paths: AHashSet<(usize, usize)> = AHashSet::new();
    paths.insert((end.x, end.y));
    heap.push((heuristic(&start, &end), 0, start, vec![start]));
    while let Some((_, current_points, position, history)) = heap.pop() {
        if position.x == end.x && position.y == end.y && current_points == record {
            for p in &history {
                paths.insert((p.x, p.y));
            }
        }
        for (next_position, points) in position.step(&field) {
            let new_points = current_points + points;
            if let Some(&prev_points) = visited.get(&next_position) {
                if prev_points == new_points {
                    let mut new_history = history.clone();
                    new_history.push(position);
                    heap.push((
                        heuristic(&next_position, &end),
                        new_points,
                        next_position,
                        new_history,
                    ));
                }
            }
        }
    }
    paths.len() as i64
}

pub fn part1(input: &str) -> i64 {
    let (field, start, end) = parse(input);
    let mut visited: AHashMap<Position, i64> = AHashMap::new();
    find_record(&field, start, end, &mut visited)
}

pub fn part2(input: &str) -> i64 {
    let (field, start, end) = parse(input);
    let mut visited: AHashMap<Position, i64> = AHashMap::new();
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
