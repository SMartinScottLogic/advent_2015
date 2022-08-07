use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> Result<Solution> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    for line in reader.lines().flatten() {
        solution.update_boss(&line)?;
    }
    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    boss: Character,

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
            boss: Character::default(),

            answer_part1: None,
            answer_part2: None,
        }
    }

    pub fn analyse(&mut self) {
        self.answer_part1 = self.analyse_part1();
        log::info!("part1: {:?}", self.answer_part1);
        self.answer_part2 = self.analyse_part2();
        log::info!("part2: {:?}", self.answer_part2);
    }

    pub fn answer_part1(&self) -> Option<u64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<u64> {
        self.answer_part2
    }
}

impl Solution {
    fn update_boss(&mut self, update: &str) -> Result<()> {
        let update = update.trim();
        if update.is_empty() {
            return Ok(());
        }
        let (attr, value) = update
            .split_once(':')
            .context("should have an attribute name")?;
        let value = value.trim().parse()?;
        log::info!("{attr} <- {value}");
        match attr.trim().to_lowercase().as_str() {
            "hit points" => self.boss.hit_points = value,
            "damage" => self.boss.damage = value,
            "armor" => self.boss.armour = value,
            _ => unreachable!("unknown attribute"),
        };
        Ok(())
    }

    fn analyse_part1(&mut self) -> Option<u64> {
        let all_weapons = vec![
            Equipment::new("Dagger", 8, 4, 0),
            Equipment::new("Shortsword", 10, 5, 0),
            Equipment::new("Warhammer", 25, 6, 0),
            Equipment::new("Longsword", 40, 7, 0),
            Equipment::new("Greataxe", 74, 8, 0),
        ];
        let all_armour = vec![
            None,
            Some(Equipment::new("Leather", 13, 0, 1)),
            Some(Equipment::new("Chainmail", 31, 0, 2)),
            Some(Equipment::new("Splintmail", 53, 0, 3)),
            Some(Equipment::new("Bandedmail", 75, 0, 4)),
            Some(Equipment::new("Platemail", 102, 0, 5)),
        ];
        let all_rings = vec![
            None,
            Some(Equipment::new("Damage +1", 25, 1, 0)),
            Some(Equipment::new("Damage +2", 50, 2, 0)),
            Some(Equipment::new("Damage +3", 100, 3, 0)),
            Some(Equipment::new("Defense +1", 20, 0, 1)),
            Some(Equipment::new("Defense +2", 40, 0, 2)),
            Some(Equipment::new("Defense +3", 80, 0, 3)),
        ];
        let no_rings = vec![None];

        let mut best_cost = None;
        for weapon in all_weapons {
            for armour in &all_armour {
                for ring1 in &all_rings {
                    for ring2 in if ring1.is_some() {
                        &all_rings
                    } else {
                        &no_rings
                    } {
                        if Self::same(ring1, ring2) {
                            continue;
                        }
                        let mut cost = weapon.cost;
                        cost += match armour {
                            Some(a) => a.cost,
                            _ => 0,
                        };
                        cost += match ring1 {
                            Some(a) => a.cost,
                            _ => 0,
                        };
                        cost += match ring2 {
                            Some(a) => a.cost,
                            _ => 0,
                        };

                        let mut player = Character::default();
                        player.hit_points += 100;
                        player.armour += weapon.armour;
                        player.damage += weapon.damage;
                        if let Some(e) = armour {
                            player.armour += e.armour;
                            player.damage += e.damage;
                        };
                        if let Some(e) = ring1 {
                            player.armour += e.armour;
                            player.damage += e.damage;
                        };
                        if let Some(e) = ring2 {
                            player.armour += e.armour;
                            player.damage += e.damage;
                        };
                        log::info!("{cost}: {player:?} {weapon:?} {armour:?} {ring1:?} {ring2:?}");
                        if (best_cost.is_none() || cost < best_cost.unwrap())
                            && Self::fight(player, self.boss)
                        {
                            best_cost = Some(cost);
                        }
                    }
                }
            }
        }
        best_cost
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        let all_weapons = vec![
            Equipment::new("Dagger", 8, 4, 0),
            Equipment::new("Shortsword", 10, 5, 0),
            Equipment::new("Warhammer", 25, 6, 0),
            Equipment::new("Longsword", 40, 7, 0),
            Equipment::new("Greataxe", 74, 8, 0),
        ];
        let all_armour = vec![
            None,
            Some(Equipment::new("Leather", 13, 0, 1)),
            Some(Equipment::new("Chainmail", 31, 0, 2)),
            Some(Equipment::new("Splintmail", 53, 0, 3)),
            Some(Equipment::new("Bandedmail", 75, 0, 4)),
            Some(Equipment::new("Platemail", 102, 0, 5)),
        ];
        let all_rings = vec![
            None,
            Some(Equipment::new("Damage +1", 25, 1, 0)),
            Some(Equipment::new("Damage +2", 50, 2, 0)),
            Some(Equipment::new("Damage +3", 100, 3, 0)),
            Some(Equipment::new("Defense +1", 20, 0, 1)),
            Some(Equipment::new("Defense +2", 40, 0, 2)),
            Some(Equipment::new("Defense +3", 80, 0, 3)),
        ];
        let no_rings = vec![None];

        let mut best_cost = None;
        for weapon in all_weapons {
            for armour in &all_armour {
                for ring1 in &all_rings {
                    for ring2 in if ring1.is_some() {
                        &all_rings
                    } else {
                        &no_rings
                    } {
                        if Self::same(ring1, ring2) {
                            continue;
                        }
                        let mut cost = weapon.cost;
                        cost += match armour {
                            Some(a) => a.cost,
                            _ => 0,
                        };
                        cost += match ring1 {
                            Some(a) => a.cost,
                            _ => 0,
                        };
                        cost += match ring2 {
                            Some(a) => a.cost,
                            _ => 0,
                        };

                        let mut player = Character::default();
                        player.hit_points += 100;
                        player.armour += weapon.armour;
                        player.damage += weapon.damage;
                        if let Some(e) = armour {
                            player.armour += e.armour;
                            player.damage += e.damage;
                        };
                        if let Some(e) = ring1 {
                            player.armour += e.armour;
                            player.damage += e.damage;
                        };
                        if let Some(e) = ring2 {
                            player.armour += e.armour;
                            player.damage += e.damage;
                        };
                        log::info!("{cost}: {player:?} {weapon:?} {armour:?} {ring1:?} {ring2:?}");
                        if (best_cost.is_none() || cost > best_cost.unwrap())
                            && !Self::fight(player, self.boss)
                        {
                            best_cost = Some(cost);
                        }
                    }
                }
            }
        }
        best_cost
    }

    fn fight(mut player: Character, mut boss: Character) -> bool {
        loop {
            let damage = if player.damage <= boss.armour {
                1
            } else {
                player.damage - boss.armour
            };
            if boss.hit_points <= damage {
                return true;
            }
            boss.hit_points -= damage;

            let damage = if boss.damage <= player.armour {
                1
            } else {
                boss.damage - player.armour
            };
            if player.hit_points <= damage {
                return false;
            }
            player.hit_points -= damage;
        }
    }

    fn same(lhs: &Option<Equipment>, rhs: &Option<Equipment>) -> bool {
        match (lhs, rhs) {
            (None, None) => true,
            (Some(l), Some(r)) if l.name == r.name => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct Character {
    hit_points: u64,
    damage: u64,
    armour: u64,
}

#[derive(Debug)]
struct Equipment {
    name: String,
    cost: u64,
    damage: u64,
    armour: u64,
}

impl Equipment {
    fn new(name: &str, cost: u64, damage: u64, armour: u64) -> Self {
        Self {
            name: name.to_string(),
            cost,
            damage,
            armour,
        }
    }
}
