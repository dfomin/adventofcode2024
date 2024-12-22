use std::cmp::Ordering;

trait Pad {
    fn move_to(&mut self, value: u8) -> Vec<u8>;
}

struct NumPad {
    x: usize,
    y: usize,
}

impl NumPad {
    fn new() -> Self {
        Self { x: 2, y: 3 }
    }
}

impl Pad for NumPad {
    fn move_to(&mut self, value: u8) -> Vec<u8> {
        let (new_x, new_y) = match value {
            b'0' => (1, 3),
            b'A' => (2, 3),
            _ => (
                ((value - b'1') % 3) as usize,
                2 - ((value - b'1') / 3) as usize,
            ),
        };

        let result = find_path(self.x, self.y, new_x, new_y, 0, 3);
        self.x = new_x;
        self.y = new_y;
        result
    }
}

struct ControlPad {
    x: usize,
    y: usize,
}

impl ControlPad {
    fn new() -> Self {
        Self { x: 2, y: 0 }
    }
}

impl Pad for ControlPad {
    fn move_to(&mut self, value: u8) -> Vec<u8> {
        let (new_x, new_y) = match value {
            b'A' => (2, 0),
            b'^' => (1, 0),
            b'<' => (0, 1),
            b'v' => (1, 1),
            b'>' => (2, 1),
            _ => panic!("Unexpected"),
        };
        let result = find_path(self.x, self.y, new_x, new_y, 0, 0);
        self.x = new_x;
        self.y = new_y;
        result
    }
}

fn find_path(
    x: usize,
    y: usize,
    target_x: usize,
    target_y: usize,
    blocked_x: usize,
    blocked_y: usize,
) -> Vec<u8> {
    if target_x == x || target_y == y {
        let mut result = vec![];
        let steps = x.abs_diff(target_x) + y.abs_diff(target_y);
        let ch = match (x.cmp(&target_x), y.cmp(&target_y)) {
            (Ordering::Less, _) => b'>',
            (Ordering::Greater, _) => b'<',
            (_, Ordering::Less) => b'v',
            (_, Ordering::Greater) => b'^',
            _ => b' ',
        };
        for _ in 0..steps {
            result.push(ch);
        }
        result.push(b'A');
        result
    } else {
        let x_dir = if x > target_x { b'<' } else { b'>' };
        let y_dir = if y > target_y { b'^' } else { b'v' };
        let mut x_first = vec![];
        let mut y_first = vec![];

        if target_x == blocked_x && y == blocked_y {
            for _ in 0..y.abs_diff(target_y) {
                y_first.push(y_dir);
            }
            for _ in 0..x.abs_diff(target_x) {
                y_first.push(x_dir);
            }
            y_first.push(b'A');
            y_first
        } else if x == blocked_x && target_y == blocked_y {
            for _ in 0..x.abs_diff(target_x) {
                x_first.push(x_dir);
            }
            for _ in 0..y.abs_diff(target_y) {
                x_first.push(y_dir);
            }
            x_first.push(b'A');
            x_first
        } else if x < target_x {
            for _ in 0..y.abs_diff(target_y) {
                y_first.push(y_dir);
            }
            for _ in 0..x.abs_diff(target_x) {
                y_first.push(x_dir);
            }
            y_first.push(b'A');
            y_first
        } else {
            for _ in 0..x.abs_diff(target_x) {
                x_first.push(x_dir);
            }
            for _ in 0..y.abs_diff(target_y) {
                x_first.push(y_dir);
            }
            x_first.push(b'A');
            x_first
        }
    }
}

fn apply_pads(
    ch: u8,
    pads: &mut [ControlPad],
    index: usize,
    map: &Vec<(Vec<u8>, usize, usize)>,
    map1: &mut Vec<(i64, usize, usize)>,
) -> i64 {
    if index == pads.len() - 1 {
        let (value, new_x, new_y) = &map[get_cache_index(pads[index].x, pads[index].y, ch)];
        pads[index].x = *new_x;
        pads[index].y = *new_y;
        return value.len() as i64;
    }

    let cache_index = get_cache_index(pads[index].x, pads[index].y, ch);
    let map_index = index * 25 + cache_index;
    if map1[map_index].0 == -1 {
        let (value, new_x, new_y) = &map[cache_index];
        pads[index].x = *new_x;
        pads[index].y = *new_y;
        let mut total = 0;
        for ch1 in value {
            total += apply_pads(*ch1, pads, index + 1, map, map1);
        }

        map1[map_index] = (total, pads[index].x, pads[index].y);
    }

    pads[index].x = map1[map_index].1;
    pads[index].y = map1[map_index].2;

    map1[map_index].0
}

fn get_cache_index(x: usize, y: usize, ch: u8) -> usize {
    ((2 - x) + 3 * (1 - y)) * 5
        + match ch {
            b'^' => 0,
            b'<' => 1,
            b'>' => 2,
            b'v' => 3,
            b'A' => 4,
            _ => panic!("Unexpected"),
        }
}

fn fill_map(map: &mut [(Vec<u8>, usize, usize)]) {
    let mut pad = ControlPad::new();
    for x in 0..3 {
        for y in 0..2 {
            if x == 0 && y == 0 {
                continue;
            }
            for ch in [b'<', b'>', b'^', b'v', b'A'] {
                pad.x = x;
                pad.y = y;
                let value = pad.move_to(ch);
                map[get_cache_index(x, y, ch)] = (value, pad.x, pad.y);
            }
        }
    }
}

fn solve_code(code: &[u8], control_pads: usize, map: &Vec<(Vec<u8>, usize, usize)>) -> i64 {
    let mut result = 0;
    let code_value =
        (code[0] - b'0') as i64 * 100 + (code[1] - b'0') as i64 * 10 + (code[2] - b'0') as i64;
    let mut num_pad = NumPad::new();
    let mut pads = vec![];
    for _ in 0..control_pads {
        pads.push(ControlPad::new());
    }
    let mut map1 = vec![(-1, 0, 0); 25 * control_pads];
    for &ch in code {
        for ch1 in num_pad.move_to(ch) {
            let index = get_cache_index(pads[0].x, pads[0].y, ch1);
            if map1[index].0 == -1 {
                let value = apply_pads(ch1, &mut pads, 0, map, &mut map1);
                map1[index] = (value, pads[0].x, pads[0].y);
            }
            result += map1[index].0;
            pads[0].x = map1[index].1;
            pads[0].y = map1[index].2;
        }
    }
    result * code_value
}

pub fn part1(input: &str) -> i64 {
    let mut map = vec![(vec![], 0, 0); 25];
    fill_map(&mut map);
    input
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(solve_code(line.trim().as_bytes(), 2, &map))
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut map = vec![(vec![], 0, 0); 25];
    fill_map(&mut map);
    input
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(solve_code(line.trim().as_bytes(), 25, &map))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        029A
        980A
        179A
        456A
        379A
        ";

    #[test]
    fn test_day21_part1() {
        assert_eq!(part1(INPUT), 126384);
    }

    #[test]
    fn test_day21_part2() {
        assert_eq!(part2(INPUT), 154115708116294);
    }
}
