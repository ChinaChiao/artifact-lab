use crate::dto::{EnhancementEvent, ItemDTO, SubstatDTO};
use crate::error::{ArtifactError, Result};
use crate::rules::{self, Game, Source, Stat};
use rand::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct GeneratedSubstat {
    pub name: String,
    pub value: f64,
    pub rolls: Vec<f64>,
}

impl GeneratedSubstat {
    pub fn create<R: Rng>(game: Game, name: String, rng: &mut R) -> Self {
        let stat = Stat::try_from(name.as_str()).expect("substat name must be valid");
        let rolls_table = rules::substat_rolls(game, stat).expect("substat must have roll table");
        let roll = *rolls_table.choose(rng).unwrap();
        Self {
            name,
            value: round2(roll),
            rolls: vec![roll],
        }
    }

    pub fn upgrade<R: Rng>(&mut self, game: Game, rng: &mut R) -> f64 {
        let stat = Stat::try_from(self.name.as_str()).expect("substat name must be valid");
        let rolls_table = rules::substat_rolls(game, stat).expect("substat must have roll table");
        let roll = *rolls_table.choose(rng).unwrap();
        self.rolls.push(roll);
        self.value = round2(self.rolls.iter().sum());
        round2(roll)
    }

    pub fn to_dto(&self) -> SubstatDTO {
        SubstatDTO {
            name: self.name.clone(),
            value: self.value,
            roll_count: self.rolls.len(),
            rolls: self.rolls.iter().map(|&roll| round2(roll)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedItem {
    pub game: Game,
    pub set: String,
    pub source: String,
    pub slot: String,
    pub rarity: u8,
    pub level: i32,
    pub main_stat: String,
    pub initial_substat_count: u8,
    pub substats: Vec<GeneratedSubstat>,
    pub enhancement_log: Vec<EnhancementEvent>,
}

impl GeneratedItem {
    pub fn crit_value(&self) -> f64 {
        let mut crit_rate = 0.0;
        let mut crit_dmg = 0.0;
        for substat in &self.substats {
            if substat.name == Stat::CritRate.as_str() {
                crit_rate = substat.value;
            } else if substat.name == Stat::CritDmg.as_str() {
                crit_dmg = substat.value;
            }
        }
        round2(crit_rate * 2.0 + crit_dmg)
    }

    pub fn speed_value(&self) -> f64 {
        self.substats
            .iter()
            .find(|substat| substat.name == Stat::Speed.as_str())
            .map(|substat| round2(substat.value))
            .unwrap_or(0.0)
    }

    pub fn to_dto(&self) -> ItemDTO {
        ItemDTO {
            set: self.set.clone(),
            source: self.source.clone(),
            slot: self.slot.clone(),
            rarity: self.rarity,
            level: self.level,
            main_stat: self.main_stat.clone(),
            main_stat_value: main_stat_value_for_name(self.game, &self.main_stat, self.level),
            initial_substat_count: self.initial_substat_count,
            substats: self
                .substats
                .iter()
                .map(|substat| substat.to_dto())
                .collect(),
            crit_value: self.crit_value(),
            enhancement_log: self.enhancement_log.clone(),
            speed_value: if self.game == Game::Hsr {
                Some(self.speed_value())
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedGenerator {
    game: Game,
    sets: Vec<String>,
    source: Source,
}

impl SharedGenerator {
    pub fn new(game: Game, sets: Vec<String>, source: String) -> Result<Self> {
        if sets.is_empty() {
            return Err(ArtifactError::EmptySets);
        }
        let source_value = Source::try_from(source.as_str())
            .map_err(|_| ArtifactError::InvalidSource(source.clone()))?;
        if !rules::source_is_valid(game, source_value) {
            return Err(ArtifactError::InvalidSource(source));
        }
        Ok(Self {
            game,
            sets,
            source: source_value,
        })
    }

    pub fn generate<R: Rng>(&self, level: i32, rng: &mut R) -> Result<GeneratedItem> {
        validate_level(self.game, level)?;
        let slot = *rules::slots_for_source(self.game, self.source)
            .choose(rng)
            .expect("valid source must expose slots");
        let main_stat = weighted_stat(rules::main_stat_weights(self.game, slot), rng)?;
        let initial_count = self.initial_substat_count(rng);
        let substats = self.draw_substats(main_stat, initial_count, rng)?;

        let mut item = GeneratedItem {
            game: self.game,
            set: self.sets.choose(rng).unwrap().clone(),
            source: self.source.as_str().to_string(),
            slot: slot.as_str().to_string(),
            rarity: 5,
            level: 0,
            main_stat: main_stat.as_str().to_string(),
            initial_substat_count: initial_count,
            substats,
            enhancement_log: vec![],
        };

        while item.level < level {
            self.enhance_once(&mut item, rng)?;
        }

        Ok(item)
    }

    pub fn enhance_once<R: Rng>(
        &self,
        item: &mut GeneratedItem,
        rng: &mut R,
    ) -> Result<EnhancementEvent> {
        if item.level >= rules::max_level(self.game) {
            return Err(ArtifactError::MaxLevelReached);
        }

        let next_level = item.level + rules::level_step(self.game);
        let event = if item.substats.len() < 4 {
            let existing: HashSet<_> = item
                .substats
                .iter()
                .map(|substat| substat.name.as_str())
                .collect();
            let main_stat = Stat::try_from(item.main_stat.as_str())
                .map_err(|_| ArtifactError::InvalidMainStat)?;
            let pool: Vec<_> = rules::substat_weights(self.game)
                .iter()
                .copied()
                .filter(|entry| entry.stat != main_stat && !existing.contains(entry.stat.as_str()))
                .collect();
            let stat = weighted_stat(&pool, rng)?;
            let substat = GeneratedSubstat::create(self.game, stat.as_str().to_string(), rng);
            let roll = substat.rolls[0];
            let value = substat.value;
            item.substats.push(substat);
            EnhancementEvent {
                level: next_level,
                kind: "new_substat".to_string(),
                stat: stat.as_str().to_string(),
                roll: round2(roll),
                value,
                roll_count: None,
            }
        } else {
            let substat = item.substats.choose_mut(rng).unwrap();
            let roll = substat.upgrade(self.game, rng);
            EnhancementEvent {
                level: next_level,
                kind: "upgrade_substat".to_string(),
                stat: substat.name.clone(),
                roll,
                value: substat.value,
                roll_count: Some(substat.rolls.len()),
            }
        };

        item.level = next_level;
        item.enhancement_log.push(event.clone());
        Ok(event)
    }

    fn initial_substat_count<R: Rng>(&self, rng: &mut R) -> u8 {
        match self.game {
            Game::Genshin => {
                let prob = rules::initial_three_substat_prob(self.source).unwrap_or(0.0);
                if rng.gen::<f64>() < prob {
                    3
                } else {
                    4
                }
            }
            Game::Hsr => {
                if rng.gen::<f64>() < 0.20 {
                    4
                } else {
                    3
                }
            }
        }
    }

    fn draw_substats<R: Rng>(
        &self,
        main_stat: Stat,
        count: u8,
        rng: &mut R,
    ) -> Result<Vec<GeneratedSubstat>> {
        let mut chosen = Vec::new();
        let mut existing = HashSet::new();

        for _ in 0..count {
            let pool: Vec<_> = rules::substat_weights(self.game)
                .iter()
                .copied()
                .filter(|entry| entry.stat != main_stat && !existing.contains(&entry.stat))
                .collect();
            let stat = weighted_stat(&pool, rng)?;
            existing.insert(stat);
            chosen.push(GeneratedSubstat::create(
                self.game,
                stat.as_str().to_string(),
                rng,
            ));
        }

        Ok(chosen)
    }
}

pub fn validate_level(game: Game, level: i32) -> Result<()> {
    let max_level = rules::max_level(game);
    let step = rules::level_step(game);
    if level < 0 || level > max_level || level % step != 0 {
        return Err(ArtifactError::InvalidLevel(level));
    }
    Ok(())
}

pub fn main_stat_value_for_name(game: Game, stat: &str, level: i32) -> f64 {
    Stat::try_from(stat)
        .ok()
        .and_then(|stat| rules::main_stat_value(game, stat, level))
        .expect("main stat must be valid")
}

fn weighted_stat<R: Rng>(weights: &[rules::WeightedStat], rng: &mut R) -> Result<Stat> {
    if weights.is_empty() {
        return Err(ArtifactError::EmptyWeightPool);
    }
    let total: f64 = weights.iter().map(|entry| entry.weight).sum();
    if total <= 0.0 {
        return Err(ArtifactError::EmptyWeightPool);
    }
    let mut roll = rng.gen::<f64>() * total;
    for entry in weights {
        roll -= entry.weight;
        if roll <= 0.0 {
            return Ok(entry.stat);
        }
    }
    Ok(weights.last().unwrap().stat)
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
