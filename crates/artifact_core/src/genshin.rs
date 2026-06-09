use crate::dto::{EnhancementEvent, ItemDTO};
use crate::error::Result;
#[cfg(test)]
use crate::generator::main_stat_value_for_name;
use crate::generator::{GeneratedItem, GeneratedSubstat, SharedGenerator};
use crate::rules::Game;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Substat {
    pub name: String,
    pub value: f64,
    pub rolls: Vec<f64>,
}

impl Substat {
    pub fn create<R: Rng>(name: String, rng: &mut R) -> Self {
        GeneratedSubstat::create(Game::Genshin, name, rng).into()
    }

    pub fn upgrade<R: Rng>(&mut self, rng: &mut R) -> f64 {
        let mut generated: GeneratedSubstat = self.clone().into();
        let roll = generated.upgrade(Game::Genshin, rng);
        *self = generated.into();
        roll
    }

    pub fn to_dto(&self) -> crate::dto::SubstatDTO {
        let generated: GeneratedSubstat = self.clone().into();
        generated.to_dto()
    }
}

impl From<GeneratedSubstat> for Substat {
    fn from(value: GeneratedSubstat) -> Self {
        Self {
            name: value.name,
            value: value.value,
            rolls: value.rolls,
        }
    }
}

impl From<Substat> for GeneratedSubstat {
    fn from(value: Substat) -> Self {
        Self {
            name: value.name,
            value: value.value,
            rolls: value.rolls,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenshinArtifact {
    pub set: String,
    pub source: String,
    pub slot: String,
    pub rarity: u8,
    pub level: i32,
    pub main_stat: String,
    pub initial_substat_count: u8,
    #[serde(skip)]
    pub substats: Vec<Substat>,
    pub enhancement_log: Vec<EnhancementEvent>,
}

impl GenshinArtifact {
    pub fn crit_value(&self) -> f64 {
        let generated: GeneratedItem = self.clone().into();
        generated.crit_value()
    }

    pub fn to_dto(&self) -> ItemDTO {
        let generated: GeneratedItem = self.clone().into();
        generated.to_dto()
    }
}

impl From<GeneratedItem> for GenshinArtifact {
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

impl From<GenshinArtifact> for GeneratedItem {
    fn from(value: GenshinArtifact) -> Self {
        Self {
            game: Game::Genshin,
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

pub struct GenshinGenerator {
    inner: SharedGenerator,
}

impl GenshinGenerator {
    pub fn new(sets: Vec<String>, source: String) -> Result<Self> {
        Ok(Self {
            inner: SharedGenerator::new(Game::Genshin, sets, source)?,
        })
    }

    pub fn generate<R: Rng>(&self, level: i32, rng: &mut R) -> Result<GenshinArtifact> {
        self.inner.generate(level, rng).map(Into::into)
    }

    pub fn enhance_once<R: Rng>(
        &self,
        artifact: &mut GenshinArtifact,
        rng: &mut R,
    ) -> Result<EnhancementEvent> {
        let mut generated: GeneratedItem = artifact.clone().into();
        let event = self.inner.enhance_once(&mut generated, rng)?;
        *artifact = generated.into();
        Ok(event)
    }
}

#[cfg(test)]
fn main_stat_value(stat: &str, level: i32) -> f64 {
    main_stat_value_for_name(Game::Genshin, stat, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_generate_artifact() {
        let sets = vec!["gladiators_finale".to_string()];
        let generator = GenshinGenerator::new(sets, "domain".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let artifact = generator.generate(0, &mut rng).unwrap();

        assert_eq!(artifact.rarity, 5);
        assert_eq!(artifact.level, 0);
        assert!(artifact.substats.len() >= 3);
        assert!(artifact.substats.len() <= 4);
    }

    #[test]
    fn test_enhance_artifact() {
        let sets = vec!["gladiators_finale".to_string()];
        let generator = GenshinGenerator::new(sets, "domain".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let mut artifact = generator.generate(0, &mut rng).unwrap();

        let initial_level = artifact.level;
        generator.enhance_once(&mut artifact, &mut rng).unwrap();
        assert_eq!(artifact.level, initial_level + 4);
    }

    #[test]
    fn test_max_level() {
        let sets = vec!["gladiators_finale".to_string()];
        let generator = GenshinGenerator::new(sets, "domain".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);
        let mut artifact = generator.generate(20, &mut rng).unwrap();

        assert_eq!(artifact.level, 20);
        assert!(generator.enhance_once(&mut artifact, &mut rng).is_err());
    }

    #[test]
    fn test_invalid_level() {
        let sets = vec!["gladiators_finale".to_string()];
        let generator = GenshinGenerator::new(sets, "domain".to_string()).unwrap();
        let mut rng = StdRng::seed_from_u64(42);

        assert!(generator.generate(5, &mut rng).is_err());
        assert!(generator.generate(-1, &mut rng).is_err());
        assert!(generator.generate(21, &mut rng).is_err());
    }

    #[test]
    fn test_main_stat_value_max_level() {
        let value = main_stat_value("hp_flat", 20);
        assert_eq!(value, 4780.0);
    }

    #[test]
    fn test_main_stat_value_interpolation() {
        let value = main_stat_value("hp_pct", 10);
        assert_eq!(value, 26.0);
    }
}
