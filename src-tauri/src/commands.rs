use artifact_core::{GenshinGenerator, GraduationConfig, HsrGenerator};
use rand::rngs::{StdRng, ThreadRng};
use rand::SeedableRng;
use rand::{CryptoRng, Error as RandError, RngCore};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct GenerateGenshinPayload {
    pub source: String,
    pub sets: Vec<String>,
    #[serde(default)]
    pub seed: Option<u64>,
    pub level: i32,
}

#[derive(Debug, Deserialize)]
pub struct GenerateHsrPayload {
    pub source: String,
    pub sets: Vec<String>,
    #[serde(default)]
    pub seed: Option<u64>,
    pub level: i32,
}

#[derive(Debug, Deserialize)]
pub struct EnhancePayload {
    pub item: ItemInput,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub to_max: bool,
}

#[derive(Debug, Deserialize)]
pub struct ItemInput {
    pub set: String,
    pub source: String,
    pub slot: String,
    pub rarity: u8,
    pub level: i32,
    pub main_stat: String,
    pub initial_substat_count: u8,
    pub substats: Vec<SubstatInput>,
    #[serde(default)]
    pub enhancement_log: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct SubstatInput {
    pub name: String,
    pub value: f64,
    pub rolls: Vec<f64>,
}

#[tauri::command]
pub fn get_meta() -> Value {
    artifact_core::get_meta()
}

#[tauri::command]
pub fn generate_artifact(payload: GenerateGenshinPayload) -> Result<Value, String> {
    let generator =
        GenshinGenerator::new(payload.sets, payload.source).map_err(|e| e.to_string())?;

    let artifact = with_seeded_rng(payload.seed, |rng| generator.generate(payload.level, rng))
        .map_err(|e| e.to_string())?;

    let dto = artifact.to_dto();
    serde_json::to_value(dto).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn enhance_artifact(payload: EnhancePayload) -> Result<Value, String> {
    // Reconstruct artifact from input
    let mut substats = Vec::new();
    for sub in payload.item.substats {
        substats.push(artifact_core::genshin::Substat {
            name: sub.name,
            value: sub.value,
            rolls: sub.rolls,
        });
    }

    let enhancement_log: Vec<artifact_core::dto::EnhancementEvent> = payload
        .item
        .enhancement_log
        .into_iter()
        .filter_map(|v| serde_json::from_value(v).ok())
        .collect();

    let mut artifact = artifact_core::genshin::GenshinArtifact {
        set: payload.item.set,
        source: payload.item.source.clone(),
        slot: payload.item.slot,
        rarity: payload.item.rarity,
        level: payload.item.level,
        main_stat: payload.item.main_stat,
        initial_substat_count: payload.item.initial_substat_count,
        substats,
        enhancement_log,
    };

    let generator = GenshinGenerator::new(vec![artifact.set.clone()], payload.item.source)
        .map_err(|e| e.to_string())?;

    let target_level = if payload.to_max {
        20
    } else {
        artifact.level + 4
    };

    with_seeded_rng(payload.seed, |rng| {
        while artifact.level < target_level && artifact.level < 20 {
            generator.enhance_once(&mut artifact, rng)?;
        }
        Ok(())
    })
    .map_err(|e: artifact_core::ArtifactError| e.to_string())?;

    let dto = artifact.to_dto();
    serde_json::to_value(dto).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn generate_relic(payload: GenerateHsrPayload) -> Result<Value, String> {
    let generator = HsrGenerator::new(payload.sets, payload.source).map_err(|e| e.to_string())?;

    let relic = with_seeded_rng(payload.seed, |rng| generator.generate(payload.level, rng))
        .map_err(|e| e.to_string())?;

    let dto = relic.to_dto();
    serde_json::to_value(dto).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn enhance_relic(payload: EnhancePayload) -> Result<Value, String> {
    // Reconstruct relic from input
    let mut substats = Vec::new();
    for sub in payload.item.substats {
        substats.push(artifact_core::hsr::RelicSubstat {
            name: sub.name,
            value: sub.value,
            rolls: sub.rolls,
        });
    }

    let enhancement_log: Vec<artifact_core::dto::EnhancementEvent> = payload
        .item
        .enhancement_log
        .into_iter()
        .filter_map(|v| serde_json::from_value(v).ok())
        .collect();

    let mut relic = artifact_core::hsr::HsrRelic {
        set: payload.item.set,
        source: payload.item.source.clone(),
        slot: payload.item.slot,
        rarity: payload.item.rarity,
        level: payload.item.level,
        main_stat: payload.item.main_stat,
        initial_substat_count: payload.item.initial_substat_count,
        substats,
        enhancement_log,
    };

    let generator = HsrGenerator::new(vec![relic.set.clone()], payload.item.source)
        .map_err(|e| e.to_string())?;

    let target_level = if payload.to_max { 15 } else { relic.level + 3 };

    with_seeded_rng(payload.seed, |rng| {
        while relic.level < target_level && relic.level < 15 {
            generator.enhance_once(&mut relic, rng)?;
        }
        Ok(())
    })
    .map_err(|e: artifact_core::ArtifactError| e.to_string())?;

    let dto = relic.to_dto();
    serde_json::to_value(dto).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn simulate_graduation(payload: GraduationConfig) -> Result<Value, String> {
    let result = artifact_core::simulate_graduation(payload)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn summarize_graduation_average(payload: GraduationConfig) -> Result<Value, String> {
    let result = artifact_core::summarize_graduation_average(payload)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn with_seeded_rng<T, E, F>(seed: Option<u64>, f: F) -> Result<T, E>
where
    F: FnOnce(&mut CommandRng) -> Result<T, E>,
{
    let mut rng = match seed {
        Some(seed) => CommandRng::Seeded(Box::new(StdRng::seed_from_u64(seed))),
        None => CommandRng::Thread(rand::thread_rng()),
    };
    f(&mut rng)
}

enum CommandRng {
    // Box the seeded RNG: StdRng is ~320 bytes vs ThreadRng's 8, so an unboxed
    // enum would carry that footprint on every variant (clippy::large_enum_variant).
    Seeded(Box<StdRng>),
    Thread(ThreadRng),
}

impl RngCore for CommandRng {
    fn next_u32(&mut self) -> u32 {
        match self {
            Self::Seeded(rng) => rng.next_u32(),
            Self::Thread(rng) => rng.next_u32(),
        }
    }

    fn next_u64(&mut self) -> u64 {
        match self {
            Self::Seeded(rng) => rng.next_u64(),
            Self::Thread(rng) => rng.next_u64(),
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        match self {
            Self::Seeded(rng) => rng.fill_bytes(dest),
            Self::Thread(rng) => rng.fill_bytes(dest),
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), RandError> {
        match self {
            Self::Seeded(rng) => rng.try_fill_bytes(dest),
            Self::Thread(rng) => rng.try_fill_bytes(dest),
        }
    }
}

impl CryptoRng for CommandRng {}
