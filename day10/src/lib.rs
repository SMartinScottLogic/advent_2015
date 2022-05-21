use anyhow::Result;

pub fn init(input: &str) -> Result<Solution> {
    let mut solution = Solution::new();
    solution.set_input(input);
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    input: String,

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
            input: String::new(),
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
    fn set_input(&mut self, input: &str) {
        self.input = input.to_owned();
    }

    fn analyse_part1(&self) -> Option<i64> {
        let mut input = self.input.clone();
        log::debug!("{input}");
        for _ in 1..=40 {
            input = self.analyse_step(&input);
            log::debug!("{input}");
        }
        Some(input.bytes().count() as i64)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut input = self.input.clone();
        log::debug!("{input}");
        for _ in 1..=50 {
            input = self.analyse_step(&input);
            log::debug!("{input}");
        }
        Some(input.bytes().count() as i64)
    }

    fn analyse_step(&self, input: &str) -> String {
        let mut last_char = None;
        let mut output = String::new();
        let mut count = 0;
        for c in input.chars() {
            match last_char {
                None => {
                    last_char = Some(c);
                    count = 1;
                }
                Some(lc) if c == lc => {
                    count += 1;
                }
                Some(lc) => {
                    output.push_str(&format!("{count}{lc}"));
                    last_char = Some(c);
                    count = 1;
                }
            }
            if last_char.is_none() {
                last_char = Some(c);
                continue;
            }
        }
        if let Some(c) = last_char {
            output.push_str(&format!("{count}{c}"));
        }
        output
    }
}
