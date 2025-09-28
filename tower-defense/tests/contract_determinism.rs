use tower_defense::{generate_contract_with_rng, Rarity};
use rand::{SeedableRng, rngs::StdRng};

// 간단 결정성 테스트: 동일 seed -> 동일 결과, 다른 seed -> 대부분 다른 결과 기대
#[test]
fn contract_generation_is_deterministic_for_same_seed() {
    let rarity = Rarity::Epic;
    let mut rng1 = StdRng::seed_from_u64(42);
    let mut rng2 = StdRng::seed_from_u64(42);

    let c1 = generate_contract_with_rng(&mut rng1, rarity);
    let c2 = generate_contract_with_rng(&mut rng2, rarity);

    // 핵심 필드들이 동일해야 함
    assert_eq!(c1.rarity as u8, c2.rarity as u8, "rarity mismatch");
    assert_eq!(format!("{:?}", c1.status), format!("{:?}", c2.status), "status mismatch");
    assert_eq!(format!("{:?}", c1.risk), format!("{:?}", c2.risk), "risk effect mismatch");
    assert_eq!(format!("{:?}", c1.reward), format!("{:?}", c2.reward), "reward effect mismatch");
}

#[test]
fn contract_generation_varies_for_different_seeds() {
    let rarity = Rarity::Rare;
    let mut rng1 = StdRng::seed_from_u64(1);
    let mut rng2 = StdRng::seed_from_u64(9999);

    let c1 = generate_contract_with_rng(&mut rng1, rarity);
    let c2 = generate_contract_with_rng(&mut rng2, rarity);

    // 완전히 같을 가능성은 있지만 낮으므로, 동일하면 실패하도록(테스트 민감도 낮추기 위해 risk+reward 둘 다 동일한 경우만 실패)
    let same_risk = format!("{:?}", c1.risk) == format!("{:?}", c2.risk);
    let same_reward = format!("{:?}", c1.reward) == format!("{:?}", c2.reward);
    assert!( !(same_risk && same_reward), "Both risk and reward identical for different seeds (rare but indicates insufficient entropy)" );
}