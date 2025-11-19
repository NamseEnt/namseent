use crate::game_state::tower::{
    TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind, TowerTemplate,
};
use crate::game_state::upgrade::{TowerSelectUpgradeTarget, TowerUpgradeState, UpgradeState};

pub fn inject_status_effects(
    tower: &mut TowerTemplate,
    upgrade_state: &UpgradeState,
    rerolled_count: usize,
) {
    let mut inject_tower_upgrades = |upgrade: &TowerUpgradeState| {
        if upgrade.damage_multiplier > 1.0 {
            let upgrade_effect = TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageMul {
                    mul: upgrade.damage_multiplier,
                },
                end_at: TowerStatusEffectEnd::NeverEnd,
            };
            tower.default_status_effects.push(upgrade_effect);
        }
    };

    if tower.kind.is_low_card_tower()
        && let Some(upgrade) = upgrade_state
            .tower_select_upgrade_states
            .get(&TowerSelectUpgradeTarget::LowCard)
    {
        inject_tower_upgrades(upgrade);
    }

    if rerolled_count == 0 {
        if let Some(upgrade) = upgrade_state
            .tower_select_upgrade_states
            .get(&TowerSelectUpgradeTarget::NoReroll)
        {
            inject_tower_upgrades(upgrade);
        }
    } else if rerolled_count > 0 {
        for _ in 0..rerolled_count {
            if let Some(upgrade) = upgrade_state
                .tower_select_upgrade_states
                .get(&TowerSelectUpgradeTarget::Reroll)
            {
                inject_tower_upgrades(upgrade);
            }
        }
    }
}
