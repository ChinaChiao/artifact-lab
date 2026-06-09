use artifact_core::{GenshinGenerator, HsrGenerator};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    println!("Testing Genshin artifact generation...");
    let sets = vec!["gladiators_finale".to_string()];
    let generator = GenshinGenerator::new(sets, "domain".to_string()).unwrap();
    let mut rng = StdRng::seed_from_u64(42);

    let artifact = generator.generate(0, &mut rng).unwrap();
    let dto = artifact.to_dto();
    println!("Generated artifact: {:?}", dto.set);
    println!("Level: {}", dto.level);
    println!("Substats count: {}", dto.substats.len());

    // Test enhancement
    let mut artifact_clone = generator.generate(0, &mut rng).unwrap();
    generator
        .enhance_once(&mut artifact_clone, &mut rng)
        .unwrap();
    println!("After enhancement, level: {}", artifact_clone.level);

    // Test rolls roundtrip
    let dto = artifact_clone.to_dto();
    for substat in &dto.substats {
        let rolls_sum: f64 = substat.rolls.iter().sum();
        let expected = (rolls_sum * 100.0).round() / 100.0;
        assert_eq!(
            substat.value, expected,
            "Roll sum mismatch for {}",
            substat.name
        );
    }
    println!("✓ Genshin rolls roundtrip test passed");

    println!("\nTesting HSR relic generation...");
    let sets = vec!["musketeer_of_wild_wheat".to_string()];
    let generator = HsrGenerator::new(sets, "cavern".to_string()).unwrap();
    let mut rng = StdRng::seed_from_u64(42);

    let relic = generator.generate(0, &mut rng).unwrap();
    let dto = relic.to_dto();
    println!("Generated relic: {:?}", dto.set);
    println!("Level: {}", dto.level);
    println!("Substats count: {}", dto.substats.len());

    // Test enhancement
    let mut relic_clone = generator.generate(0, &mut rng).unwrap();
    generator.enhance_once(&mut relic_clone, &mut rng).unwrap();
    println!("After enhancement, level: {}", relic_clone.level);

    // Test rolls roundtrip
    let dto = relic_clone.to_dto();
    for substat in &dto.substats {
        let rolls_sum: f64 = substat.rolls.iter().sum();
        let expected = (rolls_sum * 100.0).round() / 100.0;
        assert_eq!(
            substat.value, expected,
            "Roll sum mismatch for {}",
            substat.name
        );
    }
    println!("✓ HSR rolls roundtrip test passed");

    println!("\n✓ All integration tests passed!");
}
