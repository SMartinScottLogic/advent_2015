use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        let id = line.trim();
        let id = id.parse().unwrap();
        solution.add_package(id);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    packages: Vec<u64>,

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
            packages: Vec::new(),

            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn add_package(&mut self, package: u64) {
        self.packages.push(package);
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
        let mut packages = self.packages.clone();
        packages.sort_by(|a, b| b.cmp(a));
        let mut group_weight: u64 = packages.iter().sum();
        log::debug!("total = {group_weight}");
        group_weight /= 3;
        log::debug!("group weight = {group_weight}");
        self.find(0, &packages, HashSet::new(), group_weight, None)
            .map(|r| r.1)
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        let mut packages = self.packages.clone();
        packages.sort_by(|a, b| b.cmp(a));
        let mut group_weight: u64 = packages.iter().sum();
        log::debug!("total = {group_weight}");
        group_weight /= 4;
        log::debug!("group weight = {group_weight}");
        self.find(0, &packages, HashSet::new(), group_weight, None)
            .map(|r| r.1)
    }

    fn find(
        &self,
        start_idx: usize,
        packages: &Vec<u64>,
        held_packages: HashSet<u64>,
        remaining: u64,
        mut best: Option<(usize, u64)>,
    ) -> Option<(usize, u64)> {
        if remaining == 0 {
            let group_weight: u64 = held_packages.iter().sum();
            if self.can_find(packages, held_packages.clone(), group_weight) {
                let qe = held_packages.iter().product();
                log::debug!("Found a solution, {qe}: {held_packages:?}");
                return Some((held_packages.len(), qe));
            }
            return None;
        }
        for (idx, package) in packages.iter().enumerate() {
            if idx < start_idx {
                continue;
            }
            if let Some((best_len, _)) = best {
                if held_packages.len() > best_len {
                    break;
                }
            }
            if *package > remaining {
                continue;
            }
            if held_packages.contains(package) {
                continue;
            }
            let mut held_packages = held_packages.clone();
            held_packages.insert(*package);
            let sum: u64 = held_packages.iter().sum();
            let remaining = remaining - *package;
            log::debug!("grabbed {package}: {held_packages:?} {sum} {remaining}");
            if let Some((len, qe)) = self.find(idx + 1, packages, held_packages, remaining, best) {
                best = match best {
                    None => Some((len, qe)),
                    Some((best_len, _)) if len < best_len => Some((len, qe)),
                    Some((best_len, best_qe)) if len == best_len && qe < best_qe => Some((len, qe)),
                    Some((best_len, best_qe)) => Some((best_len, best_qe)),
                };
            }
        }
        best
    }

    fn can_find(&self, packages: &Vec<u64>, held_packages: HashSet<u64>, remaining: u64) -> bool {
        if remaining == 0 {
            log::debug!("Found matching phase 1: {held_packages:?}");
            return true;
        }
        for package in packages {
            if *package > remaining {
                continue;
            }
            if held_packages.contains(package) {
                continue;
            }
            let mut held_packages = held_packages.clone();
            held_packages.insert(*package);
            let sum: u64 = held_packages.iter().sum();
            let remaining = remaining - *package;
            log::debug!("grabbed {package}: {held_packages:?} {sum} {remaining}");
            if self.can_find(packages, held_packages, remaining) {
                return true;
            }
        }
        false
    }
}
