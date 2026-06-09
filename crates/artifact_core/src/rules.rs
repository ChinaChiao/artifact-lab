use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Game {
    Genshin,
    Hsr,
}

impl Game {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Genshin => "genshin",
            Self::Hsr => "hsr",
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "genshin" => Ok(Self::Genshin),
            "hsr" => Ok(Self::Hsr),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    Domain,
    Boss,
    Elite,
    Strongbox,
    Cavern,
    Planar,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Boss => "boss",
            Self::Elite => "elite",
            Self::Strongbox => "strongbox",
            Self::Cavern => "cavern",
            Self::Planar => "planar",
        }
    }
}

impl TryFrom<&str> for Source {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "domain" => Ok(Self::Domain),
            "boss" => Ok(Self::Boss),
            "elite" => Ok(Self::Elite),
            "strongbox" => Ok(Self::Strongbox),
            "cavern" => Ok(Self::Cavern),
            "planar" => Ok(Self::Planar),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot {
    Flower,
    Plume,
    Sands,
    Goblet,
    Circlet,
    Head,
    Hands,
    Body,
    Feet,
    PlanarSphere,
    LinkRope,
}

impl Slot {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Flower => "flower",
            Self::Plume => "plume",
            Self::Sands => "sands",
            Self::Goblet => "goblet",
            Self::Circlet => "circlet",
            Self::Head => "head",
            Self::Hands => "hands",
            Self::Body => "body",
            Self::Feet => "feet",
            Self::PlanarSphere => "planar_sphere",
            Self::LinkRope => "link_rope",
        }
    }
}

impl TryFrom<&str> for Slot {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "flower" => Ok(Self::Flower),
            "plume" => Ok(Self::Plume),
            "sands" => Ok(Self::Sands),
            "goblet" => Ok(Self::Goblet),
            "circlet" => Ok(Self::Circlet),
            "head" => Ok(Self::Head),
            "hands" => Ok(Self::Hands),
            "body" => Ok(Self::Body),
            "feet" => Ok(Self::Feet),
            "planar_sphere" => Ok(Self::PlanarSphere),
            "link_rope" => Ok(Self::LinkRope),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
    HpFlat,
    AtkFlat,
    DefFlat,
    HpPct,
    AtkPct,
    DefPct,
    EnergyRecharge,
    ElementalMastery,
    PyroDmg,
    ElectroDmg,
    CryoDmg,
    HydroDmg,
    DendroDmg,
    AnemoDmg,
    GeoDmg,
    PhysicalDmg,
    CritRate,
    CritDmg,
    HealingBonus,
    Speed,
    OutgoingHealing,
    EffectHitRate,
    EffectRes,
    BreakEffect,
    EnergyRegen,
    FireDmg,
    IceDmg,
    LightningDmg,
    WindDmg,
    QuantumDmg,
    ImaginaryDmg,
}

impl Stat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HpFlat => "hp_flat",
            Self::AtkFlat => "atk_flat",
            Self::DefFlat => "def_flat",
            Self::HpPct => "hp_pct",
            Self::AtkPct => "atk_pct",
            Self::DefPct => "def_pct",
            Self::EnergyRecharge => "energy_recharge",
            Self::ElementalMastery => "elemental_mastery",
            Self::PyroDmg => "pyro_dmg",
            Self::ElectroDmg => "electro_dmg",
            Self::CryoDmg => "cryo_dmg",
            Self::HydroDmg => "hydro_dmg",
            Self::DendroDmg => "dendro_dmg",
            Self::AnemoDmg => "anemo_dmg",
            Self::GeoDmg => "geo_dmg",
            Self::PhysicalDmg => "physical_dmg",
            Self::CritRate => "crit_rate",
            Self::CritDmg => "crit_dmg",
            Self::HealingBonus => "healing_bonus",
            Self::Speed => "speed",
            Self::OutgoingHealing => "outgoing_healing",
            Self::EffectHitRate => "effect_hit_rate",
            Self::EffectRes => "effect_res",
            Self::BreakEffect => "break_effect",
            Self::EnergyRegen => "energy_regen",
            Self::FireDmg => "fire_dmg",
            Self::IceDmg => "ice_dmg",
            Self::LightningDmg => "lightning_dmg",
            Self::WindDmg => "wind_dmg",
            Self::QuantumDmg => "quantum_dmg",
            Self::ImaginaryDmg => "imaginary_dmg",
        }
    }
}

