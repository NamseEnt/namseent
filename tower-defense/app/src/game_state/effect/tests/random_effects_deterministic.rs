use crate::game_state::effect::{Effect, run_effect_with_rng, tests_support::make_test_state};
use rand::{SeedableRng, rngs::StdRng};

// Deterministic tests for random range effects
#[test]
fn gain_gold_is_deterministic_with_seed() {
    let effect = Effect::GainGold {
        min_amount: 3,
        max_amount: 10,
    };

    // First run
    let mut state1 = make_test_state();
    let mut rng1 = StdRng::seed_from_u64(42);
    run_effect_with_rng(&mut state1, &effect, &mut rng1);

    // Second run with same seed should yield identical result
    let mut state2 = make_test_state();
    let mut rng2 = StdRng::seed_from_u64(42);
    run_effect_with_rng(&mut state2, &effect, &mut rng2);

    assert_eq!(
        state1.gold, state2.gold,
        "Same seed must yield identical gold"
    );
    assert!(
        state1.gold >= 3 && state1.gold <= 10,
        "Gold must be within inclusive range"
    );
}

#[test]
fn gain_shield_is_deterministic_with_seed() {
    let effect = Effect::GainShield {
        min_amount: crate::Shield::from_integer(5),
        max_amount: crate::Shield::from_integer(12),
    };

    let mut s1 = make_test_state();
    let mut r1 = StdRng::seed_from_u64(123456);
    run_effect_with_rng(&mut s1, &effect, &mut r1);

    let mut s2 = make_test_state();
    let mut r2 = StdRng::seed_from_u64(123456);
    run_effect_with_rng(&mut s2, &effect, &mut r2);

    assert!(s1.shield == s2.shield, "Shield mismatch for identical seed");
    assert!(
        s1.shield_amount() >= crate::Shield::from_integer(5)
            && s1.shield_amount() <= crate::Shield::from_integer(12),
        "Shield out of range"
    );
}

#[test]
fn heal_health_deterministic_with_seed() {
    let effect = Effect::HealHealth {
        min_amount: crate::Health::from_integer(4),
        max_amount: crate::Health::from_integer(9),
    };

    let mut s1 = make_test_state();
    s1.hp = crate::Health::from_integer(50);
    let mut r1 = StdRng::seed_from_u64(999);
    run_effect_with_rng(&mut s1, &effect, &mut r1);

    let healed_1 = s1.hp;

    let mut s2 = make_test_state();
    s2.hp = crate::Health::from_integer(50);
    let mut r2 = StdRng::seed_from_u64(999);
    run_effect_with_rng(&mut s2, &effect, &mut r2);

    assert!(healed_1 == s2.hp, "Heal amount mismatch for identical seed");
    assert!(
        s2.hp >= crate::Health::from_integer(54) && s2.hp <= crate::Health::from_integer(59),
        "Healed hp out of expected range"
    );
}
