use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        if let Ok(Replacement { source, target }) = Replacement::from_str(&line) {
            assert!(source.len() <= target.len());
            solution.add_replacement(source, target);
            continue;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        solution.set_molecule(line.to_string());
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    replacements: Vec<(String, String)>,
    molecule: String,

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
            replacements: Vec::new(),
            molecule: String::new(),

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
    fn add_replacement(&mut self, source: String, target: String) {
        self.replacements.push((source, target));
    }

    fn set_molecule(&mut self, molecule: String) {
        self.molecule = molecule;
    }

    fn analyse_part1(&mut self) -> Option<u64> {
        let mut resultants = HashSet::new();
        for idx in 0..self.molecule.len() {
            for (source, target) in &self.replacements {
                if idx + source.len() <= self.molecule.len()
                    && source == &self.molecule[idx..idx + source.len()]
                {
                    log::debug!(
                        "{} X {} @ {} + {} / {}",
                        self.molecule,
                        source,
                        idx,
                        source.len(),
                        self.molecule.len()
                    );
                    let new_molecule = if idx + source.len() < self.molecule.len() {
                        format!(
                            "{}{}{}",
                            &self.molecule[0..idx],
                            target,
                            &self.molecule[idx + source.len()..]
                        )
                    } else {
                        format!("{}{}", &self.molecule[0..idx], target)
                    };
                    log::debug!("{} X {} = {}", self.molecule, source, new_molecule);
                    resultants.insert(new_molecule);
                }
            }
        }
        Some(resultants.len() as u64)
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        //None
        /*
        for idx in 0..self.molecule.len() {
            for (source, target) in &self.replacements {
                if idx + target.len() <= self.molecule.len() && target == &self.molecule[idx..idx+target.len()] {
                    log::debug!("{} => {}", source, target);
                }
            }
        }
        self.analyse_part2_step(self.molecule.clone(), 0)
        */
        self.analyse_part2_step(self.molecule.clone(), 0)
    }

    fn analyse_part2_step(&self, cur_molecule: String, num_changes: u64) -> Option<u64> {
        // Match
        if cur_molecule == "e" {
            return Some(num_changes);
        }
        if cur_molecule.len() <= 1 {
            panic!();
        }
        let mut best_source: Option<&String> = None;
        let mut best_target: Option<&String> = None;
        for (source, target) in &self.replacements {
            if let Some(b) = best_target {
                if b.len() > target.len() {
                    continue;
                }
            }
            if cur_molecule.contains(target) {
                best_target = Some(target);
                best_source = Some(source);
            }
        }
        match best_target {
            Some(b) => {
                let next_molecule = cur_molecule.replacen(b, best_source.unwrap(), 1);
                self.analyse_part2_step(next_molecule, num_changes + 1)
            }
            None => panic!(),
        }
    }
}

struct Replacement {
    source: String,
    target: String,
}

impl FromStr for Replacement {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"^(?P<source>[a-zA-Z]+) => (?P<target>[a-zA-Z]+)$").unwrap();

        let captures = match r.captures(s) {
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, s)),
            Some(c) => c,
        };
        let source = captures
            .name("source")
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, s))?
            .as_str()
            .to_owned();
        let target = captures
            .name("target")
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, s))?
            .as_str()
            .to_owned();
        Ok(Self { source, target })
    }
}
