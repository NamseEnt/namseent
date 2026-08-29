use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[cfg(test)]
const DAMAGE_BONUS_PCT_PER_REROLL: FixedRatio = FixedRatio::from_raw(250_000);

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ResolutionUpgrade {
    pub damage_bonus_pct_per_reroll: FixedRatio,
    pub stored_rerolls: usize,
}

impl UpgradePresentation for ResolutionUpgrade {
    fn key(&self) -> &'static str {
        "resolution"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::RESOLUTION)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!(
                "{:.0}%",
                self.stored_rerolls as f32 * self.damage_bonus_pct_per_reroll.as_f32() * 100.0
            ),
            crate::theme::palette::RED,
        )]
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
                    .with_bold(format!(
                        "damage +{:.0}%",
                        self.damage_bonus_pct_per_reroll.as_f32() * 100.0
                    ))
                    .static_text("for all towers");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("남은 리롤마다 모든 타워")
                    .static_text(" ")
                    .with_bold(format!(
                        "데미지 +{:.0}%",
                        self.damage_bonus_pct_per_reroll.as_f32() * 100.0
                    ));
            }
        }
    }
}

impl ResolutionUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade(damage_bonus_pct_per_reroll: FixedRatio) -> Upgrade {
        Upgrade::Resolution(ResolutionUpgrade {
            damage_bonus_pct_per_reroll,
            stored_rerolls: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        card::{Rank, Suit},
        config::DEFAULT_BASE_DICE_CHANCE,
        game_state::upgrade::Upgrade,
    };

    #[test]
    fn resolution_applies_remaining_reroll_damage_and_consumes_it() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::ResolutionUpgrade::into_upgrade(
                crate::FixedRatio::from_raw(250_000),
            ),
            None,
        ));
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::StageEnd {
            perfect_clear: false,
            gold: 0,
            item_count: 0,
        });
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::StartStage {
            stage: 1,
        });

        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Spades,
            Rank::Ace,
        );
        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::StartPlacingTower(template),
        );

        assert!(
            game_state
                .raw_core_state()
                .upgrades()
                .upgrades
                .iter()
                .any(|upgrade| { upgrade.upgrade_kind() == Ok(td_core::UpgradeKind::Resolution) })
        );
        assert!(
            game_state
                .presentation_upgrade_state_snapshot()
                .upgrades
                .iter()
                .any(|upgrade| {
                    if let Upgrade::Resolution(upgrade) = &upgrade.upgrade {
                        upgrade.damage_bonus_pct_per_reroll == DAMAGE_BONUS_PCT_PER_REROLL
                    } else {
                        false
                    }
                })
        );

        let placing_slot_id = game_state
            .hand
            .get_slot_id_by_index(0)
            .expect("expected tower slot to be present");
        let placed_template = support::first_hand_tower_template(&game_state);
        let tower = crate::game_state::tower::Tower::new(
            &placed_template,
            crate::MapCoord::new(0, 0),
            game_state.sim_tick(),
        );
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::PlaceTower(
            Box::new(tower),
            None,
        ));
        game_state.hand.delete_slots(&[placing_slot_id]);

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        support::assert_tower_cached_damage_mul(
            placed_tower,
            DAMAGE_BONUS_PCT_PER_REROLL.as_f32() * DEFAULT_BASE_DICE_CHANCE as f32 + 1.0,
        );
    }

    #[test]
    fn resolution_keeps_stored_rerolls_when_core_input_is_unchanged() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            ResolutionUpgrade::into_upgrade(crate::FixedRatio::from_raw(250_000)),
            None,
        ));
        let mut raw = game_state.raw_core_state().clone();
        raw.edit_snapshot(|parts| parts.progress.left_dice = 2)
            .expect("valid dice count");
        raw.trigger_card_reroll_upgrades();
        let before = raw.upgrades().upgrades[0].scalar_values.clone();
        raw.trigger_card_reroll_upgrades();
        assert_eq!(raw.upgrades().upgrades[0].scalar_values, before);
    }

    #[test]
    fn resolution_updates_stored_rerolls_on_card_reroll() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            ResolutionUpgrade::into_upgrade(crate::FixedRatio::from_raw(250_000)),
            None,
        ));
        let mut raw = game_state.raw_core_state().clone();
        raw.edit_snapshot(|parts| parts.progress.left_dice = 1)
            .expect("valid dice count");
        raw.trigger_card_reroll_upgrades();
        assert_eq!(
            raw.upgrades().upgrades[0].scalar_values.first().copied(),
            Some(1)
        );
    }
}
