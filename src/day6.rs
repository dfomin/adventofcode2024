use rayon::prelude::*;

enum SimulationResult {
    Cycle,
    Route(i32, Vec<Vec<i32>>),
}

fn parse(input: &str) -> (Vec<Vec<u8>>, (i32, i32), usize) {
    let mut field = Vec::with_capacity(130);
    let mut pos = (0, 0);
    let mut dir = 0;
    let mut found = false;
    for (i, line) in input.lines().map(|x| x.trim()).enumerate() {
        let last_line = line.as_bytes().to_vec();
        if !found {
            if let Some(j) = last_line.iter().position(|&x| x == b'^') {
                pos = (j as i32, i as i32);
                dir = 0;
                found = true;
            }
            if let Some(j) = last_line.iter().position(|&x| x == b'>') {
                pos = (j as i32, i as i32);
                dir = 1;
                found = true;
            }
            if let Some(j) = last_line.iter().position(|&x| x == b'v') {
                pos = (j as i32, i as i32);
                dir = 2;
                found = true;
            }
            if let Some(j) = last_line.iter().position(|&x| x == b'<') {
                pos = (j as i32, i as i32);
                dir = 3;
                found = true;
            }
        }
        field.push(last_line);
    }
    (field, pos, dir)
}

fn simulate(field: &[Vec<u8>], mut pos: (i32, i32), mut dir: usize) -> SimulationResult {
    let mut visited = vec![vec![0; field[0].len()]; field.len()];
    let shifts = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut length = 0;

    loop {
        if visited[pos.1 as usize][pos.0 as usize] == 0 {
            length += 1;
        }
        visited[pos.1 as usize][pos.0 as usize] |= 1 << dir;

        let next_x = pos.0 + shifts[dir].0;
        let next_y = pos.1 + shifts[dir].1;
        if next_x < 0
            || next_x >= field[0].len() as i32
            || next_y < 0
            || next_y >= field.len() as i32
        {
            return SimulationResult::Route(length, visited);
        }

        if field[next_y as usize][next_x as usize] == b'#' {
            dir = (dir + 1) % shifts.len();
        } else {
            pos = (next_x, next_y);
        }

        if visited[pos.1 as usize][pos.0 as usize] & (1 << dir) > 0 {
            return SimulationResult::Cycle;
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let (field, pos, dir) = parse(input);

    if let SimulationResult::Route(length, _) = simulate(&field, pos, dir) {
        length
    } else {
        panic!("Cycle found");
    }
}

pub fn part2(input: &str) -> i32 {
    let (field, pos, dir) = parse(input);

    let visited = match simulate(&field, pos, dir) {
        SimulationResult::Route(_, visited) => visited,
        _ => panic!("Cycle found"),
    };

    (0..field.len())
        .into_par_iter()
        .map(|i| {
            (0..field[i].len())
                .into_par_iter()
                .filter(|&j| {
                    if visited[i][j] > 0 && pos != (j as i32, i as i32) {
                        let mut field_clone = field.clone();
                        field_clone[i][j] = b'#';
                        matches!(simulate(&field_clone, pos, dir), SimulationResult::Cycle)
                    } else {
                        false
                    }
                })
                .count() as i32
        })
        .sum()
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
