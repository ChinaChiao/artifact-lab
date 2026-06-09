use crate::rules::{self, Game};
use serde_json::{json, Value};

pub fn get_meta() -> Value {
    json!({
        "games": {
            "genshin": game_meta(Game::Genshin),
            "hsr": game_meta(Game::Hsr),
        },
        "completionThresholds": rules::completion_thresholds()
            .iter()
            .map(|threshold| json!({
                "key": threshold.key,
                "ratio": threshold.ratio,
                "label": threshold.label,
            }))
            .collect::<Vec<_>>(),
        "resourceQuantiles": rules::resource_quantiles()
            .iter()
            .map(|quantile| json!({
                "key": quantile.key,
                "ratio": quantile.ratio,
                "label": quantile.label,
            }))
            .collect::<Vec<_>>(),
    })
}

fn game_meta(game: Game) -> Value {
    let mut value = json!({
        "slots": rules::slot_names(game),
        "sources": rules::source_names(game),
        "dpsDefaults": rules::dps_defaults(game),
        "supportDefaults": rules::support_defaults(game),
        "effective": rules::effective_stats(game),
        "base": rules::stat_values_map(rules::base_stats(game)),
        "panelTargets": rules::stat_values_map(rules::panel_targets(game)),
        "hitTargets": rules::stat_values_map(rules::hit_targets(game)),
        "mainOptions": rules::main_options_map(game),
        "defaultMainPlan": rules::default_main_plan_map(game),
        "mainMax": rules::main_max_values_map(game),
        "mainWeights": rules::main_weights_map(game),
        "substatWeights": rules::substat_weights_map(game),
        "substatRolls": rules::substat_rolls_map(game),
    });

    if game == Game::Genshin {
        value["initialThreeSubstatProb"] = json!({
            "domain": rules::initial_three_substat_prob(rules::Source::Domain).unwrap(),
            "boss": rules::initial_three_substat_prob(rules::Source::Boss).unwrap(),
            "elite": rules::initial_three_substat_prob(rules::Source::Elite).unwrap(),
            "strongbox": rules::initial_three_substat_prob(rules::Source::Strongbox).unwrap(),
        });
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_exposes_rules_for_ui_and_simulation() {
        let meta = get_meta();
        assert!(meta["games"]["genshin"]["slots"]
            .as_array()
            .is_some_and(|slots| !slots.is_empty()));
        assert!(meta["games"]["hsr"]["mainOptions"]["body"]
            .as_array()
            .is_some_and(|options| options.contains(&json!("crit_rate"))));
        assert!(meta["completionThresholds"]
            .as_array()
            .is_some_and(|thresholds| thresholds.len() == 5));
    }

    #[test]
    fn meta_main_max_and_rolls_are_generated_from_rules() {
        let meta = get_meta();
        assert_eq!(meta["games"]["hsr"]["mainMax"]["speed"], json!(25.032));
        assert_eq!(
            meta["games"]["hsr"]["mainMax"]["outgoing_healing"],
            json!(34.5606)
        );
        assert_eq!(
            meta["games"]["genshin"]["substatRolls"]["hp_flat"],
            json!([209.13, 239.0, 268.88, 298.75])
        );
        assert_eq!(
            meta["games"]["hsr"]["substatRolls"]["speed"],
            json!([2.0, 2.3, 2.6])
        );
    }

    #[test]
    fn generator_dto_and_meta_share_max_main_values() {
        let meta = get_meta();
        for game in [Game::Genshin, Game::Hsr] {
            for entry in rules::main_values(game) {
                let generated_value =
                    rules::main_stat_value(game, entry.stat, rules::max_level(game)).unwrap();
                assert_eq!(generated_value, entry.maximum);
                assert_eq!(
                    meta["games"][game.as_str()]["mainMax"][entry.stat.as_str()],
                    json!(entry.maximum)
                );
            }
        }
    }
}
