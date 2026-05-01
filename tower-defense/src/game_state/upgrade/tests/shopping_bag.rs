use super::super::*;

#[test]
fn shopping_bag_upgrade_activates_without_stacks() {
    let mut state = UpgradeState::default();
    state.upgrade(crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(1.5));

    assert!(state.upgrades.iter().any(|u| {
        matches!(u, Upgrade::ShoppingBag(..)) && u.stacks().unwrap_or(usize::MAX) == 0
    }));
}

#[test]
fn shopping_bag_global_tower_damage_multiplier_increases_with_stacks() {
    let mut gs = super::support::create_mock_game_state();
    gs.upgrade_state
        .upgrade(crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(1.5));
    if let Some(bag) = gs.upgrade_state.upgrades.iter_mut().find_map(|u| {
        if let Upgrade::ShoppingBag(upgrade) = u {
            Some(&mut upgrade.stacks)
        } else {
            None
        }
    }) {
        *bag = 2;
    }

    assert_eq!(gs.upgrade_state.global_tower_damage_multiplier(&gs), 2.0);
}
