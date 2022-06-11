use anyhow::Result;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        let rule = Reindeer::from_str(&line).unwrap();
        solution.add_rule(rule);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    rules: Vec<Reindeer>,

    answer_part1: Option<u64>,
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
            rules: Vec::new(),

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

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_rule(&mut self, rule: Reindeer) {
        self.rules.push(rule);
    }

    fn analyse_part1(&self) -> Option<u64> {
        let mut max_distance = None;
        let time = 2503;
        for reindeer in &self.rules {
            let distance = reindeer.distance(time);
            log::debug!("{reindeer:?}");
            log::info!("part1 {}: {}", reindeer.reindeer, distance);

            max_distance = match max_distance {
                Some(max_distance) => Some(std::cmp::max(max_distance, distance)),
                _ => Some(distance),
            };
        }
        max_distance
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut state = self.rules.iter().fold(HashMap::new(), |mut state, v| {
            state.insert(v.reindeer.clone(), (true, v.flytime, 0, 0));
            state
        });
        for time in 1..=2503 {
            let mut max_position = 0;
            for reindeer in &self.rules {
                let (flying, remaining, position, _score) =
                    state.get_mut(&reindeer.reindeer).unwrap();
                *remaining -= 1;
                if *flying {
                    *position += reindeer.speed;
                }

                if *position > max_position {
                    max_position = *position;
                }

                if *remaining == 0 {
                    if *flying {
                        *flying = false;
                        *remaining = reindeer.resttime;
                    } else {
                        *flying = true;
                        *remaining = reindeer.flytime;
                    }
                }
            }
            for reindeer in &self.rules {
                let (_flying, _remaining, position, score) =
                    state.get_mut(&reindeer.reindeer).unwrap();
                if *position == max_position {
                    *score += 1;
                }
            }
            log::debug!("{time} {state:?}");
        }

        state
            .into_iter()
            .map(|(reindeer, (_, _, _, score))| {
                log::info!("part2 {}: {}", reindeer, score);
                score
            })
            .max()
    }
}

#[derive(Debug)]
struct Reindeer {
    reindeer: String,
    speed: u64,
    flytime: u64,
    resttime: u64,
}

impl Reindeer {
    fn distance(&self, mut time: u64) -> u64 {
        let mut distance = 0;

        loop {
            let flytime = std::cmp::min(time, self.flytime);
            distance += flytime * self.speed;
            time -= flytime;

            let resttime = std::cmp::min(time, self.resttime);
            time -= resttime;

            if time == 0 {
                break;
            }
        }
        distance
    }
}

// Alice would gain 2 happiness units by sitting next to Bob.
impl FromStr for Reindeer {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^(?P<reindeer>[^\s]+) can fly (?P<speed>[0-9]+) km/s for (?P<flytime>[0-9]+) seconds, but then must rest for (?P<resttime>[0-9]+) seconds.$").unwrap();

        let c = r.captures(s).unwrap();
        let reindeer = c.name("reindeer").unwrap().as_str().to_string();
        let speed = c.name("speed").unwrap().as_str().parse().unwrap();
        let flytime = c.name("flytime").unwrap().as_str().parse().unwrap();
        let resttime = c.name("resttime").unwrap().as_str().parse().unwrap();

        Ok(Reindeer {
            reindeer,
            speed,
            flytime,
            resttime,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn comet() {
        let reindeer = Reindeer {
            reindeer: "Comet".to_string(),
            speed: 14,
            flytime: 10,
            resttime: 127,
        };
        assert_eq!(1120, reindeer.distance(1000));
    }

    #[test]
    fn dancer() {
        let reindeer = Reindeer {
            reindeer: "Dancer".to_string(),
            speed: 16,
            flytime: 11,
            resttime: 162,
        };
        assert_eq!(1056, reindeer.distance(1000));
    }
}
