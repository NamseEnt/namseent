use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MetronomeUpgrade {
    pub start_stage: Option<usize>,
}

impl UpgradeBehavior for MetronomeUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::METRONOME,
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
        let cycle = if let Some(start) = self.start_stage {
            if game_state.stage <= start {
                1
            } else {
                ((game_state.stage - start) % 2) + 1
            }
        } else {
            1
        };
        let active = self.start_stage.is_some_and(|start| {
            game_state.stage >= start && (game_state.stage - start).is_multiple_of(2)
        });
        let color = if active {
            crate::theme::palette::WHITE
        } else {
            crate::theme::palette::DISABLED_TEXT
        };

        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &format!("{}/2", cycle),
            color,
        ))
    }

    fn on_stage_start(&mut self, _game_state: &mut GameState, stage: usize) -> UpgradeUpdateFlags {
        let start = self.start_stage.get_or_insert(stage);
        if stage >= *start && (stage - *start).is_multiple_of(2) {
            _game_state.left_dice += 1;
        }
        UpgradeUpdateFlags::NONE
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Metronome",
            crate::l10n::locale::Language::Korean => "메트로놈",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Gain ")
                .with_dice_text("1 extra dice")
                .static_text(" every 2 stages"),
            crate::l10n::locale::Language::Korean => builder
                .with_dice_text("주사위 +1")
                .static_text("을 얻습니다"),
        };
    }
}

impl MetronomeUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Metronome(MetronomeUpgrade { start_stage: None })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    MetronomeUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::upgrade::{UpgradeBehavior, UpgradeUpdateFlags};

    #[test]
    fn metronome_grants_extra_dice_every_two_waves() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let mut upgrade = MetronomeUpgrade { start_stage: None };

        game_state.left_dice = game_state.max_dice_chance();
        upgrade.on_stage_start(&mut game_state, 1);
        assert_eq!(game_state.left_dice, game_state.max_dice_chance() + 1);

        game_state.left_dice = game_state.max_dice_chance();
        upgrade.on_stage_start(&mut game_state, 2);
        assert_eq!(game_state.left_dice, game_state.max_dice_chance());

        game_state.left_dice = game_state.max_dice_chance();
        upgrade.on_stage_start(&mut game_state, 3);
        assert_eq!(game_state.left_dice, game_state.max_dice_chance() + 1);
    }

    #[test]
    fn metronome_returns_revision_required_only_on_first_stage_start() {
        let mut upgrade = MetronomeUpgrade { start_stage: None };
        use crate::game_state::upgrade::tests::support;
        let mut game_state = support::create_mock_game_state();

        let first_flags = upgrade.on_stage_start(&mut game_state, 1);
        let second_flags = upgrade.on_stage_start(&mut game_state, 2);

        assert_eq!(first_flags, UpgradeUpdateFlags::NONE);
        assert_eq!(second_flags, UpgradeUpdateFlags::NONE);
    }
}