impl TryFrom<&str> for Stat {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "hp_flat" => Ok(Self::HpFlat),
            "atk_flat" => Ok(Self::AtkFlat),
            "def_flat" => Ok(Self::DefFlat),
            "hp_pct" => Ok(Self::HpPct),
            "atk_pct" => Ok(Self::AtkPct),
            "def_pct" => Ok(Self::DefPct),
            "energy_recharge" => Ok(Self::EnergyRecharge),
            "elemental_mastery" => Ok(Self::ElementalMastery),
            "pyro_dmg" => Ok(Self::PyroDmg),
            "electro_dmg" => Ok(Self::ElectroDmg),
            "cryo_dmg" => Ok(Self::CryoDmg),
            "hydro_dmg" => Ok(Self::HydroDmg),
            "dendro_dmg" => Ok(Self::DendroDmg),
            "anemo_dmg" => Ok(Self::AnemoDmg),
            "geo_dmg" => Ok(Self::GeoDmg),
            "physical_dmg" => Ok(Self::PhysicalDmg),
            "crit_rate" => Ok(Self::CritRate),
            "crit_dmg" => Ok(Self::CritDmg),
            "healing_bonus" => Ok(Self::HealingBonus),
            "speed" => Ok(Self::Speed),
            "outgoing_healing" => Ok(Self::OutgoingHealing),
            "effect_hit_rate" => Ok(Self::EffectHitRate),
            "effect_res" => Ok(Self::EffectRes),
            "break_effect" => Ok(Self::BreakEffect),
            "energy_regen" => Ok(Self::EnergyRegen),
            "fire_dmg" => Ok(Self::FireDmg),
            "ice_dmg" => Ok(Self::IceDmg),
            "lightning_dmg" => Ok(Self::LightningDmg),
            "wind_dmg" => Ok(Self::WindDmg),
            "quantum_dmg" => Ok(Self::QuantumDmg),
            "imaginary_dmg" => Ok(Self::ImaginaryDmg),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WeightedStat {
    pub stat: Stat,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MainStatValue {
    pub stat: Stat,
    pub base: f64,
    pub maximum: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct CompletionThreshold {
    pub key: &'static str,
    pub ratio: f64,
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceQuantile {
    pub key: &'static str,
    pub ratio: f64,
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct StatValue {
    pub stat: Stat,
    pub value: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SlotStat {
    pub slot: Slot,
    pub stat: Stat,
}

const GENSHIN_SLOTS: [Slot; 5] = [
    Slot::Flower,
    Slot::Plume,
    Slot::Sands,
    Slot::Goblet,
    Slot::Circlet,
];
const HSR_SLOTS: [Slot; 6] = [
    Slot::Head,
    Slot::Hands,
    Slot::Body,
    Slot::Feet,
    Slot::PlanarSphere,
    Slot::LinkRope,
];
const HSR_CAVERN_SLOTS: [Slot; 4] = [Slot::Head, Slot::Hands, Slot::Body, Slot::Feet];
const HSR_PLANAR_SLOTS: [Slot; 2] = [Slot::PlanarSphere, Slot::LinkRope];

const GENSHIN_SLOT_NAMES: [&str; 5] = ["flower", "plume", "sands", "goblet", "circlet"];
const HSR_SLOT_NAMES: [&str; 6] = [
    "head",
    "hands",
    "body",
    "feet",
    "planar_sphere",
    "link_rope",
];

const GENSHIN_SOURCES: [Source; 4] = [
    Source::Domain,
    Source::Boss,
    Source::Elite,
    Source::Strongbox,
];
const HSR_SOURCES: [Source; 2] = [Source::Cavern, Source::Planar];
const GENSHIN_SOURCE_NAMES: [&str; 4] = ["domain", "boss", "elite", "strongbox"];
const HSR_SOURCE_NAMES: [&str; 2] = ["cavern", "planar"];

const GENSHIN_FLOWER_MAIN: [WeightedStat; 1] = [WeightedStat {
    stat: Stat::HpFlat,
    weight: 1.0,
}];
const GENSHIN_PLUME_MAIN: [WeightedStat; 1] = [WeightedStat {
    stat: Stat::AtkFlat,
    weight: 1.0,
}];
const GENSHIN_SANDS_MAIN: [WeightedStat; 5] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 26.68,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 26.66,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 26.66,
    },
    WeightedStat {
        stat: Stat::EnergyRecharge,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::ElementalMastery,
        weight: 10.0,
    },
];
const GENSHIN_GOBLET_MAIN: [WeightedStat; 12] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 19.25,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 19.25,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 19.0,
    },
    WeightedStat {
        stat: Stat::PyroDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::ElectroDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::CryoDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::HydroDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::DendroDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::AnemoDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::GeoDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::PhysicalDmg,
        weight: 5.0,
    },
    WeightedStat {
        stat: Stat::ElementalMastery,
        weight: 2.5,
    },
];
const GENSHIN_CIRCLET_MAIN: [WeightedStat; 7] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 22.0,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 22.0,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 22.0,
    },
    WeightedStat {
        stat: Stat::CritRate,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::CritDmg,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::HealingBonus,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::ElementalMastery,
        weight: 4.0,
    },
];

