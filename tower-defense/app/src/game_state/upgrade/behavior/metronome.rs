use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

const DICE_BONUS: usize = 2;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MetronomeUpgrade {
    pub(crate) acquired_stage: usize,
}

impl UpgradePresentation for MetronomeUpgrade {
    fn key(&self) -> &'static str {
        "metronome"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::METRONOME)
    }

    fn thumbnail_overlays(
        &self,
        game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        let cycle = self.cycle(game_state.raw_core_state().progress().stage);
        let active = cycle == 2;
        let stage_color = if active {
            crate::theme::palette::WHITE
        } else {
            crate::theme::palette::DISABLED_TEXT
        };
        vec![
            crate::thumbnail::ThumbnailOverlay::right_top(format!("{}/2", cycle), stage_color),
            crate::thumbnail::ThumbnailOverlay::right_bottom(
                format!("{}", DICE_BONUS),
                crate::theme::palette::BLUE,
            ),
        ]
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
                .l10n(Word::Dice.name(), locale)
                .with_bold(" +2")
                .static_text(" every 2 stages"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("2 스테이지마다")
                .l10n(Word::Dice.name(), locale)
                .with_bold(" +2"),
        };
    }
}

impl MetronomeUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Metronome(MetronomeUpgrade { acquired_stage: 0 })
    }

    fn cycle(&self, stage: usize) -> usize {
        (stage - self.acquired_stage) % 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metronome_grants_extra_dice_every_two_waves() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            MetronomeUpgrade::into_upgrade(),
            None,
        ));

        for stage in [1, 2, 3] {
            let mut raw = game_state.raw_core_state().clone();
            raw.edit_snapshot(|parts| {
                parts.progress.left_dice = parts.config.player.base_dice_chance;
            })
            .expect("valid dice count");
            raw.trigger_stage_start_upgrades(stage);
            game_state
                .restore_raw_core_projection(raw)
                .expect("valid metronome projection");

            let expected = if stage == 2 {
                game_state.max_dice_chance() + DICE_BONUS
            } else {
                game_state.max_dice_chance()
            };
            assert_eq!(game_state.raw_core_state().progress().left_dice, expected);
        }
    }
}
