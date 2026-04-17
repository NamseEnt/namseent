use crate::game_state::item::Item;
use crate::game_state::upgrade::UpgradeKind;
use crate::l10n;

pub enum ShopItemDescription<'a> {
    Item {
        item: &'a Item,
        locale: l10n::Locale,
    },
    Upgrade {
        upgrade_kind: &'a UpgradeKind,
        locale: l10n::Locale,
    },
}

impl<'a> ShopItemDescription<'a> {
    pub(crate) fn key(&self) -> String {
        match self {
            ShopItemDescription::Item { item, locale } => {
                format!("{:?}:{:?}", locale.language, item)
            }
            ShopItemDescription::Upgrade {
                upgrade_kind,
                locale,
            } => {
                format!("{:?}:{:?}", locale.language, upgrade_kind)
            }
        }
    }
}
