use crate::game_state::item::Effect;
use crate::game_state::upgrade::UpgradeKind;
use crate::l10n;

pub enum ShopItemTitle {
    Effect {
        effect: Effect,
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
            ShopItemTitle::Effect { effect, locale } => {
                format!("{:?}:{:?}", locale.language, effect)
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
