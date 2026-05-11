use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PopcornUpgrade {
    pub max_multiplier: f32,
    pub duration: usize,
    pub waves_remaining: usize,
    pub active_stage_damage_bonus: f32,
}

impl UpgradeBehavior for PopcornUpgrade {
    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.active_stage_damage_bonus > 0.0 {
            Some((TowerUpgradeTarget::Global, self.active_stage_damage_bonus))
        } else {
            None
        }
    }

    fn on_stage_start(&mut self, _game_state: &mut GameState, _stage: usize) -> UpgradeUpdateFlags {
        self.active_stage_damage_bonus = 0.0;
        if self.waves_remaining > 0 {
            let duration = self.duration.max(1);
            let elapsed = duration.saturating_sub(self.waves_remaining);
            let popcorn_multiplier = if duration <= 1 {
                self.max_multiplier
            } else {
                let step = (self.max_multiplier - 1.0) / (duration - 1) as f32;
                (self.max_multiplier - step * elapsed as f32).max(1.0)
            };

            self.active_stage_damage_bonus = popcorn_multiplier - 1.0;
            self.waves_remaining -= 1;
            UpgradeUpdateFlags::TOWER_STATS
        } else {
            UpgradeUpdateFlags::NONE
        }
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
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => format!(
                "Damage lasts for {} waves with a max multiplier of {:.0}%, decreasing each wave",
                self.duration,
                self.max_multiplier * 100.0,
            ),
            crate::l10n::locale::Language::Korean => format!(
                "{}웨이브 동안 데미지 최대 {:.0}%까지 증가하며, 웨이브가 지날수록 증가치가 감소합니다",
                self.duration,
                self.max_multiplier * 100.0,
            ),
        });
    }
}

impl PopcornUpgrade {
    pub fn into_upgrade(max_multiplier: f32, duration: usize, waves_remaining: usize) -> Upgrade {
        Upgrade::Popcorn(PopcornUpgrade {
            max_multiplier,
            duration,
            waves_remaining,
            active_stage_damage_bonus: 0.0,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PopcornUpgrade::into_upgrade(5.0, 5, 5)
}
#[cfg(test)]
mod tests {

    #[test]
    fn popcorn_effect_decrements_over_waves_and_expires() {
        use crate::game_state::GameFlow;
        use crate::game_state::flow::DefenseFlow;
        use crate::game_state::tower::{Tower, TowerTemplate};
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::PopcornUpgrade::into_upgrade(5.0, 5, 5),
            None,
        ));
        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 1 });

        game_state.flow = GameFlow::Defense(DefenseFlow::new(&game_state));
        let tower_template = TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Hearts,
            crate::card::Rank::Two,
        );
        let tower = Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.action(crate::game_state::GameStateAction::PlaceTower(Box::new(
            tower,
        )));

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
                crate::game_state::tick::defense_end::check_defense_end(&mut game_state);
            }
        }
    }
}
