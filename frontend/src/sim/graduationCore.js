export const GENSHIN_SLOTS = ["flower", "plume", "sands", "goblet", "circlet"];
export const HSR_CAVERN_SLOTS = ["head", "hands", "body", "feet"];
export const HSR_PLANAR_SLOTS = ["planar_sphere", "link_rope"];
export const HSR_SLOTS = [...HSR_CAVERN_SLOTS, ...HSR_PLANAR_SLOTS];

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

export const GAME_STATS = {
  genshin: {
    dpsDefaults: ["atk_pct"],
    supportDefaults: ["energy_recharge", "elemental_mastery"],
    effective: ["atk_pct", "hp_pct", "def_pct", "energy_recharge", "elemental_mastery"],
    base: { crit_rate: 5, crit_dmg: 50, atk_pct: 0, hp_pct: 0, def_pct: 0, energy_recharge: 100, elemental_mastery: 0 },
    panelTargets: { crit_rate: 70, crit_dmg: 160, atk_pct: 46, hp_pct: 46, def_pct: 58, energy_recharge: 180, elemental_mastery: 600 },
    hitTargets: { crit_rate: 6, crit_dmg: 8, atk_pct: 6, hp_pct: 6, def_pct: 6, energy_recharge: 6, elemental_mastery: 6 },
  },
  hsr: {
    dpsDefaults: ["atk_pct", "speed"],
    supportDefaults: ["speed", "effect_hit_rate"],
    effective: ["atk_pct", "hp_pct", "def_pct", "speed", "effect_hit_rate", "effect_res", "break_effect", "energy_regen"],
    base: { crit_rate: 5, crit_dmg: 50, atk_pct: 0, hp_pct: 0, def_pct: 0, speed: 100, effect_hit_rate: 0, effect_res: 0, break_effect: 0, energy_regen: 100 },
    panelTargets: { crit_rate: 70, crit_dmg: 150, atk_pct: 45, hp_pct: 45, def_pct: 45, speed: 135, effect_hit_rate: 80, effect_res: 30, break_effect: 180, energy_regen: 119 },
    hitTargets: { crit_rate: 6, crit_dmg: 8, atk_pct: 6, hp_pct: 6, def_pct: 6, speed: 5, effect_hit_rate: 6, effect_res: 6, break_effect: 6, energy_regen: 1 },
  },
};

const GENSHIN_INITIAL_THREE = { domain: 0.8 };
const GENSHIN_MAIN = {
  flower: { hp_flat: 1 },
  plume: { atk_flat: 1 },
  sands: { hp_pct: 26.68, atk_pct: 26.66, def_pct: 26.66, energy_recharge: 10, elemental_mastery: 10 },
  goblet: {
    hp_pct: 19.25,
    atk_pct: 19.25,
    def_pct: 19,
    pyro_dmg: 5,
    electro_dmg: 5,
    cryo_dmg: 5,
    hydro_dmg: 5,
    dendro_dmg: 5,
    anemo_dmg: 5,
    geo_dmg: 5,
    physical_dmg: 5,
    elemental_mastery: 2.5,
  },
  circlet: { hp_pct: 22, atk_pct: 22, def_pct: 22, crit_rate: 10, crit_dmg: 10, healing_bonus: 10, elemental_mastery: 4 },
};
export const GENSHIN_MAIN_MAX = {
  hp_flat: 4780,
  atk_flat: 311,
  hp_pct: 46.6,
  atk_pct: 46.6,
  def_pct: 58.3,
  energy_recharge: 51.8,
  elemental_mastery: 186.5,
  crit_rate: 31.1,
  crit_dmg: 62.2,
  pyro_dmg: 46.6,
  electro_dmg: 46.6,
  cryo_dmg: 46.6,
  hydro_dmg: 46.6,
  dendro_dmg: 46.6,
  anemo_dmg: 46.6,
  geo_dmg: 46.6,
  physical_dmg: 58.3,
  healing_bonus: 35.9,
};
const GENSHIN_SUB_WEIGHTS = {
  hp_flat: 6,
  atk_flat: 6,
  def_flat: 6,
  hp_pct: 4,
  atk_pct: 4,
  def_pct: 4,
  energy_recharge: 4,
  elemental_mastery: 4,
  crit_rate: 3,
  crit_dmg: 3,
};
const GENSHIN_ROLLS = {
  hp_flat: [209.13, 239, 268.88, 298.75],
  atk_flat: [13.62, 15.56, 17.51, 19.45],
  def_flat: [16.2, 18.52, 20.83, 23.15],
  hp_pct: [4.08, 4.66, 5.25, 5.83],
  atk_pct: [4.08, 4.66, 5.25, 5.83],
  def_pct: [5.1, 5.83, 6.56, 7.29],
  elemental_mastery: [16.32, 18.65, 20.98, 23.31],
  energy_recharge: [4.53, 5.18, 5.83, 6.48],
  crit_rate: [2.72, 3.11, 3.5, 3.89],
  crit_dmg: [5.44, 6.22, 6.99, 7.77],
};

