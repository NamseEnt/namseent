//! run_effect 경로를 통한 Effect 적용 통합 테스트
//! 개별 필드 조작이 아닌 실제 매핑(match) 로직을 검증한다.

use crate::game_state::effect::{Effect, run_effect, tests_support::make_test_state};

#[test]
fn increase_shop_reroll_via_run_effect() {
    let mut gs = make_test_state();
    assert_eq!(gs.max_dice_chance(), 1);
    run_effect(&mut gs, &Effect::IncreaseMaxRerolls { bonus: 1 });
    assert_eq!(gs.max_dice_chance(), 2, "run_effect 경로로 +1 반영");
}

#[test]
fn shop_reroll_penalty_then_bonus_via_run_effect() {
    let mut gs = make_test_state();
    for _ in 0..4 {
        run_effect(&mut gs, &Effect::DecreaseMaxRerolls { penalty: 1 });
    }
    assert_eq!(gs.max_dice_chance(), 0, "패널티 4회 후 0 포화");
    for _ in 0..6 {
        run_effect(&mut gs, &Effect::IncreaseMaxRerolls { bonus: 1 });
    }
    assert_eq!(gs.max_dice_chance(), 3, "-4 +6 => +2 (기본 1 → 3)");
}

#[test]
fn stacking_damage_multiplier_via_run_effect() {
    let mut gs = make_test_state();
    assert!((gs.stage_modifiers.get_damage_multiplier() - 1.0).abs() < f32::EPSILON);
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
        (gs.stage_modifiers.get_damage_multiplier() - 3.0).abs() < 1e-6,
        "누적 데미지 배율 계산"
    );
}

#[test]
fn decrease_gold_gain_percent_via_run_effect() {
    let mut gs = make_test_state();
    assert!((gs.stage_modifiers.get_gold_gain_multiplier() - 1.0).abs() < f32::EPSILON);
    run_effect(
        &mut gs,
        &Effect::DecreaseGoldGainPercent {
            reduction_percentage: 0.25,
        },
    );
    // 1.0 * (1 - 0.25) = 0.75
    assert!(
        (gs.stage_modifiers.get_gold_gain_multiplier() - 0.75).abs() < 1e-6,
        "골드 획득 감소 적용"
    );
}

#[test]
fn disable_item_use_via_run_effect() {
    let mut gs = make_test_state();
    assert!(!gs.stage_modifiers.is_item_use_disabled());
    run_effect(&mut gs, &Effect::DisableItemUse);
    assert!(
        gs.stage_modifiers.is_item_use_disabled(),
        "아이템 사용 비활성화 플래그 세팅"
    );
}

#[test]
fn heal_and_shield_and_earngold_via_run_effect() {
    let mut gs = make_test_state();
    gs.hp = 90.0;
    run_effect(&mut gs, &Effect::Heal { amount: 20.0 });
    assert_eq!(gs.hp, crate::game_state::MAX_HP, "체력은 최대치로 제한됨");

    let mut gs = make_test_state();
    run_effect(&mut gs, &Effect::Shield { amount: 15.0 });
    assert_eq!(gs.shield, 15.0, "실드 증가 적용");

    let mut gs = make_test_state();
    run_effect(&mut gs, &Effect::EarnGold { amount: 50 });
    assert_eq!(gs.gold, 50, "골드 획득 적용");
}

#[test]
fn extra_dice_via_run_effect() {
    let mut gs = make_test_state();
    gs.left_dice = 1;
    run_effect(&mut gs, &Effect::ExtraDice);
    assert_eq!(gs.left_dice, 2, "추가 주사위 1증가");
}

#[test]
fn lose_health_and_lose_gold_via_run_effect() {
    let mut gs = make_test_state();
    gs.hp = 20.0;
    run_effect(&mut gs, &Effect::LoseHealth { amount: 25.0 });
    assert_eq!(gs.hp, 1.0, "체력이 1.0 최솟값으로 포화됨");

    let mut gs = make_test_state();
    gs.gold = 10;
    gs.hp = 100.0;
    run_effect(&mut gs, &Effect::LoseGold { amount: 5 });
    assert_eq!(gs.gold, 5, "골드 감소 정상");
    assert_eq!(gs.hp, 100.0, "체력은 변함 없음");

    let mut gs = make_test_state();
    gs.gold = 3;
    gs.hp = 100.0;
    run_effect(&mut gs, &Effect::LoseGold { amount: 15 });
    assert_eq!(gs.gold, 0, "골드 부족 시 0으로");
    assert!((gs.hp - 98.8).abs() < 1e-6, "부족한 골드 비례 체력 페널티");
}

#[test]
fn damage_reduction_effects_add_status_effects() {
    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::DamageReduction {
            damage_multiply: 0.8,
            duration: namui::Duration::from_secs(5),
        },
    );
    assert_eq!(gs.user_status_effects.len(), 1);

    run_effect(
        &mut gs,
        &Effect::UserDamageReduction {
            multiply: 0.7,
            duration: namui::Duration::from_secs(3),
        },
    );
    assert_eq!(gs.user_status_effects.len(), 2);
}