const HSR_HEAD_MAIN: [WeightedStat; 1] = [WeightedStat {
    stat: Stat::HpFlat,
    weight: 1.0,
}];
const HSR_HANDS_MAIN: [WeightedStat; 1] = [WeightedStat {
    stat: Stat::AtkFlat,
    weight: 1.0,
}];
const HSR_BODY_MAIN: [WeightedStat; 7] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 20.0,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 20.0,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 20.0,
    },
    WeightedStat {
        stat: Stat::CritRate,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::CritDmg,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::OutgoingHealing,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::EffectHitRate,
        weight: 10.0,
    },
];
const HSR_FEET_MAIN: [WeightedStat; 4] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 29.17,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 29.17,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 29.16,
    },
    WeightedStat {
        stat: Stat::Speed,
        weight: 12.5,
    },
];
const HSR_SPHERE_MAIN: [WeightedStat; 10] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 12.34,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 12.33,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 12.33,
    },
    WeightedStat {
        stat: Stat::PhysicalDmg,
        weight: 9.0,
    },
    WeightedStat {
        stat: Stat::FireDmg,
        weight: 9.0,
    },
    WeightedStat {
        stat: Stat::IceDmg,
        weight: 9.0,
    },
    WeightedStat {
        stat: Stat::LightningDmg,
        weight: 9.0,
    },
    WeightedStat {
        stat: Stat::WindDmg,
        weight: 9.0,
    },
    WeightedStat {
        stat: Stat::QuantumDmg,
        weight: 9.0,
    },
    WeightedStat {
        stat: Stat::ImaginaryDmg,
        weight: 9.0,
    },
];
const HSR_ROPE_MAIN: [WeightedStat; 5] = [
    WeightedStat {
        stat: Stat::HpPct,
        weight: 26.33,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 26.34,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 26.33,
    },
    WeightedStat {
        stat: Stat::BreakEffect,
        weight: 15.0,
    },
    WeightedStat {
        stat: Stat::EnergyRegen,
        weight: 6.0,
    },
];

const GENSHIN_SUBSTAT_WEIGHTS: [WeightedStat; 10] = [
    WeightedStat {
        stat: Stat::HpFlat,
        weight: 6.0,
    },
    WeightedStat {
        stat: Stat::AtkFlat,
        weight: 6.0,
    },
    WeightedStat {
        stat: Stat::DefFlat,
        weight: 6.0,
    },
    WeightedStat {
        stat: Stat::HpPct,
        weight: 4.0,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 4.0,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 4.0,
    },
    WeightedStat {
        stat: Stat::EnergyRecharge,
        weight: 4.0,
    },
    WeightedStat {
        stat: Stat::ElementalMastery,
        weight: 4.0,
    },
    WeightedStat {
        stat: Stat::CritRate,
        weight: 3.0,
    },
    WeightedStat {
        stat: Stat::CritDmg,
        weight: 3.0,
    },
];
const HSR_SUBSTAT_WEIGHTS: [WeightedStat; 12] = [
    WeightedStat {
        stat: Stat::HpFlat,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::AtkFlat,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::DefFlat,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::HpPct,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::AtkPct,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::DefPct,
        weight: 10.0,
    },
    WeightedStat {
        stat: Stat::Speed,
        weight: 4.0,
    },
    WeightedStat {
        stat: Stat::CritRate,
        weight: 6.0,
    },
    WeightedStat {
        stat: Stat::CritDmg,
        weight: 6.0,
    },
    WeightedStat {
        stat: Stat::EffectHitRate,
        weight: 8.0,
    },
    WeightedStat {
        stat: Stat::EffectRes,
        weight: 8.0,
    },
    WeightedStat {
        stat: Stat::BreakEffect,
        weight: 8.0,
    },
];

