use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MirrorUpgrade {
    pub pending: bool,
}

impl UpgradePresentation for MirrorUpgrade {
    fn key(&self) -> &'static str {
        "mirror"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::MIRROR)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Mirror",
            crate::l10n::locale::Language::Korean => "거울",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Duplicate the next tower you place",
            crate::l10n::locale::Language::Korean => "다음에 배치하는 타워를 복제합니다",
        });
    }
}

impl MirrorUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Mirror(MirrorUpgrade { pending: true })
    }
}

#[cfg(test)]
mod tests {

    use crate::game_state::{
        card::{Rank, Suit},
        upgrade::Upgrade,
    };

    #[test]
    fn mirror_duplicates_next_acquired_tower() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::NameTagUpgrade::into_upgrade(crate::FixedRatio::ONE),
            None,
        ));
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::MirrorUpgrade::into_upgrade(),
            None,
        ));
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::MirrorUpgrade::into_upgrade(),
            None,
        ));
        game_state.left_dice = 0;

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Spades,
            Rank::Ace,
        );
        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::StartPlacingTower(tower_template),
        );

        let placing_slot_id = game_state
            .hand
            .get_slot_id_by_index(0)
            .expect("expected tower slot to be present");
        let tower_template = support::first_hand_tower_template(&game_state);
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.sim_tick(),
        );
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::PlaceTower(
            Box::new(tower),
            None,
        ));
        game_state.hand.delete_slots(&[placing_slot_id]);

        assert!(
            game_state
                .raw_core_state()
                .upgrades()
                .upgrades
                .iter()
                .any(|upgrade| { upgrade.upgrade_kind() == Ok(td_core::UpgradeKind::NameTag) })
        );
        let slot_ids = game_state.hand.active_slot_ids();
        assert_eq!(slot_ids.len(), 2);
        assert_eq!(
            game_state
                .presentation_upgrade_state_snapshot()
                .upgrades
                .iter()
                .filter(|upgrade| {
                    matches!(upgrade.upgrade, Upgrade::Mirror(upgrade) if upgrade.pending)
                })
                .count(),
            0
        );
        assert!(
            game_state
                .presentation_upgrade_state_snapshot()
                .upgrades
                .iter()
                .any(|upgrade| {
                    if let Upgrade::NameTag(upgrade) = &upgrade.upgrade {
                        upgrade.damage_bonus_pct == crate::FixedRatio::ONE
                    } else {
                        false
                    }
                })
        );

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        let base_damage = placed_tower.calculate_projectile_damage(&[], crate::FixedRatio::ONE);
        let boosted_damage = placed_tower.cached_upgrade_damage();
        assert_eq!(
            boosted_damage.ratio_of(base_damage),
            crate::FixedRatio::from_integer(2)
        );
    }
}
