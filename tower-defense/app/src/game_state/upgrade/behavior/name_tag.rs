use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct NameTagUpgrade {
    pub damage_bonus_pct: FixedRatio,
    pub target_tower_id: Option<crate::TowerId>,
}

impl UpgradePresentation for NameTagUpgrade {
    fn key(&self) -> &'static str {
        "name_tag"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::NAME_TAG)
    }

    fn is_applicable(&self, context: &SelectedTowerContext) -> bool {
        match (context.tower_id, self.target_tower_id) {
            (SelectedTowerId::Placed(selected_tower_id), Some(target_tower_id)) => {
                selected_tower_id == target_tower_id
            }
            (SelectedTowerId::ToBePlaced, None) => true,
            _ => false,
        }
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Name Tag",
            crate::l10n::locale::Language::Korean => "이름표",
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
                    .static_text("The next tower you place gains ")
                    .with_bold(format!(
                        "damage +{:.0}%",
                        self.damage_bonus_pct.as_f32() * 100.0
                    ));
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("다음 배치하는 타워 ")
                    .with_bold(format!(
                        "데미지 +{:.0}%",
                        self.damage_bonus_pct.as_f32() * 100.0
                    ));
            }
        }
    }
}

impl NameTagUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade(damage_bonus_pct: FixedRatio) -> Upgrade {
        Upgrade::NameTag(NameTagUpgrade {
            damage_bonus_pct,
            target_tower_id: None,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::game_state::{
        card::{Rank, Suit},
        upgrade::Upgrade,
    };

    #[test]
    fn name_tag_applies_to_next_tower_and_consumes_it() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::NameTagUpgrade::into_upgrade(
                crate::FixedRatio::from_integer(2),
            ),
            None,
        ));
        game_state.left_dice = 0;

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
                .any(|upgrade| { upgrade.upgrade_kind() == Ok(td_core::UpgradeKind::NameTag) })
        );
        assert!(
            game_state
                .presentation_upgrade_state_snapshot()
                .upgrades
                .iter()
                .any(|upgrade| {
                    if let Upgrade::NameTag(upgrade) = &upgrade.upgrade {
                        upgrade.damage_bonus_pct == crate::FixedRatio::from_integer(2)
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
        support::assert_tower_cached_damage_mul(placed_tower, 3.0);
    }
}