const GENSHIN_MAIN_VALUES: [MainStatValue; 18] = [
    MainStatValue {
        stat: Stat::HpFlat,
        base: 717.0,
        maximum: 4780.0,
    },
    MainStatValue {
        stat: Stat::AtkFlat,
        base: 47.0,
        maximum: 311.0,
    },
    MainStatValue {
        stat: Stat::HpPct,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::AtkPct,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::DefPct,
        base: 8.7,
        maximum: 58.3,
    },
    MainStatValue {
        stat: Stat::EnergyRecharge,
        base: 7.8,
        maximum: 51.8,
    },
    MainStatValue {
        stat: Stat::ElementalMastery,
        base: 28.0,
        maximum: 186.5,
    },
    MainStatValue {
        stat: Stat::PyroDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::ElectroDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::CryoDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::HydroDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::DendroDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::AnemoDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::GeoDmg,
        base: 7.0,
        maximum: 46.6,
    },
    MainStatValue {
        stat: Stat::PhysicalDmg,
        base: 8.7,
        maximum: 58.3,
    },
    MainStatValue {
        stat: Stat::CritRate,
        base: 4.7,
        maximum: 31.1,
    },
    MainStatValue {
        stat: Stat::CritDmg,
        base: 9.3,
        maximum: 62.2,
    },
    MainStatValue {
        stat: Stat::HealingBonus,
        base: 5.4,
        maximum: 35.9,
    },
];

const HSR_MAIN_VALUES: [MainStatValue; 19] = [
    MainStatValue {
        stat: Stat::HpFlat,
        base: 112.896,
        maximum: 705.6,
    },
    MainStatValue {
        stat: Stat::AtkFlat,
        base: 56.448,
        maximum: 352.8,
    },
    MainStatValue {
        stat: Stat::HpPct,
        base: 6.912,
        maximum: 43.2,
    },
    MainStatValue {
        stat: Stat::AtkPct,
        base: 6.912,
        maximum: 43.2,
    },
    MainStatValue {
        stat: Stat::DefPct,
        base: 8.64,
        maximum: 54.0,
    },
    MainStatValue {
        stat: Stat::Speed,
        base: 4.032,
        maximum: 25.032,
    },
    MainStatValue {
        stat: Stat::CritRate,
        base: 5.184,
        maximum: 32.4,
    },
    MainStatValue {
        stat: Stat::CritDmg,
        base: 10.368,
        maximum: 64.8,
    },
    MainStatValue {
        stat: Stat::OutgoingHealing,
        base: 5.5296,
        maximum: 34.5606,
    },
    MainStatValue {
        stat: Stat::EffectHitRate,
        base: 6.912,
        maximum: 43.2,
    },
    MainStatValue {
        stat: Stat::PhysicalDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::FireDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::IceDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::LightningDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::WindDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::QuantumDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::ImaginaryDmg,
        base: 6.2208,
        maximum: 38.8803,
    },
    MainStatValue {
        stat: Stat::BreakEffect,
        base: 10.368,
        maximum: 64.8,
    },
    MainStatValue {
        stat: Stat::EnergyRegen,
        base: 3.1104,
        maximum: 19.4394,
    },
];

