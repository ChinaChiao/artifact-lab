from __future__ import annotations

import random
import sys
from pathlib import Path
from typing import Any, Literal

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from artifact_model import (  # noqa: E402
    DEFAULT_SETS as GENSHIN_DEFAULT_SETS,
    INITIAL_THREE_SUBSTAT_PROB,
    Artifact,
    ArtifactGenerator,
    Substat,
)
from hsr_relic_model import (  # noqa: E402
    DEFAULT_CAVERN_SETS,
    DEFAULT_PLANAR_SETS,
    Relic,
    RelicGenerator,
    RelicSubstat,
)


app = FastAPI(title="Artifact Lab API", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://localhost:5173",
        "http://127.0.0.1:5173",
        "http://localhost:4173",
        "http://127.0.0.1:4173",
    ],
    allow_origin_regex=r"^http://(localhost|127\.0\.0\.1):\d+$",
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


LABELS = {
    "slots": {
        "flower": "生之花",
        "plume": "死之羽",
        "sands": "时之沙",
        "goblet": "空之杯",
        "circlet": "理之冠",
        "head": "头部",
        "hands": "手部",
        "body": "躯干",
        "feet": "脚部",
        "planar_sphere": "位面球",
        "link_rope": "连接绳",
    },
    "stats": {
        "hp_flat": "生命值",
        "atk_flat": "攻击力",
        "def_flat": "防御力",
        "hp_pct": "生命值",
        "atk_pct": "攻击力",
        "def_pct": "防御力",
        "energy_recharge": "元素充能效率",
        "elemental_mastery": "元素精通",
        "pyro_dmg": "火伤",
        "electro_dmg": "雷伤",
        "cryo_dmg": "冰伤",
        "hydro_dmg": "水伤",
        "dendro_dmg": "草伤",
        "anemo_dmg": "风伤",
        "geo_dmg": "岩伤",
        "physical_dmg": "物伤",
        "crit_rate": "暴击率",
        "crit_dmg": "暴击伤害",
        "healing_bonus": "治疗加成",
        "speed": "速度",
        "outgoing_healing": "治疗量加成",
        "effect_hit_rate": "效果命中",
        "effect_res": "效果抵抗",
        "break_effect": "击破特攻",
        "energy_regen": "能量恢复效率",
        "fire_dmg": "火伤",
        "ice_dmg": "冰伤",
        "lightning_dmg": "雷伤",
        "wind_dmg": "风伤",
        "quantum_dmg": "量子伤",
        "imaginary_dmg": "虚数伤",
    },
}

class GenerateArtifactRequest(BaseModel):
    source: Literal["domain", "boss", "elite", "strongbox"] = "domain"
    sets: list[str] = Field(default_factory=lambda: list(GENSHIN_DEFAULT_SETS))
    seed: str | int | None = None
    level: Literal[0, 4, 8, 12, 16, 20] = 0


class GenerateRelicRequest(BaseModel):
    source: Literal["cavern", "planar"] = "cavern"
    sets: list[str] | None = None
    seed: str | int | None = None
    level: Literal[0, 3, 6, 9, 12, 15] = 0


class EnhanceRequest(BaseModel):
    item: dict[str, Any]
    seed: str | int | None = None
    to_max: bool = False


def rng_from(seed: str | int | None) -> random.Random:
    return random.Random(seed) if seed not in (None, "") else random.Random()


def clean_sets(sets: list[str] | None, fallback: tuple[str, ...]) -> list[str]:
    values = [item.strip() for item in (sets or []) if item.strip()]
    return values or list(fallback)


def camelize_item(item: dict[str, Any]) -> dict[str, Any]:
    return {
        **item,
        "mainStat": item["main_stat"],
        "mainStatValue": item["main_stat_value"],
        "initialSubstatCount": item["initial_substat_count"],
        "critValue": item["crit_value"],
        "enhancementLog": item["enhancement_log"],
    }


