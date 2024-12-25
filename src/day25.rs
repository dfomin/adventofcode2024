pub fn part1(input: &str) -> i64 {
    let mut is_lock = false;
    let mut locks = vec![];
    let mut keys = vec![];
    for block in input.split("\n\n") {
        let mut new_item = true;
        for line in block.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if new_item {
                is_lock = line.as_bytes()[0] == b'#';
                if is_lock {
                    locks.push(vec![0; line.len()]);
                } else {
                    keys.push(vec![0; line.len()]);
                }
                new_item = false;
                continue;
            }

            for i in 0..line.len() {
                if is_lock {
                    let index = locks.len() - 1;
                    locks[index][i] += if line.as_bytes()[i] == b'#' { 1 } else { 0 };
                } else {
                    let index = keys.len() - 1;
                    keys[index][i] += if line.as_bytes()[i] == b'#' { 1 } else { 0 };
                }
            }
        }
    }
    let mut result = 0;
    for lock in locks {
        for key in &keys {
            let mut overlap = false;
            for (l, k) in lock.iter().zip(key) {
                if l + k > 6 {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                result += 1;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
        ";

    #[test]
    fn test_day25_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_day25_part2() {}
}
