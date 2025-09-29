//! Card selection hand reroll & slot 관련 effect 통합 테스트
//! run_effect 경로를 통해 StageModifiers와 GameState 계산(max_reroll_chance) 검증

use crate::game_state::effect::{Effect, run_effect, tests_support::make_test_state};

#[test]
fn card_selection_reroll_baseline_is_one() {
    let gs = make_test_state();
    assert_eq!(
        gs.max_reroll_chance(),
        1,
        "기본 카드 선택 핸드 reroll chance 는 1"
    );
}

#[test]
fn increase_card_selection_reroll_via_run_effect() {
    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::IncreaseCardSelectionHandMaxRerolls { bonus: 2 },
    );
    assert_eq!(gs.max_reroll_chance(), 3, "+2 보너스 => 1 + 2 = 3");
}

#[test]
fn decrease_card_selection_reroll_via_run_effect() {
    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::DecreaseCardSelectionHandMaxRerolls { penalty: 1 },
    );
    assert_eq!(gs.max_reroll_chance(), 0, "-1 패널티 포화감산 => 0");
}

#[test]
fn card_selection_reroll_penalty_then_bonus_sequence() {
    let mut gs = make_test_state();
    for _ in 0..3 {
        run_effect(
            &mut gs,
            &Effect::DecreaseCardSelectionHandMaxRerolls { penalty: 1 },
        );
    }
    assert_eq!(gs.max_reroll_chance(), 0, "패널티 3회 후 0 유지");
    for _ in 0..5 {
        run_effect(
            &mut gs,
            &Effect::IncreaseCardSelectionHandMaxRerolls { bonus: 1 },
        );
    }
    // baseline 1, -3 => 0, +5 => (1 +5) -3 = 3
    assert_eq!(gs.max_reroll_chance(), 3, "-3 +5 => +2 (최종 3)");
}

#[test]
fn card_selection_slots_bonus_and_penalty_accumulation() {
    let mut gs = make_test_state();
    // 슬롯의 최종 적용은 별도 계산 함수가 없으므로 StageModifiers의 getter 직접 확인
    assert_eq!(
        gs.stage_modifiers.get_card_selection_hand_max_slots_bonus(),
        0
    );
    assert_eq!(
        gs.stage_modifiers
            .get_card_selection_hand_max_slots_penalty(),
        0
    );

    run_effect(
        &mut gs,
        &Effect::IncreaseCardSelectionHandMaxSlots { bonus: 2 },
    );
    run_effect(
        &mut gs,
        &Effect::IncreaseCardSelectionHandMaxSlots { bonus: 1 },
    );
    assert_eq!(
        gs.stage_modifiers.get_card_selection_hand_max_slots_bonus(),
        3,
        "+2 +1 누적 보너스 = 3"
    );

    run_effect(
        &mut gs,
        &Effect::DecreaseCardSelectionHandMaxSlots { penalty: 1 },
    );
    run_effect(
        &mut gs,
        &Effect::DecreaseCardSelectionHandMaxSlots { penalty: 2 },
    );
    assert_eq!(
        gs.stage_modifiers
            .get_card_selection_hand_max_slots_penalty(),
        3,
        "-1 -2 누적 패널티 = 3"
    );
}
