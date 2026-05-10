from __future__ import annotations

import argparse
import json
import random
from dataclasses import dataclass, field
from typing import Any, Iterable


CAVERN_SLOTS = ("head", "hands", "body", "feet")
PLANAR_SLOTS = ("planar_sphere", "link_rope")

DISPLAY_FLOOR_MAIN_STATS = {"hp_flat", "atk_flat", "speed"}

DEFAULT_CAVERN_SETS = (
    "passerby_of_wandering_cloud",
    "musketeer_of_wild_wheat",
    "knight_of_purity_palace",
    "hunter_of_glacial_forest",
    "champion_of_streetwise_boxing",
    "guard_of_wuthering_snow",
    "firesmith_of_lava_forging",
    "genius_of_brilliant_stars",
    "band_of_sizzling_thunder",
    "eagle_of_twilight_line",
    "thief_of_shooting_meteor",
    "wastelander_of_banditry_desert",
    "longevous_disciple",
    "messenger_traversing_hackerspace",
    "the_ashblazing_grand_duke",
    "prisoner_in_deep_confinement",
    "pioneer_diver_of_dead_waters",
    "watchmaker_master_of_dream_machinations",
    "iron_cavalry_against_the_scourge",
    "the_wind_soaring_valorous",
    "sacerdos_relived_ordeal",
    "scholar_lost_in_erudition",
    "hero_of_triumphant_song",
    "poet_of_mourning_collapse",
    "warrior_goddess_of_sun_and_thunder",
    "wavestrider_captain",
    "world_remaking_deliverer",
    "self_enshrouded_recluse",
    "ever_glorious_magical_girl",
    "diviner_of_distant_reach",
)

DEFAULT_PLANAR_SETS = (
    "space_sealing_station",
    "fleet_of_the_ageless",
    "pan_cosmic_commercial_enterprise",
    "belobog_of_the_architects",
    "celestial_differentiator",
    "inert_salsotto",
    "talia_kingdom_of_banditry",
    "sprightly_vonwacq",
    "rutilant_arena",
    "broken_keel",
    "firmament_frontline_glamoth",
    "penacony_land_of_the_dreams",
    "sigonia_the_unclaimed_desolation",
    "izumo_gensei_and_takama_divine_realm",
    "duran_dynasty_of_running_wolves",
    "forge_of_the_kalpagni_lantern",
    "lushaka_the_sunken_seas",
    "the_wondrous_bananamusement_park",
    "amphoreus_the_eternal_land",
    "bone_collections_serene_demesne",
    "revelry_by_the_sea",
    "giant_tree_of_rapt_brooding",
    "city_of_converging_stars",
    "punklorde_stage_zero",
    "tengoku_livestream",
    "arcadia_of_woven_dreams",
)

MAIN_STAT_WEIGHTS: dict[str, dict[str, float]] = {
    "head": {"hp_flat": 1.0},
    "hands": {"atk_flat": 1.0},
    "body": {
        "hp_pct": 20.0,
        "atk_pct": 20.0,
        "def_pct": 20.0,
        "crit_rate": 10.0,
        "crit_dmg": 10.0,
        "outgoing_healing": 10.0,
        "effect_hit_rate": 10.0,
    },
    "feet": {
        "hp_pct": 29.17,
        "atk_pct": 29.17,
        "def_pct": 29.16,
        "speed": 12.5,
    },
    "planar_sphere": {
        "hp_pct": 12.34,
        "atk_pct": 12.33,
        "def_pct": 12.33,
        "physical_dmg": 9.0,
        "fire_dmg": 9.0,
        "ice_dmg": 9.0,
        "lightning_dmg": 9.0,
        "wind_dmg": 9.0,
        "quantum_dmg": 9.0,
        "imaginary_dmg": 9.0,
    },
    "link_rope": {
        "hp_pct": 26.33,
        "atk_pct": 26.34,
        "def_pct": 26.33,
        "break_effect": 15.0,
        "energy_regen": 6.0,
    },
}

SUBSTAT_WEIGHTS: dict[str, int] = {
    "hp_flat": 10,
    "atk_flat": 10,
    "def_flat": 10,
    "hp_pct": 10,
    "atk_pct": 10,
    "def_pct": 10,
    "speed": 4,
    "crit_rate": 6,
    "crit_dmg": 6,
    "effect_hit_rate": 8,
    "effect_res": 8,
    "break_effect": 8,
}

