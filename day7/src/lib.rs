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
        let mut wires = HashMap::new();
        loop {
            for i in &self.instructions {
                log::trace!("p2 {i:?}");
                i.update_wire(&mut wires);
            }
            log::trace!("p1 {wires:#?}");
            log::debug!("p1 a: {:?}", wires.get("a").unwrap_or(&None));
            if wires.get("a").unwrap_or(&None).to_owned().is_some() {
                break;
            }
        }
        wires.get("a").unwrap_or(&None).to_owned().map(|v| v as i64)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut wires = HashMap::new();
        wires.insert("b".to_owned(), self.answer_part1.map(|v| v as u16));
        loop {
            for i in &self.instructions {
                if i.get_target() == "b" {
                    continue;
                }
                log::trace!("p2 {i:?}");
                i.update_wire(&mut wires);
            }
            log::trace!("p2 {wires:#?}");
            log::debug!("p2 a: {:?}", wires.get("a").unwrap_or(&None));
            if wires.get("a").unwrap_or(&None).to_owned().is_some() {
                break;
            }
        }
        wires.get("a").unwrap_or(&None).to_owned().map(|v| v as i64)
    }
}

#[derive(Debug)]
enum Instruction {
    Set(String, Signal),
    Not(String, Signal),
    And(String, Signal, Signal),
    Or(String, Signal, Signal),
    LShift(String, Signal, Signal),
    RShift(String, Signal, Signal),
}

impl Instruction {
    fn get_target(&self) -> String {
        match self {
            Self::Set(t, _) => t.to_owned(),
            Self::Not(t, _) => t.to_owned(),
            Self::And(t, _, _) => t.to_owned(),
            Self::Or(t, _, _) => t.to_owned(),
            Self::LShift(t, _, _) => t.to_owned(),
            Self::RShift(t, _, _) => t.to_owned(),
        }
    }

    fn update_wire(&self, wires: &mut HashMap<String, Option<u16>>) {
        match self {
            Self::Set(t, s) => {
                wires.insert(t.to_owned(), s.value(wires));
            }
            Self::Not(t, s) => {
                wires.insert(t.to_owned(), s.value(wires).map(|v| !v));
            }
            Self::And(t, s1, s2) => {
                wires.insert(
                    t.to_owned(),
                    s1.value(wires)
                        .and_then(|v1| s2.value(wires).map(|v2| v1 & v2)),
                );
            }
            Self::Or(t, s1, s2) => {
                wires.insert(
                    t.to_owned(),
                    s1.value(wires)
                        .and_then(|v1| s2.value(wires).map(|v2| v1 | v2)),
                );
            }
            Self::LShift(t, s1, s2) => {
                wires.insert(
                    t.to_owned(),
                    s1.value(wires)
                        .and_then(|v1| s2.value(wires).map(|v2| v1 << v2)),
                );
            }
            Self::RShift(t, s1, s2) => {
                wires.insert(
                    t.to_owned(),
                    s1.value(wires)
                        .and_then(|v1| s2.value(wires).map(|v2| v1 >> v2)),
                );
            }
        };
    }
}

#[derive(Debug)]
enum Signal {
    Value(u16),
    Wire(String),
}

impl Signal {
    fn value(&self, wires: &HashMap<String, Option<u16>>) -> Option<u16> {
        match self {
            Self::Value(v) => Some(*v),
            Self::Wire(s) => wires.get(s).unwrap_or(&None).to_owned(),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^(((?P<source1>[0-9a-z]+) )?(?P<op>[A-Z]+) )?(?P<source2>[0-9a-z]+) -> (?P<target>[a-z]+)$").unwrap();
        if let Some(cap) = r.captures(s) {
            let source1 = cap
                .name("source1")
                .map(|s| s.as_str())
                .and_then(|s| Signal::from_str(s).ok());
            let op = cap.name("op").map(|s| s.as_str());
            let source2 = cap
                .name("source2")
                .map(|s| s.as_str())
                .and_then(|s| Signal::from_str(s).ok());
            let target = cap.name("target").map(|s| s.as_str()).unwrap().to_owned();
            println!("matched: {source1:?} {op:?} {source2:?} -> {target:?}");
            let i = match op {
                Some("AND") => Self::And(target, source1.unwrap(), source2.unwrap()),
                Some("OR") => Self::Or(target, source1.unwrap(), source2.unwrap()),
                Some("RSHIFT") => Self::RShift(target, source1.unwrap(), source2.unwrap()),
                Some("LSHIFT") => Self::LShift(target, source1.unwrap(), source2.unwrap()),
                Some("NOT") => Self::Not(target, source2.unwrap()),
                None => Self::Set(target, source2.unwrap()),
                _ => unreachable!(),
            };
            Ok(i)
        } else {
            println!("{s} failed to match");
            todo!()
        }
    }
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_numeric()) {
            Ok(Self::Value(s.parse().unwrap()))
        } else {
            Ok(Self::Wire(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parsing() -> Result<()> {
        let tests = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        for test in tests {
            Signal::from_str(test).unwrap();
        }
        Ok(())
    }
}