const GENSHIN_DEFAULT_MAIN_PLAN: [SlotStat; 5] = [
    SlotStat {
        slot: Slot::Flower,
        stat: Stat::HpFlat,
    },
    SlotStat {
        slot: Slot::Plume,
        stat: Stat::AtkFlat,
    },
    SlotStat {
        slot: Slot::Sands,
        stat: Stat::AtkPct,
    },
    SlotStat {
        slot: Slot::Goblet,
        stat: Stat::PyroDmg,
    },
    SlotStat {
        slot: Slot::Circlet,
        stat: Stat::CritRate,
    },
];
const HSR_DEFAULT_MAIN_PLAN: [SlotStat; 6] = [
    SlotStat {
        slot: Slot::Head,
        stat: Stat::HpFlat,
    },
    SlotStat {
        slot: Slot::Hands,
        stat: Stat::AtkFlat,
    },
    SlotStat {
        slot: Slot::Body,
        stat: Stat::CritRate,
    },
    SlotStat {
        slot: Slot::Feet,
        stat: Stat::Speed,
    },
    SlotStat {
        slot: Slot::PlanarSphere,
        stat: Stat::FireDmg,
    },
    SlotStat {
        slot: Slot::LinkRope,
        stat: Stat::AtkPct,
    },
];

const COMPLETION_THRESHOLDS: [CompletionThreshold; 5] = [
    CompletionThreshold {
        key: "75",
        ratio: 0.75,
        label: "75%",
    },
    CompletionThreshold {
        key: "90",
        ratio: 0.90,
        label: "90%",
    },
    CompletionThreshold {
        key: "95",
        ratio: 0.95,
        label: "95%",
    },
    CompletionThreshold {
        key: "98",
        ratio: 0.98,
        label: "98%",
    },
    CompletionThreshold {
        key: "100",
        ratio: 1.00,
        label: "100%",
    },
];
const RESOURCE_QUANTILES: [ResourceQuantile; 4] = [
    ResourceQuantile {
        key: "median",
        ratio: 0.50,
        label: "P50",
    },
    ResourceQuantile {
        key: "p75",
        ratio: 0.75,
        label: "P75",
    },
    ResourceQuantile {
        key: "p90",
        ratio: 0.90,
        label: "P90",
    },
    ResourceQuantile {
        key: "p95",
        ratio: 0.95,
        label: "P95",
    },
];

pub fn slots(game: Game) -> &'static [Slot] {
    match game {
        Game::Genshin => &GENSHIN_SLOTS,
        Game::Hsr => &HSR_SLOTS,
    }
}

pub fn slot_names(game: Game) -> &'static [&'static str] {
    match game {
        Game::Genshin => &GENSHIN_SLOT_NAMES,
        Game::Hsr => &HSR_SLOT_NAMES,
    }
}

pub fn sources(game: Game) -> &'static [Source] {
    match game {
        Game::Genshin => &GENSHIN_SOURCES,
        Game::Hsr => &HSR_SOURCES,
    }
}

pub fn source_names(game: Game) -> &'static [&'static str] {
    match game {
        Game::Genshin => &GENSHIN_SOURCE_NAMES,
        Game::Hsr => &HSR_SOURCE_NAMES,
    }
}

pub fn source_is_valid(game: Game, source: Source) -> bool {
    sources(game).contains(&source)
}

pub fn slots_for_source(game: Game, source: Source) -> &'static [Slot] {
    match (game, source) {
        (Game::Genshin, _) => &GENSHIN_SLOTS,
        (Game::Hsr, Source::Cavern) => &HSR_CAVERN_SLOTS,
        (Game::Hsr, Source::Planar) => &HSR_PLANAR_SLOTS,
        (Game::Hsr, _) => &[],
    }
}

pub fn max_level(game: Game) -> i32 {
    match game {
        Game::Genshin => 20,
        Game::Hsr => 15,
    }
}

pub fn level_step(game: Game) -> i32 {
    match game {
        Game::Genshin => 4,
        Game::Hsr => 3,
    }
}

pub fn initial_three_substat_prob(source: Source) -> Option<f64> {
    match source {
        Source::Domain => Some(0.80),
        Source::Boss => Some(0.66),
        Source::Elite => Some(0.90),
        Source::Strongbox => Some(0.66),
        _ => None,
    }
}