def artifact_from_dict(data: dict[str, Any]) -> Artifact:
    substats = [
        Substat(name=sub["name"], value=float(sub["value"]), rolls=[float(value) for value in sub.get("rolls", [])])
        for sub in data.get("substats", [])
    ]
    return Artifact(
        set_name=data.get("set") or data.get("set_name", "target"),
        source=data.get("source", "domain"),
        slot=data["slot"],
        rarity=int(data.get("rarity", 5)),
        level=int(data.get("level", 0)),
        main_stat=data.get("main_stat") or data.get("mainStat"),
        initial_substat_count=int(data.get("initial_substat_count") or data.get("initialSubstatCount") or len(substats)),
        substats=substats,
        enhancement_log=list(data.get("enhancement_log") or data.get("enhancementLog") or []),
    )


def relic_from_dict(data: dict[str, Any]) -> Relic:
    substats = [
        RelicSubstat(name=sub["name"], value=float(sub["value"]), rolls=[float(value) for value in sub.get("rolls", [])])
        for sub in data.get("substats", [])
    ]
    return Relic(
        source=data.get("source", "cavern"),
        set_name=data.get("set") or data.get("set_name", "target"),
        slot=data["slot"],
        rarity=int(data.get("rarity", 5)),
        level=int(data.get("level", 0)),
        main_stat=data.get("main_stat") or data.get("mainStat"),
        initial_substat_count=int(data.get("initial_substat_count") or data.get("initialSubstatCount") or len(substats)),
        substats=substats,
        enhancement_log=list(data.get("enhancement_log") or data.get("enhancementLog") or []),
    )


@app.get("/api/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.get("/api/meta")
def meta() -> dict[str, Any]:
    return {
        "labels": LABELS,
        "genshinSources": list(INITIAL_THREE_SUBSTAT_PROB),
        "genshinDefaultSets": list(GENSHIN_DEFAULT_SETS),
        "hsrDefaultSets": {
            "cavern": list(DEFAULT_CAVERN_SETS),
            "planar": list(DEFAULT_PLANAR_SETS),
        },
    }


@app.post("/api/genshin/artifacts/generate")
def generate_artifact(payload: GenerateArtifactRequest) -> dict[str, Any]:
    generator = ArtifactGenerator(
        sets=clean_sets(payload.sets, GENSHIN_DEFAULT_SETS),
        source=payload.source,
        rng=rng_from(payload.seed),
    )
    return camelize_item(generator.generate(level=payload.level).to_dict())


@app.post("/api/genshin/artifacts/enhance")
def enhance_artifact(payload: EnhanceRequest) -> dict[str, Any]:
    artifact = artifact_from_dict(payload.item)
    generator = ArtifactGenerator(sets=[artifact.set_name], source=artifact.source, rng=rng_from(payload.seed))
    try:
        if payload.to_max:
            while artifact.level < 20:
                generator.enhance_once(artifact)
        else:
            generator.enhance_once(artifact)
    except ValueError as exc:
        raise HTTPException(status_code=400, detail=str(exc)) from exc
    return camelize_item(artifact.to_dict())


@app.post("/api/hsr/relics/generate")
def generate_relic(payload: GenerateRelicRequest) -> dict[str, Any]:
    fallback = DEFAULT_CAVERN_SETS if payload.source == "cavern" else DEFAULT_PLANAR_SETS
    generator = RelicGenerator(source=payload.source, sets=clean_sets(payload.sets, fallback), rng=rng_from(payload.seed))
    item = camelize_item(generator.generate(level=payload.level).to_dict())
    item["speedValue"] = item["speed_value"]
    return item


@app.post("/api/hsr/relics/enhance")
def enhance_relic(payload: EnhanceRequest) -> dict[str, Any]:
    relic = relic_from_dict(payload.item)
    generator = RelicGenerator(source=relic.source, sets=[relic.set_name], rng=rng_from(payload.seed))
    try:
        if payload.to_max:
            while relic.level < 15:
                generator.enhance_once(relic)
        else:
            generator.enhance_once(relic)
    except ValueError as exc:
        raise HTTPException(status_code=400, detail=str(exc)) from exc
    item = camelize_item(relic.to_dict())
    item["speedValue"] = item["speed_value"]
    return item