const HSR_MAIN = {
  head: { hp_flat: 1 },
  hands: { atk_flat: 1 },
  body: { hp_pct: 20, atk_pct: 20, def_pct: 20, crit_rate: 10, crit_dmg: 10, outgoing_healing: 10, effect_hit_rate: 10 },
  feet: { hp_pct: 29.17, atk_pct: 29.17, def_pct: 29.16, speed: 12.5 },
  planar_sphere: {
    hp_pct: 12.34,
    atk_pct: 12.33,
    def_pct: 12.33,
    physical_dmg: 9,
    fire_dmg: 9,
    ice_dmg: 9,
    lightning_dmg: 9,
    wind_dmg: 9,
    quantum_dmg: 9,
    imaginary_dmg: 9,
  },
  link_rope: { hp_pct: 26.33, atk_pct: 26.34, def_pct: 26.33, break_effect: 15, energy_regen: 6 },
};
export const HSR_MAIN_MAX = {
  hp_flat: 705.6,
  atk_flat: 352.8,
  hp_pct: 43.2,
  atk_pct: 43.2,
  def_pct: 54,
  crit_rate: 32.4,
  crit_dmg: 64.8,
  outgoing_healing: 34.56,
  effect_hit_rate: 43.2,
  speed: 25.03,
  physical_dmg: 38.88,
  fire_dmg: 38.88,
  ice_dmg: 38.88,
  lightning_dmg: 38.88,
  wind_dmg: 38.88,
  quantum_dmg: 38.88,
  imaginary_dmg: 38.88,
  break_effect: 64.8,
  energy_regen: 19.44,
};
const HSR_SUB_WEIGHTS = {
  hp_flat: 10,
  atk_flat: 10,
  def_flat: 10,
  hp_pct: 10,
  atk_pct: 10,
  def_pct: 10,
  speed: 4,
  crit_rate: 6,
  crit_dmg: 6,
  effect_hit_rate: 8,
  effect_res: 8,
  break_effect: 8,
};
const HSR_ROLLS = {
  hp_flat: [33.87, 38.1, 42.34],
  atk_flat: [16.94, 19.05, 21.17],
  def_flat: [16.94, 19.05, 21.17],
  hp_pct: [3.46, 3.89, 4.32],
  atk_pct: [3.46, 3.89, 4.32],
  def_pct: [4.32, 4.86, 5.4],
  speed: [2, 2.3, 2.6],
  crit_rate: [2.59, 2.92, 3.24],
  crit_dmg: [5.18, 5.83, 6.48],
  effect_hit_rate: [3.46, 3.89, 4.32],
  effect_res: [3.46, 3.89, 4.32],
  break_effect: [5.18, 5.83, 6.48],
};

