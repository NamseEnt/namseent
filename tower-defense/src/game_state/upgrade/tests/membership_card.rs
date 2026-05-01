use super::super::*;

#[test]
fn membership_card_grants_free_shop_next_stage() {
    let mut state = UpgradeState::default();
    state.upgrade(crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade());

    let effects = state.stage_start_effects(3);
    assert_eq!(effects.damage_multiplier, 1.0);
    assert!(effects.free_shop_this_stage);

    let next_effects = state.stage_start_effects(4);
    assert!(!next_effects.free_shop_this_stage);
}
