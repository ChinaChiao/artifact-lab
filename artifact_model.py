from __future__ import annotations

import argparse
import json
import random
from dataclasses import dataclass, field
from typing import Any, Iterable


SLOTS = ("flower", "plume", "sands", "goblet", "circlet")

DEFAULT_SETS = (
    "gladiators_finale",
    "wanderers_troupe",
    "noblesse_oblige",
    "bloodstained_chivalry",
    "maiden_beloved",
    "viridescent_venerer",
    "archaic_petra",
    "retracing_bolide",
    "thundersoother",
    "thundering_fury",
    "lavawalker",
    "crimson_witch_of_flames",
    "blizzard_strayer",
    "heart_of_depth",
    "tenacity_of_the_millelith",
    "pale_flame",
    "shimenawas_reminiscence",
    "emblem_of_severed_fate",
    "husk_of_opulent_dreams",
    "ocean_hued_clam",
    "vermillion_hereafter",
    "echoes_of_an_offering",
    "deepwood_memories",
    "gilded_dreams",
    "desert_pavilion_chronicle",
    "flower_of_paradise_lost",
    "nymphs_dream",
    "vourukashas_glow",
    "marechaussee_hunter",
    "golden_troupe",
    "song_of_days_past",
    "nighttime_whispers_in_the_echoing_woods",
    "fragment_of_harmonic_whimsy",
    "unfinished_reverie",
    "scroll_of_the_hero_of_cinder_city",
    "obsidian_codex",
    "long_nights_oath",
    "finale_of_the_deep_galleries",
    "night_of_the_skys_unveiling",
    "silken_moons_serenade",
    "a_day_carved_from_rising_winds",
    "aubade_of_morningstar_and_moon",
    "celestial_gift",
    "disenchantment_in_deep_shadow",
)

INITIAL_THREE_SUBSTAT_PROB: dict[str, float] = {
    "domain": 0.80,
    "boss": 0.66,
    "elite": 0.90,
    "strongbox": 0.66,
}

MAIN_STAT_WEIGHTS: dict[str, dict[str, float]] = {
    "flower": {"hp_flat": 1.0},
    "plume": {"atk_flat": 1.0},
    "sands": {
        "hp_pct": 26.68,
        "atk_pct": 26.66,
        "def_pct": 26.66,
        "energy_recharge": 10.0,
        "elemental_mastery": 10.0,
    },
    "goblet": {
        "hp_pct": 19.25,
        "atk_pct": 19.25,
        "def_pct": 19.0,
        "pyro_dmg": 5.0,
        "electro_dmg": 5.0,
        "cryo_dmg": 5.0,
        "hydro_dmg": 5.0,
        "dendro_dmg": 5.0,
        "anemo_dmg": 5.0,
        "geo_dmg": 5.0,
        "physical_dmg": 5.0,
        "elemental_mastery": 2.5,
    },
    "circlet": {
        "hp_pct": 22.0,
        "atk_pct": 22.0,
        "def_pct": 22.0,
        "crit_rate": 10.0,
        "crit_dmg": 10.0,
        "healing_bonus": 10.0,
        "elemental_mastery": 4.0,
    },
}

SUBSTAT_WEIGHTS: dict[str, int] = {
    "hp_flat": 6,
    "atk_flat": 6,
    "def_flat": 6,
    "hp_pct": 4,
    "atk_pct": 4,
    "def_pct": 4,
    "energy_recharge": 4,
    "elemental_mastery": 4,
    "crit_rate": 3,
    "crit_dmg": 3,
}

SUBSTAT_ROLLS_5_STAR: dict[str, tuple[float, float, float, float]] = {
    "hp_flat": (209.13, 239.00, 268.88, 298.75),
    "atk_flat": (13.62, 15.56, 17.51, 19.45),
    "def_flat": (16.20, 18.52, 20.83, 23.15),
    "hp_pct": (4.08, 4.66, 5.25, 5.83),
    "atk_pct": (4.08, 4.66, 5.25, 5.83),
    "def_pct": (5.10, 5.83, 6.56, 7.29),
    "elemental_mastery": (16.32, 18.65, 20.98, 23.31),
    "energy_recharge": (4.53, 5.18, 5.83, 6.48),
    "crit_rate": (2.72, 3.11, 3.50, 3.89),
    "crit_dmg": (5.44, 6.22, 6.99, 7.77),
}

MAIN_STAT_BASE_VALUES_5_STAR: dict[str, float] = {
    "hp_flat": 717.0,
    "atk_flat": 47.0,
    "hp_pct": 7.0,
    "atk_pct": 7.0,
    "def_pct": 8.7,
    "energy_recharge": 7.8,
    "elemental_mastery": 28.0,
    "pyro_dmg": 7.0,
    "electro_dmg": 7.0,
    "cryo_dmg": 7.0,
    "hydro_dmg": 7.0,
    "dendro_dmg": 7.0,
    "anemo_dmg": 7.0,
    "geo_dmg": 7.0,
    "physical_dmg": 8.7,
    "crit_rate": 4.7,
    "crit_dmg": 9.3,
    "healing_bonus": 5.4,
}

