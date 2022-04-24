use anyhow::{Error, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let solution = Solution::from_str(&line)?;
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    definition: Vec<Direction>,

    answer: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        self.answer = Some(self.num_houses());
    }

    pub fn answer(&self) -> Option<i64> {
        self.answer
    }

    fn num_houses(&self) -> i64 {
        let mut cur_x = [0; 2];
        let mut cur_y = [0; 2];
        let mut num_visits = std::collections::HashMap::new();
        *num_visits.entry((cur_x[0], cur_y[0])).or_insert(0) += 1;
        *num_visits.entry((cur_x[1], cur_y[1])).or_insert(0) += 1;
        for (idx, d) in self.definition.iter().enumerate() {
            let idx = idx % 2;
            match d {
                Direction::North => cur_y[idx] += 1,
                Direction::East => cur_x[idx] += 1,
                Direction::South => cur_y[idx] -= 1,
                Direction::West => cur_x[idx] -= 1,
            }
            *num_visits.entry((cur_x[idx], cur_y[idx])).or_insert(0) += 1;
        }
        num_visits.len() as i64
    }
}

impl FromStr for Solution {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            definition: s.chars().map(Direction::from).collect(),
            answer: None,
        })
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use utils::map;

    #[test]
    fn known_results() -> Result<()> {
        let m = map![
            "^v" => 3,
            "^>v<" => 3,
            "^v^v^v^v^v" => 11];

        for (input, expected) in m {
            let route = Solution::from_str(input)?;
            assert_eq!(expected, route.num_houses());
        }
        Ok(())
    }
}
