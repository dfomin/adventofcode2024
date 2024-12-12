const SHIFTS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbor_fence(
    field: &[Vec<u8>],
    visited: &mut [Vec<u8>],
    fences: &mut [Vec<u8>],
    x: usize,
    y: usize,
    i: usize,
) -> i64 {
    let shifts = if i >= 2 {
        [SHIFTS[0], SHIFTS[1]]
    } else {
        [SHIFTS[2], SHIFTS[3]]
    };
    let width = field[0].len() as i32;
    let height = field.len() as i32;
    shifts
        .iter()
        .filter_map(|&shift| {
            let (new_x, new_y) = (x as i32 + shift.0, y as i32 + shift.1);
            if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                Some((new_x as usize, new_y as usize))
            } else {
                None
            }
        })
        .filter(|&(new_x, new_y)| visited[new_y][new_x] == field[y][x])
        .filter(|&(new_x, new_y)| {
            field[y][x] == field[new_y][new_x] && (fences[new_y][new_x] & (1 << i) != 0)
        })
        .count() as i64
}

fn dfs(
    field: &[Vec<u8>],
    visited: &mut [Vec<u8>],
    fences: &mut [Vec<u8>],
    x: usize,
    y: usize,
    value: u8,
    reuse_fence: bool,
) -> (i64, i64) {
    if visited[y][x] != 0 {
        return (0, 0);
    }

    let width = field[0].len() as i32;
    let height = field.len() as i32;

    visited[y][x] = value;
    let (p, a) = SHIFTS
        .iter()
        .map(|shift| (x as i32 + shift.0, y as i32 + shift.1))
        .enumerate()
        .fold((0, 0), |acc, (i, new_pos)| {
            if new_pos.0 < 0 || new_pos.0 >= width || new_pos.1 < 0 || new_pos.1 >= height {
                if reuse_fence {
                    fences[y][x] |= 1 << i;
                    let fence = neighbor_fence(field, visited, fences, x, y, i);
                    (acc.0 + 1 - fence, acc.1)
                } else {
                    (acc.0 + 1, acc.1)
                }
            } else {
                let new_x = new_pos.0 as usize;
                let new_y = new_pos.1 as usize;
                if field[new_y][new_x] != value {
                    if reuse_fence {
                        fences[y][x] |= 1 << i;
                        let fence = neighbor_fence(field, visited, fences, x, y, i);
                        (acc.0 + 1 - fence, acc.1)
                    } else {
                        (acc.0 + 1, acc.1)
                    }
                } else {
                    let (p, a) = dfs(field, visited, fences, new_x, new_y, value, reuse_fence);
                    (acc.0 + p, acc.1 + a)
                }
            }
        });
    (p, a + 1)
}

pub fn part1(input: &str) -> i64 {
    let field = input
        .trim()
        .lines()
        .map(|x| x.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let width = field[0].len();
    let height = field.len();
    let mut visited = vec![vec![0; width]; height];
    let mut fences = vec![];
    (0..height)
        .map(|y| {
            (0..width).fold(0, |acc, x| {
                let (p, a) = dfs(&field, &mut visited, &mut fences, x, y, field[y][x], false);
                acc + p * a
            })
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let field = input
        .trim()
        .lines()
        .map(|x| x.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let width = field[0].len();
    let height = field.len();
    let mut visited = vec![vec![0; width]; height];
    let mut fences = vec![vec![0; width]; height];
    (0..height)
        .map(|y| {
            (0..width).fold(0, |acc, x| {
                let (p, a) = dfs(&field, &mut visited, &mut fences, x, y, field[y][x], true);
                // println!("{} {} {}({},{})", p, a, field[y][x] as char, x, y);
                acc + p * a
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        AAAA
        BBCD
        BBCC
        EEEC";

    const INPUT2: &str = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO";

    const INPUT3: &str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE";

    const INPUT4: &str = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE";

    const INPUT5: &str = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA";

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(INPUT1), 140);
        assert_eq!(part1(INPUT2), 772);
        assert_eq!(part1(INPUT3), 1930);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(INPUT1), 80);
        assert_eq!(part2(INPUT2), 436);
        assert_eq!(part2(INPUT4), 236);
        assert_eq!(part2(INPUT5), 368);
        assert_eq!(part2(INPUT3), 1206);
    }
}
