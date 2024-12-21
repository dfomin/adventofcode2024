use core::num;
use std::cmp::Ordering;

use rand::Rng;

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
        } else {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
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
}

// fn generate<T: Pad>(code: &[u8], mut pad: T, cur: i64, depth: i64) -> i64 {
//     if code.len() == 0 {
//         return cur;
//     }
//     for &ch in code {
//         let mut value = 0;
//         for path in pad.move_to(ch) {}
//     }
// }

fn apply_pads(ch: u8, pads: &mut [ControlPad], index: usize) -> i64 {
    if index == pads.len() - 1 {
        return pads[index].move_to(ch).len() as i64;
    }

    let mut result = 0;
    for ch1 in pads[index].move_to(ch) {
        result += apply_pads(ch1, pads, index + 1);
    }
    result
}

fn solve_code(code: &[u8], control_pads: usize) -> i64 {
    let mut result = i64::MAX;
    let code_value =
        (code[0] - b'0') as i64 * 100 + (code[1] - b'0') as i64 * 10 + (code[2] - b'0') as i64;
    for _ in 0..1 {
        let mut num_pad = NumPad::new();
        let mut pads = vec![];
        for _ in 0..control_pads {
            pads.push(ControlPad::new());
        }
        let mut cur = 0;
        for &ch in code {
            for _ in 0..control_pads - 1 {
                for ch1 in num_pad.move_to(ch) {
                    cur += apply_pads(ch1, &mut pads, 0);
                }
            }
        }
        result = result.min(cur)
    }
    result * code_value
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(solve_code(line.trim().as_bytes(), 2))
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(solve_code(line.trim().as_bytes(), 25))
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
        // let mut num_pad = NumPad::new();
        // for &ch in "029A".as_bytes() {
        //     println!(
        //         "{:?}",
        //         num_pad
        //             .move_to(ch)
        //             .iter()
        //             .map(|&ch| ch as char)
        //             .collect::<Vec<_>>()
        //     );
        // }
        assert_eq!(part1(INPUT), 126384);
    }

    #[test]
    fn test_day21_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
