use ahash::{AHashMap, AHashSet};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Option<Point> {
        if x < 0 || y < 0 || x >= width || y >= height {
            return None;
        }

        Some(Point { x, y })
    }

    fn antinodes(&self, other: &Point, width: i32, height: i32, harmonics: bool) -> Vec<Point> {
        let mut result = vec![];
        if harmonics {
            for i in 0.. {
                if let Some(point) = Point::new(
                    self.x + i * (self.x - other.x),
                    self.y + i * (self.y - other.y),
                    width,
                    height,
                ) {
                    result.push(point);
                } else {
                    break;
                }
            }

            for i in 0.. {
                if let Some(point) = Point::new(
                    other.x + i * (other.x - self.x),
                    other.y + i * (other.y - self.y),
                    width,
                    height,
                ) {
                    result.push(point);
                } else {
                    break;
                }
            }
        } else {
            if let Some(point) = Point::new(
                self.x + (self.x - other.x),
                self.y + (self.y - other.y),
                width,
                height,
            ) {
                result.push(point);
            }
            if let Some(point) = Point::new(
                other.x + (other.x - self.x),
                other.y + (other.y - self.y),
                width,
                height,
            ) {
                result.push(point);
            }
        }

        result
    }
}

fn parse(input: &str) -> (AHashMap<u8, Vec<Point>>, i32, i32) {
    let mut locations: AHashMap<u8, Vec<Point>> = AHashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (i, line) in input.lines().map(|x| x.trim()).enumerate() {
        for (j, &ch) in line.as_bytes().iter().enumerate() {
            if ch != b'.' {
                locations.entry(ch).or_default().push(Point {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
        height += 1;
        width = line.as_bytes().len() as i32;
    }
    (locations, width, height)
}

pub fn part1(input: &str) -> i64 {
    let (locations, width, height) = parse(input);
    locations
        .values()
        .flat_map(|loc| {
            loc.iter()
                .combinations(2)
                .flat_map(|pair| pair[0].antinodes(pair[1], width, height, false))
        })
        .collect::<AHashSet<Point>>()
        .len() as i64
}

pub fn part2(input: &str) -> i64 {
    let (locations, width, height) = parse(input);
    locations
        .values()
        .flat_map(|loc| {
            loc.iter()
                .combinations(2)
                .flat_map(|pair| pair[0].antinodes(pair[1], width, height, true))
        })
        .collect::<AHashSet<Point>>()
        .len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const INPUT_3T: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test_day8_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_day8_part2() {
        assert_eq!(part2(INPUT_3T), 9);
        assert_eq!(part2(INPUT), 34);
    }
}
