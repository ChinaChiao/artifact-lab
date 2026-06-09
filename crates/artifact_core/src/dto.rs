use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstatDTO {
    pub name: String,
    pub value: f64,
    pub roll_count: usize,
    pub rolls: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDTO {
    pub set: String,
    pub source: String,
    pub slot: String,
    pub rarity: u8,
    pub level: i32,
    pub main_stat: String,
    pub main_stat_value: f64,
    pub initial_substat_count: u8,
    pub substats: Vec<SubstatDTO>,
    pub crit_value: f64,
    pub enhancement_log: Vec<EnhancementEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementEvent {
    pub level: i32,
    pub kind: String,
    pub stat: String,
    pub roll: f64,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll_count: Option<usize>,
}
