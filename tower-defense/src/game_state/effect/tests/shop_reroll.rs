//! Shop reroll (shop refresh chance) 관련 effect 동작 테스트
//!
//! 이 테스트는 계약/이펙트 시스템을 직접 호출하지 않고, StageModifiers 변화를 통해
//! effect 적용 결과를 검증한다. 추후 필요 시 실제 Effect::IncreaseShopMaxRerolls 등을
//! run_effect 경로로 호출하는 통합 테스트를 추가할 수 있다.

use crate::game_state::effect::tests_support::make_test_state;

#[test]
fn shop_reroll_baseline_is_one() {
    let gs = make_test_state();
    assert_eq!(
        gs.max_shop_refresh_chance(),
        1,
        "업그레이드/이펙트가 없을 때 기본값은 1이어야 합니다"
    );
}

#[test]
fn shop_reroll_increase_effect_applies() {
    let mut gs = make_test_state();
    gs.stage_modifiers.apply_shop_max_rerolls_bonus(1); // +1
    assert_eq!(
        gs.max_shop_refresh_chance(),
        2,
        "+1 보너스 적용 후 2가 되어야 함"
    );
}

#[test]
fn shop_reroll_decrease_effect_applies() {
    let mut gs = make_test_state();
    gs.stage_modifiers.apply_shop_max_rerolls_penalty(1); // -1 (saturating)
    assert_eq!(
        gs.max_shop_refresh_chance(),
        0,
        "-1 패널티 적용 후 0으로 내림(포화 감산)"
    );
}

#[test]
fn shop_reroll_stacking_sequence_penalty_then_bonus() {
    let mut gs = make_test_state();
    for _ in 0..4 {
    gs.stage_modifiers.apply_shop_max_rerolls_penalty(1);
    }
    assert_eq!(gs.max_shop_refresh_chance(), 0, "패널티 4회 후 포화로 0");
    for _ in 0..6 {
    gs.stage_modifiers.apply_shop_max_rerolls_bonus(1);
    }
    assert_eq!(
        gs.max_shop_refresh_chance(),
        3,
        "패널티 4회, 보너스 6회 누적 후 최종 3 (기본 1 대비 +2)"
    );
}
