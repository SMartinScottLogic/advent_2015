use std::{collections::HashSet, fmt::Debug, str::FromStr};

use regex::Regex;

pub trait Rule {
    fn source(&self) -> &str;
    fn target(&self) -> &str;
}

#[derive(Debug)]
pub struct Grammar<T> {
    rules: Vec<T>,
}

impl<T: Debug + Clone + Rule> Grammar<T> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: &T) {
        self.rules.push(rule.clone());
    }

    pub fn convert_to_cnf<F>(&self, start_symbol: &str, splitter: F) -> CNFGrammar
    where
        F: Fn(&str) -> Vec<String>,
    {
        let rules = self
            .rules
            .iter()
            .map(|rule| {
                let target_chain = splitter(rule.target());
                CNFRule::new(rule.source(), &target_chain)
            })
            .collect::<Vec<_>>();
        let known_symbols = rules
            .iter()
            .flat_map(|rule| {
                std::iter::once(rule.source.to_owned()).chain(rule.target.clone().into_iter())
            })
            .collect::<HashSet<_>>();

        log::debug!("known symbols: {known_symbols:?}");
        let mut cnf_grammar = CNFGrammar::new();
        for rule in &self.rules {
            let target_chain = splitter(rule.target());
            if (1..=2).contains(&target_chain.len()) {
                cnf_grammar.add_rule(&CNFRule::new(rule.source(), &target_chain));
            } else {
                log::error!(
                    "unhandled source rule (len: {}): {:?}",
                    target_chain.len(),
                    rule
                );
            }
        }
        cnf_grammar
    }
}

#[derive(Debug, Default)]
pub struct CNFGrammar {
    rules: Vec<CNFRule>,
}

impl CNFGrammar {
    pub fn rules(&self) -> std::slice::Iter<'_, CNFRule> {
        self.rules.iter()
    }

    fn new() -> Self {
        Self { rules: Vec::new() }
    }

    fn add_rule(&mut self, rule: &CNFRule) {
        self.rules.push(rule.clone());
    }
}

impl Clone for CNFGrammar {
    fn clone(&self) -> Self {
        Self {
            rules: self.rules.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleRule {
    pub source: String,
    pub target: String,
}

impl Rule for SimpleRule {
    fn source(&self) -> &str {
        &self.source
    }

    fn target(&self) -> &str {
        &self.target
    }
}

impl FromStr for SimpleRule {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"^(?P<source>[a-zA-Z]+) => (?P<target>[a-zA-Z ]+)$").unwrap();

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

#[derive(Debug, Clone)]
pub struct CNFRule {
    pub source: String,
    pub target: Vec<String>,
}

impl CNFRule {
    fn new(source: &str, target: &Vec<String>) -> Self {
        Self {
            source: source.to_owned(),
            target: target.to_owned(),
        }
    }
}
