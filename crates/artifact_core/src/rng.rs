use crate::error::{ArtifactError, Result};
use rand::Rng;
use std::collections::HashMap;

/// Weighted choice from a distribution
pub fn weighted_choice<R: Rng>(weights: &HashMap<String, f64>, rng: &mut R) -> Result<String> {
    let total: f64 = weights.values().sum();
    if total <= 0.0 {
        return Err(ArtifactError::EmptyWeightPool);
    }

    let point = rng.gen_range(0.0..total);
    let mut cumulative = 0.0;
    let mut entries: Vec<_> = weights.iter().collect();
    entries.sort_by(|(left, _), (right, _)| left.cmp(right));

    for (item, weight) in &entries {
        cumulative += **weight;
        if point <= cumulative {
            return Ok((*item).clone());
        }
    }

    // Fallback to last item (handles floating point precision)
    Ok(entries.last().unwrap().0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_weighted_choice() {
        let mut weights = HashMap::new();
        weights.insert("a".to_string(), 1.0);
        weights.insert("b".to_string(), 2.0);
        weights.insert("c".to_string(), 3.0);

        let mut rng = StdRng::seed_from_u64(42);
        let choice = weighted_choice(&weights, &mut rng).unwrap();
        assert!(["a", "b", "c"].contains(&choice.as_str()));
    }

    #[test]
    fn test_weighted_choice_empty() {
        let weights = HashMap::new();
        let mut rng = StdRng::seed_from_u64(42);
        assert!(weighted_choice(&weights, &mut rng).is_err());
    }

    #[test]
    fn test_weighted_choice_zero_weights() {
        let mut weights = HashMap::new();
        weights.insert("a".to_string(), 0.0);
        let mut rng = StdRng::seed_from_u64(42);
        assert!(weighted_choice(&weights, &mut rng).is_err());
    }
}
