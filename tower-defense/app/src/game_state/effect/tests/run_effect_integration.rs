//! run_effect 경로를 통한 Effect 적용 통합 테스트
//! 개별 필드 조작이 아닌 실제 매핑(match) 로직을 검증한다.

use crate::game_state::{
    card::{Rank, Suit},
    effect::{Effect, run_effect, tests_support::make_test_state},
};

#[test]
fn increase_reroll_via_run_effect() {
    let mut gs = make_test_state();
    let default_dice = gs.config.player.base_dice_chance;
    assert_eq!(gs.max_dice_chance(), default_dice);
    run_effect(&mut gs, &Effect::IncreaseMaxRerolls { bonus: 1 });
    assert_eq!(
        gs.max_dice_chance(),
        default_dice + 1,
        "run_effect 경로로 +1 반영"
    );
}

#[test]
fn reroll_penalty_then_bonus_via_run_effect() {
    let mut gs = make_test_state();
    let default_dice = gs.config.player.base_dice_chance;
    for _ in 0..4 {
        run_effect(&mut gs, &Effect::DecreaseMaxRerolls { penalty: 1 });
    }
    assert_eq!(gs.max_dice_chance(), 0, "패널티 4회 후 0 포화");
    for _ in 0..6 {
        run_effect(&mut gs, &Effect::IncreaseMaxRerolls { bonus: 1 });
    }
    assert_eq!(
        gs.max_dice_chance(),
        default_dice + 2,
        "-4 +6 => +2 (기본 {} → {})",
        default_dice,
        default_dice + 2
    );
}

#[test]
fn stacking_damage_multiplier_via_run_effect() {
    let mut gs = make_test_state();
    assert_eq!(
        gs.stage_modifiers.get_damage_multiplier(),
        crate::FixedRatio::ONE
    );
    run_effect(
        &mut gs,
        &Effect::IncreaseAllTowersDamage {
            multiplier: crate::FixedRatio::from_raw(1_500_000),
        },
    );
    run_effect(
        &mut gs,
        &Effect::IncreaseAllTowersDamage {
            multiplier: crate::FixedRatio::from_integer(2),
        },
    );
    // 1.0 * 1.5 * 2.0 = 3.0
    assert!(
        gs.stage_modifiers.get_damage_multiplier() == crate::FixedRatio::from_integer(3),
        "누적 데미지 배율 계산"
    );
}

#[test]
fn decrease_gold_gain_percent_via_run_effect() {
    let mut gs = make_test_state();
    assert_eq!(
        gs.stage_modifiers.get_gold_gain_multiplier(),
        crate::FixedRatio::ONE
    );
    run_effect(
        &mut gs,
        &Effect::DecreaseGoldGainPercent {
            reduction_percentage: crate::FixedRatio::from_raw(250_000),
        },
    );
    // 1.0 * (1 - 0.25) = 0.75
    assert!(
        gs.stage_modifiers.get_gold_gain_multiplier() == crate::FixedRatio::from_raw(750_000),
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
    gs.hp = crate::Health::from_integer(90);
    run_effect(
        &mut gs,
        &Effect::Heal {
            amount: crate::Health::from_integer(20),
        },
    );
    assert_eq!(gs.hp, gs.config.player.max_hp, "체력은 최대치로 제한됨");

    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::Shield {
            amount: crate::Shield::from_integer(15),
        },
    );
    assert_eq!(
        gs.shield_amount(),
        crate::Shield::from_integer(15),
        "실드 증가 적용"
    );

    let mut gs = make_test_state();
    run_effect(&mut gs, &Effect::EarnGold { amount: 50 });
    assert_eq!(gs.gold, 50, "골드 획득 적용");
}

