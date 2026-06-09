use crate::rules::{self, Game, Stat};
use crate::{GenshinGenerator, HsrGenerator};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraduationConfig {
    pub game: String,
    pub role: String,
    #[serde(default, alias = "effective_tags")]
    pub effective_tags: Vec<String>,
    pub stats: Vec<String>,
    pub base: HashMap<String, f64>,
    #[serde(default, alias = "panel_goals")]
    pub panel_goals: HashMap<String, f64>,
    #[serde(default, alias = "hit_goals")]
    pub hit_goals: HashMap<String, f64>,
    #[serde(default, alias = "main_plan")]
    pub main_plan: HashMap<String, String>,
    #[serde(alias = "max_runs")]
    pub max_runs: usize,
    #[serde(alias = "runs_per_day")]
    pub runs_per_day: f64,
    #[serde(default = "default_sample_count", alias = "sample_count")]
    pub sample_count: usize,
    #[serde(default)]
    pub seed: String,
}

fn default_sample_count() -> usize {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimSubstat {
    pub name: String,
    pub value: f64,
    pub rolls: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimItem {
    pub game: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub slot: String,
    pub main_stat: String,
    pub level: i32,
    pub substats: Vec<SimSubstat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equipped_at: Option<EquippedAt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evaluation {
    pub panel: HashMap<String, f64>,
    pub hits: HashMap<String, usize>,
    pub filled: usize,
    pub completion: f64,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub run: usize,
    pub runs: usize,
    pub days: f64,
    pub completion: f64,
    pub values: HashMap<String, f64>,
    pub hits: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneItem {
    pub slot: String,
    pub main_stat: String,
    pub crit_value: f64,
    pub substats: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildSnapshotItem {
    pub slot: String,
    #[serde(default)]
    pub empty: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_stat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crit_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substats: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    #[serde(rename = "type")]
    pub kind: String,
    pub run: usize,
    pub days: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    pub title: String,
    pub panel: HashMap<String, f64>,
    pub hits: HashMap<String, usize>,
    pub completion: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_stat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<MilestoneItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_snapshot: Option<Vec<BuildSnapshotItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plateau_run: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stall_runs: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stall_days: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquippedAt {
    #[serde(rename = "type")]
    pub kind: String,
    pub run: usize,
    pub days: f64,
    pub panel: HashMap<String, f64>,
    pub hits: HashMap<String, usize>,
    pub completion: f64,
    pub chart_stat: Option<String>,
    pub build_snapshot: Vec<BuildSnapshotItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleSimulationResult {
    pub config: GraduationConfig,
    pub build: HashMap<String, Option<SimItem>>,
    pub evaluation: Evaluation,
    pub history: Vec<Snapshot>,
    pub milestones: Vec<Milestone>,
    pub final_markers: Vec<Milestone>,
    pub plateau_markers: Vec<Milestone>,
    pub threshold_runs: HashMap<String, Option<usize>>,
    pub stop_run: usize,
    pub stop_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleResult {
    pub graduated: bool,
    pub stop_run: usize,
    pub threshold_runs: HashMap<String, Option<usize>>,
    pub final_completion: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdSummary {
    pub key: String,
    pub ratio: f64,
    pub label: String,
    pub runs: Vec<usize>,
    pub reached: usize,
    pub rate: f64,
    pub average: f64,
    pub average_days: f64,
    pub median: f64,
    pub p90: f64,
    pub p95: f64,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionProbability {
    pub ratio: f64,
    pub label: String,
    pub run: usize,
    pub days: f64,
    pub reached: usize,
    pub probability: f64,
    pub survival: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConservativeQuantile {
    pub key: String,
    pub ratio: f64,
    pub label: String,
    pub required: usize,
    pub run: f64,
    pub censored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub start: usize,
    pub end: usize,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionStats {
    pub average: f64,
    pub median: f64,
    pub p10: f64,
    pub p25: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AverageSimulationResult {
    pub done: usize,
    pub total: usize,
    pub max_runs: usize,
    pub runs_per_day: f64,
    pub success: usize,
    pub failed: usize,
    pub success_runs: Vec<usize>,
    pub success_rate: f64,
    pub average: f64,
    pub average_days: f64,
    pub median: f64,
    pub p90: f64,
    pub p95: f64,
    pub p10: f64,
    pub p25: f64,
    pub p75: f64,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub stdev: f64,
    pub buckets: Vec<Bucket>,
    pub censored_rate: f64,
    pub censored_run_lower_bound: f64,
    pub censored_days_lower_bound: f64,
    pub completion_stats: CompletionStats,
    pub completion_probability: Vec<CompletionProbability>,
    pub conservative_quantiles: Vec<ConservativeQuantile>,
    pub thresholds: Vec<ThresholdSummary>,
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn hash_seed(seed: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in seed.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn slots_for_game(game: &str) -> &'static [&'static str] {
    let game = Game::try_from(game).unwrap_or(Game::Hsr);
    rules::slot_names(game)
}

pub fn main_max_values(game: &str) -> HashMap<String, f64> {
    let game = Game::try_from(game).unwrap_or(Game::Hsr);
    rules::main_max_values_map(game)
}

fn average_roll_value(game: &str, stat: &str) -> f64 {
    let game = Game::try_from(game).unwrap_or(Game::Hsr);
    let stat = Stat::try_from(stat).ok();
    stat.and_then(|stat| rules::substat_rolls(game, stat))
        .map(|values| values.iter().sum::<f64>() / values.len() as f64)
        .unwrap_or(1.0)
}

pub fn planned_main_contribution(
    game: &str,
    stat: &str,
    main_plan: &HashMap<String, String>,
) -> f64 {
    let main_values = main_max_values(game);
    if stat == "crit_rate" || stat == "crit_dmg" {
        let crit_slot = if game == "genshin" { "circlet" } else { "body" };
        return if main_plan.get(crit_slot).is_some_and(|value| value == stat) {
            *main_values.get(stat).unwrap_or(&0.0)
        } else {
            0.0
        };
    }
    slots_for_game(game)
        .iter()
        .filter(|slot| main_plan.get(**slot).is_some_and(|value| value == stat))
        .map(|_| *main_values.get(stat).unwrap_or(&0.0))
        .sum()
}

pub fn convert_goal_value(
    value: f64,
    stat: &str,
    from_mode: &str,
    to_mode: &str,
    config: &GraduationConfig,
) -> f64 {
    let roll_value = average_roll_value(&config.game, stat);
    let fixed_panel = config.base.get(stat).copied().unwrap_or(0.0)
        + planned_main_contribution(&config.game, stat, &config.main_plan);
    if from_mode == "hits" && to_mode == "panel" {
        return if value <= 0.0 {
            fixed_panel
        } else {
            round2(fixed_panel + value * roll_value)
        };
    }
    if from_mode == "panel" && to_mode == "hits" {
        return ((value - fixed_panel) / roll_value).ceil().max(0.0);
    }
    value
}

fn empty_build(config: &GraduationConfig) -> HashMap<String, Option<SimItem>> {
    slots_for_game(&config.game)
        .iter()
        .map(|slot| ((*slot).to_string(), None))
        .collect()
}

fn item_matches_plan(item: &SimItem, config: &GraduationConfig) -> bool {
    config
        .main_plan
        .get(&item.slot)
        .is_some_and(|main| main == &item.main_stat)
}

fn item_contribution(item: &SimItem) -> (HashMap<String, f64>, HashMap<String, usize>) {
    let mut values = HashMap::new();
    let mut hits = HashMap::new();
    let main_values = main_max_values(&item.game);
    *values.entry(item.main_stat.clone()).or_insert(0.0) +=
        main_values.get(&item.main_stat).copied().unwrap_or(0.0);
    for substat in &item.substats {
        *values.entry(substat.name.clone()).or_insert(0.0) += substat.value;
        *hits.entry(substat.name.clone()).or_insert(0) += substat.rolls.len();
    }
    (values, hits)
}

fn evaluate_build(
    build: &HashMap<String, Option<SimItem>>,
    config: &GraduationConfig,
) -> Evaluation {
    let mut panel = config.base.clone();
    let mut hits: HashMap<String, usize> =
        config.stats.iter().map(|stat| (stat.clone(), 0)).collect();
    let mut filled = 0;
    for item in build.values().flatten() {
        filled += 1;
        let (values, item_hits) = item_contribution(item);
        for (stat, value) in values {
            *panel.entry(stat).or_insert(0.0) += value;
        }
        for (stat, value) in item_hits {
            *hits.entry(stat).or_insert(0) += value;
        }
    }

    let mut capped = filled as f64 / slots_for_game(&config.game).len() as f64;
    let mut uncapped = capped;
    for stat in &config.stats {
        let panel_target = config.panel_goals.get(stat).copied().unwrap_or(0.0);
        let hit_target = config.hit_goals.get(stat).copied().unwrap_or(0.0);
        let panel_ratio = if panel_target <= 0.0 {
            1.0
        } else {
            panel.get(stat).copied().unwrap_or(0.0) / panel_target
        };
        let hit_ratio = if hit_target <= 0.0 {
            1.0
        } else {
            hits.get(stat).copied().unwrap_or(0) as f64 / hit_target
        };
        let ratio = panel_ratio.min(hit_ratio);
        capped += ratio.min(1.0);
        uncapped += ratio;
    }

    Evaluation {
        panel,
        hits,
        filled,
        completion: if config.stats.is_empty() {
            0.0
        } else {
            capped / (config.stats.len() as f64 + 1.0)
        },
        score: capped * 1000.0 + uncapped,
    }
}

fn is_graduated(evaluation: &Evaluation, config: &GraduationConfig) -> bool {
    if evaluation.filled < slots_for_game(&config.game).len() {
        return false;
    }
    config.stats.iter().all(|stat| {
        let panel_ok = config.panel_goals.get(stat).copied().unwrap_or(0.0) <= 0.0
            || evaluation.panel.get(stat).copied().unwrap_or(0.0)
                >= config.panel_goals.get(stat).copied().unwrap_or(0.0);
        let hits_ok = config.hit_goals.get(stat).copied().unwrap_or(0.0) <= 0.0
            || evaluation.hits.get(stat).copied().unwrap_or(0) as f64
                >= config.hit_goals.get(stat).copied().unwrap_or(0.0);
        panel_ok && hits_ok
    })
}

fn crit_value(item: &SimItem) -> f64 {
    let crit_rate = item
        .substats
        .iter()
        .find(|sub| sub.name == "crit_rate")
        .map(|sub| sub.value)
        .unwrap_or(0.0);
    let crit_dmg = item
        .substats
        .iter()
        .find(|sub| sub.name == "crit_dmg")
        .map(|sub| sub.value)
        .unwrap_or(0.0);
    round2(crit_rate * 2.0 + crit_dmg)
}

fn describe_substats(item: &SimItem) -> String {
    item.substats
        .iter()
        .map(|substat| {
            format!(
                "{} {} (+{})",
                substat.name,
                substat.value,
                substat.rolls.len().saturating_sub(1)
            )
        })
        .collect::<Vec<_>>()
        .join(" / ")
}

fn milestone_stat(
    item: Option<&SimItem>,
    evaluation: &Evaluation,
    config: &GraduationConfig,
) -> Option<String> {
    if item.is_none() {
        return config.stats.first().cloned();
    }
    let item = item.unwrap();
    let (values, hits) = item_contribution(item);
    let mut best_stat = None;
    let mut best_gain = f64::NEG_INFINITY;
    for stat in &config.stats {
        let panel_target = config
            .panel_goals
            .get(stat)
            .copied()
            .unwrap_or(1.0)
            .max(1.0);
        let hit_target = config.hit_goals.get(stat).copied().unwrap_or(1.0).max(1.0);
        let panel_gain = values.get(stat).copied().unwrap_or(0.0) / panel_target;
        let hit_gain = hits.get(stat).copied().unwrap_or(0) as f64 / hit_target;
        let gain = panel_gain.max(hit_gain);
        if gain > best_gain {
            best_gain = gain;
            best_stat = Some(stat.clone());
        }
    }
    best_stat
        .or_else(|| config.stats.first().cloned())
        .or_else(|| evaluation.panel.keys().next().cloned())
}

fn build_snapshot(build: &HashMap<String, Option<SimItem>>, game: &str) -> Vec<BuildSnapshotItem> {
    slots_for_game(game)
        .iter()
        .map(
            |slot| match build.get(*slot).and_then(|item| item.as_ref()) {
                Some(item) => BuildSnapshotItem {
                    slot: (*slot).to_string(),
                    empty: false,
                    main_stat: Some(item.main_stat.clone()),
                    crit_value: Some(crit_value(item)),
                    substats: Some(describe_substats(item)),
                },
                None => BuildSnapshotItem {
                    slot: (*slot).to_string(),
                    empty: true,
                    main_stat: None,
                    crit_value: None,
                    substats: None,
                },
            },
        )
        .collect()
}

fn make_milestone(
    kind: &str,
    run: usize,
    item: Option<&SimItem>,
    evaluation: &Evaluation,
    config: &GraduationConfig,
) -> Milestone {
    let title = if kind == "graduated" {
        "整套首次达标".to_string()
    } else {
        format!(
            "{}{}",
            item.map(|i| i.slot.as_str()).unwrap_or(""),
            if kind == "equip_first" {
                "首次装备"
            } else {
                "刷新装备"
            }
        )
    };
    Milestone {
        kind: kind.to_string(),
        run,
        days: run as f64 / config.runs_per_day,
        slot: item.map(|i| i.slot.clone()),
        title,
        panel: evaluation.panel.clone(),
        hits: evaluation.hits.clone(),
        completion: evaluation.completion,
        chart_stat: milestone_stat(item, evaluation, config),
        item: item.map(|i| MilestoneItem {
            slot: i.slot.clone(),
            main_stat: i.main_stat.clone(),
            crit_value: crit_value(i),
            substats: describe_substats(i),
        }),
        build_snapshot: None,
        plateau_run: None,
        stall_runs: None,
        stall_days: None,
    }
}

fn make_plateau_marker(
    source: &Milestone,
    current_run: usize,
    config: &GraduationConfig,
) -> Milestone {
    let mut marker = source.clone();
    marker.kind = "plateau".to_string();
    marker.title = format!(
        "卡点快照：{}后停滞",
        source.slot.clone().unwrap_or_default()
    );
    marker.plateau_run = Some(current_run);
    marker.stall_runs = Some(current_run.saturating_sub(source.run));
    marker.stall_days = Some(current_run.saturating_sub(source.run) as f64 / config.runs_per_day);
    marker
}

fn snapshot(run: usize, evaluation: &Evaluation, config: &GraduationConfig) -> Snapshot {
    Snapshot {
        run,
        runs: run,
        days: run as f64 / config.runs_per_day,
        completion: evaluation.completion,
        values: config
            .stats
            .iter()
            .map(|stat| {
                (
                    stat.clone(),
                    evaluation.panel.get(stat).copied().unwrap_or(0.0),
                )
            })
            .collect(),
        hits: config
            .stats
            .iter()
            .map(|stat| {
                (
                    stat.clone(),
                    evaluation.hits.get(stat).copied().unwrap_or(0),
                )
            })
            .collect(),
    }
}

fn create_threshold_runs() -> HashMap<String, Option<usize>> {
    rules::completion_thresholds()
        .iter()
        .map(|threshold| (threshold.key.to_string(), None))
        .collect()
}

fn record_threshold_runs(
    threshold_runs: &mut HashMap<String, Option<usize>>,
    completion: f64,
    run: usize,
) {
    for threshold in rules::completion_thresholds() {
        if threshold_runs
            .get(threshold.key)
            .is_some_and(|value| value.is_none())
            && completion >= threshold.ratio
        {
            threshold_runs.insert(threshold.key.to_string(), Some(run));
        }
    }
}

fn generate_genshin_item(rng: &mut StdRng) -> Result<SimItem, String> {
    let generator = GenshinGenerator::new(vec!["simulated".to_string()], "domain".to_string())
        .map_err(|e| e.to_string())?;
    let artifact = generator.generate(20, rng).map_err(|e| e.to_string())?;
    Ok(SimItem {
        game: "genshin".to_string(),
        source: Some("domain".to_string()),
        slot: artifact.slot,
        main_stat: artifact.main_stat,
        level: artifact.level,
        substats: artifact
            .substats
            .into_iter()
            .map(|sub| SimSubstat {
                name: sub.name,
                value: sub.value,
                rolls: sub.rolls,
            })
            .collect(),
        equipped_at: None,
    })
}

fn generate_hsr_item(source: &str, rng: &mut StdRng) -> Result<SimItem, String> {
    let generator = HsrGenerator::new(vec!["simulated".to_string()], source.to_string())
        .map_err(|e| e.to_string())?;
    let relic = generator.generate(15, rng).map_err(|e| e.to_string())?;
    Ok(SimItem {
        game: "hsr".to_string(),
        source: Some(source.to_string()),
        slot: relic.slot,
        main_stat: relic.main_stat,
        level: relic.level,
        substats: relic
            .substats
            .into_iter()
            .map(|sub| SimSubstat {
                name: sub.name,
                value: sub.value,
                rolls: sub.rolls,
            })
            .collect(),
        equipped_at: None,
    })
}

fn generate_items(
    config: &GraduationConfig,
    rng: &mut StdRng,
    cavern_rng: &mut StdRng,
    planar_rng: &mut StdRng,
) -> Result<Vec<SimItem>, String> {
    if config.game == "genshin" {
        Ok(vec![generate_genshin_item(rng)?])
    } else {
        Ok(vec![
            generate_hsr_item("cavern", cavern_rng)?,
            generate_hsr_item("planar", planar_rng)?,
        ])
    }
}

fn try_equip_detailed(
    mut item: SimItem,
    build: &mut HashMap<String, Option<SimItem>>,
    config: &GraduationConfig,
    run: usize,
) -> Option<(String, Evaluation, SimItem)> {
    if !item_matches_plan(&item, config) {
        return None;
    }
    let current_evaluation = evaluate_build(build, config);
    let slot = item.slot.clone();
    let previous = build.insert(slot.clone(), Some(item.clone()));
    let had_existing_item = previous.as_ref().and_then(|slot| slot.as_ref()).is_some();
    let candidate_evaluation = evaluate_build(build, config);
    if candidate_evaluation.score <= current_evaluation.score + 0.0001 {
        restore_slot(build, slot, previous);
        return None;
    }
    let kind = if had_existing_item {
        "equip_upgrade".to_string()
    } else {
        "equip_first".to_string()
    };
    item.equipped_at = Some(EquippedAt {
        kind: kind.clone(),
        run,
        days: run as f64 / config.runs_per_day,
        panel: candidate_evaluation.panel.clone(),
        hits: candidate_evaluation.hits.clone(),
        completion: candidate_evaluation.completion,
        chart_stat: milestone_stat(Some(&item), &candidate_evaluation, config),
        build_snapshot: build_snapshot(build, &config.game),
    });
    build.insert(item.slot.clone(), Some(item.clone()));
    Some((kind, candidate_evaluation, item))
}

fn try_equip_average(
    item: SimItem,
    build: &mut HashMap<String, Option<SimItem>>,
    config: &GraduationConfig,
    current_evaluation: &Evaluation,
) -> Option<Evaluation> {
    if !item_matches_plan(&item, config) {
        return None;
    }
    let slot = item.slot.clone();
    let previous = build.insert(slot.clone(), Some(item));
    let candidate_evaluation = evaluate_build(build, config);
    if candidate_evaluation.score <= current_evaluation.score + 0.0001 {
        restore_slot(build, slot, previous);
        return None;
    }
    Some(candidate_evaluation)
}

fn restore_slot(
    build: &mut HashMap<String, Option<SimItem>>,
    slot: String,
    previous: Option<Option<SimItem>>,
) {
    match previous {
        Some(value) => {
            build.insert(slot, value);
        }
        None => {
            build.remove(&slot);
        }
    }
}

fn final_piece_markers(
    build: &HashMap<String, Option<SimItem>>,
    config: &GraduationConfig,
) -> Vec<Milestone> {
    let mut markers = Vec::new();
    for item in build.values().flatten() {
        if let Some(equipped_at) = &item.equipped_at {
            markers.push(Milestone {
                kind: equipped_at.kind.clone(),
                run: equipped_at.run,
                days: equipped_at.days,
                slot: Some(item.slot.clone()),
                title: format!("{}最终件出现", item.slot),
                panel: equipped_at.panel.clone(),
                hits: equipped_at.hits.clone(),
                completion: equipped_at.completion,
                chart_stat: equipped_at.chart_stat.clone(),
                item: Some(MilestoneItem {
                    slot: item.slot.clone(),
                    main_stat: item.main_stat.clone(),
                    crit_value: crit_value(item),
                    substats: describe_substats(item),
                }),
                build_snapshot: Some(equipped_at.build_snapshot.clone()),
                plateau_run: None,
                stall_runs: None,
                stall_days: None,
            });
        }
    }
    markers.sort_by(|left, right| {
        left.run
            .cmp(&right.run)
            .then_with(|| left.title.cmp(&right.title))
    });
    markers
        .into_iter()
        .filter(|marker| {
            slots_for_game(&config.game).contains(&marker.slot.as_deref().unwrap_or(""))
        })
        .collect()
}

pub fn simulate_graduation(config: GraduationConfig) -> Result<SingleSimulationResult, String> {
    let mut build = empty_build(&config);
    let mut history = Vec::new();
    let mut milestones = Vec::new();
    let mut plateau_markers = Vec::new();
    let mut threshold_runs = create_threshold_runs();
    let mut rng = StdRng::seed_from_u64(hash_seed(&config.seed));
    let mut cavern_rng = StdRng::seed_from_u64(hash_seed(&format!("{}:cavern", config.seed)));
    let mut planar_rng = StdRng::seed_from_u64(hash_seed(&format!("{}:planar", config.seed)));
    let interval = (config.max_runs / 600).max(1);
    let plateau_window = (config.max_runs / 20).max(50);
    let mut stop_run = config.max_runs;
    let mut stop_reason = "达到上限".to_string();
    let mut last_evaluation = evaluate_build(&build, &config);
    let mut last_completion_run = 0;
    let mut last_completion = last_evaluation.completion;
    let mut last_change_marker: Option<Milestone> = None;
    let mut last_plateau_key = String::new();

    history.push(snapshot(0, &last_evaluation, &config));

    for run in 1..=config.max_runs {
        let items = generate_items(&config, &mut rng, &mut cavern_rng, &mut planar_rng)?;
        for item in items {
            let Some((kind, candidate_evaluation, equipped_item)) =
                try_equip_detailed(item, &mut build, &config, run)
            else {
                continue;
            };
            last_evaluation = candidate_evaluation;
            let mut milestone =
                make_milestone(&kind, run, Some(&equipped_item), &last_evaluation, &config);
            milestone.build_snapshot = equipped_item
                .equipped_at
                .as_ref()
                .map(|event| event.build_snapshot.clone());
            last_change_marker = Some(milestone.clone());
            if last_evaluation.completion > last_completion + 0.0001 {
                last_completion = last_evaluation.completion;
                last_completion_run = run;
            }
            record_threshold_runs(&mut threshold_runs, last_evaluation.completion, run);
            if milestones.len() < 160 {
                milestones.push(milestone);
            }
            if is_graduated(&last_evaluation, &config) {
                stop_run = run;
                stop_reason = "已达标".to_string();
                milestones.push(make_milestone(
                    "graduated",
                    run,
                    None,
                    &last_evaluation,
                    &config,
                ));
                history.push(snapshot(run, &last_evaluation, &config));
                let final_markers = final_piece_markers(&build, &config);
                return Ok(SingleSimulationResult {
                    config: config.clone(),
                    build,
                    evaluation: last_evaluation,
                    history,
                    milestones,
                    final_markers,
                    plateau_markers,
                    threshold_runs,
                    stop_run,
                    stop_reason,
                });
            }
        }

        if run % interval == 0 || run == config.max_runs {
            last_evaluation = evaluate_build(&build, &config);
            history.push(snapshot(run, &last_evaluation, &config));
            if let Some(marker) = &last_change_marker {
                if run - marker.run >= plateau_window
                    && run - last_completion_run >= plateau_window
                    && plateau_markers.len() < 80
                {
                    let key = format!("{}:{}", marker.run, marker.slot.clone().unwrap_or_default());
                    if key != last_plateau_key {
                        plateau_markers.push(make_plateau_marker(marker, run, &config));
                        last_plateau_key = key;
                    }
                }
            }
        }
    }

    last_evaluation = evaluate_build(&build, &config);
    record_threshold_runs(&mut threshold_runs, last_evaluation.completion, stop_run);
    let final_markers = final_piece_markers(&build, &config);
    Ok(SingleSimulationResult {
        config: config.clone(),
        build,
        evaluation: last_evaluation,
        history,
        milestones,
        final_markers,
        plateau_markers,
        threshold_runs,
        stop_run,
        stop_reason,
    })
}

fn simulate_graduation_sample(
    config: &GraduationConfig,
    sample_index: usize,
) -> Result<SampleResult, String> {
    let seed = format!("{}:{}", config.seed, sample_index);
    let mut build = empty_build(config);
    let mut threshold_runs = create_threshold_runs();
    let mut rng = StdRng::seed_from_u64(hash_seed(&seed));
    let mut cavern_rng = StdRng::seed_from_u64(hash_seed(&format!("{}:cavern", seed)));
    let mut planar_rng = StdRng::seed_from_u64(hash_seed(&format!("{}:planar", seed)));
    let mut evaluation = evaluate_build(&build, config);

    for run in 1..=config.max_runs {
        let items = generate_items(config, &mut rng, &mut cavern_rng, &mut planar_rng)?;
        for item in items {
            let Some(next_evaluation) = try_equip_average(item, &mut build, config, &evaluation)
            else {
                continue;
            };
            evaluation = next_evaluation;
            record_threshold_runs(&mut threshold_runs, evaluation.completion, run);
            if is_graduated(&evaluation, config) {
                return Ok(SampleResult {
                    graduated: true,
                    stop_run: run,
                    threshold_runs,
                    final_completion: evaluation.completion,
                });
            }
        }
    }

    record_threshold_runs(&mut threshold_runs, evaluation.completion, config.max_runs);
    Ok(SampleResult {
        graduated: false,
        stop_run: config.max_runs,
        threshold_runs,
        final_completion: evaluation.completion,
    })
}

fn percentile_index(len: usize, ratio: f64) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(((len as f64 * ratio).ceil() as isize - 1).clamp(0, len as isize - 1) as usize)
}

fn percentile_usize(sorted: &[usize], ratio: f64) -> f64 {
    percentile_index(sorted.len(), ratio)
        .map(|index| sorted[index] as f64)
        .unwrap_or(f64::NAN)
}

fn percentile_f64(sorted: &[f64], ratio: f64) -> f64 {
    percentile_index(sorted.len(), ratio)
        .map(|index| sorted[index])
        .unwrap_or(f64::NAN)
}

fn standard_deviation(values: &[usize], average: f64) -> f64 {
    if values.len() < 2 || !average.is_finite() {
        return f64::NAN;
    }
    let variance = values
        .iter()
        .map(|value| (*value as f64 - average).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    variance.sqrt()
}

fn make_buckets(values: &[usize], max_runs: usize, bucket_count: usize) -> Vec<Bucket> {
    let bucket_size = ((max_runs as f64 / bucket_count as f64).ceil() as usize).max(1);
    let mut buckets: Vec<_> = (0..bucket_count)
        .map(|index| Bucket {
            start: index * bucket_size + 1,
            end: max_runs.min((index + 1) * bucket_size),
            count: 0,
        })
        .collect();
    for value in values {
        let index = (((*value).saturating_sub(1)) / bucket_size).min(bucket_count - 1);
        buckets[index].count += 1;
    }
    buckets
}

fn summarize_values(values: &[f64]) -> CompletionStats {
    let mut sorted: Vec<f64> = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    sorted.sort_by(|left, right| left.total_cmp(right));
    if sorted.is_empty() {
        return CompletionStats {
            average: f64::NAN,
            median: f64::NAN,
            p10: f64::NAN,
            p25: f64::NAN,
            p75: f64::NAN,
            p90: f64::NAN,
            p95: f64::NAN,
            min: f64::NAN,
            max: f64::NAN,
        };
    }
    let average = sorted.iter().sum::<f64>() / sorted.len() as f64;
    CompletionStats {
        average,
        median: percentile_f64(&sorted, 0.5),
        p10: percentile_f64(&sorted, 0.1),
        p25: percentile_f64(&sorted, 0.25),
        p75: percentile_f64(&sorted, 0.75),
        p90: percentile_f64(&sorted, 0.9),
        p95: percentile_f64(&sorted, 0.95),
        min: *sorted.first().unwrap(),
        max: *sorted.last().unwrap(),
    }
}

fn summarize_completion_probability(
    runs: &[usize],
    done: usize,
    max_runs: usize,
    runs_per_day: f64,
) -> Vec<CompletionProbability> {
    let total = done.max(1);
    let mut sorted_runs = runs.to_vec();
    sorted_runs.sort_unstable();
    [
        (0.10, "10% 上限"),
        (0.25, "25% 上限"),
        (0.50, "50% 上限"),
        (0.75, "75% 上限"),
        (1.00, "完整上限"),
    ]
    .into_iter()
    .map(|(ratio, label)| {
        let run = ((max_runs as f64 * ratio).round() as usize).max(1);
        let reached = sorted_runs.iter().filter(|value| **value <= run).count();
        let probability = reached as f64 / total as f64;
        CompletionProbability {
            ratio,
            label: label.to_string(),
            run,
            days: run as f64 / runs_per_day,
            reached,
            probability,
            survival: 1.0 - probability,
        }
    })
    .collect()
}

fn summarize_conservative_quantiles(runs: &[usize], done: usize) -> Vec<ConservativeQuantile> {
    let mut sorted_runs = runs.to_vec();
    sorted_runs.sort_unstable();
    rules::resource_quantiles()
        .iter()
        .map(|quantile| {
            let required = (done as f64 * quantile.ratio).ceil() as usize;
            let run = required
                .checked_sub(1)
                .and_then(|index| sorted_runs.get(index))
                .copied();
            ConservativeQuantile {
                key: quantile.key.to_string(),
                ratio: quantile.ratio,
                label: quantile.label.to_string(),
                required,
                run: run.map(|value| value as f64).unwrap_or(f64::NAN),
                censored: run.is_none(),
            }
        })
        .collect()
}

fn summarize_threshold_samples(
    config: &GraduationConfig,
    sample_results: &[SampleResult],
    done: usize,
) -> Vec<ThresholdSummary> {
    rules::completion_thresholds()
        .iter()
        .map(|threshold| {
            let mut runs: Vec<usize> = sample_results
                .iter()
                .filter_map(|result| {
                    result
                        .threshold_runs
                        .get(threshold.key)
                        .and_then(|value| *value)
                })
                .collect();
            runs.sort_unstable();
            let reached = runs.len();
            let average = if reached > 0 {
                runs.iter().sum::<usize>() as f64 / reached as f64
            } else {
                f64::NAN
            };
            ThresholdSummary {
                key: threshold.key.to_string(),
                ratio: threshold.ratio,
                label: threshold.label.to_string(),
                runs: runs.clone(),
                reached,
                rate: reached as f64 / done.max(1) as f64,
                average,
                average_days: average / config.runs_per_day,
                median: percentile_usize(&runs, 0.5),
                p90: percentile_usize(&runs, 0.9),
                p95: percentile_usize(&runs, 0.95),
                min: runs.first().copied(),
                max: runs.last().copied(),
            }
        })
        .collect()
}

fn summarize_average(
    config: &GraduationConfig,
    sample_results: Vec<SampleResult>,
    done: usize,
) -> AverageSimulationResult {
    let thresholds = summarize_threshold_samples(config, &sample_results, done);
    let graduation = thresholds.iter().find(|threshold| threshold.key == "100");
    let sorted = graduation
        .map(|threshold| threshold.runs.clone())
        .unwrap_or_default();
    let success = graduation.map(|threshold| threshold.reached).unwrap_or(0);
    let average = graduation
        .map(|threshold| threshold.average)
        .unwrap_or(f64::NAN);
    let failed = done.saturating_sub(success);
    let capped_runs: Vec<usize> = sample_results
        .iter()
        .map(|result| {
            if result.graduated {
                result.stop_run
            } else {
                config.max_runs
            }
        })
        .collect();
    let completion_values: Vec<f64> = sample_results
        .iter()
        .map(|result| result.final_completion)
        .collect();
    let completion_stats = summarize_values(&completion_values);
    let censored_run_lower_bound = if capped_runs.is_empty() {
        f64::NAN
    } else {
        capped_runs.iter().sum::<usize>() as f64 / capped_runs.len() as f64
    };

    AverageSimulationResult {
        done,
        total: config.sample_count,
        max_runs: config.max_runs,
        runs_per_day: config.runs_per_day,
        success,
        failed,
        success_runs: sorted.clone(),
        success_rate: graduation.map(|threshold| threshold.rate).unwrap_or(0.0),
        average,
        average_days: graduation
            .map(|threshold| threshold.average_days)
            .unwrap_or(f64::NAN),
        median: graduation
            .map(|threshold| threshold.median)
            .unwrap_or(f64::NAN),
        p90: graduation
            .map(|threshold| threshold.p90)
            .unwrap_or(f64::NAN),
        p95: graduation
            .map(|threshold| threshold.p95)
            .unwrap_or(f64::NAN),
        p10: percentile_usize(&sorted, 0.1),
        p25: percentile_usize(&sorted, 0.25),
        p75: percentile_usize(&sorted, 0.75),
        min: sorted.first().copied(),
        max: sorted.last().copied(),
        stdev: standard_deviation(&sorted, average),
        buckets: make_buckets(&sorted, config.max_runs, 10),
        censored_rate: failed as f64 / done.max(1) as f64,
        censored_run_lower_bound,
        censored_days_lower_bound: censored_run_lower_bound / config.runs_per_day,
        completion_stats,
        completion_probability: summarize_completion_probability(
            &sorted,
            done,
            config.max_runs,
            config.runs_per_day,
        ),
        conservative_quantiles: summarize_conservative_quantiles(&sorted, done),
        thresholds,
    }
}

pub fn summarize_graduation_average(
    config: GraduationConfig,
) -> Result<AverageSimulationResult, String> {
    let sample_count = config.sample_count.max(1);
    let results: Result<Vec<_>, String> = (0..sample_count)
        .into_par_iter()
        .map(|index| simulate_graduation_sample(&config, index))
        .collect();
    Ok(summarize_average(&config, results?, sample_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(max_runs: usize, sample_count: usize) -> GraduationConfig {
        GraduationConfig {
            game: "genshin".to_string(),
            role: "dps".to_string(),
            effective_tags: vec!["atk_pct".to_string()],
            stats: vec![
                "crit_rate".to_string(),
                "crit_dmg".to_string(),
                "atk_pct".to_string(),
            ],
            base: HashMap::from([
                ("crit_rate".to_string(), 5.0),
                ("crit_dmg".to_string(), 50.0),
                ("atk_pct".to_string(), 0.0),
            ]),
            panel_goals: HashMap::from([
                ("crit_rate".to_string(), 20.0),
                ("crit_dmg".to_string(), 80.0),
                ("atk_pct".to_string(), 20.0),
            ]),
            hit_goals: HashMap::from([
                ("crit_rate".to_string(), 1.0),
                ("crit_dmg".to_string(), 1.0),
                ("atk_pct".to_string(), 1.0),
            ]),
            main_plan: HashMap::from([
                ("flower".to_string(), "hp_flat".to_string()),
                ("plume".to_string(), "atk_flat".to_string()),
                ("sands".to_string(), "atk_pct".to_string()),
                ("goblet".to_string(), "pyro_dmg".to_string()),
                ("circlet".to_string(), "crit_rate".to_string()),
            ]),
            max_runs,
            runs_per_day: 8.0,
            sample_count,
            seed: "test-seed".to_string(),
        }
    }

    #[test]
    fn simulate_graduation_is_reproducible() {
        let config = test_config(200, 1);
        let first = simulate_graduation(config.clone()).unwrap();
        let second = simulate_graduation(config).unwrap();
        assert_eq!(first.stop_run, second.stop_run);
        assert_eq!(first.threshold_runs, second.threshold_runs);
    }

    #[test]
    fn simulate_graduation_reports_limit_when_not_graduated() {
        let mut config = test_config(3, 1);
        config.panel_goals.insert("crit_rate".to_string(), 9999.0);
        let result = simulate_graduation(config).unwrap();
        assert_eq!(result.stop_reason, "达到上限");
        assert_eq!(result.stop_run, 3);
    }

    #[test]
    fn average_summary_counts_all_samples() {
        let config = test_config(50, 8);
        let result = summarize_graduation_average(config).unwrap();
        assert_eq!(result.success + result.failed, result.done);
        assert_eq!(result.done, 8);
        assert!(result
            .thresholds
            .iter()
            .all(|threshold| threshold.rate >= 0.0 && threshold.rate <= 1.0));
    }

    #[test]
    fn hsr_main_max_values_are_precise_rules_values() {
        let max_values = main_max_values("hsr");
        assert_eq!(max_values["speed"], 25.032);
        assert_eq!(max_values["outgoing_healing"], 34.5606);
        assert_eq!(max_values["physical_dmg"], 38.8803);
        assert_eq!(
            planned_main_contribution(
                "hsr",
                "speed",
                &HashMap::from([("feet".to_string(), "speed".to_string())])
            ),
            25.032
        );
    }

    #[test]
    fn goal_conversion_uses_rules_roll_tables() {
        let mut config = test_config(50, 1);
        config.game = "hsr".to_string();
        config.base = HashMap::from([("speed".to_string(), 100.0)]);
        config.main_plan = HashMap::from([("feet".to_string(), "speed".to_string())]);
        let average_speed_roll = rules::substat_rolls(Game::Hsr, Stat::Speed)
            .unwrap()
            .iter()
            .sum::<f64>()
            / 3.0;

        let value = convert_goal_value(2.0, "speed", "hits", "panel", &config);

        assert_eq!(value, round2(100.0 + 25.032 + average_speed_roll * 2.0));
    }
}
