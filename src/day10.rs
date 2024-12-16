use rayon::prelude::*;

fn dfs(field: &[Vec<u8>], visited: &mut [Vec<bool>], use_visited: bool, x: i32, y: i32) -> i64 {
    let ux = x as usize;
    let uy = y as usize;

    if use_visited {
        visited[uy][ux] = true;
    }

    let value = field[uy][ux];
    if value == b'9' {
        1
    } else {
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|shift| (x + shift.0, y + shift.1))
            .filter(|&(x, y)| {
                x >= 0 && x < field[0].len() as i32 && y >= 0 && y < field.len() as i32
            })
            .map(|(x, y)| (x as usize, y as usize))
            .filter_map(|(x, y)| {
                if (!use_visited || !visited[y][x]) && field[y][x] == value + 1 {
                    Some(dfs(field, visited, use_visited, x as i32, y as i32))
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn part1(input: &str) -> i64 {
    let field = input
        .trim()
        .lines()
        .map(|x| x.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();
    field
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &value)| value == b'0')
                .map(|(x, _)| {
                    let mut visited = vec![vec![false; field[0].len()]; field.len()];
                    dfs(&field, &mut visited, true, x as i32, y as i32)
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let field = input
        .trim()
        .lines()
        .map(|x| x.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();
    field
        .par_iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &value)| value == b'0')
                .map(|(x, _)| {
                    let mut visited = vec![];
                    dfs(&field, &mut visited, false, x as i32, y as i32)
                })
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9
        "#;

    const INPUT2: &str = r#"
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
        "#;

    const INPUT3: &str = r#"
        10..9..
        2...8..
        3...7..
        4567654
        ...8..3
        ...9..2
        .....01
        "#;

    const INPUT4: &str = r#"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        "#;

    const INPUT5: &str = r#"
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....
        "#;

    const INPUT6: &str = r#"
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
        "#;

    const INPUT7: &str = r#"
        012345
        123456
        234567
        345678
        4.6789
        56789.
        "#;

    #[test]
    fn test_day10_part1() {
        assert_eq!(part1(INPUT1), 2);
        assert_eq!(part1(INPUT2), 4);
        assert_eq!(part1(INPUT3), 3);
        assert_eq!(part1(INPUT4), 36);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(part2(INPUT5), 3);
        assert_eq!(part2(INPUT6), 13);
        assert_eq!(part2(INPUT7), 227);
        assert_eq!(part2(INPUT4), 81);
    }
}
