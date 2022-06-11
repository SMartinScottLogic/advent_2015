use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        let rule = Rule::from_str(&line).unwrap();
        solution.add_rule(rule);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    rules: HashMap<(String, String), i64>,
    people: HashSet<String>,

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
            rules: HashMap::new(),
            people: HashSet::new(),

            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn analyse(&mut self) {
        let start = self.people.iter().next().map(|s| s.to_string());
        self.answer_part1 =
            self.analyse_part1(0, start.clone().unwrap(), start, self.people.clone());
        let me = "Me".to_string();
        self.people.insert(me.clone());
        self.answer_part2 = self.analyse_part1(0, me.clone(), Some(me), self.people.clone());
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_rule(&mut self, rule: Rule) {
        self.people.insert(rule.subject.clone());
        self.people.insert(rule.target.clone());
        self.rules
            .insert((rule.subject.clone(), rule.target.clone()), rule.score);
    }

    fn analyse_part1(
        &self,
        prior_score: i64,
        first: String,
        last: Option<String>,
        mut unseated: HashSet<String>,
    ) -> Option<i64> {
        log::debug!("{first} {last:?} {unseated:?} {prior_score}");
        if let Some(ref last) = last {
            assert!(unseated.remove(last), "Cannot occupy multiple seats");
        }

        if unseated.is_empty() {
            let mut total = prior_score;
            let last = last.unwrap();
            let score = self
                .rules
                .get(&(last.to_string(), first.clone()))
                .unwrap_or(&0);
            log::trace!("ab({last}, {first}): {score}");
            total += score;
            let score = self
                .rules
                .get(&(first.clone(), last.to_string()))
                .unwrap_or(&0);
            log::trace!("ba({first}, {last}): {score}");
            total += score;
            log::debug!("result: {total}");
            Some(total)
        } else {
            let mut best_total = None;
            for person in &unseated {
                if let Some(ref last) = last {
                    let mut prior = prior_score;
                    let score = self
                        .rules
                        .get(&(last.to_string(), person.clone()))
                        .unwrap_or(&0);
                    log::trace!("ab({last}, {person}): {score}");
                    prior += score;
                    let score = self
                        .rules
                        .get(&(person.clone(), last.to_string()))
                        .unwrap_or(&0);
                    log::trace!("ba({person}, {last}): {score}");
                    prior += score;

                    let total = self
                        .analyse_part1(prior, first.clone(), Some(person.clone()), unseated.clone())
                        .unwrap();
                    best_total = match best_total {
                        Some(t) => Some(std::cmp::max(t, total)),
                        None => Some(total),
                    };
                }
            }
            best_total
        }
    }
}

#[derive(Debug)]
struct Rule {
    subject: String,
    target: String,
    score: i64,
}

// Alice would gain 2 happiness units by sitting next to Bob.
impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^(?P<subject>[^\s]+) would (?P<sign>[^\s]+) (?P<scale>\d+) happiness units by sitting next to (?P<target>[^\.]+)\.$").unwrap();
        let c = r.captures(s).unwrap();
        let subject = c.name("subject").unwrap().as_str().to_string();
        let sign = Sign::from_str(c.name("sign").unwrap().as_str()).unwrap();
        let scale: i64 = c.name("scale").unwrap().as_str().parse().unwrap();

        let score = match sign {
            Sign::Gain => scale,
            Sign::Lose => -scale,
        };
        let target = c.name("target").unwrap().as_str().to_string();

        Ok(Rule {
            subject,
            target,
            score,
        })
    }
}

#[derive(Debug)]
enum Sign {
    Lose,
    Gain,
}

impl FromStr for Sign {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lose" => Ok(Self::Lose),
            "gain" => Ok(Self::Gain),
            _ => panic!("Invalid sign: {s}"),
        }
    }
}
