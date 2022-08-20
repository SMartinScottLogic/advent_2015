use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        solution.add_instruction(Instruction::from_str(&line)?);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    instructions: Vec<Instruction>,

    answer_part1: Option<u64>,
    answer_part2: Option<u64>,
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),

            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn analyse(&mut self) {
        self.answer_part1 = self.analyse_part1();
        log::info!("part1: {:?}", self.answer_part1);
        self.answer_part2 = self.analyse_part2();
        log::info!("part2: {:?}", self.answer_part2);
    }

    pub fn answer_part1(&self) -> Option<u64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<u64> {
        self.answer_part2
    }
}

impl Solution {
    fn analyse_part1(&mut self) -> Option<u64> {
        let (_, b) = self.run_program(0, 0, 0);
        Some(b)
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        let (_, b) = self.run_program(1, 0, 0);
        Some(b)
    }

    fn run_program(&self, a: u64, b: u64, ip: usize) -> (u64, u64) {
        let mut a = a;
        let mut b = b;
        let mut ip = ip;

        while let Some(instruction) = self.instructions.get(ip) {
            log::debug!("{ip}: {instruction:?} ({a} {b})");
            ip += 1;
            match instruction {
                Instruction::Hlf(Param::A) => {
                    a /= 2;
                }
                Instruction::Hlf(Param::B) => {
                    b /= 2;
                }
                Instruction::Tpl(Param::A) => {
                    a *= 3;
                }
                Instruction::Tpl(Param::B) => {
                    b *= 3;
                }
                Instruction::Inc(Param::A) => {
                    a += 1;
                }
                Instruction::Inc(Param::B) => {
                    b += 1;
                }
                Instruction::Jmp(Param::Value(delta)) => {
                    let new_ip = (ip as i64) + delta - 1;
                    if new_ip < 0 || new_ip > self.instructions.len().try_into().unwrap() {
                        ip = self.instructions.len();
                    } else {
                        ip = new_ip as usize;
                    }
                }
                Instruction::Jie(Param::A, Param::Value(delta)) => {
                    if a % 2 == 0 {
                        let new_ip = (ip as i64) + delta - 1;
                        if new_ip < 0 || new_ip > self.instructions.len().try_into().unwrap() {
                            ip = self.instructions.len();
                        } else {
                            ip = new_ip as usize;
                        }
                    }
                }
                Instruction::Jie(Param::B, Param::Value(delta)) => {
                    if b % 2 == 0 {
                        let new_ip = (ip as i64) + delta - 1;
                        if new_ip < 0 || new_ip > self.instructions.len().try_into().unwrap() {
                            ip = self.instructions.len();
                        } else {
                            ip = new_ip as usize;
                        }
                    }
                }
                Instruction::Jio(Param::A, Param::Value(delta)) => {
                    if a == 1 {
                        let new_ip = (ip as i64) + delta - 1;
                        if new_ip < 0 || new_ip > self.instructions.len().try_into().unwrap() {
                            ip = self.instructions.len();
                        } else {
                            ip = new_ip as usize;
                        }
                    }
                }
                Instruction::Jio(Param::B, Param::Value(delta)) => {
                    if b == 1 {
                        let new_ip = (ip as i64) + delta - 1;
                        if new_ip < 0 || new_ip > self.instructions.len().try_into().unwrap() {
                            ip = self.instructions.len();
                        } else {
                            ip = new_ip as usize;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        (a, b)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Hlf(Param),
    Tpl(Param),
    Inc(Param),
    Jmp(Param),
    Jie(Param, Param),
    Jio(Param, Param),
}

#[derive(Debug, Clone, Copy)]
pub enum Param {
    Value(i64),
    A,
    B,
}

impl FromStr for Instruction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let s = s.trim().replace(',', " ");

        let (op, params) =
            s.split_whitespace()
                .enumerate()
                .fold(("", Vec::new()), |mut acc, (idx, v)| {
                    match idx {
                        0 => acc.0 = v,
                        _ => acc.1.push(Param::from_str(v).unwrap()),
                    }
                    acc
                });

        let instruction = match op {
            "hlf" => Hlf(params[0]),
            "tpl" => Tpl(params[0]),
            "inc" => Inc(params[0]),
            "jmp" => Jmp(params[0]),
            "jie" => Jie(params[0], params[1]),
            "jio" => Jio(params[0], params[1]),
            _ => unreachable!(),
        };
        Ok(instruction)
    }
}

impl FromStr for Param {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Param::*;
        let param = match s {
            "a" => A,
            "b" => B,
            _ => Value(s.parse().unwrap()),
        };
        Ok(param)
    }
}
