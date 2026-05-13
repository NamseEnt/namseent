use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DiceBundleUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for DiceBundleUpgrade {
    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::DiceBundle(upgrade) = upgrade {
                upgrade.add += self.add;
                return UpgradeUpdateFlags::NONE;
            }
        }

        game_state.upgrade_state.upgrades.push(self.into());
        UpgradeUpdateFlags::NONE
    }

    fn dice_chance_plus(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Dice Bundle",
            crate::l10n::locale::Language::Korean => "주사위 꾸러미",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Dice ")
                .with_icon_bold(crate::icon::IconKind::Refresh, format!("+{}", self.add)),
            crate::l10n::locale::Language::Korean => {
                builder.with_icon_bold(crate::icon::IconKind::Refresh, "+1")
            }
        };
    }
}

impl DiceBundleUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::DiceBundle(DiceBundleUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    DiceBundleUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.dice_chance_plus(),
        super::MAX_DICE_CHANCE_PLUS,
    ))
}
