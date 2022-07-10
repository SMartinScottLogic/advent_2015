use anyhow::Result;
use std::collections::HashSet;
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
        self.answer_part2 = self.analyse_part2();
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
        let mut target = self.target.unwrap();
        let mut active = HashSet::new();
        loop {
            let mut top_elf = Self::analyse_part1_partial(target);
            let top_elf = loop {
                if !active.contains(&top_elf) {
                    break top_elf;
                }
                top_elf += 1;
            };
            for elf in 1..=top_elf {
                if top_elf % elf == 0 {
                    active.insert(elf);
                    if target >= elf * 10 {
                        target -= elf * 10;
                    }
                }
            }
            if target <= 0 {
                break;
            }
        };
        log::info!("{active:?}");
        log::info!("{}", active.iter().sum::<u64>());
        let mut total = 0;
        let mut elf = 1;
        let top_elf = loop {
            total += elf * 10;
            log::debug!("{elf} {total}");
            if total >= target {
                break elf;
            }
            elf += 1;
        };
        log::debug!("top_elf: {top_elf}");
        None
    }

    fn analyse_part1_partial(target: u64) -> u64 {
        let mut total = 0;
        let mut elf = 1;
        loop {
            total += elf * 10;
            log::debug!("{elf} {total}");
            if total >= target {
                break elf;
            }
            elf += 1;
        }
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        None
    }
}
