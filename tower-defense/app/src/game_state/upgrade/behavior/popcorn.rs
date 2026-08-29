use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PopcornUpgrade {
    pub max_multiplier: FixedRatio,
    pub duration: usize,
    pub waves_remaining: usize,
    pub active_stage_damage_bonus: FixedRatio,
}

impl UpgradePresentation for PopcornUpgrade {
    fn key(&self) -> &'static str {
        "popcorn"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::POPCORN)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        if self.active_stage_damage_bonus.is_zero() {
            return Vec::new();
        }
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("{:.0}%", self.active_stage_damage_bonus.as_f32() * 100.0),
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
            crate::l10n::locale::Language::English => "Popcorn",
            crate::l10n::locale::Language::Korean => "팝콘",
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
                    .static_text("For ")
                    .text(self.duration.to_string())
                    .static_text(" stages, all tower")
                    .with_bold(format!(
                        "damage +{:.0}%",
                        self.max_multiplier.as_f32() * 100.0
                    ))
                    .static_text(", decreasing each stage");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .text(self.duration.to_string())
                    .static_text("스테이지 동안 모든 타워 ")
                    .with_bold(format!(
                        "데미지 +{:.0}%",
                        self.max_multiplier.as_f32() * 100.0
                    ))
                    .static_text(", 스테이지가 지날수록 감소합니다");
            }
        }
    }
}

impl PopcornUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade(
        max_multiplier: FixedRatio,
        duration: usize,
        waves_remaining: usize,
    ) -> Upgrade {
        Upgrade::Popcorn(PopcornUpgrade {
            max_multiplier,
            duration,
            waves_remaining,
            active_stage_damage_bonus: FixedRatio::ZERO,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    #[test]
    fn popcorn_effect_decrements_over_waves_and_expires() {
        use crate::game_state::GameFlow;
        use crate::game_state::flow::DefenseFlow;
        use crate::game_state::tower::{Tower, TowerTemplate};
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::PopcornUpgrade::into_upgrade(
                crate::FixedRatio::from_integer(5),
                5,
                5,
            ),
            None,
        ));
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::StartStage {
            stage: 1,
        });

        game_state.flow = GameFlow::Defense(DefenseFlow::new(&game_state));
        let tower_template = TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Hearts,
            Rank::Two,
        );
        let tower = Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.sim_tick(),
        );
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::PlaceTower(
            Box::new(tower),
            None,
        ));

        let expected_multipliers = [5.0, 4.0, 3.0, 2.0, 1.0, 1.0];
        for expected_multiplier in expected_multipliers {
            let tower = game_state
                .towers
                .iter()
                .next()
                .expect("expected tower still present");
            support::assert_tower_cached_damage_mul(tower, expected_multiplier);

            if expected_multiplier > 1.0 {
                game_state.flow = GameFlow::Defense(DefenseFlow::new(&game_state));
                support::check_defense_end_for_test(&mut game_state);
            }
        }
    }
}
