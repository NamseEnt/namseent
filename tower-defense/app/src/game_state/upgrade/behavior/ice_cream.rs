use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct IceCreamUpgrade {
    pub damage_bonus_pct: FixedRatio,
    pub waves_remaining: usize,
}

impl UpgradePresentation for IceCreamUpgrade {
    fn key(&self) -> &'static str {
        "ice_cream"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::ICE_CREAM)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        if self.waves_remaining == 0 {
            return Vec::new();
        }
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("{:.0}%", self.damage_bonus_pct.as_f32() * 100.0),
            crate::theme::palette::RED,
        )]
    }

    fn is_applicable(&self, _context: &SelectedTowerContext) -> bool {
        if self.waves_remaining == 0 {
            return false;
        }
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Ice Cream",
            crate::l10n::locale::Language::Korean => "아이스크림",
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
                    .with_bold(format!(
                        "Damage +{:.0}%",
                        self.damage_bonus_pct.as_f32() * 100.0
                    ))
                    .static_text(" for ")
                    .text(self.waves_remaining.to_string())
                    .static_text(" stages");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .text(self.waves_remaining.to_string())
                    .static_text("스테이지 동안 모든 타워 ")
                    .with_bold(format!(
                        "데미지 +{:.0}%",
                        self.damage_bonus_pct.as_f32() * 100.0
                    ));
            }
        }
    }
}

impl IceCreamUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade(damage_bonus_pct: FixedRatio, waves_remaining: usize) -> Upgrade {
        Upgrade::IceCream(IceCreamUpgrade {
            damage_bonus_pct,
            waves_remaining,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    #[test]
    fn ice_cream_effect_applies_to_placed_tower_and_expires_after_waves() {
        use crate::game_state::flow::DefenseFlow;
        use crate::game_state::tower::TowerTemplate;
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.flow = crate::game_state::GameFlow::Defense(DefenseFlow::new(&game_state));
        let upgrade = crate::game_state::upgrade::IceCreamUpgrade::into_upgrade(
            crate::FixedRatio::from_integer(2),
            2,
        );
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            upgrade, None,
        ));

        let tower_template = TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Hearts,
            Rank::Two,
        );
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.sim_tick(),
        );
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::PlaceTower(
            Box::new(tower),
            None,
        ));

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        let base_damage = placed_tower.calculate_projectile_damage(&[], crate::FixedRatio::ONE);
        let boosted_damage = placed_tower.cached_upgrade_damage();

        assert!(boosted_damage > base_damage);
        assert_eq!(
            boosted_damage.ratio_of(base_damage),
            crate::FixedRatio::from_integer(3)
        );

        support::check_defense_end_for_test(&mut game_state);
        let first_tower_after_second_wave = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
            .expect("expected tower to still exist after first stage");
        let second_boosted_damage = first_tower_after_second_wave.cached_upgrade_damage();
        assert_eq!(
            second_boosted_damage.ratio_of(base_damage),
            crate::FixedRatio::from_integer(3)
        );

        game_state.flow = crate::game_state::GameFlow::Defense(DefenseFlow::new(&game_state));
        support::check_defense_end_for_test(&mut game_state);

        let expired_tower = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
            .expect("expected tower to still exist after second stage");
        let expired_damage = expired_tower.cached_upgrade_damage();

        assert_eq!(expired_damage.ratio_of(base_damage), crate::FixedRatio::ONE);
    }
}
