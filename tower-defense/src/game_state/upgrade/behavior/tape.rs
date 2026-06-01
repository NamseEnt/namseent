use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const TAPE_WAVE_INTERVAL: usize = 4;
const TAPE_ENEMY_SPEED_MULTIPLIER: f32 = 0.75;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TapeUpgrade {
    pub acquired_stage: usize,
}

impl UpgradeBehavior for TapeUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::TAPE,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn thumbnail_overlay(
        &self,
        width_height: Wh<Px>,
        game_state: &GameState,
    ) -> Option<RenderingTree> {
        let cycle = if game_state.stage <= self.acquired_stage {
            1
        } else {
            ((game_state.stage - self.acquired_stage - 1) % 4) + 1
        };
        let active = cycle == 4;
        let color = if active {
            crate::theme::palette::WHITE
        } else {
            crate::theme::palette::DISABLED_TEXT
        };

        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &format!("{}/4", cycle),
            color,
        ))
    }

    fn acquire(mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.acquired_stage = game_state.stage;
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::NONE
    }

    fn on_stage_start(&mut self, game_state: &mut GameState, stage: usize) -> UpgradeUpdateFlags {
        if stage > self.acquired_stage
            && (stage - self.acquired_stage - 1).is_multiple_of(TAPE_WAVE_INTERVAL)
        {
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
                    .with_movement_speed_debuff_value(format!(
                        "{:.0}%",
                        (1.0 - TAPE_ENEMY_SPEED_MULTIPLIER) * 100.0
                    ))
                    .static_text(" every ")
                    .text(TAPE_WAVE_INTERVAL.to_string())
                    .static_text(" waves after acquisition");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("획득 후 매 ")
                    .text(TAPE_WAVE_INTERVAL.to_string())
                    .static_text("웨이브마다 적의 이동속도가 ")
                    .with_movement_speed_debuff_value(format!(
                        "{:.0}%",
                        (1.0 - TAPE_ENEMY_SPEED_MULTIPLIER) * 100.0
                    ))
                    .static_text(" 느려집니다");
            }
        }
    }
}

impl TapeUpgrade {
    pub fn into_upgrade(acquired_stage: usize) -> Upgrade {
        Upgrade::Tape(TapeUpgrade { acquired_stage })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    TapeUpgrade::into_upgrade(0)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::upgrade::UpgradeBehavior;

    #[test]
    fn tape_applies_enemy_speed_reduction_every_four_waves() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.stage = 3;
        let mut upgrade = TapeUpgrade { acquired_stage: 3 };

        upgrade.on_stage_start(&mut game_state, 3);
        assert_eq!(game_state.stage_modifiers.get_enemy_speed_multiplier(), 1.0);

        upgrade.on_stage_start(&mut game_state, 4);
        assert_eq!(
            game_state.stage_modifiers.get_enemy_speed_multiplier(),
            0.75
        );

        game_state.stage_modifiers = crate::game_state::StageModifiers::default();
        upgrade.on_stage_start(&mut game_state, 5);
        assert_eq!(game_state.stage_modifiers.get_enemy_speed_multiplier(), 1.0);

        game_state.stage_modifiers = crate::game_state::StageModifiers::default();
        upgrade.on_stage_start(&mut game_state, 8);
        assert_eq!(
            game_state.stage_modifiers.get_enemy_speed_multiplier(),
            0.75
        );
    }
}
