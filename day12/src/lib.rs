use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let v: serde_json::Value = serde_json::from_str(&line)?;
    log::debug!("{v:?}");
    let mut solution = Solution::new();
    solution.set_raw_value(v);
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    raw_value: serde_json::Value,

    answer_part1: Option<f64>,
    answer_part2: Option<f64>,
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            raw_value: serde_json::Value::Null,
            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn analyse(&mut self) {
        self.answer_part1 = Self::analyse_part1(&self.raw_value);
        self.answer_part2 = Self::analyse_part2(&self.raw_value);
    }

    pub fn answer_part1(&self) -> Option<f64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<f64> {
        self.answer_part2
    }
}

impl Solution {
    fn set_raw_value(&mut self, value: serde_json::Value) {
        self.raw_value = value;
    }

    fn analyse_part1(value: &serde_json::Value) -> Option<f64> {
        let r = match value {
            serde_json::Value::Null => Some(0_f64),
            serde_json::Value::Bool(_) => Some(0_f64),
            serde_json::Value::Number(v) => v.as_f64(),
            serde_json::Value::String(_) => Some(0_f64),
            serde_json::Value::Array(a) => a.iter().map(Self::analyse_part1).sum(),
            serde_json::Value::Object(o) => o.iter().map(|(_, v)| Self::analyse_part1(v)).sum(),
        };

        log::debug!("{value:?} -> {r:?}");
        r
    }

    fn analyse_part2(value: &serde_json::Value) -> Option<f64> {
        let r = match value {
            serde_json::Value::Null => Some(0_f64),
            serde_json::Value::Bool(_) => Some(0_f64),
            serde_json::Value::Number(v) => v.as_f64(),
            serde_json::Value::String(_) => Some(0_f64),
            serde_json::Value::Array(a) => a.iter().map(Self::analyse_part2).sum(),
            serde_json::Value::Object(o) => {
                if o.iter().any(|(_, v)| {
                    if let serde_json::Value::String(v) = v {
                        v == "red"
                    } else {
                        false
                    }
                }) {
                    Some(0_f64)
                } else {
                    o.iter().map(|(_, v)| Self::analyse_part2(v)).sum()
                }
            }
        };

        log::debug!("{value:?} -> {r:?}");
        r
    }
}
