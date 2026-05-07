use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct NameTagUpgrade {
    pub damage_bonus_pct: f32,
    pub target_tower_id: Option<usize>,
}

impl UpgradeBehavior for NameTagUpgrade {
    fn on_tower_placed(&mut self, tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        if self.target_tower_id.is_some() {
            return (TowerPlacementResult::default(), UpgradeUpdateFlags::NONE);
        }

        self.target_tower_id = Some(tower.id());
        (
            TowerPlacementResult::default(),
            UpgradeUpdateFlags::TOWER_STATS,
        )
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        self.target_tower_id.map(|tower_id| {
            (
                TowerUpgradeTarget::TowerId { tower_id },
                self.damage_bonus_pct,
            )
        })
    }
}

impl NameTagUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::NameTag(NameTagUpgrade {
            damage_bonus_pct,
            target_tower_id: None,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    NameTagUpgrade::into_upgrade(2.0)
}
#[cfg(test)]
mod tests {

    use crate::game_state::upgrade::Upgrade;

    #[test]
    fn name_tag_applies_to_next_tower_and_consumes_it() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state
            .upgrade_state
            .upgrade(crate::game_state::upgrade::NameTagUpgrade::into_upgrade(
                2.0,
            ));
        game_state.left_dice = 0;

        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        game_state.goto_placing_tower(template);

        assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
            if let Upgrade::NameTag(upgrade) = upgrade {
                (upgrade.damage_bonus_pct - 2.0).abs() < f32::EPSILON
            } else {
                false
            }
        }));

        let placing_slot_id = game_state
            .hand
            .get_slot_id_by_index(0)
            .expect("expected tower slot to be present");
        let placed_template = support::first_hand_tower_template(&game_state);
        let tower = crate::game_state::tower::Tower::new(
            &placed_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.place_tower(tower);
        game_state.hand.delete_slots(&[placing_slot_id]);

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        support::assert_tower_cached_damage_mul(placed_tower, 3.0);
    }
}
