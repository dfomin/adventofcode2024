pub fn part1(input: &str) -> i32 {
    let shifts = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let xmas = vec![b'X', b'M', b'A', b'S'];

    let mut letters: Vec<Vec<u8>> = vec![];
    let mut width = 0;
    for line in input.lines().map(|x| x.trim()) {
        if width == 0 {
            width = line.len();
            for _ in 0..3 {
                letters.push(vec![b'.'; width + 6]);
            }
        }
        letters.push(vec![b'.'; width + 6]);
        let last_index = letters.len() - 1;
        for i in 0..width {
            letters[last_index][3 + i] = line.as_bytes()[i];
        }
    }
    for _ in 0..3 {
        letters.push(vec![b'.'; width + 6]);
    }

    let mut result = 0;
    for i in 3..letters.len() - 3 {
        for j in 3..width + 3 {
            for shift in shifts {
                let word = (0..4)
                    .map(|k| {
                        letters[(i as i32 + shift.0 * k) as usize]
                            [(j as i32 + shift.1 * k) as usize]
                    })
                    .collect::<Vec<u8>>();
                if word == xmas {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i32 {
    let mut result = 0;
    let ms = vec![b'M', b'S'];
    let sm = vec![b'S', b'M'];
    let mut letters = vec![];
    for line in input.lines().map(|x| x.trim()) {
        letters.push(line.as_bytes().to_vec());
    }
    for i in 1..letters.len() - 1 {
        for j in 1..letters[i].len() - 1 {
            if letters[i][j] == b'A' {
                let first = vec![letters[i - 1][j - 1], letters[i + 1][j + 1]];
                let second = vec![letters[i - 1][j + 1], letters[i + 1][j - 1]];
                if (first == ms || first == sm) && (second == ms || second == sm) {
                    result += 1;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const INPUT2: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const INPUT3: &str = "M.S
.A.
M.S";

    #[test]
    fn test_day4_part1() {
        assert_eq!(part1(INPUT1), 4);
        assert_eq!(part1(INPUT2), 18);
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!(part2(INPUT3), 1);
        assert_eq!(part2(INPUT2), 9);
    }
}
