struct Computer {
    a: i64,
    b: i64,
    c: i64,
    pointer: usize,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self {
            a,
            b,
            c,
            pointer: 0,
        }
    }

    fn combo(&self, operand: i64) -> i64 {
        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => operand,
        }
    }

    fn execute(&mut self, program: &[i64]) -> Vec<i64> {
        let mut output = vec![];
        while self.pointer < program.len() - 1 {
            match program[self.pointer] {
                0 => {
                    let operand = self.combo(program[self.pointer + 1]);
                    self.a = self.a / (2i64.pow(operand as u32));
                    self.pointer += 2;
                }
                1 => {
                    let operand = program[self.pointer + 1];
                    self.b = self.b ^ operand;
                    self.pointer += 2;
                }
                2 => {
                    let operand = self.combo(program[self.pointer + 1]);
                    self.b = operand % 8;
                    self.pointer += 2;
                }
                3 => {
                    if self.a != 0 {
                        let operand = program[self.pointer + 1];
                        self.pointer = operand as usize;
                    } else {
                        self.pointer += 2;
                    }
                }
                4 => {
                    self.b = self.b ^ self.c;
                    self.pointer += 2;
                }
                5 => {
                    let operand = self.combo(program[self.pointer + 1]);
                    output.push(operand % 8);
                    self.pointer += 2;
                }
                6 => {
                    let operand = self.combo(program[self.pointer + 1]);
                    self.b = self.a / (2i64.pow(operand as u32));
                    self.pointer += 2;
                }
                7 => {
                    let operand = self.combo(program[self.pointer + 1]);
                    self.c = self.a / (2i64.pow(operand as u32));
                    self.pointer += 2;
                }
                _ => panic!("Unexpected"),
            }
        }
        output
    }
}

fn parse(input: &str) -> (Computer, Vec<i64>) {
    let mut iter = input.split("\n\n");
    let mut registers = iter
        .next()
        .unwrap()
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|x| {
            x.trim()
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        });
    let a = registers.next().unwrap();
    let b = registers.next().unwrap();
    let c = registers.next().unwrap();
    let computer = Computer::new(a, b, c);
    let program = iter
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    (computer, program)
}

pub fn part1(input: &str) -> String {
    let (mut computer, program) = parse(input);
    computer
        .execute(&program)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &str) -> i64 {
    let (mut computer, program) = parse(input);
    let mut results = vec![0];
    for digit in 0..program.len() {
        let mut next_results = vec![];
        for r in results {
            for i in 0..8 {
                computer.a = r * 8 + i;
                computer.b = 0;
                computer.c = 0;
                computer.pointer = 0;
                let output = computer.execute(&program);
                if output.len() != digit + 1 {
                    continue;
                }
                if output[output.len() - 1 - digit] == program[program.len() - 1 - digit] {
                    next_results.push(r * 8 + i);
                }
            }
        }
        results = next_results;
    }
    results[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
        ";

    const INPUT2: &str = "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
        ";

    #[test]
    fn test_day17_part1() {
        assert_eq!(part1(INPUT1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_day17_part2() {
        assert_eq!(part2(INPUT2), 117440);
    }
}
