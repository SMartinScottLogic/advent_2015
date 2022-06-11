use anyhow::Result;
use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{AddAssign, Mul};
use std::str::FromStr;

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        let ingredient = Ingredient::from_str(&line).unwrap();
        solution.add_ingredient(ingredient);
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    ingredients: Vec<Ingredient>,

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
            ingredients: Vec::new(),

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
    fn add_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

    fn analyse_part1(&self) -> Option<i64> {
        Some(Self::inner_part1(self, 0, 100, Ingredient::default()))
    }

    fn analyse_part2(&self) -> Option<i64> {
        Some(Self::inner_part2(self, 0, 100, Ingredient::default()))
    }

    fn inner_part1(&self, index: usize, remaining_teaspoons: i64, properties: Ingredient) -> i64 {
        // Last ingredient must use ALL remaining
        if index == self.ingredients.len() - 1 {
            let mut i = self.ingredients.get(index).unwrap() * remaining_teaspoons;
            i += &properties;
            i.clamp();
            let score = i.capacity * i.durability * i.flavor * i.texture;
            log::debug!("part1 end: {i:?} {score}");
            score
        } else {
            let mut best_score = 0;
            for spoons in 0..=remaining_teaspoons {
                let mut total = self.ingredients.get(index).unwrap() * spoons;
                total += &properties;
                let total = self.inner_part1(index + 1, remaining_teaspoons - spoons, total);
                if total > best_score {
                    best_score = total;
                }
            }
            best_score
        }
    }

    fn inner_part2(&self, index: usize, remaining_teaspoons: i64, properties: Ingredient) -> i64 {
        // Last ingredient must use ALL remaining
        if index == self.ingredients.len() - 1 {
            let mut i = self.ingredients.get(index).unwrap() * remaining_teaspoons;
            i += &properties;
            i.clamp();
            let score = match i.calories {
                500 => i.capacity * i.durability * i.flavor * i.texture,
                _ => 0,
            };
            log::debug!("part2 end: {i:?} {score}");
            score
        } else {
            let mut best_score = 0;
            for spoons in 0..=remaining_teaspoons {
                let mut total = self.ingredients.get(index).unwrap() * spoons;
                total += &properties;
                let total = self.inner_part2(index + 1, remaining_teaspoons - spoons, total);
                if total > best_score {
                    best_score = total;
                }
            }
            best_score
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn clamp(&mut self) {
        self.capacity = std::cmp::max(0, self.capacity);
        self.durability = std::cmp::max(0, self.durability);
        self.flavor = std::cmp::max(0, self.flavor);
        self.texture = std::cmp::max(0, self.texture);
        self.calories = std::cmp::max(0, self.calories);
    }
}

impl AddAssign<&Ingredient> for Ingredient {
    fn add_assign(&mut self, rhs: &Ingredient) {
        self.name += " ";
        self.name += &rhs.name;
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
    }
}

impl Mul<i64> for &Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::Output {
            name: format!("{}x{}", rhs, self.name),
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl FromStr for Ingredient {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)$").unwrap();

        let c = r.captures(s).unwrap();
        let name = c.name("name").unwrap().as_str().to_string();
        let capacity = c.name("capacity").unwrap().as_str().parse().unwrap();
        let durability = c.name("durability").unwrap().as_str().parse().unwrap();
        let flavor = c.name("flavor").unwrap().as_str().parse().unwrap();
        let texture = c.name("texture").unwrap().as_str().parse().unwrap();
        let calories = c.name("calories").unwrap().as_str().parse().unwrap();

        Ok(Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}
