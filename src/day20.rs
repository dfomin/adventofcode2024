use rayon::prelude::*;
use std::collections::BinaryHeap;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step_to_dir(&self, dir: (i32, i32), field: &[Vec<u8>]) -> Option<Position> {
        let new_x = (self.x as i32 + dir.0) as usize;
        let new_y = (self.y as i32 + dir.1) as usize;
        if field[new_y][new_x] != b'#' {
            Some(Position { x: new_x, y: new_y })
        } else {
            None
        }
    }

    fn step(&self, field: &[Vec<u8>]) -> Vec<Position> {
        DIRS.iter()
            .filter_map(|&dir| self.step_to_dir(dir, field))
            .collect()
    }
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Position, Position) {
    let field: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect();
    let (mut start, mut end) = (Position { x: 0, y: 0 }, Position { x: 0, y: 0 });

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
            1000
        } else {
            0
        }) as i64
}

fn find_record(field: &[Vec<u8>], start: Position, end: Position, visited: &mut [Vec<i64>]) -> i64 {
    let mut heap: BinaryHeap<(i64, i64, Position)> = BinaryHeap::with_capacity(1024);
    let mut result = i64::MAX;
    visited[start.y][start.x] = 0;
    heap.push((heuristic(&start, &end), 0, start));
    while let Some((_, current_points, position)) = heap.pop() {
        if current_points >= result {
            continue;
        }
        for next_position in position.step(field) {
            let new_points = current_points + 1;
            if new_points >= result {
                continue;
            }
            if next_position.x == end.x && next_position.y == end.y {
                result = result.min(new_points);
            }
            let prev_points = visited[next_position.y][next_position.x];
            if prev_points > new_points {
                visited[next_position.y][next_position.x] = new_points;
                heap.push((
                    -heuristic(&next_position, &end) - new_points,
                    new_points,
                    next_position,
                ));
            }
        }
    }
    result
}

fn find_path(
    field: &[Vec<u8>],
    end: Position,
    visited: &[Vec<i64>],
    record: i64,
) -> (Vec<Vec<i64>>, Vec<Position>) {
    let mut point = (Position { x: end.x, y: end.y }, record);
    let mut result = vec![vec![-1; field[0].len()]; field.len()];
    result[end.y][end.x] = record;
    let mut path = vec![Position { x: end.x, y: end.y }];
    loop {
        let (position, value) = point;
        for prev_position in position.step(field) {
            if visited[prev_position.y][prev_position.x] == value - 1 {
                result[prev_position.y][prev_position.x] = value - 1;
                path.push(prev_position);
                if value > 0 {
                    point = (prev_position, value - 1);
                }
            }
        }
        if value == 0 {
            break;
        }
    }
    (result, path)
}

fn solve(input: &str, cheats: i64, difference: i64) -> i64 {
    let (field, start, end) = parse(input);
    let mut visited = vec![vec![i64::MAX; field[0].len()]; field.len()];
    let record = find_record(&field, start, end, &mut visited);
    let (new_field, path) = find_path(&field, end, &visited, record);
    path.par_iter()
        .rev()
        .map(|position| {
            (-cheats..=cheats)
                .map(|y_diff| {
                    (-cheats + y_diff.abs()..=cheats - y_diff.abs()).fold(0, |acc, x_diff| {
                        let new_x = position.x as i64 + x_diff;
                        let new_y = position.y as i64 + y_diff;
                        if new_x > 0
                            && new_x < field[0].len() as i64 - 1
                            && new_y > 0
                            && new_y < field.len() as i64 - 1
                        {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;
                            if new_field[new_y][new_x]
                                >= new_field[position.y][position.x]
                                    + x_diff.abs()
                                    + y_diff.abs()
                                    + difference
                            {
                                acc + 1
                            } else {
                                acc
                            }
                        } else {
                            acc
                        }
                    })
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 2, 100)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
        ";

    #[test]
    fn test_day20_part1() {
        assert_eq!(solve(INPUT, 2, 2), 44);
    }

    #[test]
    fn test_day20_part2() {
        assert_eq!(solve(INPUT, 20, 50), 285);
    }
}