#[test]
fn grant_upgrade_and_item_via_run_effect() {
    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::GrantUpgrade {
            rarity: crate::rarity::Rarity::Common,
        },
    );
    assert!(
        gs.upgrade_state.gold_earn_plus > 0
            || gs.upgrade_state.shop_slot_expand > 0
            || gs.upgrade_state.dice_chance_plus > 0
            || gs.upgrade_state.shop_item_price_minus > 0
            || gs.upgrade_state.shorten_straight_flush_to_4_cards
            || gs.upgrade_state.skip_rank_for_straight
            || gs.upgrade_state.treat_suits_as_same
            || !gs.upgrade_state.tower_upgrade_states.is_empty()
            || !gs.upgrade_state.tower_select_upgrade_states.is_empty(),
        "업그레이드 상태 변화 확인"
    );

    let mut gs = make_test_state();
    assert!(gs.items.is_empty());
    run_effect(
        &mut gs,
        &Effect::GrantItem {
            rarity: crate::rarity::Rarity::Common,
        },
    );
    assert!(!gs.items.is_empty(), "아이템 획득 확인");
}

#[test]
fn add_tower_card_to_placement_hand_flow_dependent() {
    let mut gs = make_test_state();
    gs.flow =
        crate::game_state::flow::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
    run_effect(
        &mut gs,
        &Effect::AddTowerCardToPlacementHand {
            tower_kind: crate::game_state::tower::TowerKind::Barricade,
            suit: crate::card::Suit::Spades,
            rank: crate::card::Rank::Ace,
            count: 1,
        },
    );
    assert_eq!(
        gs.stage_modifiers.drain_extra_tower_cards().len(),
        1,
        "비배치 플로우에서는 stage grant로 들어감"
    );

    let mut gs = make_test_state();
    gs.flow = crate::game_state::flow::GameFlow::PlacingTower;
    run_effect(
        &mut gs,
        &Effect::AddTowerCardToPlacementHand {
            tower_kind: crate::game_state::tower::TowerKind::Barricade,
            suit: crate::card::Suit::Spades,
            rank: crate::card::Rank::Ace,
            count: 1,
        },
    );
    assert!(
        !gs.hand.active_slot_ids().is_empty(),
        "배치 플로우에서는 즉시 핸드 추가"
    );
}

#[test]
fn stage_modifiers_via_run_effect() {
    let mut gs = make_test_state();
    run_effect(&mut gs, &Effect::IncreaseIncomingDamage { multiplier: 1.2 });
    run_effect(&mut gs, &Effect::DecreaseIncomingDamage { multiplier: 0.8 });
    assert!((gs.stage_modifiers.get_incoming_damage_multiplier() - 1.2).abs() < 1e-6);
    assert!((gs.stage_modifiers.get_damage_reduction_multiplier() - 0.8).abs() < 1e-6);

    run_effect(
        &mut gs,
        &Effect::IncreaseEnemyHealthPercent { percentage: 20.0 },
    );
    run_effect(
        &mut gs,
        &Effect::DecreaseEnemyHealthPercent { percentage: 10.0 },
    );
    assert!((gs.stage_modifiers.get_enemy_health_multiplier() - 1.08).abs() < 1e-6);

    run_effect(&mut gs, &Effect::IncreaseEnemySpeed { multiplier: 1.3 });
    run_effect(&mut gs, &Effect::DecreaseEnemySpeed { multiplier: 0.5 });
    assert!((gs.stage_modifiers.get_enemy_speed_multiplier() - 0.65).abs() < 1e-6);

    run_effect(&mut gs, &Effect::DisableItemAndUpgradePurchases);
    assert!(gs.stage_modifiers.is_item_and_upgrade_purchases_disabled());

    run_effect(&mut gs, &Effect::IncreaseMaxHandSlots { bonus: 2 });
    run_effect(&mut gs, &Effect::DecreaseMaxHandSlots { penalty: 1 });
    assert_eq!(gs.stage_modifiers.get_max_hand_slots_delta(), 1);

    run_effect(&mut gs, &Effect::DecreaseMaxRerolls { penalty: 1 });
    run_effect(&mut gs, &Effect::IncreaseMaxRerolls { bonus: 2 });
    assert_eq!(gs.max_dice_chance(), 2);

    run_effect(
        &mut gs,
        &Effect::RankTowerDisable {
            rank: crate::card::Rank::Ace,
        },
    );
    run_effect(
        &mut gs,
        &Effect::SuitTowerDisable {
            suit: crate::card::Suit::Spades,
        },
    );
    assert!(
        gs.stage_modifiers
            .get_disabled_ranks()
            .contains(&crate::card::Rank::Ace)
    );
    assert!(
        gs.stage_modifiers
            .get_disabled_suits()
            .contains(&crate::card::Suit::Spades)
    );
}
