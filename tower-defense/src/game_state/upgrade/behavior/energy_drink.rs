use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EnergyDrinkUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for EnergyDrinkUpgrade {
    fn shop_item_price_minus(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Energy Drink",
            crate::l10n::locale::Language::Korean => "에너지드링크",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Shop price ")
                .with_icon_bold(crate::icon::IconKind::Gold, format!("-{}", self.add)),
            crate::l10n::locale::Language::Korean => builder
                .static_text("상점 가격 ")
                .with_icon_bold(crate::icon::IconKind::Gold, format!("-{}", self.add))
                .static_text(" 할인"),
        };
    }
}

impl EnergyDrinkUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::EnergyDrink(EnergyDrinkUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    EnergyDrinkUpgrade::into_upgrade(5)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.shop_item_price_minus(),
        super::MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
    ))
}

