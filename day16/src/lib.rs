use anyhow::Result;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use utils::map;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        let sue = Sue::from_str(&line).unwrap();
        solution.add_aunt(sue);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    aunts: Vec<Sue>,

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
            aunts: Vec::new(),

            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn analyse(&mut self) {
        self.answer_part1 = self.analyse_part1();
        self.answer_part2 = self.analyse_part2();
    }

    pub fn answer_part1(&self) -> Option<u64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<u64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_aunt(&mut self, sue: Sue) {
        self.aunts.push(sue);
    }

    fn analyse_part1(&self) -> Option<u64> {
        let target = map! {
            "children" => 3,
            "cats" => 7,
            "samoyeds" => 2,
            "pomeranians" => 3,
            "akitas" => 0,
            "vizslas" => 0,
            "goldfish" => 5,
            "trees" => 3,
            "cars" => 2,
            "perfumes" => 1
        };
        self.aunts
            .iter()
            .find(|aunt| aunt.matches_part1(&target))
            .map(|aunt| aunt.id)
    }

    fn analyse_part2(&self) -> Option<u64> {
        let target = map! {
            "children" => 3,
            "cats" => 7,
            "samoyeds" => 2,
            "pomeranians" => 3,
            "akitas" => 0,
            "vizslas" => 0,
            "goldfish" => 5,
            "trees" => 3,
            "cars" => 2,
            "perfumes" => 1
        };
        self.aunts
            .iter()
            .find(|aunt| aunt.matches_part2(&target))
            .map(|aunt| aunt.id)
    }
}

#[derive(Debug, Default)]
struct Sue {
    id: u64,
    expected: HashMap<String, u64>,
}

impl Sue {
    fn matches_part1(&self, target: &HashMap<&str, u64>) -> bool {
        log::trace!("{:?} vs {:?}", self, target);
        for (compound, count) in target {
            if let Some(c) = self.expected.get(*compound) {
                log::trace!("{} {} vs {}", compound, c, count);
                if c != count {
                    return false;
                }
            }
        }
        log::debug!("{:?} matches {:?}", self, target);
        true
    }

    fn matches_part2(&self, target: &HashMap<&str, u64>) -> bool {
        log::trace!("{:?} vs {:?}", self, target);
        for (compound, count) in target {
            if let Some(c) = self.expected.get(*compound) {
                log::trace!("{} {} vs {}", compound, c, count);
                match *compound {
                    "cats" | "trees" => {
                        if count >= c {
                            return false;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if count <= c {
                            return false;
                        }
                    }
                    _ => {
                        if count != c {
                            return false;
                        }
                    }
                }
            }
        }
        log::debug!("{:?} matches {:?}", self, target);
        true
    }
}

impl FromStr for Sue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r1 = regex::Regex::new(r"^Sue (?P<id>\d+): (?P<compounds>.*)$").unwrap();
        let r2 = regex::Regex::new(r"^(?P<compound>\w+): (?P<count>\d+)$").unwrap();
        let c1 = r1.captures(s).unwrap();
        let id = c1.name("id").unwrap().as_str().parse::<u64>().unwrap();
        let compounds = c1.name("compounds").unwrap().as_str();
        let expected = compounds.split(", ").fold(HashMap::new(), |mut acc, s| {
            let c = r2.captures(s).unwrap();
            let compound = c.name("compound").unwrap().as_str().to_owned();
            let count = c.name("count").unwrap().as_str().parse::<u64>().unwrap();
            acc.insert(compound, count);
            acc
        });
        Ok(Sue { id, expected })
    }
}
