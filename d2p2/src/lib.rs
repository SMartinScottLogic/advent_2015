use anyhow::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::min;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = line?;
        let line = Present::from_str(&line)?;
        solution.add(line);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    presents: Vec<Present>,

    answer: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Present {
    l: i32,
    w: i32,
    h: i32,
}

impl FromStr for Present {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<l>\d+)x(?P<w>\d+)x(?P<h>\d+)$").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        let l = cap.name("l").unwrap().as_str().parse().unwrap();
        let w = cap.name("w").unwrap().as_str().parse().unwrap();
        let h = cap.name("h").unwrap().as_str().parse().unwrap();

        Ok(Self { l, w, h })
    }
}

impl Present {
    fn required_paper(&self) -> i64 {
        let actual_need = 2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l;
        let smallest = min(min(self.l * self.w, self.w * self.h), self.h * self.l);
        (actual_need + smallest).into()
    }
    fn required_ribbon(&self) -> i64 {
        let wrap = 2 * min(min(self.l + self.w, self.w + self.h), self.h + self.l) as i64;
        let bow = (self.l * self.w * self.h) as i64;
        wrap + bow
    }
}

impl Solution {
    pub fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, present: Present) {
        self.presents.push(present);
    }

    pub fn analyse(&mut self) {
        let total = self.presents.iter().map(|p| p.required_ribbon()).sum();
        self.answer = Some(total);
    }

    pub fn answer(&self) -> Option<i64> {
        self.answer
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn known_paper() -> Result<()> {
        let m = map![Present::from_str("2x3x4")? => 58, Present::from_str("1x1x10")? => 43];
        for (input, expected) in m {
            assert_eq!(expected, input.required_paper());
        }
        Ok(())
    }

    #[test]
    fn known_ribbon() -> Result<()> {
        let m = map![Present::from_str("2x3x4")? => 34, Present::from_str("1x1x10")? => 14];
        for (input, expected) in m {
            assert_eq!(expected, input.required_ribbon());
        }
        Ok(())
    }
}
