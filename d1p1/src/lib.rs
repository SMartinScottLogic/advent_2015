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

pub struct Solution {
    instructions: String,
}

impl From<String> for Solution {
    fn from(input: String) -> Self {
        Solution {
            instructions: input,
        }
    }
}

impl From<&str> for Solution {
    fn from(input: &str) -> Self {
        Solution {
            instructions: input.into(),
        }
    }
}

impl Solution {
    pub fn answer(&self) -> i32 {
        self.instructions
            .chars()
            .map(|v| {
                let s = match v {
                    '(' => 1,
                    ')' => -1,
                    _ => unreachable!(),
                };
                log::debug!("{} => {}", v, s);
                s
            })
            .sum()
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
        let m = map!["(())" => 0, "()()" => 0,
            "(((" => 3, "(()(()(" => 3,
            "))(((((" => 3,
            "())" => -1, "))(" => -1,
            ")))" => -3, ")())())" => -3];
        for (input, expected) in m {
            let solution = Solution::from(input);
            let actual = solution.answer();
            assert_eq!(expected, actual);
        }
    }
}
