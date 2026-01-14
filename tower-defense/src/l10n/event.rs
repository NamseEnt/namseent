use super::{Language, Locale, LocalizedText};
use crate::game_state::play_history::HistoryEventType;
use crate::l10n::effect::EffectText;
use crate::l10n::upgrade::UpgradeKindText;

#[derive(Debug, Clone)]
pub enum EventText<'a> {
    Description(&'a HistoryEventType, &'a Locale),
}

impl LocalizedText for EventText<'_> {
    fn localized_text(&self, locale: &Locale) -> String {
        match self {
            EventText::Description(event_type, _) => match locale.language {
                Language::Korean => event_type.description_korean(locale),
                Language::English => event_type.description_english(locale),
            },
        }
    }
}

impl HistoryEventType {
    pub fn description_korean(&self, locale: &Locale) -> String {
        match self {
            HistoryEventType::TowerPlaced {
                tower_kind,
                rank,
                suit,
                ..
            } => {
                format!("타워 배치: {:?} {} {}", tower_kind, rank, suit)
            }
            HistoryEventType::DamageTaken { amount } => {
                format!("데미지 피격: {:.0}", amount)
            }
            HistoryEventType::ItemPurchased { item, cost } => {
                let item_name = EffectText::Name(item.effect.clone()).localized_text(locale);
                format!("아이템 구매: {} ({}G)", item_name, cost)
            }
            HistoryEventType::ItemUsed { item_effect } => {
                let effect_name = EffectText::Name(item_effect.clone()).localized_text(locale);
                format!("아이템 사용: {}", effect_name)
            }
            HistoryEventType::UpgradeSelected { upgrade } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                format!("업그레이드 선택: {}", upgrade_name)
            }
            HistoryEventType::UpgradePurchased { upgrade, cost } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                format!("업그레이드 구매: {} ({}G)", upgrade_name, cost)
            }
            HistoryEventType::ContractPurchased { contract, cost } => {
                format!("계약 구매: {:?} ({}G)", contract, cost)
            }
        }
    }

    pub fn description_english(&self, locale: &Locale) -> String {
        match self {
            HistoryEventType::TowerPlaced {
                tower_kind,
                rank,
                suit,
                ..
            } => {
                format!("Tower Placed: {:?} {} {}", tower_kind, rank, suit)
            }
            HistoryEventType::DamageTaken { amount } => {
                format!("Damage Taken: {:.0}", amount)
            }
            HistoryEventType::ItemPurchased { item, cost } => {
                let item_name = EffectText::Name(item.effect.clone()).localized_text(locale);
                format!("Item Purchased: {} ({}G)", item_name, cost)
            }
            HistoryEventType::ItemUsed { item_effect } => {
                let effect_name = EffectText::Name(item_effect.clone()).localized_text(locale);
                format!("Item Used: {}", effect_name)
            }
            HistoryEventType::UpgradeSelected { upgrade } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                format!("Upgrade Selected: {}", upgrade_name)
            }
            HistoryEventType::UpgradePurchased { upgrade, cost } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                format!("Upgrade Purchased: {} ({}G)", upgrade_name, cost)
            }
            HistoryEventType::ContractPurchased { contract, cost } => {
                format!("Contract Purchased: {:?} ({}G)", contract, cost)
            }
        }
    }
}
