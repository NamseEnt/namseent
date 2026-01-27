use super::{Language, Locale, LocalizedRichText, LocalizedText};
use crate::game_state::play_history::HistoryEventType;
use crate::l10n::contract::ContractText;
use crate::l10n::effect::EffectText;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

#[derive(Debug, Clone)]
pub enum EventText<'a> {
    Description(&'a HistoryEventType, &'a Locale),
}

impl LocalizedText for EventText<'_> {
    fn localized_text(&self, locale: &Locale) -> String {
        match self {
            EventText::Description(event_type, _) => match locale.language {
                Language::Korean => match event_type {
                    HistoryEventType::GameStart => "게임 시작".to_string(),
                    HistoryEventType::StageStart { stage } => {
                        format!("스테이지 {} 시작", stage)
                    }
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
                        let item_name =
                            EffectText::Name(item.effect.clone()).localized_text(locale);
                        format!("아이템 구매: {} ({}G)", item_name, cost)
                    }
                    HistoryEventType::ItemUsed { item_effect } => {
                        let effect_name =
                            EffectText::Name(item_effect.clone()).localized_text(locale);
                        format!("아이템 사용: {}", effect_name)
                    }
                    HistoryEventType::UpgradeSelected { upgrade } => {
                        let upgrade_name =
                            UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                        format!("업그레이드 선택: {}", upgrade_name)
                    }
                    HistoryEventType::UpgradePurchased { upgrade, cost } => {
                        let upgrade_name =
                            UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                        format!("업그레이드 구매: {} ({}G)", upgrade_name, cost)
                    }
                    HistoryEventType::ContractPurchased { contract, cost } => {
                        let risk_text = ContractText::Risk(&contract.risk).localized_text(locale);
                        let reward_text =
                            ContractText::Reward(&contract.reward).localized_text(locale);
                        format!("계약 구매: {} / {} ({}G)", risk_text, reward_text, cost)
                    }
                    HistoryEventType::GameOver => "게임 오버".to_string(),
                },
                Language::English => match event_type {
                    HistoryEventType::GameStart => "Game Started".to_string(),
                    HistoryEventType::StageStart { stage } => {
                        format!("Stage {} Started", stage)
                    }
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
                        let item_name =
                            EffectText::Name(item.effect.clone()).localized_text(locale);
                        format!("Item Purchased: {} ({}G)", item_name, cost)
                    }
                    HistoryEventType::ItemUsed { item_effect } => {
                        let effect_name =
                            EffectText::Name(item_effect.clone()).localized_text(locale);
                        format!("Item Used: {}", effect_name)
                    }
                    HistoryEventType::UpgradeSelected { upgrade } => {
                        let upgrade_name =
                            UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                        format!("Upgrade Selected: {}", upgrade_name)
                    }
                    HistoryEventType::UpgradePurchased { upgrade, cost } => {
                        let upgrade_name =
                            UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                        format!("Upgrade Purchased: {} ({}G)", upgrade_name, cost)
                    }
                    HistoryEventType::ContractPurchased { contract, cost } => {
                        let risk_text = ContractText::Risk(&contract.risk).localized_text(locale);
                        let reward_text =
                            ContractText::Reward(&contract.reward).localized_text(locale);
                        format!(
                            "Contract Purchased: {} / {} ({}G)",
                            risk_text, reward_text, cost
                        )
                    }
                    HistoryEventType::GameOver => "Game Over".to_string(),
                },
            },
        }
    }
}

impl LocalizedRichText for EventText<'_> {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match self {
            EventText::Description(event_type, _) => match locale.language {
                Language::Korean => event_type.description_korean_builder(builder, locale),
                Language::English => event_type.description_english_builder(builder, locale),
            },
        }
    }
}

