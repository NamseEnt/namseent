use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ResolutionUpgrade {
    pub damage_bonus_pct_per_reroll: f32,
    pub stored_rerolls: usize,
}

impl UpgradeBehavior for ResolutionUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::RESOLUTION,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn thumbnail_overlay(
        &self,
        width_height: Wh<Px>,
        _game_state: &GameState,
    ) -> Option<RenderingTree> {
        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &format!(
                "{:.0}%",
                self.stored_rerolls as f32 * self.damage_bonus_pct_per_reroll * 100.0
            ),
            crate::theme::palette::RED,
        ))
    }

    fn on_stage_start(&mut self, game_state: &mut GameState, _stage: usize) -> UpgradeUpdateFlags {
        self.update_stored_rerolls(game_state)
    }

    fn on_card_reroll(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.update_stored_rerolls(game_state)
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stored_rerolls > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.stored_rerolls as f32 * self.damage_bonus_pct_per_reroll,
            ))
        } else {
            None
        }
    }

    fn is_applicable(&self, _context: &SelectedTowerContext) -> bool {
        if self.stored_rerolls > 0 {
            return true;
        }
        false
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
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder
                    .static_text("Remaining rerolls give ")
                    .with_damage_value(format!(
                        "damage +{:.0}%",
                        self.damage_bonus_pct_per_reroll * 100.0
                    ))
                    .static_text("for all towers");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("남은 리롤마다 모든 타워")
                    .static_text(" ")
                    .with_damage_value(format!(
                        "데미지 +{:.0}%",
                        self.damage_bonus_pct_per_reroll * 100.0
                    ));
            }
        }
    }
}

impl ResolutionUpgrade {
    pub fn into_upgrade(damage_bonus_pct_per_reroll: f32) -> Upgrade {
        Upgrade::Resolution(ResolutionUpgrade {
            damage_bonus_pct_per_reroll,
            stored_rerolls: 0,
        })
    }

    fn update_stored_rerolls(&mut self, game_state: &GameState) -> UpgradeUpdateFlags {
        if self.stored_rerolls != game_state.left_dice {
            self.stored_rerolls = game_state.left_dice;
            UpgradeUpdateFlags::TOWER_STATS
        } else {
            UpgradeUpdateFlags::NONE
        }
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_epic,
);

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
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::ResolutionUpgrade::into_upgrade(0.25),
            None,
        ));
        game_state.left_dice = 2;
        game_state.action(crate::game_state::GameStateAction::StageEnd {
            perfect_clear: false,
            gold: 0,
            item_count: 0,
        });

        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        game_state.action(crate::game_state::GameStateAction::StartPlacingTower(
            template,
        ));

        assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
            if let Upgrade::Resolution(upgrade) = &upgrade.upgrade {
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
        game_state.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(tower),
            None,
        ));
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

        let flags = upgrade.on_stage_end(&mut game_state, false, 0, 0);

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

        let flags = upgrade.on_stage_end(&mut game_state, false, 0, 0);

        assert_eq!(flags, UpgradeUpdateFlags::NONE);
    }

    #[test]
    fn resolution_updates_tower_stats_on_card_reroll() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let mut upgrade = ResolutionUpgrade {
            damage_bonus_pct_per_reroll: 0.25,
            stored_rerolls: 2,
        };
        game_state.left_dice = 1;

        let flags = upgrade.on_card_reroll(&mut game_state);

        assert!(flags.contains(UpgradeUpdateFlags::TOWER_STATS));
        assert_eq!(upgrade.stored_rerolls, 1);
    }
}
