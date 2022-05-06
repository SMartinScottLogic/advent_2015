use anyhow::{Error, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for s in reader.lines().flatten() {
        solution.add_string(s);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    strings: Vec<String>,

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
            strings: Vec::new(),
            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn analyse(&mut self) {
        let (nice, _naughty): (Vec<_>, Vec<_>) = self
            .strings
            .iter()
            .partition(|s| Judgement1::from_str(s).unwrap() == Judgement1::Nice);
        self.answer_part1 = Some(nice.len() as i64);

        let (nice, _naughty): (Vec<_>, Vec<_>) = self
            .strings
            .iter()
            .partition(|s| Judgement2::from_str(s).unwrap() == Judgement2::Nice);
        self.answer_part2 = Some(nice.len() as i64);
    }

    pub fn add_string(&mut self, input: String) {
        self.strings.push(input);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

#[derive(Debug, PartialEq)]
enum Judgement1 {
    Nice,
    Naughty,
}

impl Judgement1 {
    fn count_vowels(s: &str) -> usize {
        s.chars()
            .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
            .count()
    }

    fn contains_naughty_part(s: &str) -> bool {
        s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
    }

    fn contains_pair(s: &str) -> bool {
        let mut last = None;
        for c in s.chars() {
            last = match last {
                Some(last_c) if c == last_c => return true,
                _ => Some(c),
            }
        }
        false
    }
}

impl FromStr for Judgement1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_vowels = Self::count_vowels(s);
        let contains_naughty = Self::contains_naughty_part(s);
        let contains_pair = Self::contains_pair(s);
        let judgement = if num_vowels >= 3 && !contains_naughty && contains_pair {
            Self::Nice
        } else {
            Self::Naughty
        };
        Ok(judgement)
    }
}

#[derive(Debug, PartialEq)]
enum Judgement2 {
    Nice,
    Naughty,
}

impl Judgement2 {
    fn has_duplicate_pair(s: &str) -> bool {
        let mut last = None;
        let mut pairs = HashMap::new();
        for (idx, c) in s.chars().enumerate() {
            if let Some(last_c) = last {
                pairs
                    .entry((last_c, c))
                    .or_insert_with(Vec::new)
                    .push(idx - 1);
            }
            last = Some(c);
        }
        pairs
            .into_iter()
            .filter(|(_, o)| o.len() >= 2)
            .any(|(_, o)| {
                let first = o.first().copied().unwrap();
                let last = o.last().copied().unwrap();
                last - first >= 2
            })
    }

    fn has_jump_pair(s: &str) -> bool {
        let mut last2 = None;
        let mut last1 = None;
        for c in s.chars() {
            if let Some(last_c2) = last2 {
                if last_c2 == c {
                    return true;
                }
            }
            last2 = last1;
            last1 = Some(c);
        }
        false
    }
}

impl FromStr for Judgement2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let has_duplicate_pair = Self::has_duplicate_pair(s);
        let has_jump_pair = Self::has_jump_pair(s);
        println!("{s} {has_duplicate_pair} {has_jump_pair}");
        let judgement = if has_duplicate_pair && has_jump_pair {
            Self::Nice
        } else {
            Self::Naughty
        };
        Ok(judgement)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use utils::map;

    #[test]
    fn known_results_part1() -> Result<()> {
        let m = map![
        "ugknbfddgicrmopn" => Judgement1::Nice,
        "aaa" => Judgement1::Nice,
        "jchzalrnumimnmhp" => Judgement1::Naughty,
        "haegwjzuvuyypxyu" => Judgement1::Naughty,
        "dvszwmarrgswjxmb" => Judgement1::Naughty
            ];

        for (input, expected) in m {
            assert_eq!(Judgement1::from_str(input).unwrap(), expected);
        }
        Ok(())
    }

    #[test]
    fn known_results_part2() -> Result<()> {
        let m = map![
        "qjhvhtzxzqqjkmpb" => Judgement2::Nice,
        "xxyxx" => Judgement2::Nice,
        "uurcxstgmygtbstg" => Judgement2::Naughty,
        "ieodomkazucvgmuy" => Judgement2::Naughty
            ];

        for (input, expected) in m {
            assert_eq!(Judgement2::from_str(input).unwrap(), expected);
        }
        Ok(())
    }
}
