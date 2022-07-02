use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        let container = line.parse()?;
        solution.add(container);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    data: Vec<u64>,

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
            data: Vec::new(),

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
    fn add(&mut self, container: u64) {
        self.data.push(container);
    }

    fn analyse_part1(&self) -> Option<u64> {
        Some(self.analyse_part1_step(0, 150))
    }

    fn analyse_part1_step(&self, idx: usize, liters: u64) -> u64 {
        let mut count = 0;
        if let Some(container) = self.data.get(idx) {
            match liters.cmp(container) {
                Ordering::Greater => {
                    log::debug!("{} {}", liters, *container);
                    log::debug!("{} {}: {}", idx + 1, liters - *container, count);
                    count += self.analyse_part1_step(idx + 1, liters - container);
                    log::debug!("{} {}: {}", idx + 1, liters - *container, count);
                }
                Ordering::Equal => {
                    count += 1;
                }
                Ordering::Less => {}
            }
            log::debug!("{} {}: {}", idx + 1, liters, count);
            count += self.analyse_part1_step(idx + 1, liters);
            log::debug!("{} {}: {}", idx + 1, liters, count);
        }
        count
    }

    fn analyse_part2(&self) -> Option<u64> {
        let mut counts = HashMap::new();
        self.analyse_part2_step(0, 0, 150, &mut counts);
        log::debug!("{counts:?}");
        counts
            .iter()
            .min_by_key(|(k, _)| k.to_owned())
            .map(|count| count.1)
            .cloned()
    }

    fn analyse_part2_step(
        &self,
        num_used: usize,
        idx: usize,
        liters: u64,
        counts: &mut HashMap<usize, u64>,
    ) {
        if let Some(container) = self.data.get(idx) {
            match liters.cmp(container) {
                Ordering::Greater => {
                    log::debug!("{} {}", liters, *container);
                    log::debug!("{} {}: {:?}", idx + 1, liters - *container, counts);
                    self.analyse_part2_step(num_used + 1, idx + 1, liters - container, counts);
                    log::debug!("{} {}: {:?}", idx + 1, liters - *container, counts);
                }
                Ordering::Equal => {
                    *counts.entry(num_used + 1).or_default() += 1;
                }
                Ordering::Less => {}
            }
            log::debug!("{} {}: {:?}", idx + 1, liters, counts);
            self.analyse_part2_step(num_used, idx + 1, liters, counts);
            log::debug!("{} {}: {:?}", idx + 1, liters, counts);
        }
    }
}
