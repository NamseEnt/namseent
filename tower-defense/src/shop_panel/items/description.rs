use crate::game_state::item::ItemKind;
use crate::game_state::upgrade::UpgradeKind;
use crate::l10n;

pub enum ShopItemDescription<'a> {
    Item {
        item_kind: &'a ItemKind,
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
            ShopItemDescription::Item { item_kind, locale } => {
                format!("{:?}:{:?}", locale.language, item_kind)
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
