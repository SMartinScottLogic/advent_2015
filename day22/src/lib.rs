use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
// You need to bring the trait into scope to use it!
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use once_cell::sync::Lazy;

static GLOBAL_BEST_KNOWN: Lazy<Mutex<Option<u64>>> = Lazy::new(|| Mutex::new(None));

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
        let player = Character {
            hit_points: 50,
            mana: 500,
            hard_difficulty: false,
            ..Character::default()
        };
        Self::fight(player, self.boss.clone())
    }

    fn analyse_part2(&mut self) -> Option<u64> {
        let player = Character {
            hit_points: 50,
            mana: 500,
            hard_difficulty: true,
            ..Character::default()
        };
        Self::fight(player, self.boss.clone())
    }

    fn fight(player: Character, boss: Character) -> Option<u64> {
        {
            let mut best_known = GLOBAL_BEST_KNOWN.lock().unwrap();
            *best_known = Some(1400);
        }
        let mut best_cost = None;
        for spell in Spell::iter() {
            if spell == Spell::None {
                continue;
            }
            if let Some(cost) = Self::fight_pass(player.clone(), boss.clone(), spell, 0) {
                best_cost = match best_cost {
                    Some(bc) if bc < cost => best_cost,
                    _ => Some(cost),
                }
            }
        }
        best_cost
    }

    fn fight_pass(
        mut player: Character,
        mut boss: Character,
        mut spell: Spell,
        mana_used: u64,
    ) -> Option<u64> {
        {
            let best_known = GLOBAL_BEST_KNOWN.lock().unwrap();
            if let Some(best) = *best_known {
                if mana_used > best {
                    log::info!(
                        "Abort, already have a better solution ({}): {} {:?}",
                        best,
                        player.hard_difficulty,
                        player.cast_spells
                    );
                    return None;
                }
            }
        }
        // 1. Player turn
        log::debug!("-- Player turn --");
        log::debug!(
            "- Player has {} hit point(s), {} armour, {} mana",
            player.hit_points,
            player.armour,
            player.mana
        );
        log::debug!("- Boss has {} hit point(s)", boss.hit_points);
        // 1.0 Difficulty
        if player.hard_difficulty {
            if player.hit_points <= 1 {
                log::debug!("Difficulty penalty, player dies");
                return None;
            }
            player.hit_points -= 1;
        }
        // 1.1 Active spell effects
        player.apply_spells(&mut boss);
        if boss.hit_points == 0 {
            log::debug!("Boss dead");
            // Boss dies, no mana used this pass
            return Some(mana_used);
        }
        // 1.2 Player cast spell
        if player.mana < spell.cost() {
            log::debug!("Can't cast {:?}, insufficient mana.", spell);
            spell = Spell::None;
        }
        if let Some(turns) = player.spells.get(&spell) {
            if *turns > 0 {
                log::debug!("Can't cast {:?}, previous cast still active.", spell);
                return None;
            }
        }
        let cost = spell.cost();
        player.mana -= cost;
        match spell {
            Spell::None => {
                log::debug!("Player waits.");
            }
            Spell::MagicMissile => {
                let damage = 4;
                if boss.hit_points <= damage {
                    // Boss dies
                    log::debug!(
                        "{:?} deals {} damage. This kills the boss, and the player wins.",
                        spell,
                        damage
                    );
                    return Some(mana_used + spell.cost());
                }
                boss.hit_points -= damage;
                log::debug!("Player casts {:?}, dealing {} damage.", spell, damage);
            }
            Spell::Drain => {
                let drain = 2;
                player.hit_points += drain;
                if boss.hit_points <= drain {
                    // Boss dies
                    log::debug!("Player casts Drain, dealing {} damage, and healing {} hit points. This kills the boss, and the player wins.", drain, drain);
                    return Some(mana_used + spell.cost());
                }
                boss.hit_points -= drain;
                log::debug!(
                    "Player casts Drain, dealing {} damage, and healing {} hit points.",
                    drain,
                    drain
                );
            }
            Spell::Shield => {
                player.spells.insert(spell, 6);
                player.armour += 7;
                log::debug!("Player casts Shield, increasing armor by 7.");
            }
            Spell::Poison => {
                player.spells.insert(spell, 6);
                log::debug!("Player casts Poison.");
            }
            Spell::Recharge => {
                player.spells.insert(spell, 5);
                log::debug!("Player casts Recharge.");
            }
        };
        player.cast_spells.push(spell);
        if player.armour > 7 {
            log::debug!("player: {:?}", player);
            panic!("ffs");
        }
        // 2. Boss turn
        log::debug!("-- Boss turn --");
        log::debug!(
            "- Player has {} hit point(s), {} armour, {} mana",
            player.hit_points,
            player.armour,
            player.mana
        );
        log::debug!("- Boss has {} hit point(s)", boss.hit_points);
        // 2.1 Active spell effects
        player.apply_spells(&mut boss);
        if boss.hit_points == 0 {
            // Boss dies
            return Some(mana_used + cost);
        }
        // 2.2 Boss attack
        let damage = if boss.damage <= player.armour {
            1
        } else {
            boss.damage - player.armour
        };

        log::debug!(
            "Boss attacks for {} - {} = {} damage!",
            boss.damage,
            player.armour,
            damage
        );
        if player.hit_points <= damage {
            log::debug!("Player dead.");
            return None;
        }
        player.hit_points -= damage;
        let mut best_cost = None;
        for spell in Spell::iter() {
            if spell == Spell::None {
                continue;
            }
            if let Some(cost) =
                Self::fight_pass(player.clone(), boss.clone(), spell, mana_used + cost)
            {
                log::info!("Defeated boss, spending {} mana", cost);
                best_cost = match best_cost {
                    Some(bc) if bc < cost => best_cost,
                    _ => Some(cost),
                }
            }
        }
        if let Some(this_best) = best_cost {
            let mut best_known = GLOBAL_BEST_KNOWN.lock().unwrap();
            *best_known = match *best_known {
                Some(cost) if cost < this_best => Some(cost),
                _ => Some(this_best),
            };
        }

        best_cost
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Character {
    hit_points: u64,
    damage: u64,
    armour: u64,
    mana: u64,
    spells: HashMap<Spell, u64>,
    cast_spells: Vec<Spell>,
    hard_difficulty: bool,
}

impl Character {
    fn apply_spells(&mut self, other: &mut Character) {
        for (active_spell, turns) in self.spells.clone() {
            if turns == 0 {
                continue;
            }
            self.spells
                .entry(active_spell)
                .and_modify(|turns| *turns -= 1);
            active_spell.apply(self, other, turns - 1);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
enum Spell {
    None,
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u64 {
        match self {
            Self::None => 0,
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn apply(&self, player: &mut Character, boss: &mut Character, turns: u64) {
        match self {
            Spell::Shield => {
                log::debug!("Shield's timer is now {}.", turns);
                if turns == 0 {
                    log::debug!("Shield wears off, decreasing armor by 7.");
                    player.armour -= 7;
                }
            }
            Spell::Poison => {
                let damage = 3;
                if boss.hit_points <= damage {
                    boss.hit_points = 0;
                } else {
                    boss.hit_points -= 3;
                }
                log::debug!(
                    "Poison deals {} damage; its timer is now {}.",
                    damage,
                    turns
                );
            }
            Spell::Recharge => {
                player.mana += 101;
                log::debug!("Recharge provides 101 mana; its timer is now {}.", turns);
            }
            _ => unreachable!(),
        }
    }
}
