use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let mut buf = String::new();

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    reader.read_line(&mut buf)?;

    let mut solution = Solution::new();
    solution.set_target(buf.parse()?);
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    target: Option<u64>,

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
            target: None,

            answer_part1: None,
            answer_part2: None,
        }
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
    fn set_target(&mut self, target: u64) {
        self.target = Some(target);
    }

    fn analyse_part1(&mut self) -> Option<u64> {
        let mut houses = HashMap::new();
        let target = self.target? / 10;
        for i in 1..=target {
            for j in (i..=target).step_by(i.try_into().unwrap()) {
                *houses.entry(j).or_insert(0) += i * 10;
            }
        }
        log::debug!("{houses:?}");
        let mut i = 0;
        loop {
            if let Some(v) = houses.get(&i) {
                if *v >= self.target? {
                    break Some(i);
                }
            }
            i += 1;
        }
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        let mut houses = HashMap::new();
        let target = self.target? / 11;
        for i in 1..=target {
            for j in 1..=50 {
                *houses.entry(j * i).or_insert(0) += i * 11;
            }
        }
        log::debug!("{houses:?}");
        let mut i = 0;
        loop {
            if let Some(v) = houses.get(&i) {
                if *v >= self.target? {
                    break Some(i);
                }
            }
            i += 1;
        }
    }
}
