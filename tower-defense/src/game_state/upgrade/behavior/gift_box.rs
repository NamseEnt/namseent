use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct GiftBoxUpgrade;

impl UpgradeBehavior for GiftBoxUpgrade {
    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        _gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        let bonus_gold = item_count * 10;
        (bonus_gold, UpgradeUpdateFlags::RESOURCE)
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Gift Box",
            crate::l10n::locale::Language::Korean => "선물 상자",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Earn 10 gold per item at the end of each stage",
            crate::l10n::locale::Language::Korean => "각 아이템마다 스테이지 종료 시 10골드를 얻습니다",
        });
    }
}

impl GiftBoxUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::GiftBox(GiftBoxUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    GiftBoxUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn gift_box_awards_gold_per_item_on_stage_end() {
        use crate::game_state::upgrade::tests::support;
        use crate::game_state::item::ItemKind;

        let mut gs = support::create_mock_game_state();
        gs.flow = crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
        gs.items = vec![
            crate::game_state::item::Item {
                kind: ItemKind::LumpSugar,
                effect: crate::game_state::item::Effect::ExtraDice,
            },
            crate::game_state::item::Item {
                kind: ItemKind::LumpSugar,
                effect: crate::game_state::item::Effect::ExtraDice,
            },
        ];
        gs.upgrade_state
            .upgrade(crate::game_state::upgrade::GiftBoxUpgrade::into_upgrade());

        crate::game_state::tick::defense_end::check_defense_end(&mut gs);

        assert_eq!(gs.gold, gs.config.player.starting_gold + 20);
    }
}

