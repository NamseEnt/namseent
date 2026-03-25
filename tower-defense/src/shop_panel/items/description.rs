use crate::game_state::upgrade::UpgradeKind;
use crate::l10n;

pub enum ShopItemDescription<'a> {
    Effect {
        effect: crate::game_state::item::Effect,
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
            ShopItemDescription::Effect { effect, locale } => {
                format!("{:?}:{:?}", locale.language, effect)
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
