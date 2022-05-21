use anyhow::Result;
use std::collections::HashSet;

pub fn init(input: &str) -> Result<Solution> {
    let mut solution = Solution::new();
    solution.set_input(input);
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    input: String,

    answer_part1: Option<String>,
    answer_part2: Option<String>,
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
            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn analyse(&mut self) {
        self.answer_part1 = Self::analyse_str(&self.input);
        self.answer_part2 = Self::analyse_str(self.answer_part1.as_ref().unwrap());
    }

    pub fn answer_part1(&self) -> Option<&String> {
        self.answer_part1.as_ref()
    }

    pub fn answer_part2(&self) -> Option<&String> {
        self.answer_part2.as_ref()
    }
}

impl Solution {
    fn set_input(&mut self, input: &str) {
        self.input = input.to_owned();
    }

    fn analyse_str(input: &str) -> Option<String> {
        let mut input = input.to_string();
        loop {
            input = Self::next(&input, 7);

            if Self::is_permitted(&input) {
                break;
            }
        }
        Some(input)
    }

    fn has_only_permitted(s: &str) -> bool {
        !(s.contains('i') || s.contains('o') || s.contains('l'))
    }

    fn contains_two_pairs(s: &str) -> bool {
        let mut pairs = HashSet::new();
        for i in 0..=6 {
            if s.chars().nth(i).unwrap() == s.chars().nth(i + 1).unwrap() {
                pairs.insert(s.get(i..=i + 1).unwrap());
            }
        }
        pairs.len() > 1
    }

    fn contains_run(s: &str) -> bool {
        for i in 0..=5 {
            let ci0 = s.chars().nth(i).unwrap() as u32;
            let ci1 = s.chars().nth(i + 1).unwrap() as u32;
            let ci2 = s.chars().nth(i + 2).unwrap() as u32;
            if ci0 == ci1 - 1 && ci1 == ci2 - 1 {
                return true;
            }
        }
        false
    }

    fn is_permitted(s: &str) -> bool {
        Self::has_only_permitted(s) && Self::contains_two_pairs(s) && Self::contains_run(s)
    }

    fn next(input: &str, pos: usize) -> String {
        let mut output = input.to_string();
        match input.chars().nth(pos).unwrap() {
            'z' => {
                output.replace_range(pos..=pos, "a");
                if pos == 0 {
                    output
                } else {
                    Self::next(&output, pos - 1)
                }
            }
            c => {
                let next_char = std::char::from_u32(c as u32 + 1).unwrap();
                output.replace_range(pos..=pos, &format!("{next_char}"));
                output
            }
        }
    }
}