MAIN_STAT_MAX_VALUES_5_STAR: dict[str, float] = {
    "hp_flat": 4780.0,
    "atk_flat": 311.0,
    "hp_pct": 46.6,
    "atk_pct": 46.6,
    "def_pct": 58.3,
    "energy_recharge": 51.8,
    "elemental_mastery": 186.5,
    "pyro_dmg": 46.6,
    "electro_dmg": 46.6,
    "cryo_dmg": 46.6,
    "hydro_dmg": 46.6,
    "dendro_dmg": 46.6,
    "anemo_dmg": 46.6,
    "geo_dmg": 46.6,
    "physical_dmg": 58.3,
    "crit_rate": 31.1,
    "crit_dmg": 62.2,
    "healing_bonus": 35.9,
}


@dataclass
class Substat:
    name: str
    value: float
    rolls: list[float] = field(default_factory=list)

    @classmethod
    def create(cls, name: str, rng: random.Random) -> "Substat":
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
class Artifact:
    set_name: str
    source: str
    slot: str
    rarity: int
    level: int
    main_stat: str
    initial_substat_count: int
    substats: list[Substat]
    enhancement_log: list[dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "set": self.set_name,
            "source": self.source,
            "slot": self.slot,
            "rarity": self.rarity,
            "level": self.level,
            "main_stat": self.main_stat,
            "main_stat_value": main_stat_value(self.main_stat, self.level),
            "initial_substat_count": self.initial_substat_count,
            "substats": [substat.to_dict() for substat in self.substats],
            "crit_value": self.crit_value(),
            "enhancement_log": self.enhancement_log,
        }

    def crit_value(self) -> float:
        by_name = {substat.name: substat.value for substat in self.substats}
        return round(by_name.get("crit_rate", 0.0) * 2 + by_name.get("crit_dmg", 0.0), 2)


class ArtifactGenerator:
    """A small stochastic model for 5-star Genshin artifact generation."""

    def __init__(
        self,
        sets: Iterable[str] = DEFAULT_SETS,
        source: str = "domain",
        rng: random.Random | None = None,
    ) -> None:
        self.sets = tuple(sets)
        if not self.sets:
            raise ValueError("At least one artifact set is required.")
        if source not in INITIAL_THREE_SUBSTAT_PROB:
            valid_sources = ", ".join(INITIAL_THREE_SUBSTAT_PROB)
            raise ValueError(f"source must be one of: {valid_sources}")
        self.source = source
        self.rng = rng or random.Random()

    def generate(self, level: int = 20) -> Artifact:
        if level < 0 or level > 20 or level % 4 != 0:
            raise ValueError("level must be one of 0, 4, 8, 12, 16, 20")

        set_name = self.rng.choice(self.sets)
        slot = self.rng.choice(SLOTS)
        main_stat = weighted_choice(MAIN_STAT_WEIGHTS[slot], self.rng)
        initial_count = self._initial_substat_count()
        substats = [Substat.create(name, self.rng) for name in self._draw_substats(main_stat, initial_count)]

        artifact = Artifact(
            set_name=set_name,
            source=self.source,
            slot=slot,
            rarity=5,
            level=0,
            main_stat=main_stat,
            initial_substat_count=initial_count,
            substats=substats,
        )
        self._enhance_to_level(artifact, level)
        return artifact

    def _initial_substat_count(self) -> int:
        # Probability of lower/higher initial substat count for 5-star artifacts.
        return 3 if self.rng.random() < INITIAL_THREE_SUBSTAT_PROB[self.source] else 4

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

    def _enhance_to_level(self, artifact: Artifact, target_level: int) -> None:
        while artifact.level < target_level:
            self.enhance_once(artifact)

    def enhance_once(self, artifact: Artifact) -> dict[str, Any]:
        """Apply the next +4 enhancement step to one artifact."""
        if artifact.level >= 20:
            raise ValueError("artifact is already level 20")

        next_level = artifact.level + 4
        if len(artifact.substats) < 4:
            existing = {substat.name for substat in artifact.substats}
            pool = {
                stat: weight
                for stat, weight in SUBSTAT_WEIGHTS.items()
                if stat != artifact.main_stat and stat not in existing
            }
            substat = Substat.create(weighted_choice(pool, self.rng), self.rng)
            artifact.substats.append(substat)
            event = {
                "level": next_level,
                "kind": "new_substat",
                "stat": substat.name,
                "roll": substat.rolls[-1],
                "value": substat.value,
            }
        else:
            substat = self.rng.choice(artifact.substats)
            roll = substat.upgrade(self.rng)
            event = {
                "level": next_level,
                "kind": "upgrade_substat",
                "stat": substat.name,
                "roll": roll,
                "value": substat.value,
                "roll_count": len(substat.rolls),
            }

        artifact.level = next_level
        artifact.enhancement_log.append(event)
        return event


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
    if level == 20:
        return maximum
    return round(base + (maximum - base) * (level / 20), 2)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Generate Genshin-style 5-star artifacts.")
    parser.add_argument("-n", "--count", type=int, default=1, help="number of artifacts to generate")
    parser.add_argument("--level", type=int, default=20, choices=(0, 4, 8, 12, 16, 20))
    parser.add_argument("--seed", type=int, default=None, help="random seed for reproducible output")
    parser.add_argument("--source", choices=tuple(INITIAL_THREE_SUBSTAT_PROB), default="domain")
    parser.add_argument(
        "--sets",
        nargs="+",
        default=list(DEFAULT_SETS),
        help="artifact set names to sample uniformly",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    rng = random.Random(args.seed)
    generator = ArtifactGenerator(sets=args.sets, source=args.source, rng=rng)
    artifacts = [generator.generate(level=args.level).to_dict() for _ in range(args.count)]
    print(json.dumps(artifacts if args.count != 1 else artifacts[0], indent=2))


if __name__ == "__main__":
    main()
