use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MirrorUpgrade {
    pub pending: bool,
}

impl UpgradeBehavior for MirrorUpgrade {
    fn on_tower_placement(
        &mut self,
        _tower_template: &mut TowerTemplate,
        _left_dice: usize,
    ) -> usize {
        0
    }

    fn on_tower_placed_mut(
        &mut self,
        game_state: &mut GameState,
        tower: &Tower,
    ) -> UpgradeUpdateFlags {
        if !self.pending {
            return UpgradeUpdateFlags::NONE;
        }

        let tower_template = (**tower).clone();
        game_state
            .hand
            .push(crate::hand::HandItem::Tower(tower_template));
        self.pending = false;
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl MirrorUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Mirror(MirrorUpgrade { pending: true })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    MirrorUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    use crate::game_state::upgrade::Upgrade;

    #[test]
    fn mirror_duplicates_next_acquired_tower() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state
            .upgrade_state
            .upgrade(crate::game_state::upgrade::NameTagUpgrade::into_upgrade(
                2.0,
            ));
        game_state
            .upgrade_state
            .upgrade(crate::game_state::upgrade::MirrorUpgrade::into_upgrade());
        game_state
            .upgrade_state
            .upgrade(crate::game_state::upgrade::MirrorUpgrade::into_upgrade());
        game_state.left_dice = 0;

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        game_state.goto_placing_tower(tower_template);

        let placing_slot_id = game_state
            .hand
            .get_slot_id_by_index(0)
            .expect("expected tower slot to be present");
        let tower_template = support::first_hand_tower_template(&game_state);
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.place_tower(tower);
        game_state.hand.delete_slots(&[placing_slot_id]);

        let slot_ids = game_state.hand.active_slot_ids();
        assert_eq!(slot_ids.len(), 2);
        assert_eq!(
            game_state
                .upgrade_state
                .upgrades
                .iter()
                .filter(|upgrade| { matches!(upgrade, Upgrade::Mirror(upgrade) if upgrade.pending) })
                .count(),
            0
        );
        assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
            if let Upgrade::NameTag(upgrade) = upgrade {
                (upgrade.damage_multiplier - 2.0).abs() < f32::EPSILON
            } else {
                false
            }
        }));

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        let base_damage = placed_tower.calculate_projectile_damage(&[], 1.0);
        let boosted_damage = placed_tower.cached_upgrade_damage();
        assert!((boosted_damage / base_damage - 2.0).abs() < f32::EPSILON);
    }
}
