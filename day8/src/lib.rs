use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for s in reader.lines().flatten() {
        solution.add_entry(Entry::from_str(&s).unwrap());
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    entries: Vec<Entry>,

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
            entries: Vec::new(),
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
    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn analyse_part1(&self) -> Option<i64> {
        let mut code_len = 0_i64;
        let mut mem_len = 0_i64;
        for e in &self.entries {
            code_len += e.code as i64;
            mem_len += e.memory as i64;
        }
        Some(code_len - mem_len)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut code_len = 0_i64;
        let mut encoded_len = 0_i64;
        for e in &self.entries {
            code_len += e.code as i64;
            encoded_len += Entry::encode(&e.input).chars().count() as i64;
        }
        Some(encoded_len - code_len)
    }
}

#[derive(Debug)]
struct Entry {
    code: usize,
    memory: usize,
    input: String,
}

impl Entry {
    fn encode(s: &str) -> String {
        let o: String = s
            .chars()
            .map(|c| match c {
                '"' => r#"\""#.to_string(),
                '\\' => r#"\\"#.to_string(),
                _ => format!("{c}"),
            })
            .collect();
        format!("\"{o}\"")
    }

    fn memory(s: &str) -> usize {
        let mut memory = 0;
        let mut escape = false;
        let mut in_hex = false;
        let mut in_string = false;
        let mut hex_remaining = 0;
        for c in s.chars() {
            memory += match c {
                '"' if !in_string => {
                    in_string = true;
                    0
                }
                '"' if escape => {
                    escape = false;
                    1
                }
                '"' if in_string => {
                    in_string = false;
                    0
                }
                _ if !in_string => {
                    unreachable!()
                }
                '\\' if escape => {
                    escape = false;
                    1
                }
                '\\' => {
                    escape = true;
                    0
                }
                'x' if escape => {
                    in_hex = true;
                    hex_remaining = 2;
                    0
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' if escape && in_hex && hex_remaining > 1 => {
                    hex_remaining -= 1;
                    0
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' if escape && in_hex => {
                    hex_remaining -= 1;
                    escape = false;
                    in_hex = false;
                    1
                }
                _ if escape => unreachable!("unexpected char {} in escape {}", c, s),
                _ => 1,
            };
        }
        memory
    }
}

impl FromStr for Entry {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.bytes().count();
        let memory = Self::memory(s);
        println!("{s}: {code} {memory}");
        Ok(Self {
            code,
            memory,
            input: s.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use utils::map;

    #[test]
    fn parsing() -> Result<()> {
        let tests = map![
            r#""""# => (2, 0),
            r#""abc""# => (5, 3),
            r#""aaa\"aaa""# => (10, 7),
            r#""\x27""# => (6, 1)
        ];
        for (test, (code, memory)) in tests {
            let entry = Entry::from_str(test).unwrap();
            assert_eq!(entry.code, code);
            assert_eq!(entry.memory, memory);
        }
        Ok(())
    }

    #[test]
    fn encoding() -> Result<()> {
        let tests = map![
            r#""""# => r#""\"\"""#,
            r#""abc""# => r#""\"abc\"""#,
            r#""aaa\"aaa""# => r#""\"aaa\\\"aaa\"""#,
            r#""\x27""# => r#""\"\\x27\"""#

        ];
        for (test, expected) in tests {
            let result = Entry::encode(test);
            println!("{expected} vs {result}");
            assert_eq!(expected, result);
        }
        Ok(())
    }
}
