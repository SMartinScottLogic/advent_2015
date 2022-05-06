use anyhow::{Error, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for s in reader.lines().flatten() {
        solution.add_instruction(Instruction::from_str(&s).unwrap());
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    instructions: Vec<Instruction>,

    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
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

    pub fn analyse(&mut self) {
        self.answer_part1 = self.analyse_part1();
        self.answer_part2 = self.analyse_part2();
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn analyse_part1(&self) -> Option<i64> {
        let mut lights = HashMap::new();
        for i in &self.instructions {
            for x in i.sx..=i.ex {
                for y in i.sy..=i.ey {
                    match i.mode {
                        Mode::On => lights.insert((x, y), 1),
                        Mode::Off => lights.insert((x, y), 0),
                        Mode::Toggle => lights.insert((x, y), lights.get(&(x, y)).unwrap_or(&0) ^ 1)
                    };
                }
            }
        }
        let mut total = 0;
        for (_, v) in lights {
            total += v;
        }
        Some(total)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut lights = HashMap::new();
        for i in &self.instructions {
            for x in i.sx..=i.ex {
                for y in i.sy..=i.ey {
                    let cur_value = *lights.get(&(x, y)).unwrap_or(&0);
                    let new_value = match i.mode {
                        Mode::On => cur_value + 1,
                        Mode::Off if cur_value > 0 => cur_value - 1,
                        Mode::Off => cur_value,
                        Mode::Toggle => cur_value + 2,
                    };
                    lights.insert((x, y), new_value);
                }
            }
        }
        let mut total = 0;
        for (_, v) in lights {
            total += v;
        }
        Some(total)
    }
}

#[derive(Debug)]
struct Instruction {
    mode: Mode,
    sx: u32,
    sy: u32,
    ex: u32,
    ey: u32
}

#[derive(Debug)]
enum Mode {
    On,Off,Toggle
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^(?P<mode>(turn on)|(turn off)|(toggle)) (?P<sx>[0-9]+),(?P<sy>[0-9]+) through (?P<ex>[0-9]+),(?P<ey>[0-9]+)$").unwrap();
        let cap = r.captures(s).unwrap();
        let mode = Mode::from_str(cap.name("mode").unwrap().as_str())?;
        let sx = cap.name("sx").unwrap().as_str().parse().unwrap();
        let sy = cap.name("sy").unwrap().as_str().parse().unwrap();
        let ex = cap.name("ex").unwrap().as_str().parse().unwrap();
        let ey = cap.name("ey").unwrap().as_str().parse().unwrap();
        Ok(Self { mode, sx, sy, ex, ey })
    }
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = match s {
            "turn on" => Self::On,
            "toggle" => Self::Toggle,
            "turn off" => Self::Off,
            _ => unreachable!()
        };
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use utils::map;

    #[test]
    fn known_results_part1() -> Result<()> {
        let m = map![
        "ugknbfddgicrmopn" => Judgement1::Nice,
        "aaa" => Judgement1::Nice,
        "jchzalrnumimnmhp" => Judgement1::Naughty,
        "haegwjzuvuyypxyu" => Judgement1::Naughty,
        "dvszwmarrgswjxmb" => Judgement1::Naughty
            ];

        for (input, expected) in m {
            assert_eq!(Judgement1::from_str(input).unwrap(), expected);
        }
        Ok(())
    }

    #[test]
    fn known_results_part2() -> Result<()> {
        let m = map![
        "qjhvhtzxzqqjkmpb" => Judgement2::Nice,
        "xxyxx" => Judgement2::Nice,
        "uurcxstgmygtbstg" => Judgement2::Naughty,
        "ieodomkazucvgmuy" => Judgement2::Naughty
            ];

        for (input, expected) in m {
            assert_eq!(Judgement2::from_str(input).unwrap(), expected);
        }
        Ok(())
    }
}