pub fn main_stat_weights(game: Game, slot: Slot) -> &'static [WeightedStat] {
    match (game, slot) {
        (Game::Genshin, Slot::Flower) => &GENSHIN_FLOWER_MAIN,
        (Game::Genshin, Slot::Plume) => &GENSHIN_PLUME_MAIN,
        (Game::Genshin, Slot::Sands) => &GENSHIN_SANDS_MAIN,
        (Game::Genshin, Slot::Goblet) => &GENSHIN_GOBLET_MAIN,
        (Game::Genshin, Slot::Circlet) => &GENSHIN_CIRCLET_MAIN,
        (Game::Hsr, Slot::Head) => &HSR_HEAD_MAIN,
        (Game::Hsr, Slot::Hands) => &HSR_HANDS_MAIN,
        (Game::Hsr, Slot::Body) => &HSR_BODY_MAIN,
        (Game::Hsr, Slot::Feet) => &HSR_FEET_MAIN,
        (Game::Hsr, Slot::PlanarSphere) => &HSR_SPHERE_MAIN,
        (Game::Hsr, Slot::LinkRope) => &HSR_ROPE_MAIN,
        _ => &[],
    }
}

pub fn substat_weights(game: Game) -> &'static [WeightedStat] {
    match game {
        Game::Genshin => &GENSHIN_SUBSTAT_WEIGHTS,
        Game::Hsr => &HSR_SUBSTAT_WEIGHTS,
    }
}

pub fn main_values(game: Game) -> &'static [MainStatValue] {
    match game {
        Game::Genshin => &GENSHIN_MAIN_VALUES,
        Game::Hsr => &HSR_MAIN_VALUES,
    }
}

pub fn main_stat_value_rule(game: Game, stat: Stat) -> Option<MainStatValue> {
    main_values(game)
        .iter()
        .find(|entry| entry.stat == stat)
        .copied()
}

pub fn substat_rolls(game: Game, stat: Stat) -> Option<&'static [f64]> {
    match (game, stat) {
        (Game::Genshin, Stat::HpFlat) => Some(&[209.13, 239.0, 268.88, 298.75]),
        (Game::Genshin, Stat::AtkFlat) => Some(&[13.62, 15.56, 17.51, 19.45]),
        (Game::Genshin, Stat::DefFlat) => Some(&[16.2, 18.52, 20.83, 23.15]),
        (Game::Genshin, Stat::HpPct) | (Game::Genshin, Stat::AtkPct) => {
            Some(&[4.08, 4.66, 5.25, 5.83])
        }
        (Game::Genshin, Stat::DefPct) => Some(&[5.1, 5.83, 6.56, 7.29]),
        (Game::Genshin, Stat::ElementalMastery) => Some(&[16.32, 18.65, 20.98, 23.31]),
        (Game::Genshin, Stat::EnergyRecharge) => Some(&[4.53, 5.18, 5.83, 6.48]),
        (Game::Genshin, Stat::CritRate) => Some(&[2.72, 3.11, 3.5, 3.89]),
        (Game::Genshin, Stat::CritDmg) => Some(&[5.44, 6.22, 6.99, 7.77]),
        (Game::Hsr, Stat::HpFlat) => Some(&[33.87, 38.1, 42.34]),
        (Game::Hsr, Stat::AtkFlat) | (Game::Hsr, Stat::DefFlat) => Some(&[16.94, 19.05, 21.17]),
        (Game::Hsr, Stat::HpPct)
        | (Game::Hsr, Stat::AtkPct)
        | (Game::Hsr, Stat::EffectHitRate)
        | (Game::Hsr, Stat::EffectRes) => Some(&[3.46, 3.89, 4.32]),
        (Game::Hsr, Stat::DefPct) => Some(&[4.32, 4.86, 5.4]),
        (Game::Hsr, Stat::Speed) => Some(&[2.0, 2.3, 2.6]),
        (Game::Hsr, Stat::CritRate) => Some(&[2.59, 2.92, 3.24]),
        (Game::Hsr, Stat::CritDmg) | (Game::Hsr, Stat::BreakEffect) => Some(&[5.18, 5.83, 6.48]),
        _ => None,
    }
}

pub fn main_stat_value(game: Game, stat: Stat, level: i32) -> Option<f64> {
    let rule = main_stat_value_rule(game, stat)?;
    if level == max_level(game) {
        return Some(rule.maximum);
    }
    let value = rule.base + (rule.maximum - rule.base) * (level as f64 / max_level(game) as f64);
    match game {
        Game::Genshin => Some(value.trunc()),
        Game::Hsr if matches!(stat, Stat::HpFlat | Stat::AtkFlat | Stat::Speed) => {
            Some(value.trunc())
        }
        Game::Hsr => Some((value * 100.0).round() / 100.0),
    }
}

