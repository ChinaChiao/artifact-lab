export const SLOT_LABELS = {
  flower: "生之花",
  plume: "死之羽",
  sands: "时之沙",
  goblet: "空之杯",
  circlet: "理之冠",
  head: "头部",
  hands: "手部",
  body: "躯干",
  feet: "脚部",
  planar_sphere: "位面球",
  link_rope: "连结绳",
};

export const STAT_LABELS = {
  hp_flat: "生命值",
  atk_flat: "攻击力",
  def_flat: "防御力",
  hp_pct: "生命值%",
  atk_pct: "攻击力%",
  def_pct: "防御力%",
  energy_recharge: "元素充能效率",
  elemental_mastery: "元素精通",
  pyro_dmg: "火伤",
  electro_dmg: "雷伤",
  cryo_dmg: "冰伤",
  hydro_dmg: "水伤",
  dendro_dmg: "草伤",
  anemo_dmg: "风伤",
  geo_dmg: "岩伤",
  physical_dmg: "物伤",
  crit_rate: "暴击率",
  crit_dmg: "暴击伤害",
  healing_bonus: "治疗加成",
  speed: "速度",
  outgoing_healing: "治疗量加成",
  effect_hit_rate: "效果命中",
  effect_res: "效果抵抗",
  break_effect: "击破特攻",
  energy_regen: "能量恢复效率",
  fire_dmg: "火伤",
  ice_dmg: "冰伤",
  lightning_dmg: "雷伤",
  wind_dmg: "风伤",
  quantum_dmg: "量子伤",
  imaginary_dmg: "虚数伤",
};

export function round2(value) {
  return Math.round(value * 100) / 100;
}

export function trackedStats(_game, role, effectiveTags) {
  return role === "dps" ? ["crit_rate", "crit_dmg", ...effectiveTags] : [...effectiveTags];
}

export function formatStatValue(stat, value, options = {}) {
  if (!Number.isFinite(value)) return "-";
  const rounded = options.integer ? Math.round(value) : value;
  const percentStats = new Set([
    "hp_pct",
    "atk_pct",
    "def_pct",
    "energy_recharge",
    "crit_rate",
    "crit_dmg",
    "healing_bonus",
    "outgoing_healing",
    "effect_hit_rate",
    "effect_res",
    "break_effect",
    "energy_regen",
    "pyro_dmg",
    "electro_dmg",
    "cryo_dmg",
    "hydro_dmg",
    "dendro_dmg",
    "anemo_dmg",
    "geo_dmg",
    "physical_dmg",
    "fire_dmg",
    "ice_dmg",
    "lightning_dmg",
    "wind_dmg",
    "quantum_dmg",
    "imaginary_dmg",
  ]);
  const integerStats = new Set(["elemental_mastery", "hp_flat", "atk_flat", "def_flat"]);
  const display =
    integerStats.has(stat) || options.integer
      ? String(Math.round(rounded))
      : Math.abs(rounded) >= 100
        ? String(Math.round(rounded))
        : rounded.toFixed(2).replace(/\.?0+$/, "");
  return percentStats.has(stat) ? `${display}%` : display;
}

// NOTE: averageRollValue / plannedMainContribution / convertGoalValue mirror the
// Rust implementations in crates/artifact_core/src/graduation.rs. They exist only
// for live goal-input conversion in the UI; the authoritative simulation runs in
// Rust. Keep the formulas (and the crit_rate/crit_dmg single-slot rule) in sync —
// any change here must be made in graduation.rs too, or panel/hit goals will drift.
export function averageRollValue(stat, rules = {}) {
  const rolls = rules.substatRolls?.[stat];
  if (!rolls?.length) return 1;
  return rolls.reduce((sum, value) => sum + value, 0) / rolls.length;
}

export function plannedMainContribution(stat, mainPlan, rules = {}) {
  const mainValues = rules.mainMax ?? {};
  if (stat === "crit_rate" || stat === "crit_dmg") {
    const critSlot = rules.slots?.includes("circlet") ? "circlet" : "body";
    return mainPlan?.[critSlot] === stat ? mainValues[stat] || 0 : 0;
  }
  return (rules.slots ?? []).reduce((total, slot) => total + (mainPlan?.[slot] === stat ? mainValues[stat] || 0 : 0), 0);
}

export function convertGoalValue(value, stat, fromMode, toMode, config) {
  const rollValue = averageRollValue(stat, config.rules);
  const fixedPanel = (Number(config.base?.[stat]) || 0) + plannedMainContribution(stat, config.mainPlan, config.rules);
  if (fromMode === "hits" && toMode === "panel") {
    return value <= 0 ? fixedPanel : round2(fixedPanel + value * rollValue);
  }
  if (fromMode === "panel" && toMode === "hits") {
    return Math.max(0, Math.ceil((value - fixedPanel) / rollValue));
  }
  return value;
}

export function critValue(item) {
  const values = Object.fromEntries((item?.substats ?? []).map((substat) => [substat.name, substat.value]));
  return round2((values.crit_rate || 0) * 2 + (values.crit_dmg || 0));
}

export function describeSubstats(item) {
  return (item?.substats ?? [])
    .map((substat) => `${STAT_LABELS[substat.name] ?? substat.name} ${formatStatValue(substat.name, substat.value)} (+${Math.max(0, substat.rolls.length - 1)})`)
    .join(" / ");
}

export function formatNumber(value) {
  if (!Number.isFinite(value)) return "-";
  if (value >= 10000) return Math.round(value).toLocaleString("zh-CN");
  if (Number.isInteger(value)) return String(value);
  if (value >= 100) return String(Math.round(value));
  if (value >= 10) return value.toFixed(1);
  return value.toFixed(2);
}

export function visibleMilestones(result) {
  return [...(result?.finalMarkers || []), ...(result?.plateauMarkers || [])].sort((a, b) => a.run - b.run || a.title.localeCompare(b.title, "zh-CN"));
}

export function milestoneTypeLabel(type) {
  if (type === "plateau") return "卡点快照";
  if (type === "equip_first") return "最终新部位";
  if (type === "equip_upgrade") return "最终替换件";
  if (type === "graduated") return "达标";
  return "节点";
}
