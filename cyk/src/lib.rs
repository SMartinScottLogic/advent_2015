use anyhow::Result;
use grammar::{Grammar, SimpleRule};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod grammar;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        if let Ok(rule) = SimpleRule::from_str(&line) {
            solution.add_rule(rule);
            continue;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        solution.set_input(line.to_string());
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    input: String,
    grammar: Grammar<SimpleRule>,

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
            input: String::new(),
            grammar: Grammar::new(),

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
    fn add_rule(&mut self, rule: SimpleRule) {
        self.grammar.add_rule(&rule);
    }

    fn set_input(&mut self, input: String) {
        self.input = input;
    }

    fn split(s: &str) -> Vec<String> {
        let v = s.chars().rev().collect::<String>();
        let v = v
            .split_inclusive(char::is_uppercase)
            .map(|s| s.chars().rev().collect::<String>())
            .rev()
            .collect();
        log::debug!(r#"split "{s}" into {v:?}"#);
        v
    }

    fn analyse_part1(&mut self) -> Option<u64> {
        let words = Self::split(&self.input);
        let cnf_grammar = self.grammar.convert_to_cnf("e", Self::split);
        log::debug!("{cnf_grammar:?}");
        let nt: HashSet<_> = cnf_grammar
            .rules()
            .map(|rule| rule.source.to_owned())
            .collect();
        let mut nonterms = HashMap::new();
        for r in nt {
            let idx = nonterms.len();
            nonterms.insert(r, idx);
        }

        log::debug!("nonterms: {:?}", nonterms);
        log::debug!("words: {:?}", words);
        for (s, word) in words.iter().enumerate() {
            for rule in cnf_grammar.rules() {
                if rule.target.len() == 1 && rule.target.get(0).unwrap() == word {
                    let v = nonterms.get(&rule.source).unwrap();
                    log::debug!("{word}: {rule:?} => P[1, {s}, {v}] = true");
                }
            }
        }

        None
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        None
    }
}
