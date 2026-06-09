use crate::dto::{EnhancementEvent, ItemDTO};
use crate::error::Result;
#[cfg(test)]
use crate::generator::main_stat_value_for_name;
use crate::generator::{GeneratedItem, GeneratedSubstat, SharedGenerator};
use crate::rules::Game;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RelicSubstat {
    pub name: String,
    pub value: f64,
    pub rolls: Vec<f64>,
}

impl RelicSubstat {
    pub fn create<R: Rng>(name: String, rng: &mut R) -> Self {
        GeneratedSubstat::create(Game::Hsr, name, rng).into()
    }

    pub fn upgrade<R: Rng>(&mut self, rng: &mut R) -> f64 {
        let mut generated: GeneratedSubstat = self.clone().into();
        let roll = generated.upgrade(Game::Hsr, rng);
        *self = generated.into();
        roll
    }

    pub fn to_dto(&self) -> crate::dto::SubstatDTO {
        let generated: GeneratedSubstat = self.clone().into();
        generated.to_dto()
    }
}

impl From<GeneratedSubstat> for RelicSubstat {
    fn from(value: GeneratedSubstat) -> Self {
        Self {
            name: value.name,
            value: value.value,
            rolls: value.rolls,
        }
    }
}

impl From<RelicSubstat> for GeneratedSubstat {
    fn from(value: RelicSubstat) -> Self {
        Self {
            name: value.name,
            value: value.value,
            rolls: value.rolls,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsrRelic {
    pub set: String,
    pub source: String,
    pub slot: String,
    pub rarity: u8,
    pub level: i32,
    pub main_stat: String,
    pub initial_substat_count: u8,
    #[serde(skip)]
    pub substats: Vec<RelicSubstat>,
    pub enhancement_log: Vec<EnhancementEvent>,
}

impl HsrRelic {
    pub fn crit_value(&self) -> f64 {
        let generated: GeneratedItem = self.clone().into();
        generated.crit_value()
    }

    pub fn speed_value(&self) -> f64 {
        let generated: GeneratedItem = self.clone().into();
        generated.speed_value()
    }

    pub fn to_dto(&self) -> ItemDTO {
        let generated: GeneratedItem = self.clone().into();
        generated.to_dto()
    }
}

impl From<GeneratedItem> for HsrRelic {
    fn from(value: GeneratedItem) -> Self {
        Self {
            set: value.set,
            source: value.source,
            slot: value.slot,
            rarity: value.rarity,
            level: value.level,
            main_stat: value.main_stat,
            initial_substat_count: value.initial_substat_count,
            substats: value.substats.into_iter().map(Into::into).collect(),
            enhancement_log: value.enhancement_log,
        }
    }
}

impl From<HsrRelic> for GeneratedItem {
    fn from(value: HsrRelic) -> Self {
        Self {
            game: Game::Hsr,
            set: value.set,
            source: value.source,
            slot: value.slot,
            rarity: value.rarity,
            level: value.level,
            main_stat: value.main_stat,
            initial_substat_count: value.initial_substat_count,
            substats: value.substats.into_iter().map(Into::into).collect(),
            enhancement_log: value.enhancement_log,
        }
    }
}

pub struct HsrGenerator {
    inner: SharedGenerator,
}

impl HsrGenerator {
    pub fn new(sets: Vec<String>, source: String) -> Result<Self> {
        Ok(Self {
            inner: SharedGenerator::new(Game::Hsr, sets, source)?,
        })
    }

    pub fn generate<R: Rng>(&self, level: i32, rng: &mut R) -> Result<HsrRelic> {
        self.inner.generate(level, rng).map(Into::into)
    }

    pub fn enhance_once<R: Rng>(
        &self,
        relic: &mut HsrRelic,
        rng: &mut R,
    ) -> Result<EnhancementEvent> {
        let mut generated: GeneratedItem = relic.clone().into();
        let event = self.inner.enhance_once(&mut generated, rng)?;
        *relic = generated.into();
        Ok(event)
    }
}

#[cfg(test)]
fn main_stat_value(stat: &str, level: i32) -> f64 {
    main_stat_value_for_name(Game::Hsr, stat, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{self, Source};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_generate_cavern_relic() {
        let sets = vec!["musketeer_of_wild_wheat".to_string()];
        let generator = HsrGenerator::new(sets, "cavern".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let relic = generator.generate(0, &mut rng).unwrap();

        assert_eq!(relic.rarity, 5);
        assert_eq!(relic.level, 0);
        assert!(relic.substats.len() >= 3);
        assert!(relic.substats.len() <= 4);
        assert!(rules::slots_for_source(Game::Hsr, Source::Cavern)
            .iter()
            .any(|slot| slot.as_str() == relic.slot));
    }

    #[test]
    fn test_generate_planar_relic() {
        let sets = vec!["space_sealing_station".to_string()];
        let generator = HsrGenerator::new(sets, "planar".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let relic = generator.generate(0, &mut rng).unwrap();

        assert_eq!(relic.source, "planar");
        assert!(rules::slots_for_source(Game::Hsr, Source::Planar)
            .iter()
            .any(|slot| slot.as_str() == relic.slot));
    }

    #[test]
    fn test_enhance_relic() {
        let sets = vec!["musketeer_of_wild_wheat".to_string()];
        let generator = HsrGenerator::new(sets, "cavern".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let mut relic = generator.generate(0, &mut rng).unwrap();

        let initial_level = relic.level;
        generator.enhance_once(&mut relic, &mut rng).unwrap();
        assert_eq!(relic.level, initial_level + 3);
    }

    #[test]
    fn test_max_level() {
        let sets = vec!["musketeer_of_wild_wheat".to_string()];
        let generator = HsrGenerator::new(sets, "cavern".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let mut relic = generator.generate(15, &mut rng).unwrap();

        assert_eq!(relic.level, 15);
        assert!(generator.enhance_once(&mut relic, &mut rng).is_err());
    }

    #[test]
    fn test_invalid_level() {
        let sets = vec!["musketeer_of_wild_wheat".to_string()];
        let generator = HsrGenerator::new(sets, "cavern".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);

        assert!(generator.generate(5, &mut rng).is_err());
        assert!(generator.generate(-1, &mut rng).is_err());
        assert!(generator.generate(16, &mut rng).is_err());
    }

    #[test]
    fn test_main_stat_value_max_level() {
        let value = main_stat_value("hp_flat", 15);
        assert_eq!(value, 705.6);
    }

    #[test]
    fn test_main_stat_value_truncation() {
        let value = main_stat_value("hp_flat", 9);
        assert_eq!(value, 468.0);

        let value = main_stat_value("speed", 9);
        assert_eq!(value, 16.0);

        let value = main_stat_value("crit_rate", 9);
        assert_eq!(value, 21.51);
    }
}