SUBSTAT_ROLLS_5_STAR: dict[str, tuple[float, float, float]] = {
    "hp_flat": (33.87, 38.10, 42.34),
    "atk_flat": (16.94, 19.05, 21.17),
    "def_flat": (16.94, 19.05, 21.17),
    "hp_pct": (3.46, 3.89, 4.32),
    "atk_pct": (3.46, 3.89, 4.32),
    "def_pct": (4.32, 4.86, 5.40),
    "speed": (2.00, 2.30, 2.60),
    "crit_rate": (2.59, 2.92, 3.24),
    "crit_dmg": (5.18, 5.83, 6.48),
    "effect_hit_rate": (3.46, 3.89, 4.32),
    "effect_res": (3.46, 3.89, 4.32),
    "break_effect": (5.18, 5.83, 6.48),
}

MAIN_STAT_BASE_VALUES_5_STAR: dict[str, float] = {
    "hp_flat": 112.896,
    "atk_flat": 56.448,
    "hp_pct": 6.912,
    "atk_pct": 6.912,
    "def_pct": 8.64,
    "speed": 4.032,
    "crit_rate": 5.184,
    "crit_dmg": 10.368,
    "outgoing_healing": 5.5296,
    "effect_hit_rate": 6.912,
    "physical_dmg": 6.2208,
    "fire_dmg": 6.2208,
    "ice_dmg": 6.2208,
    "lightning_dmg": 6.2208,
    "wind_dmg": 6.2208,
    "quantum_dmg": 6.2208,
    "imaginary_dmg": 6.2208,
    "break_effect": 10.368,
    "energy_regen": 3.1104,
}

MAIN_STAT_MAX_VALUES_5_STAR: dict[str, float] = {
    "hp_flat": 705.6,
    "atk_flat": 352.8,
    "hp_pct": 43.2,
    "atk_pct": 43.2,
    "def_pct": 54.0,
    "speed": 25.032,
    "crit_rate": 32.4,
    "crit_dmg": 64.8,
    "outgoing_healing": 34.5606,
    "effect_hit_rate": 43.2,
    "physical_dmg": 38.8803,
    "fire_dmg": 38.8803,
    "ice_dmg": 38.8803,
    "lightning_dmg": 38.8803,
    "wind_dmg": 38.8803,
    "quantum_dmg": 38.8803,
    "imaginary_dmg": 38.8803,
    "break_effect": 64.8,
    "energy_regen": 19.4394,
}


@dataclass
class RelicSubstat:
    name: str
    value: float
    rolls: list[float] = field(default_factory=list)

    @classmethod
    def create(cls, name: str, rng: random.Random) -> "RelicSubstat":
        roll = rng.choice(SUBSTAT_ROLLS_5_STAR[name])
        return cls(name=name, value=round(roll, 2), rolls=[roll])

    def upgrade(self, rng: random.Random) -> float:
        roll = rng.choice(SUBSTAT_ROLLS_5_STAR[self.name])
        self.rolls.append(roll)
        self.value = round(sum(self.rolls), 2)
        return round(roll, 2)

    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "value": self.value,
            "roll_count": len(self.rolls),
            "rolls": [round(value, 2) for value in self.rolls],
        }


@dataclass
class Relic:
    source: str
    set_name: str
    slot: str
    rarity: int
    level: int
    main_stat: str
    initial_substat_count: int
    substats: list[RelicSubstat]
    enhancement_log: list[dict[str, Any]] = field(default_factory=list)

    def crit_value(self) -> float:
        by_name = {substat.name: substat.value for substat in self.substats}
        return round(by_name.get("crit_rate", 0.0) * 2 + by_name.get("crit_dmg", 0.0), 2)

    def speed_value(self) -> float:
        by_name = {substat.name: substat.value for substat in self.substats}
        return round(by_name.get("speed", 0.0), 2)

    def to_dict(self) -> dict[str, Any]:
        return {
            "source": self.source,
            "set": self.set_name,
            "slot": self.slot,
            "rarity": self.rarity,
            "level": self.level,
            "main_stat": self.main_stat,
            "main_stat_value": main_stat_value(self.main_stat, self.level),
            "initial_substat_count": self.initial_substat_count,
            "substats": [substat.to_dict() for substat in self.substats],
            "crit_value": self.crit_value(),
            "speed_value": self.speed_value(),
            "enhancement_log": self.enhancement_log,
        }


