use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};

#[derive(Debug, Clone, Copy)]
enum OperationType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
enum Wire {
    X(usize),
    Y(usize),
    Operation(usize, usize, OperationType, String),
    Value(u64),
}

impl Wire {
    fn eval(&mut self, wires: &mut [Wire], x: u64, y: u64) -> u64 {
        match self {
            Wire::X(index) => {
                if x & (1 << *index) > 0 {
                    1
                } else {
                    0
                }
            }
            Wire::Y(index) => {
                if y & (1 << *index) > 0 {
                    1
                } else {
                    0
                }
            }
            Wire::Value(value) => *value,
            Wire::Operation(a_index, b_index, op, _) => {
                let mut a = wires[*a_index].clone();
                let mut b = wires[*b_index].clone();
                let value = match op {
                    OperationType::And => a.eval(wires, x, y) & b.eval(wires, x, y),
                    OperationType::Or => a.eval(wires, x, y) | b.eval(wires, x, y),
                    OperationType::Xor => a.eval(wires, x, y) ^ b.eval(wires, x, y),
                };
                *self = Wire::Value(value);
                value
            }
        }
    }
}

fn calc(wires: &mut [Wire], x: u64, y: u64, z_indices: Vec<usize>) -> u64 {
    let mut result = 0;
    for i in 0..z_indices.len() {
        let mut wire = wires[z_indices[i]].clone();
        let value = wire.eval(wires, x, y);
        result |= value << i;
    }
    result
}

fn parse(input: &str) -> (Vec<Wire>, u64, u64, Vec<usize>, AHashMap<String, usize>) {
    let mut iter = input.trim().split("\n\n");
    let (x, y, xy_names) = iter
        .next()
        .unwrap()
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.trim().split(": "))
        .fold((0, 0, vec![]), |mut acc, mut i| {
            let name = i.next().unwrap();
            let value = i.next().unwrap().parse::<u64>().unwrap();
            let bytes = name.as_bytes();
            let shift = (bytes[1] - b'0') * 10 + bytes[2] - b'0';
            if bytes[0] == b'x' {
                acc.0 |= value << shift;
            } else {
                acc.1 |= value << shift;
            }
            acc.2.push(name);
            acc
        });

    let rules = iter
        .next()
        .unwrap()
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.trim().split(" -> ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let names = rules
        .iter()
        .map(|x| x[1])
        .chain(xy_names.clone())
        .collect::<AHashSet<&str>>()
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name.to_string(), i))
        .collect::<AHashMap<_, _>>();

    let mut wires = vec![Wire::Value(0); names.len()];
    for name in xy_names {
        let index = *names.get(name).unwrap();
        let shift = ((name.as_bytes()[1] - b'0') * 10 + name.as_bytes()[2] - b'0') as usize;
        if name.as_bytes()[0] == b'x' {
            wires[index] = Wire::X(shift);
        } else {
            wires[index] = Wire::Y(shift);
        }
    }

    let mut z_indices = vec![];
    for i in 0..64 {
        if let Some(&index) = names.get(&format!("z{:02}", i)[..]) {
            z_indices.push(index);
        } else {
            break;
        }
    }

    rules.into_iter().fold(&mut wires, |acc, rule| {
        let parts = rule[0].split(" ").collect::<Vec<_>>();
        let name1 = *names.get(parts[0]).unwrap();
        let op = match parts[1] {
            "AND" => OperationType::And,
            "OR" => OperationType::Or,
            "XOR" => OperationType::Xor,
            _ => panic!(""),
        };
        let name2 = *names.get(parts[2]).unwrap();
        let name = *names.get(rule[1]).unwrap();
        acc[name] = Wire::Operation(name1, name2, op, rule[1].to_string());
        acc
    });
    (wires, x, y, z_indices, names)
}

pub fn part1(input: &str) -> u64 {
    let (mut wires, x, y, z_indices, _) = parse(input);
    calc(&mut wires, x, y, z_indices)
}

pub fn part2(input: &str) -> String {
    let (wires, _, _, _, names) = parse(input);
    let mut result: Vec<String> = vec![];
    for wire in &wires {
        match wire {
            Wire::Operation(_, _, op, name) if name.as_bytes()[0] == b'z' => {
                if name != "z45" && !matches!(op, OperationType::Xor) {
                    result.push(name.clone());
                }
            }
            Wire::Operation(x, y, op, name)
                if matches!(wires[*x], Wire::X(_)) && matches!(wires[*y], Wire::Y(_))
                    || matches!(wires[*x], Wire::Y(_)) && matches!(wires[*y], Wire::X(_)) =>
            {
                let x_index = match wires[*x] {
                    Wire::X(v) => v,
                    Wire::Y(v) => v,
                    _ => panic!(),
                };
                if x_index == 0 {
                    continue;
                }
                if matches!(op, OperationType::Xor) {
                    let index = *names.get(name).unwrap();
                    for i in 0..wires.len() {
                        match &wires[i] {
                            Wire::Operation(a, b, op, _) if *a == index || *b == index => {
                                if matches!(op, OperationType::Or) {
                                    result.push(name.clone());
                                }
                                break;
                            }
                            _ => (),
                        }
                    }
                } else if matches!(op, OperationType::And) {
                    let index = *names.get(name).unwrap();
                    for i in 0..wires.len() {
                        match &wires[i] {
                            Wire::Operation(a, b, op, _) if *a == index || *b == index => {
                                if !matches!(op, OperationType::Or) {
                                    result.push(name.clone());
                                }
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Wire::Operation(x, y, op, name) => match (&wires[*x], &wires[*y]) {
                (Wire::Operation(_, _, _, _), Wire::Operation(_, _, _, _)) => {
                    if name.as_bytes()[0] != b'z' && matches!(op, OperationType::Xor) {
                        result.push(name.clone());
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    result.sort_unstable();
    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
        ";

    const INPUT2: &str = "
        x00: 0
        x01: 1
        x02: 0
        x03: 1
        x04: 0
        x05: 1
        y00: 0
        y01: 0
        y02: 1
        y03: 1
        y04: 0
        y05: 1

        x00 AND y00 -> z05
        x01 AND y01 -> z02
        x02 AND y02 -> z01
        x03 AND y03 -> z03
        x04 AND y04 -> z04
        x05 AND y05 -> z00
        ";

    #[test]
    fn test_day22_part1() {
        assert_eq!(part1(INPUT1), 2024);
    }

    #[test]
    fn test_day22_part2() {
        assert_eq!(part2(INPUT2), "z00,z01,z02,z05");
    }
}
