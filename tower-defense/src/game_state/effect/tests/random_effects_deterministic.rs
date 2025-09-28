use rand::{SeedableRng, rngs::StdRng};
use crate::game_state::effect::{run_effect_with_rng, Effect, tests_support::make_test_state};

// Deterministic tests for random range effects
#[test]
fn gain_gold_is_deterministic_with_seed() {
    let effect = Effect::GainGold { min_amount: 3.0, max_amount: 10.0 };

    // First run
    let mut state1 = make_test_state();
    let mut rng1 = StdRng::seed_from_u64(42);
    run_effect_with_rng(&mut state1, &effect, &mut rng1);

    // Second run with same seed should yield identical result
    let mut state2 = make_test_state();
    let mut rng2 = StdRng::seed_from_u64(42);
    run_effect_with_rng(&mut state2, &effect, &mut rng2);

    assert_eq!(state1.gold, state2.gold, "Same seed must yield identical gold");
    assert!(state1.gold >= 3 && state1.gold <= 10, "Gold must be within inclusive range");
}

#[test]
fn gain_shield_is_deterministic_with_seed() {
    let effect = Effect::GainShield { min_amount: 5.0, max_amount: 12.0 };

    let mut s1 = make_test_state();
    let mut r1 = StdRng::seed_from_u64(123456);
    run_effect_with_rng(&mut s1, &effect, &mut r1);

    let mut s2 = make_test_state();
    let mut r2 = StdRng::seed_from_u64(123456);
    run_effect_with_rng(&mut s2, &effect, &mut r2);

    assert!((s1.shield - s2.shield).abs() < f32::EPSILON, "Shield mismatch for identical seed");
    assert!(s1.shield >= 5.0 && s1.shield <= 12.0, "Shield out of range");
}

#[test]
fn heal_health_deterministic_with_seed() {
    let effect = Effect::HealHealth { min_amount: 4.0, max_amount: 9.0 };

    let mut s1 = make_test_state();
    s1.hp = 50.0;
    let mut r1 = StdRng::seed_from_u64(999);
    run_effect_with_rng(&mut s1, &effect, &mut r1);

    let healed_1 = s1.hp;

    let mut s2 = make_test_state();
    s2.hp = 50.0;
    let mut r2 = StdRng::seed_from_u64(999);
    run_effect_with_rng(&mut s2, &effect, &mut r2);

    assert!((healed_1 - s2.hp).abs() < f32::EPSILON, "Heal amount mismatch for identical seed");
    assert!(s2.hp >= 50.0 + 4.0 && s2.hp <= 50.0 + 9.0, "Healed hp out of expected range");
}

#[test]
fn lottery_wins_or_loses_deterministically() {
    let effect = Effect::Lottery { amount: 25.0, probability: 0.4 };

    // Same seed -> same outcome
    let (mut s1, mut r1) = (make_test_state(), StdRng::seed_from_u64(2024));
    run_effect_with_rng(&mut s1, &effect, &mut r1);
    let gold_1 = s1.gold;

    let (mut s2, mut r2) = (make_test_state(), StdRng::seed_from_u64(2024));
    run_effect_with_rng(&mut s2, &effect, &mut r2);
    assert_eq!(gold_1, s2.gold, "Same seed must yield identical lottery result");
    assert!(s2.gold == 0 || s2.gold == 25, "Lottery payout must be 0 or amount");

    // We do NOT assert observing both branches (that would make the test brittle on RNG implementation details).
    // Branch coverage for win/loss can be done by an explicit targeted test with mocked RNG if desired later.
}