class RelicGenerator:
    """A compact stochastic model for 5-star Honkai: Star Rail relics."""

    def __init__(
        self,
        source: str = "cavern",
        sets: Iterable[str] | None = None,
        rng: random.Random | None = None,
    ) -> None:
        if source not in {"cavern", "planar"}:
            raise ValueError("source must be one of: cavern, planar")
        self.source = source
        self.sets = tuple(sets or (DEFAULT_CAVERN_SETS if source == "cavern" else DEFAULT_PLANAR_SETS))
        if not self.sets:
            raise ValueError("At least one relic set is required.")
        self.rng = rng or random.Random()

    def generate(self, level: int = 15) -> Relic:
        if level < 0 or level > 15 or level % 3 != 0:
            raise ValueError("level must be one of 0, 3, 6, 9, 12, 15")

        slot = self.rng.choice(CAVERN_SLOTS if self.source == "cavern" else PLANAR_SLOTS)
        main_stat = weighted_choice(MAIN_STAT_WEIGHTS[slot], self.rng)
        initial_count = 4 if self.rng.random() < 0.20 else 3
        substats = [
            RelicSubstat.create(name, self.rng)
            for name in self._draw_substats(main_stat, initial_count)
        ]
        relic = Relic(
            source=self.source,
            set_name=self.rng.choice(self.sets),
            slot=slot,
            rarity=5,
            level=0,
            main_stat=main_stat,
            initial_substat_count=initial_count,
            substats=substats,
        )
        while relic.level < level:
            self.enhance_once(relic)
        return relic

    def enhance_once(self, relic: Relic) -> dict[str, Any]:
        if relic.level >= 15:
            raise ValueError("relic is already level 15")

        next_level = relic.level + 3
        if len(relic.substats) < 4:
            existing = {substat.name for substat in relic.substats}
            pool = {
                stat: weight
                for stat, weight in SUBSTAT_WEIGHTS.items()
                if stat != relic.main_stat and stat not in existing
            }
            substat = RelicSubstat.create(weighted_choice(pool, self.rng), self.rng)
            relic.substats.append(substat)
            event = {
                "level": next_level,
                "kind": "new_substat",
                "stat": substat.name,
                "roll": substat.rolls[-1],
                "value": substat.value,
            }
        else:
            substat = self.rng.choice(relic.substats)
            roll = substat.upgrade(self.rng)
            event = {
                "level": next_level,
                "kind": "upgrade_substat",
                "stat": substat.name,
                "roll": roll,
                "value": substat.value,
                "roll_count": len(substat.rolls),
            }

        relic.level = next_level
        relic.enhancement_log.append(event)
        return event

    def _draw_substats(self, main_stat: str, count: int) -> list[str]:
        chosen: list[str] = []
        for _ in range(count):
            pool = {
                stat: weight
                for stat, weight in SUBSTAT_WEIGHTS.items()
                if stat != main_stat and stat not in chosen
            }
            chosen.append(weighted_choice(pool, self.rng))
        return chosen


def weighted_choice(weights: dict[str, float], rng: random.Random) -> str:
    total = sum(weights.values())
    if total <= 0:
        raise ValueError("weights must have a positive sum")

    point = rng.uniform(0, total)
    cumulative = 0.0
    for item, weight in weights.items():
        cumulative += weight
        if point <= cumulative:
            return item
    return next(reversed(weights))


def main_stat_value(stat: str, level: int) -> float:
    base = MAIN_STAT_BASE_VALUES_5_STAR[stat]
    maximum = MAIN_STAT_MAX_VALUES_5_STAR[stat]
    value = maximum if level == 15 else base + (maximum - base) * (level / 15)
    if stat in DISPLAY_FLOOR_MAIN_STATS:
        return int(value)
    return round(value, 2)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Generate Honkai: Star Rail 5-star relics.")
    parser.add_argument("-n", "--count", type=int, default=1, help="number of relics to generate")
    parser.add_argument("--level", type=int, default=15, choices=(0, 3, 6, 9, 12, 15))
    parser.add_argument("--seed", type=int, default=None, help="random seed for reproducible output")
    parser.add_argument("--source", choices=("cavern", "planar"), default="cavern")
    parser.add_argument("--sets", nargs="+", default=None, help="relic set names to sample uniformly")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    rng = random.Random(args.seed)
    generator = RelicGenerator(source=args.source, sets=args.sets, rng=rng)
    relics = [generator.generate(level=args.level).to_dict() for _ in range(args.count)]
    print(json.dumps(relics if args.count != 1 else relics[0], ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