#[test]
fn lose_health_and_lose_gold_via_run_effect() {
    let mut gs = make_test_state();
    gs.hp = crate::Health::from_integer(20);
    run_effect(
        &mut gs,
        &Effect::LoseHealth {
            amount: crate::Health::from_integer(25),
        },
    );
    assert_eq!(
        gs.hp,
        crate::Health::from_integer(1),
        "체력이 1.0 최솟값으로 포화됨"
    );

    let mut gs = make_test_state();
    gs.gold = 10;
    gs.hp = crate::Health::from_integer(100);
    run_effect(&mut gs, &Effect::LoseGold { amount: 5 });
    assert_eq!(gs.gold, 5, "골드 감소 정상");
    assert_eq!(gs.hp, crate::Health::from_integer(100), "체력은 변함 없음");

    let mut gs = make_test_state();
    gs.gold = 3;
    gs.hp = crate::Health::from_integer(100);
    run_effect(&mut gs, &Effect::LoseGold { amount: 15 });
    assert_eq!(gs.gold, 0, "골드 부족 시 0으로");
    assert_eq!(
        gs.hp,
        crate::Health::from_raw(98_800),
        "부족한 골드 비례 체력 페널티"
    );
}

#[test]
fn damage_reduction_effects_add_status_effects() {
    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::DamageReduction {
            damage_multiply: crate::FixedRatio::from_raw(800_000),
            duration: crate::SimTickSpan::from_millis_ceil(5_000),
        },
    );
    assert_eq!(gs.user_status_effects.len(), 1);
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
        !gs.raw_core_state().upgrades().upgrades.is_empty(),
        "authoritative upgrade state changed"
    );
    assert!(
        !gs.presentation_upgrade_state_snapshot().upgrades.is_empty(),
        "presentation upgrade state changed"
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
fn stage_modifiers_via_run_effect() {
    let mut gs = make_test_state();
    run_effect(
        &mut gs,
        &Effect::IncreaseIncomingDamage {
            multiplier: crate::FixedRatio::from_raw(1_200_000),
        },
    );
    run_effect(
        &mut gs,
        &Effect::DecreaseIncomingDamage {
            multiplier: crate::FixedRatio::from_raw(800_000),
        },
    );
    assert_eq!(
        gs.stage_modifiers.get_incoming_damage_multiplier(),
        crate::FixedRatio::from_raw(1_200_000)
    );
    assert_eq!(
        gs.stage_modifiers.get_damage_reduction_multiplier(),
        crate::FixedRatio::from_raw(800_000)
    );

    run_effect(
        &mut gs,
        &Effect::IncreaseEnemyHealthPercent {
            percentage: crate::FixedRatio::from_integer(20),
        },
    );
    run_effect(
        &mut gs,
        &Effect::DecreaseEnemyHealthPercent {
            percentage: crate::FixedRatio::from_integer(10),
        },
    );
    assert_eq!(
        gs.stage_modifiers.get_enemy_health_multiplier(),
        crate::FixedRatio::from_raw(1_080_000)
    );

    run_effect(
        &mut gs,
        &Effect::IncreaseEnemySpeed {
            multiplier: crate::FixedRatio::from_raw(1_300_000),
        },
    );
    run_effect(
        &mut gs,
        &Effect::DecreaseEnemySpeed {
            multiplier: crate::FixedRatio::from_raw(500_000),
        },
    );
    assert_eq!(
        gs.stage_modifiers.get_enemy_speed_multiplier(),
        crate::FixedRatio::from_raw(650_000)
    );

    run_effect(&mut gs, &Effect::DisableItemAndUpgradePurchases);
    assert!(gs.stage_modifiers.is_item_and_upgrade_purchases_disabled());

    run_effect(&mut gs, &Effect::IncreaseMaxHandSlots { bonus: 2 });
    run_effect(&mut gs, &Effect::DecreaseMaxHandSlots { penalty: 1 });
    assert_eq!(gs.stage_modifiers.get_max_hand_slots_delta(), 1);

    let default_dice = gs.config.player.base_dice_chance;
    run_effect(&mut gs, &Effect::DecreaseMaxRerolls { penalty: 1 });
    run_effect(&mut gs, &Effect::IncreaseMaxRerolls { bonus: 2 });
    assert_eq!(
        gs.max_dice_chance(),
        default_dice + 1,
        "-1 +2 => {}",
        default_dice + 1
    );

    run_effect(&mut gs, &Effect::RankTowerDisable { rank: Rank::Ace });
    run_effect(&mut gs, &Effect::SuitTowerDisable { suit: Suit::Spades });
    assert!(gs.stage_modifiers.get_disabled_ranks().contains(&Rank::Ace));
    assert!(
        gs.stage_modifiers
            .get_disabled_suits()
            .contains(&Suit::Spades)
    );
}
