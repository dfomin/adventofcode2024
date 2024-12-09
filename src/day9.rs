#[derive(Debug, Clone, Copy)]
struct Element {
    number: i64,
    index: i64,
    enabled: bool,
}

fn parse(input: &str) -> Vec<Element> {
    input
        .trim()
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &x)| Element {
            number: (x - b'0') as i64,
            index: i as i64 / 2,
            enabled: i % 2 == 0,
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> i64 {
    let mut numbers = parse(input);
    let mut result = 0;
    let (mut i, mut j) = (0, numbers.len() - 1);
    let mut index = 0;
    while i <= j {
        if numbers[i].enabled {
            result +=
                numbers[i].index * (numbers[i].number * (2 * index + numbers[i].number - 1)) / 2;
            index += numbers[i].number;
        } else {
            while numbers[i].number > 0 {
                while numbers[j].number == 0 || !numbers[j].enabled {
                    j -= 1;
                    if i > j {
                        break;
                    }
                }
                if i > j {
                    break;
                }

                result += index * numbers[j].index;
                index += 1;

                numbers[i].number -= 1;
                numbers[j].number -= 1;
            }
        }
        i += 1;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let mut numbers = parse(input);
    let mut j = numbers.len() - 1;
    while j > 0 {
        while 0 < j && !numbers[j].enabled {
            j -= 1;
        }
        let mut i = 0;
        while i < j && (numbers[i].enabled || numbers[j].number > numbers[i].number) {
            i += 1;
        }
        if i < j {
            let t = numbers[i];
            numbers[i] = numbers[j];
            numbers[j] = t;
            if numbers[i].number < numbers[j].number {
                numbers.insert(
                    i + 1,
                    Element {
                        number: numbers[j].number - numbers[i].number,
                        index: 0,
                        enabled: false,
                    },
                );
                j += 1;
            }
            numbers[j].number = numbers[i].number;
        }
        j -= 1;
    }
    let mut result = 0;
    let mut index = 0;
    for n in numbers {
        if n.enabled {
            result += n.index * (n.number * (2 * index + n.number - 1)) / 2;
        }
        index += n.number;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_day9_part1() {
        assert_eq!(part1("12345"), 60);
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_day9_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
