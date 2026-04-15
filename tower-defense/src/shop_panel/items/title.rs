use crate::game_state::item::ItemKind;
use crate::game_state::upgrade::UpgradeKind;
use crate::l10n;

pub enum ShopItemTitle {
    Item {
        item_kind: ItemKind,
        locale: l10n::Locale,
    },
    Upgrade {
        upgrade_kind: UpgradeKind,
        locale: l10n::Locale,
    },
}

impl ShopItemTitle {
    pub(crate) fn key(&self) -> String {
        match self {
            ShopItemTitle::Item { item_kind, locale } => {
                format!("{:?}:{:?}", locale.language, item_kind)
            }
            ShopItemTitle::Upgrade {
                upgrade_kind,
                locale,
            } => {
                format!("{:?}:{:?}", locale.language, upgrade_kind)
            }
        }
    }
}
