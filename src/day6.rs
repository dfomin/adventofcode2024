use ahash::{AHashMap, AHashSet};

pub fn part1(input: &str) -> i32 {
    let mut field = vec![];
    let shifts = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited = AHashSet::new();
    let mut pos = (0, 0);
    let mut dir = 0;
    let mut x_obstacles: AHashMap<usize, Vec<usize>> = AHashMap::new();
    let mut y_obstacles: AHashMap<usize, Vec<usize>> = AHashMap::new();
    for (i, line) in input.lines().map(|x| x.trim()).enumerate() {
        let last_line = line.as_bytes().to_vec();
        for j in 0..last_line.len() {
            match last_line[j] {
                b'#' => {
                    x_obstacles.entry(j).or_default().push(i);
                    y_obstacles.entry(i).or_default().push(j);
                }
                b'^' => {
                    pos = (j as i32, i as i32);
                    dir = 0;
                }
                b'>' => {
                    pos = (j as i32, i as i32);
                    dir = 1;
                }
                b'v' => {
                    pos = (j as i32, i as i32);
                    dir = 2;
                }
                b'<' => {
                    pos = (j as i32, i as i32);
                    dir = 3;
                }
                _ => (),
            }
        }
        field.push(last_line);
    }

    loop {
        visited.insert(pos);

        let next_x = pos.0 + shifts[dir].0;
        let next_y = pos.1 + shifts[dir].1;
        if next_x < 0
            || next_x >= field[0].len() as i32
            || next_y < 0
            || next_y >= field.len() as i32
        {
            break;
        }

        if field[next_y as usize][next_x as usize] == b'#' {
            dir = (dir + 1) % shifts.len();
        } else {
            pos = (next_x, next_y);
        }
    }

    visited.len() as i32
}

pub fn part2(input: &str) -> i32 {
    let mut field = vec![];
    let shifts = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut pos = (0, 0);
    let mut dir = 0;
    let mut x_obstacles: AHashMap<usize, Vec<usize>> = AHashMap::new();
    let mut y_obstacles: AHashMap<usize, Vec<usize>> = AHashMap::new();
    for (i, line) in input.lines().map(|x| x.trim()).enumerate() {
        let last_line = line.as_bytes().to_vec();
        for j in 0..last_line.len() {
            match last_line[j] {
                b'#' => {
                    x_obstacles.entry(j).or_default().push(i);
                    y_obstacles.entry(i).or_default().push(j);
                }
                b'^' => {
                    pos = (j as i32, i as i32);
                    dir = 0;
                }
                b'>' => {
                    pos = (j as i32, i as i32);
                    dir = 1;
                }
                b'v' => {
                    pos = (j as i32, i as i32);
                    dir = 2;
                }
                b'<' => {
                    pos = (j as i32, i as i32);
                    dir = 3;
                }
                _ => (),
            }
        }
        field.push(last_line);
    }

    let mut result = 0;
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == b'.' {
                field[i][j] = b'#';
                let orig_pos = pos;
                let orig_dir = dir;
                let mut visited = AHashSet::new();
                let mut escape = false;
                loop {
                    if !visited.insert((pos.0, pos.1, dir)) {
                        break;
                    }

                    let next_x = pos.0 + shifts[dir].0;
                    let next_y = pos.1 + shifts[dir].1;
                    if next_x < 0
                        || next_x >= field[0].len() as i32
                        || next_y < 0
                        || next_y >= field.len() as i32
                    {
                        escape = true;
                        break;
                    }

                    if field[next_y as usize][next_x as usize] == b'#' {
                        dir = (dir + 1) % shifts.len();
                    } else {
                        pos = (next_x, next_y);
                    }
                }

                if !escape {
                    result += 1;
                }

                field[i][j] = b'.';
                pos = orig_pos;
                dir = orig_dir;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_day6_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
