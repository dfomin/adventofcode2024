fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<u8>, (usize, usize)) {
    let mut iter = input.split("\n\n");
    let field = iter
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let instructions = iter.next().unwrap().trim().as_bytes().to_vec();
    for (y, line) in field.iter().enumerate() {
        if let Some(x) = line.iter().position(|&ch| ch == b'@') {
            return (field, instructions, (x, y));
        }
    }
    panic!("Incorrect input");
}

fn move_box(
    field: &mut Vec<Vec<u8>>,
    x: usize,
    y: usize,
    dir: (i32, i32),
) -> Option<(usize, usize)> {
    match field[y][x] {
        b'#' => None,
        b'.' => Some((x, y)),
        b'O' => move_box(
            field,
            (x as i32 + dir.0) as usize,
            (y as i32 + dir.1) as usize,
            dir,
        ),
        _ => panic!("Unexpected"),
    }
}

fn is_possible_wide_box(field: &mut Vec<Vec<u8>>, x: usize, y: usize, dir: (i32, i32)) -> bool {
    match field[y][x] {
        b'#' => false,
        b'.' => true,
        b'[' => {
            is_possible_wide_box(
                field,
                (x as i32 + dir.0) as usize,
                (y as i32 + dir.1) as usize,
                dir,
            ) && (dir.0 != 0
                || is_possible_wide_box(
                    field,
                    (x as i32 + dir.0 + 1) as usize,
                    (y as i32 + dir.1) as usize,
                    dir,
                ))
        }
        b']' => {
            is_possible_wide_box(
                field,
                (x as i32 + dir.0) as usize,
                (y as i32 + dir.1) as usize,
                dir,
            ) && (dir.0 != 0
                || is_possible_wide_box(
                    field,
                    (x as i32 + dir.0 - 1) as usize,
                    (y as i32 + dir.1) as usize,
                    dir,
                ))
        }
        _ => panic!("Unexpected {}", field[y][x]),
    }
}

fn move_wide_box(field: &mut Vec<Vec<u8>>, x: usize, y: usize, dir: (i32, i32)) {
    if dir.0 != 0 {
        match field[y][x] {
            b'.' => (),
            b'#' => panic!("Unexpected # at {} {}", x, y),
            _ => {
                move_wide_box(field, (x as i32 + dir.0) as usize, y, dir);
                field[y][(x as i32 + dir.0) as usize] = field[y][x];
                field[y][x] = b'.';
            }
        };
    } else {
        match field[y][x] {
            b'.' => (),
            b'#' => panic!("Unexpected # at {} {}", x, y),
            b']' => move_wide_box(field, x - 1, y, dir),
            b'[' => {
                move_wide_box(field, x, (y as i32 + dir.1) as usize, dir);
                move_wide_box(field, x + 1, (y as i32 + dir.1) as usize, dir);
                field[(y as i32 + dir.1) as usize][x] = b'[';
                field[(y as i32 + dir.1) as usize][x + 1] = b']';
                field[y][x] = b'.';
                field[y][x + 1] = b'.';
            }
            _ => panic!("Unexpected"),
        }
    }
}

fn print_field(field: &Vec<Vec<u8>>) {
    for row in field {
        let line: String = row.into_iter().map(|&byte| char::from(byte)).collect();
        println!("{}", line);
    }
}

pub fn part1(input: &str) -> i64 {
    let (mut field, instructions, (mut x, mut y)) = parse(input);
    for direction in instructions {
        let Some(dir) = (match direction {
            b'^' => Some((0, -1)),
            b'>' => Some((1, 0)),
            b'v' => Some((0, 1)),
            b'<' => Some((-1, 0)),
            _ => None,
        }) else {
            continue;
        };

        let target = ((x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize);
        let Some(next_position) = move_box(&mut field, target.0, target.1, dir) else {
            continue;
        };

        field[y][x] = field[next_position.1][next_position.0];
        field[next_position.1][next_position.0] = field[target.1][target.0];
        field[target.1][target.0] = b'@';
        x = target.0;
        y = target.1;
    }
    let mut result = 0;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] == b'O' {
                result += (100 * i + j) as i64;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let (mut field, instructions, (mut x, mut y)) = parse(input);
    let mut wide_field = vec![];
    for line in field {
        let mut new_line = vec![];
        for ch in line {
            match ch {
                b'.' => {
                    new_line.push(b'.');
                    new_line.push(b'.');
                }
                b'#' => {
                    new_line.push(b'#');
                    new_line.push(b'#');
                }
                b'O' => {
                    new_line.push(b'[');
                    new_line.push(b']');
                }
                b'@' => {
                    new_line.push(b'@');
                    new_line.push(b'.');
                }
                _ => panic!("Unexpected"),
            }
        }
        wide_field.push(new_line);
    }
    field = wide_field;
    x *= 2;
    for direction in instructions {
        let Some(dir) = (match direction {
            b'^' => Some((0, -1)),
            b'>' => Some((1, 0)),
            b'v' => Some((0, 1)),
            b'<' => Some((-1, 0)),
            _ => None,
        }) else {
            continue;
        };

        let target = ((x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize);
        if is_possible_wide_box(&mut field, target.0, target.1, dir) {
            move_wide_box(&mut field, target.0, target.1, dir);
            field[target.1][target.0] = b'@';
            field[y][x] = b'.';
            x = target.0;
            y = target.1;
        }
    }
    let mut result = 0;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] == b'[' {
                result += (100 * i + j) as i64;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
        ";

    const INPUT2: &str = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        ";

    #[test]
    fn test_day15_part1() {
        assert_eq!(part1(INPUT1), 2028);
        assert_eq!(part1(INPUT2), 10092);
    }

    #[test]
    fn test_day15_part2() {
        assert_eq!(part2(INPUT2), 9021);
    }
}