impl HistoryEventType {
    fn description_korean_builder<'a>(
        &self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match self {
            HistoryEventType::GameStart => builder.static_text("게임 시작"),
            HistoryEventType::StageStart { stage } => builder
                .static_text("스테이지 ")
                .text(format!("{}", stage))
                .static_text(" 시작"),
            HistoryEventType::TowerPlaced {
                tower_kind,
                rank,
                suit,
                ..
            } => builder
                .static_text("타워 배치: ")
                .text(format!("{:?} {} {}", tower_kind, rank, suit)),
            HistoryEventType::DamageTaken { amount } => builder
                .static_text("데미지 피격: ")
                .text(format!("{:.0}", amount)),
            HistoryEventType::ItemPurchased { item, cost } => {
                let item_name = EffectText::Name(item.effect.clone()).localized_text(locale);
                builder
                    .static_text("아이템 구매: ")
                    .text(item_name)
                    .static_text(" (")
                    .text(format!("{}G", cost))
                    .static_text(")")
            }
            HistoryEventType::ItemUsed { item_effect } => {
                let effect_name = EffectText::Name(item_effect.clone()).localized_text(locale);
                builder.static_text("아이템 사용: ").text(effect_name)
            }
            HistoryEventType::UpgradeSelected { upgrade } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                builder.static_text("업그레이드 선택: ").text(upgrade_name)
            }
            HistoryEventType::UpgradePurchased { upgrade, cost } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                builder
                    .static_text("업그레이드 구매: ")
                    .text(upgrade_name)
                    .static_text(" (")
                    .text(format!("{}G", cost))
                    .static_text(")")
            }
            HistoryEventType::ContractPurchased { contract, cost } => {
                let risk_text = ContractText::Risk(&contract.risk).localized_text(locale);
                let reward_text = ContractText::Reward(&contract.reward).localized_text(locale);
                builder
                    .static_text("계약 구매: ")
                    .text(risk_text)
                    .static_text(" / ")
                    .text(reward_text)
                    .static_text(" (")
                    .text(format!("{}G", cost))
                    .static_text(")")
            }
            HistoryEventType::GameOver => builder.static_text("게임 오버"),
        }
    }

    fn description_english_builder<'a>(
        &self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match self {
            HistoryEventType::GameStart => builder.static_text("Game Started"),
            HistoryEventType::StageStart { stage } => builder
                .static_text("Stage ")
                .text(format!("{}", stage))
                .static_text(" Started"),
            HistoryEventType::TowerPlaced {
                tower_kind,
                rank,
                suit,
                ..
            } => builder
                .static_text("Tower Placed: ")
                .text(format!("{:?} {} {}", tower_kind, rank, suit)),
            HistoryEventType::DamageTaken { amount } => builder
                .static_text("Damage Taken: ")
                .text(format!("{:.0}", amount)),
            HistoryEventType::ItemPurchased { item, cost } => {
                let item_name = EffectText::Name(item.effect.clone()).localized_text(locale);
                builder
                    .static_text("Item Purchased: ")
                    .text(item_name)
                    .static_text(" (")
                    .text(format!("{}G", cost))
                    .static_text(")")
            }
            HistoryEventType::ItemUsed { item_effect } => {
                let effect_name = EffectText::Name(item_effect.clone()).localized_text(locale);
                builder.static_text("Item Used: ").text(effect_name)
            }
            HistoryEventType::UpgradeSelected { upgrade } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                builder.static_text("Upgrade Selected: ").text(upgrade_name)
            }
            HistoryEventType::UpgradePurchased { upgrade, cost } => {
                let upgrade_name = UpgradeKindText::Name(&upgrade.kind).localized_text(locale);
                builder
                    .static_text("Upgrade Purchased: ")
                    .text(upgrade_name)
                    .static_text(" (")
                    .text(format!("{}G", cost))
                    .static_text(")")
            }
            HistoryEventType::ContractPurchased { contract, cost } => {
                let risk_text = ContractText::Risk(&contract.risk).localized_text(locale);
                let reward_text = ContractText::Reward(&contract.reward).localized_text(locale);
                builder
                    .static_text("Contract Purchased: ")
                    .text(risk_text)
                    .static_text(" / ")
                    .text(reward_text)
                    .static_text(" (")
                    .text(format!("{}G", cost))
                    .static_text(")")
            }
            HistoryEventType::GameOver => builder.static_text("Game Over"),
        }
    }
}

impl HistoryEventType {}
