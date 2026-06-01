use crate::game_state::item::Item;
use crate::game_state::upgrade::Upgrade;
use crate::l10n;

pub enum ShopItemTitle<'a> {
    Item {
        item: &'a Item,
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
            ShopItemTitle::Item { item, locale } => {
                format!("{:?}:{:?}", locale.language, item.discriminant())
            }
            ShopItemTitle::Upgrade { upgrade, locale } => {
                format!("{:?}:{:?}", locale.language, upgrade)
            }
        }
    }
}
