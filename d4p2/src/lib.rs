use anyhow::{Error, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let solution = Solution::from_str(&line)?;
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    prefix: String,

    answer: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        let mut val = 0i64;
        let answer = loop {
            if Self::start(&self.prefix, val) == "000000" {
                break Some(val);
            }
            val += 1;
        };
        self.answer = answer;
    }

    fn start(prefix: &str, val: i64) -> String {
        let digest = md5::compute(format!("{}{}", prefix, val));
        format!("{:x}", digest).chars().take(6).collect()
    }

    pub fn answer(&self) -> Option<i64> {
        self.answer
    }
}

impl FromStr for Solution {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            prefix: s.to_string(),
            answer: None,
        })
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }
}