export const MAIN_OPTIONS = {
  genshin: {
    flower: ["hp_flat"],
    plume: ["atk_flat"],
    sands: ["atk_pct", "hp_pct", "def_pct", "energy_recharge", "elemental_mastery"],
    goblet: ["pyro_dmg", "hydro_dmg", "electro_dmg", "cryo_dmg", "dendro_dmg", "anemo_dmg", "geo_dmg", "physical_dmg", "atk_pct", "hp_pct", "elemental_mastery"],
    circlet: ["crit_rate", "crit_dmg", "atk_pct", "hp_pct", "def_pct", "healing_bonus", "elemental_mastery"],
  },
  hsr: {
    head: ["hp_flat"],
    hands: ["atk_flat"],
    body: ["crit_rate", "crit_dmg", "atk_pct", "hp_pct", "def_pct", "outgoing_healing", "effect_hit_rate"],
    feet: ["speed", "atk_pct", "hp_pct", "def_pct"],
    planar_sphere: ["fire_dmg", "quantum_dmg", "imaginary_dmg", "lightning_dmg", "ice_dmg", "wind_dmg", "physical_dmg", "atk_pct", "hp_pct", "def_pct"],
    link_rope: ["atk_pct", "energy_regen", "break_effect", "hp_pct", "def_pct"],
  },
};

export const DEFAULT_MAIN_PLAN = {
  genshin: { flower: "hp_flat", plume: "atk_flat", sands: "atk_pct", goblet: "pyro_dmg", circlet: "crit_rate" },
  hsr: { head: "hp_flat", hands: "atk_flat", body: "crit_rate", feet: "speed", planar_sphere: "fire_dmg", link_rope: "atk_pct" },
};

export const COMPLETION_THRESHOLDS = [
  { key: "75", ratio: 0.75, label: "75%" },
  { key: "90", ratio: 0.9, label: "90%" },
  { key: "95", ratio: 0.95, label: "95%" },
  { key: "98", ratio: 0.98, label: "98%" },
  { key: "100", ratio: 1, label: "100%" },
];

