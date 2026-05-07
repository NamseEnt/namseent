use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ResolutionUpgrade {
    pub damage_bonus_pct_per_reroll: f32,
    pub stored_rerolls: usize,
}

impl UpgradeBehavior for ResolutionUpgrade {
    fn on_stage_end_with_state(
        &mut self,
        game_state: &GameState,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        let before = self.stored_rerolls;
        self.stored_rerolls = game_state.left_dice;
        if self.stored_rerolls != before {
            (0, UpgradeUpdateFlags::TOWER_STATS)
        } else {
            (0, UpgradeUpdateFlags::NONE)
        }
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stored_rerolls > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.stored_rerolls as f32 * self.damage_bonus_pct_per_reroll,
            ))
        } else {
            None
        }
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Resolution",
            crate::l10n::locale::Language::Korean => "결심",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => format!(
                "Remaining rerolls add +{:.0}% damage to the next tower (currently +{:.0}%)",
                self.damage_bonus_pct_per_reroll * 100.0,
                self.stored_rerolls as f32 * self.damage_bonus_pct_per_reroll * 100.0,
            ),
            crate::l10n::locale::Language::Korean => format!(
                "남은 리롤마다 피해 +{:.0}% (현재 +{:.0}%)",
                self.damage_bonus_pct_per_reroll * 100.0,
                self.stored_rerolls as f32 * self.damage_bonus_pct_per_reroll * 100.0,
            ),
        });
    }
}

impl ResolutionUpgrade {
    pub fn into_upgrade(damage_bonus_pct_per_reroll: f32) -> Upgrade {
        Upgrade::Resolution(ResolutionUpgrade {
            damage_bonus_pct_per_reroll,
            stored_rerolls: 0,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    ResolutionUpgrade::into_upgrade(0.25)
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::game_state::upgrade::{Upgrade, UpgradeUpdateFlags};

    #[test]
    fn resolution_applies_remaining_reroll_damage_and_consumes_it() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.upgrade_state.upgrade(
            crate::game_state::upgrade::ResolutionUpgrade::into_upgrade(0.25),
        );
        game_state.left_dice = 2;
        game_state.apply_stage_end(false, 0, 0);

        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        game_state.goto_placing_tower(template);

        assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
            if let Upgrade::Resolution(upgrade) = upgrade {
                (upgrade.damage_bonus_pct_per_reroll - 0.25).abs() < f32::EPSILON
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
        support::assert_tower_cached_damage_mul(placed_tower, 1.5);
    }

    #[test]
    fn resolution_returns_tower_stats_when_stored_rerolls_changed() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let mut upgrade = ResolutionUpgrade {
            damage_bonus_pct_per_reroll: 0.25,
            stored_rerolls: 0,
        };
        game_state.left_dice = 3;

        let (_, flags) = upgrade.on_stage_end_with_state(&game_state, false, 0, 0);

        assert!(flags.contains(UpgradeUpdateFlags::TOWER_STATS));
    }

    #[test]
    fn resolution_returns_none_when_stored_rerolls_unchanged() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let mut upgrade = ResolutionUpgrade {
            damage_bonus_pct_per_reroll: 0.25,
            stored_rerolls: 2,
        };
        game_state.left_dice = 2;

        let (_, flags) = upgrade.on_stage_end_with_state(&game_state, false, 0, 0);

        assert_eq!(flags, UpgradeUpdateFlags::NONE);
    }
}