pub fn main_max_values_map(game: Game) -> HashMap<String, f64> {
    main_values(game)
        .iter()
        .map(|entry| (entry.stat.as_str().to_string(), entry.maximum))
        .collect()
}

pub fn weights_map(weights: &[WeightedStat]) -> HashMap<String, f64> {
    weights
        .iter()
        .map(|entry| (entry.stat.as_str().to_string(), entry.weight))
        .collect()
}

pub fn main_weights_map(game: Game) -> HashMap<String, HashMap<String, f64>> {
    slots(game)
        .iter()
        .map(|slot| {
            (
                slot.as_str().to_string(),
                weights_map(main_stat_weights(game, *slot)),
            )
        })
        .collect()
}

pub fn substat_weights_map(game: Game) -> HashMap<String, f64> {
    weights_map(substat_weights(game))
}

pub fn substat_rolls_map(game: Game) -> HashMap<String, Vec<f64>> {
    substat_weights(game)
        .iter()
        .filter_map(|entry| {
            substat_rolls(game, entry.stat)
                .map(|rolls| (entry.stat.as_str().to_string(), rolls.to_vec()))
        })
        .collect()
}

pub fn default_main_plan(game: Game) -> &'static [SlotStat] {
    match game {
        Game::Genshin => &GENSHIN_DEFAULT_MAIN_PLAN,
        Game::Hsr => &HSR_DEFAULT_MAIN_PLAN,
    }
}

pub fn default_main_plan_map(game: Game) -> HashMap<String, String> {
    default_main_plan(game)
        .iter()
        .map(|entry| {
            (
                entry.slot.as_str().to_string(),
                entry.stat.as_str().to_string(),
            )
        })
        .collect()
}

pub fn main_options_map(game: Game) -> HashMap<String, Vec<String>> {
    slots(game)
        .iter()
        .map(|slot| {
            (
                slot.as_str().to_string(),
                main_stat_weights(game, *slot)
                    .iter()
                    .map(|entry| entry.stat.as_str().to_string())
                    .collect(),
            )
        })
        .collect()
}

pub fn completion_thresholds() -> &'static [CompletionThreshold] {
    &COMPLETION_THRESHOLDS
}

pub fn resource_quantiles() -> &'static [ResourceQuantile] {
    &RESOURCE_QUANTILES
}

pub fn dps_defaults(game: Game) -> &'static [&'static str] {
    match game {
        Game::Genshin => &["atk_pct"],
        Game::Hsr => &["atk_pct", "speed"],
    }
}

pub fn support_defaults(game: Game) -> &'static [&'static str] {
    match game {
        Game::Genshin => &["energy_recharge", "elemental_mastery"],
        Game::Hsr => &["speed", "effect_hit_rate"],
    }
}

pub fn effective_stats(game: Game) -> &'static [&'static str] {
    match game {
        Game::Genshin => &[
            "atk_pct",
            "hp_pct",
            "def_pct",
            "energy_recharge",
            "elemental_mastery",
        ],
        Game::Hsr => &[
            "atk_pct",
            "hp_pct",
            "def_pct",
            "speed",
            "effect_hit_rate",
            "effect_res",
            "break_effect",
            "energy_regen",
        ],
    }
}

pub fn base_stats(game: Game) -> &'static [StatValue] {
    match game {
        Game::Genshin => &[
            StatValue {
                stat: Stat::CritRate,
                value: 5.0,
            },
            StatValue {
                stat: Stat::CritDmg,
                value: 50.0,
            },
            StatValue {
                stat: Stat::AtkPct,
                value: 0.0,
            },
            StatValue {
                stat: Stat::HpPct,
                value: 0.0,
            },
            StatValue {
                stat: Stat::DefPct,
                value: 0.0,
            },
            StatValue {
                stat: Stat::EnergyRecharge,
                value: 100.0,
            },
            StatValue {
                stat: Stat::ElementalMastery,
                value: 0.0,
            },
        ],
        Game::Hsr => &[
            StatValue {
                stat: Stat::CritRate,
                value: 5.0,
            },
            StatValue {
                stat: Stat::CritDmg,
                value: 50.0,
            },
            StatValue {
                stat: Stat::AtkPct,
                value: 0.0,
            },
            StatValue {
                stat: Stat::HpPct,
                value: 0.0,
            },
            StatValue {
                stat: Stat::DefPct,
                value: 0.0,
            },
            StatValue {
                stat: Stat::Speed,
                value: 100.0,
            },
            StatValue {
                stat: Stat::EffectHitRate,
                value: 0.0,
            },
            StatValue {
                stat: Stat::EffectRes,
                value: 0.0,
            },
            StatValue {
                stat: Stat::BreakEffect,
                value: 0.0,
            },
            StatValue {
                stat: Stat::EnergyRegen,
                value: 100.0,
            },
        ],
    }
}

