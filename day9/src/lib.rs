use anyhow::{Error, Result};
use std::{
    collections::HashMap,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for s in reader.lines().flatten() {
        solution.add_path(Path::from_str(&s).unwrap());
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    paths: Vec<Path>,

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
            paths: Vec::new(),
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
    fn add_path(&mut self, path: Path) {
        self.paths.push(path.reverse());
        self.paths.push(path);
    }

    fn analyse_part1(&self) -> Option<i64> {
        let locations = self.paths.iter().fold(HashSet::new(), |mut a, v| {
            a.insert(v.a.as_str());
            a.insert(v.b.as_str());
            a
        });
        println!("{locations:?}");

        let mut min_cost = None;
        for location in &locations {
            let mut visited = HashSet::new();
            visited.insert(location.to_string());
            let cost = self.analyse_part1_r(location, &locations, &mut visited).unwrap();
            min_cost = match min_cost {
                None => Some(cost),
                Some(v) => Some(std::cmp::min(v, cost))
            };
            println!("{location} {cost:?}");
        }
        min_cost
    }

    fn analyse_part2(&self) -> Option<i64> {
        let locations = self.paths.iter().fold(HashSet::new(), |mut a, v| {
            a.insert(v.a.as_str());
            a.insert(v.b.as_str());
            a
        });
        println!("{locations:?}");

        let mut max_cost = None;
        for location in &locations {
            let mut visited = HashSet::new();
            visited.insert(location.to_string());
            let cost = self.analyse_part2_r(location, &locations, &mut visited).unwrap();
            max_cost = match max_cost {
                None => Some(cost),
                Some(v) => Some(std::cmp::max(v, cost))
            };
            println!("{location} {cost:?}");
        }
        max_cost
    }

    fn analyse_part1_r(&self, cur_location: &str, locations: &HashSet<&str>, visited: &mut HashSet<String>) -> Option<i64> {
        let mut min_cost = None;
        for location in locations {
            if visited.contains(&location.to_string()) {
                continue;
            }
            if let Some(path) = self.paths.iter().find(|p| p.a==cur_location && p.b == *location) {
                visited.insert(location.to_string());
                let inner_cost = self.analyse_part1_r(location, locations, visited);
                visited.remove(&location.to_string());
                let this_cost = inner_cost.unwrap_or(0) + path.distance;
                min_cost = match min_cost {
                    None => Some(this_cost),
                    Some(v) => Some(std::cmp::min(v, this_cost))
                };
            }
        }
        min_cost
    }

    fn analyse_part2_r(&self, cur_location: &str, locations: &HashSet<&str>, visited: &mut HashSet<String>) -> Option<i64> {
        let mut max_cost = None;
        for location in locations {
            if visited.contains(&location.to_string()) {
                continue;
            }
            if let Some(path) = self.paths.iter().find(|p| p.a==cur_location && p.b == *location) {
                visited.insert(location.to_string());
                let inner_cost = self.analyse_part2_r(location, locations, visited);
                visited.remove(&location.to_string());
                let this_cost = inner_cost.unwrap_or(0) + path.distance;
                max_cost = match max_cost {
                    None => Some(this_cost),
                    Some(v) => Some(std::cmp::max(v, this_cost))
                };
            }
        }
        max_cost
    }
}

#[derive(Debug)]
struct Path {
    a: String,
    b: String,
    distance: i64
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Path {
    fn reverse(&self) -> Self {
        Self { a: self.b.clone(), b: self.a.clone(), distance: self.distance }
    }
}

impl FromStr for Path {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^(?P<a>[0-9a-zA-Z]+) to (?P<b>[0-9a-zA-Z]+) = (?P<distance>[0-9]+)$").unwrap();
        if let Some(cap) = r.captures(s) {
            let a = cap.name("a").unwrap().as_str().to_owned();
            let b = cap.name("b").unwrap().as_str().to_owned();
            let distance: i64 = cap.name("distance").unwrap().as_str().parse().unwrap();
            Ok(Self { a, b, distance })
        } else {
            println!("{s} failed to match");
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use utils::map;

    #[test]
    fn parsing() -> Result<()> {
        /*
        let tests = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        for test in tests {
            Signal::from_str(test).unwrap();
        }
        */
        Ok(())
    }
}
