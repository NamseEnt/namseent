use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const DICE_BONUS: usize = 2;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MetronomeUpgrade {
    acquired_stage: usize,
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
        let cycle = self.cycle(game_state.stage);
        let active = cycle == 2;
        let stage_color = if active {
            crate::theme::palette::WHITE
        } else {
            crate::theme::palette::DISABLED_TEXT
        };

        Some(render([
            crate::thumbnail::render_right_top_overlay(
                width_height.width,
                &format!("{}/2", cycle),
                stage_color,
            ),
            crate::thumbnail::render_right_bottom_overlay(
                width_height,
                &format!("{}", DICE_BONUS),
                crate::theme::palette::BLUE,
            ),
        ]))
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
        let active = cycle == 2;
        if active {
            game_state.left_dice += DICE_BONUS;
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
                .with_dice_text("Dice +2")
                .static_text(" every 2 stages"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("2 스테이지마다")
                .with_dice_text("주사위 +2"),
        };
    }
}

impl MetronomeUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Metronome(MetronomeUpgrade { acquired_stage: 0 })
    }

    fn cycle(&self, stage: usize) -> usize {
        (stage - self.acquired_stage) % 2 + 1
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    MetronomeUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::upgrade::UpgradeBehavior;

    #[test]
    fn metronome_grants_extra_dice_every_two_waves() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let mut upgrade = MetronomeUpgrade { acquired_stage: 1 };

        game_state.left_dice = game_state.max_dice_chance();
        upgrade.on_stage_start(&mut game_state, 1);
        assert_eq!(game_state.left_dice, game_state.max_dice_chance());

        game_state.left_dice = game_state.max_dice_chance();
        upgrade.on_stage_start(&mut game_state, 2);
        assert_eq!(
            game_state.left_dice,
            game_state.max_dice_chance() + DICE_BONUS
        );

        game_state.left_dice = game_state.max_dice_chance();
        upgrade.on_stage_start(&mut game_state, 3);
        assert_eq!(game_state.left_dice, game_state.max_dice_chance());
    }
}
