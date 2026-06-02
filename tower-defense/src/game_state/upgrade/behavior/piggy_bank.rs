use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const PIGGY_BANK_GOLD_STEP: usize = 500;
const PIGGY_BANK_GOLD_REWARD_PER_STEP: usize = 50;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PiggyBankUpgrade;

impl UpgradeBehavior for PiggyBankUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::PIGGY_BANK,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn on_stage_end(
        &mut self,
        game_state: &mut GameState,
        _perfect_clear: bool,
        gold: usize,
        _item_count: usize,
    ) -> UpgradeUpdateFlags {
        let bonus_gold = if gold >= PIGGY_BANK_GOLD_STEP {
            gold / PIGGY_BANK_GOLD_STEP * PIGGY_BANK_GOLD_REWARD_PER_STEP
        } else {
            0
        };
        if bonus_gold > 0 {
            game_state.action(crate::game_state::GameStateAction::EarnGold(bonus_gold));
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
            crate::l10n::locale::Language::English => "Piggy Bank",
            crate::l10n::locale::Language::Korean => "돼지저금통",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("At stage end, gain ")
                .with_gold_value(format!("{}", PIGGY_BANK_GOLD_REWARD_PER_STEP))
                .static_text(" gold for every ")
                .with_gold_value(format!("{}", PIGGY_BANK_GOLD_STEP))
                .static_text(" gold you hold"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("스테이지 종료 시 보유한 골드 ")
                .with_gold_value(format!("{}골드", PIGGY_BANK_GOLD_STEP))
                .static_text("당 ")
                .with_gold_value(format!("{}골드", PIGGY_BANK_GOLD_REWARD_PER_STEP))
                .static_text("를 획득합니다"),
        };
    }
}

impl PiggyBankUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::PiggyBank(PiggyBankUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PiggyBankUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn piggy_bank_awards_gold_on_stage_end_with_enough_gold() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.flow =
            crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
        gs.gold = 500;
        gs.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::PiggyBankUpgrade::into_upgrade(),
            None,
        ));

        crate::game_state::tick::defense_end::check_defense_end(&mut gs);

        assert_eq!(gs.gold, 550);
    }
}
