use std::usize;

use common::io::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve_part_1(parse_into_lines_automatic("17")));
    dbg!(parse_and_solve_part_2(parse_into_lines_automatic("17")));
}

fn parse_and_solve_part_1(line_iter: impl Iterator<Item = String>) -> String {
    let mut computer = Computer::from_iter(line_iter);
    computer.run()
}

fn parse_and_solve_part_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut computer = Computer::from_iter(line_iter);
    let instructions = computer.instructions.len();

    let mut a: usize = 0;
    for tail_slice in 1..=computer.instructions.len() {
        let curnt_fasit = computer.instructions[instructions - tail_slice..].to_vec();

        // Prepares `a` for determining the next `b`
        // resulting in the correct output.
        // This is because the program always start by
        // setting `b` to `a % 8`, which effectively
        // extracts the last 3 bits of `a`.
        let mut test_a = a << 3;
        loop {
            computer.reset(test_a);
            let mut curnt = Vec::new();
            while let Some(step_result) = computer.step() {
                // Break early if the sequence is incorrect
                if step_result != curnt_fasit[curnt.len()] {
                    break;
                }
                curnt.push(step_result);
            }
            // Move on if the sequence is correct
            if curnt == curnt_fasit {
                a = test_a;
                break;
            }
            test_a += 1;
        }
    }
    a
}

struct Computer {
    r_a: usize,
    r_b: usize,
    r_c: usize,
    pc: usize,
    instructions: Vec<usize>,
}

impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "A: {}", self.r_a)?;
        writeln!(f, "B: {}", self.r_b)?;
        writeln!(f, "C: {}", self.r_c)?;
        writeln!(f, "Pc: {}", self.pc)?;
        writeln!(f, "\nInstruction: {:?}", self.instructions)
    }
}

impl Computer {
    fn new(r_a: usize, r_b: usize, r_c: usize, instructions: Vec<usize>) -> Self {
        Self {
            r_a,
            r_b,
            r_c,
            instructions,
            pc: 0,
        }
    }

    fn step(&mut self) -> Option<usize> {
        while self.pc + 1 < self.instructions.len() {
            if let Some(num) = self.read_instruction() {
                return Some(num);
            }
        }
        None
    }

    fn reset(&mut self, a: usize) {
        self.r_a = a;
        self.pc = 0;
        self.r_b = 0;
        self.r_c = 0;
    }

    fn run(&mut self) -> String {
        let mut out = Vec::new();
        while self.pc + 1 < self.instructions.len() {
            if let Some(num) = self.read_instruction() {
                out.push(num);
            }
        }
        out.iter().join(",")
    }

    fn read_instruction(&mut self) -> Option<usize> {
        let opcode = self.instructions[self.pc];
        self.pc += 2;
        let res = match opcode {
            0 => self.adv(self.operand()),
            1 => self.bxl(self.instructions[self.pc - 1]),
            2 => self.bst(self.operand()),
            3 => self.jnz(self.instructions[self.pc - 1]),
            4 => self.bxc(),
            5 => self.out(self.operand()),
            6 => self.bdv(self.operand()),
            7 => self.cdv(self.operand()),
            _ => panic!("unknown opcode {}", opcode),
        };
        res
    }

    fn operand(&self) -> usize {
        let operand = self.instructions[self.pc - 1];
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.r_a,
            5 => self.r_b,
            6 => self.r_c,
            _ => panic!("unknown operand {}", operand),
        }
    }

    fn div(&self, pow: usize) -> usize {
        let numerator = self.r_a;
        let denominator = 1 << pow;
        return numerator / denominator;
    }

    fn adv(&mut self, operand: usize) -> Option<usize> {
        self.r_a = self.div(operand);
        None
    }

    fn bxl(&mut self, operand: usize) -> Option<usize> {
        self.r_b ^= operand;
        None
    }

    fn bst(&mut self, operand: usize) -> Option<usize> {
        self.r_b = operand % 8;
        None
    }

    fn jnz(&mut self, operand: usize) -> Option<usize> {
        if self.r_a != 0 {
            self.pc = operand;
        }
        None
    }

    fn bxc(&mut self) -> Option<usize> {
        self.r_b ^= self.r_c;
        None
    }

    fn out(&mut self, operand: usize) -> Option<usize> {
        Some(operand % 8)
    }

    fn bdv(&mut self, operand: usize) -> Option<usize> {
        self.r_b = self.div(operand);
        None
    }

    fn cdv(&mut self, operand: usize) -> Option<usize> {
        self.r_c = self.div(operand);
        None
    }
}

impl FromIterator<String> for Computer {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let inps = iter
            .into_iter()
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                }
                let nums = line.split(':').collect::<Vec<_>>()[1];
                let comma_nums = nums.split(',').collect::<Vec<_>>();
                if 1 < comma_nums.len() {
                    Some(
                        comma_nums
                            .iter()
                            .filter_map(|n| n.trim().parse::<usize>().ok())
                            .collect::<Vec<usize>>(),
                    )
                } else {
                    Some(vec![nums.trim().parse::<usize>().unwrap()])
                }
            })
            .collect::<Vec<Vec<usize>>>();
        Self::new(
            inps[0][0],
            inps[1][0],
            inps[2][0],
            inps.last().unwrap().to_owned(),
        )
    }
}

#[test]
fn day17_1a() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
}

#[test]
fn day17_1b() {
    let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
    computer.run();
    assert_eq!(computer.r_b, 1);
}

#[test]
fn day17_1c() {
    let mut computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
    let out = computer.run();
    assert_eq!(out, "0,1,2".to_string());
}

#[test]
fn day17_1d() {
    let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
    let out = computer.run();
    assert_eq!(out, "4,2,5,6,7,7,7,7,3,1,0".to_string());
    assert_eq!(computer.r_a, 0);
}

#[test]
fn day17_1e() {
    let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
    computer.bxl(7);
    assert_eq!(computer.r_b, 26);
}

#[test]
fn day17_1f() {
    let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
    computer.run();
    assert_eq!(computer.r_b, 44354);
}

#[test]
fn day17_2() {
    let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 117440);
}
