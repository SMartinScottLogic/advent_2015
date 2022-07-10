use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            solution.set(x, y, c);
        }
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    data1: HashMap<(usize, usize), char>,
    data2: HashMap<(usize, usize), char>,
    maxx: usize,
    maxy: usize,

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
            data1: HashMap::new(),
            data2: HashMap::new(),
            maxx: 0,
            maxy: 0,

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
    fn set(&mut self, x: usize, y: usize, c: char) {
        self.data1.insert((x, y), c);
        self.data2.insert((x, y), c);
        if x > self.maxx {
            self.maxx = x;
        }
        if y > self.maxy {
            self.maxy = y;
        }
    }

    fn analyse_part1(&mut self) -> Option<u64> {
        for _step in 1..=100 {
            self.analyse_part1_step();
        }
        let mut count = 0;
        for v in self.data1.values() {
            count += match v {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            };
        }
        Some(count)
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        for _step in 1..=100 {
            self.analyse_part2_step();
        }
        let mut count = 0;
        for v in self.data2.values() {
            count += match v {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            };
        }
        Some(count)
    }

    /// Generate neighbour counts
    fn gen_neigh(data: &HashMap<(usize, usize), char>) -> HashMap<(i64, i64), i32> {
        let mut neigh = HashMap::new();
        for ((sx, sy), v) in data {
            if *v != '#' {
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let tx = dx + *sx as i64;
                    let ty = dy + *sy as i64;

                    *neigh.entry((tx, ty)).or_insert(0) += 1;
                }
            }
        }
        neigh
    }

    fn analyse_part1_step(&mut self) {
        let neigh = Self::gen_neigh(&self.data1);
        for ((x, y), v) in self.data1.iter_mut() {
            let num = *neigh.get(&(*x as i64, *y as i64)).unwrap_or(&0);
            let nv = match v {
                '#' if (2..=3).contains(&num) => '#',
                '#' => '.',
                '.' if (num == 3) => '#',
                '.' => '.',
                _ => unreachable!(),
            };
            *v = nv;
        }
    }

    fn corners_on(data: &mut HashMap<(usize, usize), char>, maxx: usize, maxy: usize) {
        data.insert((0, 0), '#');
        data.insert((0, maxy), '#');
        data.insert((maxx, 0), '#');
        data.insert((maxx, maxy), '#');
    }

    fn analyse_part2_step(&mut self) {
        Self::corners_on(&mut self.data2, self.maxx, self.maxy);
        let neigh = Self::gen_neigh(&self.data2);
        for y in 0..=self.maxy {
            for x in 0..=self.maxx {
                let v = self.data2.entry((x, y)).or_insert('.');
                if x == 0 && y == 0 {
                    *v = '#';
                    continue;
                }
                if x == 0 && y == self.maxy {
                    *v = '#';
                    continue;
                }
                if x == self.maxx && y == 0 {
                    *v = '#';
                    continue;
                }
                if x == self.maxx && y == self.maxy {
                    *v = '#';
                    continue;
                }
                let num = *neigh.get(&(x as i64, y as i64)).unwrap_or(&0);
                let nv = match v {
                    '#' if (2..=3).contains(&num) => '#',
                    '#' => '.',
                    '.' if (num == 3) => '#',
                    '.' => '.',
                    _ => unreachable!(),
                };
                *v = nv;
            }
        }
    }
}