pub fn panel_targets(game: Game) -> &'static [StatValue] {
    match game {
        Game::Genshin => &[
            StatValue {
                stat: Stat::CritRate,
                value: 70.0,
            },
            StatValue {
                stat: Stat::CritDmg,
                value: 160.0,
            },
            StatValue {
                stat: Stat::AtkPct,
                value: 46.0,
            },
            StatValue {
                stat: Stat::HpPct,
                value: 46.0,
            },
            StatValue {
                stat: Stat::DefPct,
                value: 58.0,
            },
            StatValue {
                stat: Stat::EnergyRecharge,
                value: 180.0,
            },
            StatValue {
                stat: Stat::ElementalMastery,
                value: 600.0,
            },
        ],
        Game::Hsr => &[
            StatValue {
                stat: Stat::CritRate,
                value: 70.0,
            },
            StatValue {
                stat: Stat::CritDmg,
                value: 150.0,
            },
            StatValue {
                stat: Stat::AtkPct,
                value: 45.0,
            },
            StatValue {
                stat: Stat::HpPct,
                value: 45.0,
            },
            StatValue {
                stat: Stat::DefPct,
                value: 45.0,
            },
            StatValue {
                stat: Stat::Speed,
                value: 135.0,
            },
            StatValue {
                stat: Stat::EffectHitRate,
                value: 80.0,
            },
            StatValue {
                stat: Stat::EffectRes,
                value: 30.0,
            },
            StatValue {
                stat: Stat::BreakEffect,
                value: 180.0,
            },
            StatValue {
                stat: Stat::EnergyRegen,
                value: 119.0,
            },
        ],
    }
}

pub fn hit_targets(game: Game) -> &'static [StatValue] {
    match game {
        Game::Genshin => &[
            StatValue {
                stat: Stat::CritRate,
                value: 6.0,
            },
            StatValue {
                stat: Stat::CritDmg,
                value: 8.0,
            },
            StatValue {
                stat: Stat::AtkPct,
                value: 6.0,
            },
            StatValue {
                stat: Stat::HpPct,
                value: 6.0,
            },
            StatValue {
                stat: Stat::DefPct,
                value: 6.0,
            },
            StatValue {
                stat: Stat::EnergyRecharge,
                value: 6.0,
            },
            StatValue {
                stat: Stat::ElementalMastery,
                value: 6.0,
            },
        ],
        Game::Hsr => &[
            StatValue {
                stat: Stat::CritRate,
                value: 6.0,
            },
            StatValue {
                stat: Stat::CritDmg,
                value: 8.0,
            },
            StatValue {
                stat: Stat::AtkPct,
                value: 6.0,
            },
            StatValue {
                stat: Stat::HpPct,
                value: 6.0,
            },
            StatValue {
                stat: Stat::DefPct,
                value: 6.0,
            },
            StatValue {
                stat: Stat::Speed,
                value: 5.0,
            },
            StatValue {
                stat: Stat::EffectHitRate,
                value: 6.0,
            },
            StatValue {
                stat: Stat::EffectRes,
                value: 6.0,
            },
            StatValue {
                stat: Stat::BreakEffect,
                value: 6.0,
            },
            StatValue {
                stat: Stat::EnergyRegen,
                value: 1.0,
            },
        ],
    }
}

pub fn stat_values_map(values: &[StatValue]) -> HashMap<String, f64> {
    values
        .iter()
        .map(|entry| (entry.stat.as_str().to_string(), entry.value))
        .collect()
}