export function hashSeed(seed) {
  let hash = 2166136261;
  const text = String(seed ?? "");
  for (let index = 0; index < text.length; index += 1) {
    hash ^= text.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return hash >>> 0;
}

export function mulberry32(seed) {
  return function next() {
    let value = (seed += 0x6d2b79f5);
    value = Math.imul(value ^ (value >>> 15), value | 1);
    value ^= value + Math.imul(value ^ (value >>> 7), value | 61);
    return ((value ^ (value >>> 14)) >>> 0) / 4294967296;
  };
}

function choice(items, rng) {
  return items[Math.floor(rng() * items.length)];
}

function weightedChoice(weights, rng) {
  const entries = Object.entries(weights);
  const total = entries.reduce((sum, [, weight]) => sum + weight, 0);
  let point = rng() * total;
  for (const [item, weight] of entries) {
    point -= weight;
    if (point <= 0) return item;
  }
  return entries.at(-1)[0];
}

export function round2(value) {
  return Math.round(value * 100) / 100;
}

export function slotList(game) {
  return game === "genshin" ? GENSHIN_SLOTS : HSR_SLOTS;
}

export function trackedStats(game, role, effectiveTags) {
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

export function averageRollValue(game, stat) {
  const rolls = game === "genshin" ? GENSHIN_ROLLS[stat] : HSR_ROLLS[stat];
  if (!rolls) return 1;
  return rolls.reduce((sum, value) => sum + value, 0) / rolls.length;
}

export function plannedMainContribution(game, stat, mainPlan) {
  const mainValues = game === "genshin" ? GENSHIN_MAIN_MAX : HSR_MAIN_MAX;
  if (stat === "crit_rate" || stat === "crit_dmg") {
    const critSlot = game === "genshin" ? "circlet" : "body";
    return mainPlan?.[critSlot] === stat ? mainValues[stat] || 0 : 0;
  }
  return slotList(game).reduce((total, slot) => total + (mainPlan?.[slot] === stat ? mainValues[stat] || 0 : 0), 0);
}

export function convertGoalValue(value, stat, fromMode, toMode, config) {
  const game = config.game;
  const rollValue = averageRollValue(game, stat);
  const fixedPanel = (Number(config.base?.[stat]) || 0) + plannedMainContribution(game, stat, config.mainPlan);
  if (fromMode === "hits" && toMode === "panel") {
    return value <= 0 ? fixedPanel : round2(fixedPanel + value * rollValue);
  }
  if (fromMode === "panel" && toMode === "hits") {
    return Math.max(0, Math.ceil((value - fixedPanel) / rollValue));
  }
  return value;
}

function drawSubstats(mainStat, count, weights, rng) {
  const chosen = [];
  for (let index = 0; index < count; index += 1) {
    const pool = {};
    for (const [stat, weight] of Object.entries(weights)) {
      if (stat !== mainStat && !chosen.includes(stat)) pool[stat] = weight;
    }
    chosen.push(weightedChoice(pool, rng));
  }
  return chosen;
}

function rollSubstat(name, rolls, rng) {
  const value = choice(rolls[name], rng);
  return { name, value: round2(value), rolls: [value] };
}

function enhance(item, maxLevel, step, weights, rolls, rng) {
  while (item.level < maxLevel) {
    if (item.substats.length < 4) {
      const existing = new Set(item.substats.map((substat) => substat.name));
      const pool = {};
      for (const [stat, weight] of Object.entries(weights)) {
        if (stat !== item.mainStat && !existing.has(stat)) pool[stat] = weight;
      }
      item.substats.push(rollSubstat(weightedChoice(pool, rng), rolls, rng));
    } else {
      const substat = choice(item.substats, rng);
      const roll = choice(rolls[substat.name], rng);
      substat.rolls.push(roll);
      substat.value = round2(substat.rolls.reduce((sum, value) => sum + value, 0));
    }
    item.level += step;
  }
  return item;
}

function generateGenshin(rng) {
  const slot = choice(GENSHIN_SLOTS, rng);
  const mainStat = weightedChoice(GENSHIN_MAIN[slot], rng);
  const initialCount = rng() < GENSHIN_INITIAL_THREE.domain ? 3 : 4;
  const substats = drawSubstats(mainStat, initialCount, GENSHIN_SUB_WEIGHTS, rng).map((name) =>
    rollSubstat(name, GENSHIN_ROLLS, rng),
  );
  return enhance({ game: "genshin", slot, mainStat, level: 0, substats }, 20, 4, GENSHIN_SUB_WEIGHTS, GENSHIN_ROLLS, rng);
}

function generateHsr(source, rng) {
  const slot = choice(source === "cavern" ? HSR_CAVERN_SLOTS : HSR_PLANAR_SLOTS, rng);
  const mainStat = weightedChoice(HSR_MAIN[slot], rng);
  const initialCount = rng() < 0.2 ? 4 : 3;
  const substats = drawSubstats(mainStat, initialCount, HSR_SUB_WEIGHTS, rng).map((name) =>
    rollSubstat(name, HSR_ROLLS, rng),
  );
  return enhance({ game: "hsr", source, slot, mainStat, level: 0, substats }, 15, 3, HSR_SUB_WEIGHTS, HSR_ROLLS, rng);
}

function itemMatchesPlan(item, config) {
  return config.mainPlan[item.slot] === item.mainStat;
}

function emptyBuild(config) {
  return Object.fromEntries(slotList(config.game).map((slot) => [slot, null]));
}

function cloneBuild(build) {
  return { ...build };
}

function itemContribution(item) {
  const values = {};
  const hits = {};
  const mainValues = item.game === "genshin" ? GENSHIN_MAIN_MAX : HSR_MAIN_MAX;
  values[item.mainStat] = (values[item.mainStat] || 0) + (mainValues[item.mainStat] || 0);
  for (const substat of item.substats) {
    values[substat.name] = (values[substat.name] || 0) + substat.value;
    hits[substat.name] = (hits[substat.name] || 0) + substat.rolls.length;
  }
  return { values, hits };
}

export function evaluateBuild(build, config) {
  const panel = { ...config.base };
  const hits = Object.fromEntries(config.stats.map((stat) => [stat, 0]));
  let filled = 0;
  for (const item of Object.values(build)) {
    if (!item) continue;
    filled += 1;
    const contribution = itemContribution(item);
    for (const [stat, value] of Object.entries(contribution.values)) {
      panel[stat] = (panel[stat] || 0) + value;
    }
    for (const [stat, value] of Object.entries(contribution.hits)) {
      hits[stat] = (hits[stat] || 0) + value;
    }
  }

  let capped = filled / slotList(config.game).length;
  let uncapped = capped;
  for (const stat of config.stats) {
    const panelTarget = config.panelGoals[stat] || 0;
    const hitTarget = config.hitGoals[stat] || 0;
    const panelRatio = panelTarget <= 0 ? 1 : (panel[stat] || 0) / panelTarget;
    const hitRatio = hitTarget <= 0 ? 1 : (hits[stat] || 0) / hitTarget;
    const ratio = Math.min(panelRatio, hitRatio);
    capped += Math.min(ratio, 1);
    uncapped += ratio;
  }

  return {
    panel,
    hits,
    filled,
    completion: config.stats.length ? capped / (config.stats.length + 1) : 0,
    score: capped * 1000 + uncapped,
  };
}

function isGraduated(evaluation, config) {
  if (evaluation.filled < slotList(config.game).length) return false;
  return config.stats.every((stat) => {
    const panelOk = (config.panelGoals[stat] || 0) <= 0 || (evaluation.panel[stat] || 0) >= config.panelGoals[stat];
    const hitsOk = (config.hitGoals[stat] || 0) <= 0 || (evaluation.hits[stat] || 0) >= config.hitGoals[stat];
    return panelOk && hitsOk;
  });
}

export function critValue(item) {
  const values = Object.fromEntries(item.substats.map((substat) => [substat.name, substat.value]));
  return round2((values.crit_rate || 0) * 2 + (values.crit_dmg || 0));
}

export function describeSubstats(item) {
  return item.substats
    .map((substat) => `${STAT_LABELS[substat.name]} ${formatStatValue(substat.name, substat.value)} (+${Math.max(0, substat.rolls.length - 1)})`)
    .join(" / ");
}

function milestoneStat(item, evaluation, config) {
  if (!item) return config.stats[0] || null;
  let bestStat = null;
  let bestGain = -Infinity;
  const contribution = itemContribution(item);
  for (const stat of config.stats) {
    const panelTarget = config.panelGoals[stat] || 1;
    const hitTarget = config.hitGoals[stat] || 1;
    const panelGain = (contribution.values[stat] || 0) / panelTarget;
    const hitGain = (contribution.hits[stat] || 0) / hitTarget;
    const gain = Math.max(panelGain, hitGain);
    if (gain > bestGain) {
      bestGain = gain;
      bestStat = stat;
    }
  }
  return bestStat || config.stats[0] || null;
}

function makeMilestone(type, run, item, evaluation, config) {
  const chartStat = milestoneStat(item, evaluation, config);
  return {
    type,
    run,
    days: run / config.runsPerDay,
    slot: item?.slot || null,
    title: type === "graduated" ? "整套首次达标" : `${SLOT_LABELS[item.slot]}${type === "equip_first" ? "首次装备" : "刷新装备"}`,
    panel: { ...evaluation.panel },
    hits: { ...evaluation.hits },
    completion: evaluation.completion,
    chartStat,
    item: item
      ? {
          slot: item.slot,
          mainStat: item.mainStat,
          critValue: critValue(item),
          substats: describeSubstats(item),
        }
      : null,
  };
}

function buildSnapshot(build, game) {
  return slotList(game).map((slot) => {
    const item = build[slot];
    return item
      ? {
          slot,
          mainStat: item.mainStat,
          critValue: critValue(item),
          substats: describeSubstats(item),
        }
      : { slot, empty: true };
  });
}

function finalPieceMarkers(build, config) {
  return Object.values(build)
    .filter((item) => item?.equippedAt)
    .map((item) => ({
      ...item.equippedAt,
      slot: item.slot,
      title: `${SLOT_LABELS[item.slot]}最终件出现`,
      item: {
        slot: item.slot,
        mainStat: item.mainStat,
        critValue: critValue(item),
        substats: describeSubstats(item),
      },
      buildSnapshot: item.equippedAt.buildSnapshot,
    }))
    .sort((a, b) => a.run - b.run || a.title.localeCompare(b.title, "zh-CN"));
}

function tryEquipDetailed(item, build, config, run) {
  if (!itemMatchesPlan(item, config)) return null;
  const currentEvaluation = evaluateBuild(build, config);
  const candidate = cloneBuild(build);
  candidate[item.slot] = item;
  const candidateEvaluation = evaluateBuild(candidate, config);
  if (candidateEvaluation.score <= currentEvaluation.score + 0.0001) return null;
  const type = build[item.slot] ? "equip_upgrade" : "equip_first";
  item.equippedAt = {
    type,
    run,
    days: run / config.runsPerDay,
    panel: { ...candidateEvaluation.panel },
    hits: { ...candidateEvaluation.hits },
    completion: candidateEvaluation.completion,
    chartStat: milestoneStat(item, candidateEvaluation, config),
    buildSnapshot: buildSnapshot(candidate, config.game),
  };
  build[item.slot] = item;
  return { type, evaluation: candidateEvaluation };
}

function makePlateauMarker(source, currentRun, config) {
  return {
    ...source,
    type: "plateau",
    title: `卡点快照：${SLOT_LABELS[source.slot]}后停滞`,
    plateauRun: currentRun,
    stallRuns: currentRun - source.run,
    stallDays: (currentRun - source.run) / config.runsPerDay,
  };
}

function snapshot(run, evaluation, config) {
  return {
    run,
    runs: run,
    days: run / config.runsPerDay,
    completion: evaluation.completion,
    values: Object.fromEntries(config.stats.map((stat) => [stat, evaluation.panel[stat] || 0])),
    hits: Object.fromEntries(config.stats.map((stat) => [stat, evaluation.hits[stat] || 0])),
  };
}

function createThresholdRuns() {
  return Object.fromEntries(COMPLETION_THRESHOLDS.map((threshold) => [threshold.key, null]));
}

function recordThresholdRuns(thresholdRuns, completion, run) {
  for (const threshold of COMPLETION_THRESHOLDS) {
    if (thresholdRuns[threshold.key] == null && completion >= threshold.ratio) {
      thresholdRuns[threshold.key] = run;
    }
  }
}

export function simulate(config) {
  const build = emptyBuild(config);
  const history = [];
  const milestones = [];
  const plateauMarkers = [];
  const thresholdRuns = createThresholdRuns();
  const rng = mulberry32(hashSeed(config.seed));
  const cavernRng = mulberry32(hashSeed(`${config.seed}:cavern`));
  const planarRng = mulberry32(hashSeed(`${config.seed}:planar`));
  const interval = Math.max(1, Math.floor(config.maxRuns / 600));
  const plateauWindow = Math.max(50, Math.floor(config.maxRuns / 20));
  let stopRun = config.maxRuns;
  let stopReason = "达到上限";
  let lastEvaluation = evaluateBuild(build, config);
  let lastCompletionRun = 0;
  let lastCompletion = lastEvaluation.completion;
  let lastChangeMarker = null;
  let lastPlateauKey = null;

  function recordPlateauIfNeeded(run) {
    if (!lastChangeMarker || run - lastChangeMarker.run < plateauWindow || run - lastCompletionRun < plateauWindow || plateauMarkers.length >= 80) return;
    const key = `${lastChangeMarker.run}:${lastChangeMarker.slot}`;
    if (key === lastPlateauKey) return;
    plateauMarkers.push(makePlateauMarker(lastChangeMarker, run, config));
    lastPlateauKey = key;
  }

  history.push(snapshot(0, lastEvaluation, config));

  for (let run = 1; run <= config.maxRuns; run += 1) {
    const items = config.game === "genshin" ? [generateGenshin(rng)] : [generateHsr("cavern", cavernRng), generateHsr("planar", planarRng)];
    for (const item of items) {
      const change = tryEquipDetailed(item, build, config, run);
      if (!change) continue;
      lastEvaluation = change.evaluation;
      const milestone = makeMilestone(change.type, run, item, lastEvaluation, config);
      milestone.buildSnapshot = item.equippedAt.buildSnapshot;
      lastChangeMarker = milestone;
      if (lastEvaluation.completion > lastCompletion + 0.0001) {
        lastCompletion = lastEvaluation.completion;
        lastCompletionRun = run;
      }
      recordThresholdRuns(thresholdRuns, lastEvaluation.completion, run);
      if (milestones.length < 160) milestones.push(milestone);
      if (isGraduated(lastEvaluation, config)) {
        stopRun = run;
        stopReason = "已达标";
        milestones.push(makeMilestone("graduated", run, null, lastEvaluation, config));
        history.push(snapshot(run, lastEvaluation, config));
        return {
          config,
          build,
          evaluation: lastEvaluation,
          history,
          milestones,
          finalMarkers: finalPieceMarkers(build, config),
          plateauMarkers,
          thresholdRuns,
          stopRun,
          stopReason,
        };
      }
    }

    if (run % interval === 0 || run === config.maxRuns) {
      lastEvaluation = evaluateBuild(build, config);
      history.push(snapshot(run, lastEvaluation, config));
      recordPlateauIfNeeded(run);
    }
  }

  lastEvaluation = evaluateBuild(build, config);
  recordPlateauIfNeeded(stopRun);
  recordThresholdRuns(thresholdRuns, lastEvaluation.completion, stopRun);
  return {
    config,
    build,
    evaluation: lastEvaluation,
    history,
    milestones,
    finalMarkers: finalPieceMarkers(build, config),
    plateauMarkers,
    thresholdRuns,
    stopRun,
    stopReason,
  };
}

function tryEquipAverage(item, build, config, currentEvaluation) {
  if (!itemMatchesPlan(item, config)) return null;
  const candidate = cloneBuild(build);
  candidate[item.slot] = item;
  const candidateEvaluation = evaluateBuild(candidate, config);
  if (candidateEvaluation.score <= currentEvaluation.score + 0.0001) return null;
  build[item.slot] = item;
  return candidateEvaluation;
}

export function simulateGraduation(config, sampleIndex) {
  const seed = `${config.seed}:${sampleIndex}`;
  const build = emptyBuild(config);
  const thresholdRuns = createThresholdRuns();
  const rng = mulberry32(hashSeed(seed));
  const cavernRng = mulberry32(hashSeed(`${seed}:cavern`));
  const planarRng = mulberry32(hashSeed(`${seed}:planar`));
  let evaluation = evaluateBuild(build, config);

  for (let run = 1; run <= config.maxRuns; run += 1) {
    const items = config.game === "genshin" ? [generateGenshin(rng)] : [generateHsr("cavern", cavernRng), generateHsr("planar", planarRng)];
    for (const item of items) {
      const nextEvaluation = tryEquipAverage(item, build, config, evaluation);
      if (!nextEvaluation) continue;
      evaluation = nextEvaluation;
      recordThresholdRuns(thresholdRuns, evaluation.completion, run);
      if (isGraduated(evaluation, config)) return { graduated: true, stopRun: run, thresholdRuns };
    }
  }

  recordThresholdRuns(thresholdRuns, evaluation.completion, config.maxRuns);
  return { graduated: false, stopRun: config.maxRuns, thresholdRuns };
}

export function formatNumber(value) {
  if (!Number.isFinite(value)) return "-";
  if (value >= 10000) return Math.round(value).toLocaleString("zh-CN");
  if (Number.isInteger(value)) return String(value);
  if (value >= 100) return String(Math.round(value));
  if (value >= 10) return value.toFixed(1);
  return value.toFixed(2);
}

export function percentile(sorted, ratio) {
  if (!sorted.length) return NaN;
  const index = Math.min(sorted.length - 1, Math.max(0, Math.ceil(sorted.length * ratio) - 1));
  return sorted[index];
}

export function standardDeviation(values, average) {
  if (values.length < 2 || !Number.isFinite(average)) return NaN;
  const variance = values.reduce((sum, value) => sum + (value - average) ** 2, 0) / values.length;
  return Math.sqrt(variance);
}

export function makeBuckets(values, maxRuns, bucketCount = 10) {
  const bucketSize = Math.max(1, Math.ceil(maxRuns / bucketCount));
  const buckets = Array.from({ length: bucketCount }, (_, index) => {
    const start = index * bucketSize + 1;
    const end = Math.min(maxRuns, (index + 1) * bucketSize);
    return { start, end, count: 0 };
  });
  for (const value of values) {
    const index = Math.min(bucketCount - 1, Math.max(0, Math.floor((value - 1) / bucketSize)));
    buckets[index].count += 1;
  }
  return buckets;
}

function summarizeThresholdSamples(config, sampleResults, done) {
  return COMPLETION_THRESHOLDS.map((threshold) => {
    const runs = sampleResults
      .map((result) => result?.thresholdRuns?.[threshold.key])
      .filter((run) => Number.isFinite(run))
      .sort((a, b) => a - b);
    const reached = runs.length;
    const average = reached ? runs.reduce((total, run) => total + run, 0) / reached : NaN;
    return {
      ...threshold,
      runs,
      reached,
      rate: reached / Math.max(1, done),
      average,
      averageDays: average / config.runsPerDay,
      median: percentile(runs, 0.5),
      p90: percentile(runs, 0.9),
      p95: percentile(runs, 0.95),
      min: runs[0],
      max: runs.at(-1),
    };
  });
}

export function summarizeAverage(config, sampleResults, done = config.sampleCount) {
  const thresholds = summarizeThresholdSamples(config, sampleResults, done);
  const graduation = thresholds.find((threshold) => threshold.key === "100") ?? {
    runs: [],
    reached: 0,
    rate: 0,
    average: NaN,
    averageDays: NaN,
    median: NaN,
    p90: NaN,
    p95: NaN,
    min: NaN,
    max: NaN,
  };
  const sorted = graduation.runs;
  const success = graduation.reached;
  const average = graduation.average;
  return {
    done,
    total: config.sampleCount,
    success,
    failed: done - success,
    successRuns: sorted,
    successRate: graduation.rate,
    average,
    averageDays: graduation.averageDays,
    median: graduation.median,
    p90: graduation.p90,
    p95: graduation.p95,
    p10: percentile(sorted, 0.1),
    p25: percentile(sorted, 0.25),
    p75: percentile(sorted, 0.75),
    min: graduation.min,
    max: graduation.max,
    stdev: standardDeviation(sorted, average),
    buckets: makeBuckets(sorted, config.maxRuns),
    thresholds,
  };
}

export function averageSummary(config) {
  const sampleResults = [];
  for (let index = 0; index < config.sampleCount; index += 1) {
    const result = simulateGraduation(config, index);
    sampleResults.push(result);
  }
  return summarizeAverage(config, sampleResults, config.sampleCount);
}

export function visibleMilestones(result) {
  return [...(result.finalMarkers || []), ...(result.plateauMarkers || [])].sort((a, b) => a.run - b.run || a.title.localeCompare(b.title, "zh-CN"));
}

export function milestoneTypeLabel(type) {
  if (type === "plateau") return "卡点快照";
  if (type === "equip_first") return "最终新部位";
  if (type === "equip_upgrade") return "最终替换件";
  if (type === "graduated") return "达标";
  return "节点";
}
