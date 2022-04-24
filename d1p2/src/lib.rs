use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load(filename: &str) -> std::io::Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let solution = Solution::from(input);
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    instructions: String,

    answer: Option<i64>,
}

impl From<String> for Solution {
    fn from(input: String) -> Self {
        Solution {
            instructions: input,
            answer: None,
        }
    }
}

impl From<&str> for Solution {
    fn from(input: &str) -> Self {
        Solution {
            instructions: input.into(),
            answer: None,
        }
    }
}

impl Solution {
    pub fn analyse(&mut self) {
        let mut floor = 0i32;
        for (pos, v) in self.instructions.chars().enumerate() {
            let delta = match v {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            };
            floor += delta;
            log::debug!("{pos}: {floor}");
            if floor == -1 {
                self.answer = Some(1 + (pos as i64));
                return;
            }
        }
        self.answer = None;
    }
    pub fn answer(&self) -> Option<i64> {
        self.answer
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn known_results() {
        let m = map![")" => 1, "()())" => 5];
        for (input, expected) in m {
            let mut solution = Solution::from(input);
            solution.analyse();
            let actual = solution.answer();
            assert_eq!(Some(expected), actual);
        }
    }
}
