//! run_effect 경로를 통한 Effect 적용 통합 테스트
//! 개별 필드 조작이 아닌 실제 매핑(match) 로직을 검증한다.

use crate::game_state::effect::{Effect, run_effect, tests_support::make_test_state};

#[test]
fn increase_shop_reroll_via_run_effect() {
    let mut gs = make_test_state();
    assert_eq!(gs.max_shop_refresh_chance(), 1);
    run_effect(&mut gs, &Effect::IncreaseShopMaxRerolls { bonus: 1 });
    assert_eq!(gs.max_shop_refresh_chance(), 2, "run_effect 경로로 +1 반영");
}

#[test]
fn shop_reroll_penalty_then_bonus_via_run_effect() {
    let mut gs = make_test_state();
    for _ in 0..4 {
        run_effect(&mut gs, &Effect::DecreaseShopMaxRerolls { penalty: 1 });
    }
    assert_eq!(gs.max_shop_refresh_chance(), 0, "패널티 4회 후 0 포화");
    for _ in 0..6 {
        run_effect(&mut gs, &Effect::IncreaseShopMaxRerolls { bonus: 1 });
    }
    assert_eq!(gs.max_shop_refresh_chance(), 3, "-4 +6 => +2 (기본 1 → 3)");
}

#[test]
fn stacking_damage_multiplier_via_run_effect() {
    let mut gs = make_test_state();
    assert!((gs.contract_state.get_damage_multiplier() - 1.0).abs() < f32::EPSILON);
    run_effect(
        &mut gs,
        &Effect::IncreaseAllTowersDamage { multiplier: 1.5 },
    );
    run_effect(
        &mut gs,
        &Effect::IncreaseAllTowersDamage { multiplier: 2.0 },
    );
    // 1.0 * 1.5 * 2.0 = 3.0
    assert!(
        (gs.contract_state.get_damage_multiplier() - 3.0).abs() < 1e-6,
        "누적 데미지 배율 계산"
    );
}

#[test]
fn decrease_gold_gain_percent_via_run_effect() {
    let mut gs = make_test_state();
    assert!((gs.contract_state.get_gold_gain_multiplier() - 1.0).abs() < f32::EPSILON);
    run_effect(
        &mut gs,
        &Effect::DecreaseGoldGainPercent {
            reduction_percentage: 0.25,
        },
    );
    // 1.0 * (1 - 0.25) = 0.75
    assert!(
        (gs.contract_state.get_gold_gain_multiplier() - 0.75).abs() < 1e-6,
        "골드 획득 감소 적용"
    );
}

#[test]
fn disable_item_use_via_run_effect() {
    let mut gs = make_test_state();
    assert!(!gs.contract_state.is_item_use_disabled());
    run_effect(&mut gs, &Effect::DisableItemUse);
    assert!(
        gs.contract_state.is_item_use_disabled(),
        "아이템 사용 비활성화 플래그 세팅"
    );
}
