use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SlotMachineUpgrade {
    pub next_round_dice: usize,
}

impl UpgradeBehavior for SlotMachineUpgrade {
    fn key(&self) -> &'static str {
        "slot_machine"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::SLOT_MACHINE,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn on_stage_start(&mut self, game_state: &mut GameState, _stage: usize) -> UpgradeUpdateFlags {
        if self.next_round_dice > 0 {
            game_state.left_dice += self.next_round_dice;
            self.next_round_dice = 0;
            UpgradeUpdateFlags::NONE
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
            crate::l10n::locale::Language::English => "Slot Machine",
            crate::l10n::locale::Language::Korean => "슬롯머신",
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
                .with_dice_value(format!(" +{}", self.next_round_dice))
                .static_text(" next stage"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("다음 스테이지 ")
                .l10n(Word::Dice.name(), locale)
                .with_dice_value(format!(" +{}", self.next_round_dice)),
        };
    }
}

impl SlotMachineUpgrade {
    pub fn into_upgrade(next_round_dice: usize) -> Upgrade {
        Upgrade::SlotMachine(SlotMachineUpgrade { next_round_dice })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_rare,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    SlotMachineUpgrade::into_upgrade(10)
}
#[cfg(test)]
mod tests {

    #[test]
    fn slot_machine_grants_extra_dice_on_stage_start_only_once() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::SlotMachineUpgrade::into_upgrade(10),
            None,
        ));

        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 1 });
        assert_eq!(
            game_state.left_dice,
            game_state.max_dice_chance() + 10,
            "slot machine should add extra dice on the first stage start",
        );

        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 2 });
        assert_eq!(
            game_state.left_dice,
            game_state.max_dice_chance(),
            "slot machine should only apply extra dice once",
        );
    }
}
