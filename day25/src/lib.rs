use anyhow::{Result, Context};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let solution = Solution::from_str(&buf)?;
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    row: u64,
    column: u64,

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
            row: 0,
            column: 0,
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
    fn analyse_part1(&mut self) -> Option<u64> {
        let mut r = 1;
        let mut c = 1;
        let mut code = 20151125_u64;
        loop {
            log::debug!("({r}, {c}) = {code}");
            if r == self.row && c == self.column {
                break Some(code);
            }
            code *= 252533;
            code %= 33554393;
            if r == 1 {
                r = c+1;
                c = 1;

            } else {
                r -= 1;
                c += 1;
            }
        }
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        None
    }
}

impl FromStr for Solution {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r".*row (?P<row>[0-9]+), column (?P<column>[0-9]+)\.")?;
        let captures = r.captures(s).context("context")?;
        let row = captures.name("row").context("get capture <row>")?.as_str().parse()?;
        let column = captures.name("column").context("get capture <column>")?.as_str().parse()?;
        Ok(Self {
            row,
            column,
            ..Self::default()
        })
    }
}
//row 2947, column 3029
