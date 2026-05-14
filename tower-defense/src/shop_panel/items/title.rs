use crate::game_state::item::ItemKind;
use crate::game_state::upgrade::Upgrade;
use crate::l10n;

pub enum ShopItemTitle<'a> {
    Item {
        item_kind: ItemKind,
        locale: l10n::Locale,
    },
    Upgrade {
        upgrade: &'a Upgrade,
        locale: l10n::Locale,
    },
}

impl<'a> ShopItemTitle<'a> {
    pub(crate) fn key(&self) -> String {
        match self {
            ShopItemTitle::Item { item_kind, locale } => {
                format!("{:?}:{:?}", locale.language, item_kind)
            }
            ShopItemTitle::Upgrade { upgrade, locale } => {
                format!("{:?}:{:?}", locale.language, upgrade)
            }
        }
    }
}
