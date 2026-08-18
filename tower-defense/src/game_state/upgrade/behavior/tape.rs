use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const TAPE_WAVE_INTERVAL: usize = 4;
const TAPE_ENEMY_SPEED_MULTIPLIER: f32 = 0.75;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TapeUpgrade {
    pub acquired_stage: usize,
}

impl UpgradeBehavior for TapeUpgrade {
    fn key(&self) -> &'static str {
        "tape"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::TAPE)
    }

    fn thumbnail_overlays(
        &self,
        game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        let cycle = self.cycle(game_state.stage);
        let active = cycle == TAPE_WAVE_INTERVAL;
        let stage_color = if active {
            crate::theme::palette::WHITE
        } else {
            crate::theme::palette::DISABLED_TEXT
        };
        vec![
            crate::thumbnail::ThumbnailOverlay::right_top(
                format!("{}/{}", cycle, TAPE_WAVE_INTERVAL),
                stage_color,
            ),
            crate::thumbnail::ThumbnailOverlay::right_bottom(
                format!("{}%", (1.0 - TAPE_ENEMY_SPEED_MULTIPLIER) * 100.0),
                crate::theme::palette::BLUE,
            ),
        ]
    }

    fn acquire(mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.acquired_stage = game_state.stage;
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::REVISION
    }

    fn on_stage_start(&mut self, game_state: &mut GameState, stage: usize) -> UpgradeUpdateFlags {
        let cycle = self.cycle(stage);
        let active = cycle == TAPE_WAVE_INTERVAL;

        if active {
            game_state
                .stage_modifiers
                .apply_enemy_speed_multiplier(TAPE_ENEMY_SPEED_MULTIPLIER);
        }
        UpgradeUpdateFlags::NONE
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Tape",
            crate::l10n::locale::Language::Korean => "테이프",
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
                    .static_text("Slow enemies by ")
                    .with_bold(format!(
                        "-{:.0}%",
                        (1.0 - TAPE_ENEMY_SPEED_MULTIPLIER) * 100.0
                    ))
                    .static_text(" every ")
                    .text(TAPE_WAVE_INTERVAL.to_string())
                    .static_text(" stages");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("매 ")
                    .text(TAPE_WAVE_INTERVAL.to_string())
                    .static_text("스테이지마다 적 ")
                    .with_bold(format!(
                        "이동속도 -{:.0}%",
                        (1.0 - TAPE_ENEMY_SPEED_MULTIPLIER) * 100.0
                    ));
            }
        }
    }
}

impl TapeUpgrade {
    pub fn into_upgrade(acquired_stage: usize) -> Upgrade {
        Upgrade::Tape(TapeUpgrade { acquired_stage })
    }

    fn cycle(&self, stage: usize) -> usize {
        (stage - self.acquired_stage) % TAPE_WAVE_INTERVAL + 1
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_epic,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    TapeUpgrade::into_upgrade(0)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tape_applies_enemy_speed_reduction_every_four_waves() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 3 });
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            TapeUpgrade::into_upgrade(0),
            None,
        ));
        assert_eq!(game_state.stage_modifiers.get_enemy_speed_multiplier(), 1.0);

        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 4 });
        assert_eq!(game_state.stage_modifiers.get_enemy_speed_multiplier(), 1.0);

        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 5 });
        assert_eq!(game_state.stage_modifiers.get_enemy_speed_multiplier(), 1.0);

        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 6 });
        assert_eq!(
            game_state.stage_modifiers.get_enemy_speed_multiplier(),
            TAPE_ENEMY_SPEED_MULTIPLIER
        );
    }
}
