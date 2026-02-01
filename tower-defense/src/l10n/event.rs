use super::{Language, Locale, LocalizedText, rich_text_helpers::RichTextHelpers};
use crate::game_state::play_history::HistoryEventType;
use crate::l10n::contract::ContractText;
use crate::l10n::effect::EffectText;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

#[derive(Debug, Clone)]
pub enum EventText<'a> {
    Description(&'a HistoryEventType, &'a Locale),
}

impl EventText<'_> {}

impl LocalizedText for EventText<'_> {
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
        _locale: &Locale,
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
            HistoryEventType::ItemPurchased { item, cost } => builder
                .static_text("아이템 구매: ")
                .l10n(EffectText::Name(item.effect.clone()), _locale)
                .static_text(" (")
                .with_gold_value(format!("{}G", cost))
                .static_text(")"),
            HistoryEventType::ItemUsed { item_effect } => builder
                .static_text("아이템 사용: ")
                .l10n(EffectText::Name(item_effect.clone()), _locale),
            HistoryEventType::UpgradeSelected { upgrade } => builder
                .static_text("업그레이드 선택: ")
                .l10n(UpgradeKindText::Name(&upgrade.kind), _locale),
            HistoryEventType::UpgradePurchased { upgrade, cost } => builder
                .static_text("업그레이드 구매: ")
                .l10n(UpgradeKindText::Name(&upgrade.kind), _locale)
                .static_text(" (")
                .with_gold_value(format!("{}G", cost))
                .static_text(")"),
            HistoryEventType::ContractPurchased { contract, cost } => builder
                .static_text("계약 구매: ")
                .l10n(ContractText::Risk(&contract.risk), _locale)
                .static_text(" / ")
                .l10n(ContractText::Reward(&contract.reward), _locale)
                .static_text(" (")
                .with_gold_value(format!("{}G", cost))
                .static_text(")"),
            HistoryEventType::GameOver => builder.static_text("게임 오버"),
        }
    }

    fn description_english_builder<'a>(
        &self,
        builder: TypographyBuilder<'a>,
        _locale: &Locale,
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
            HistoryEventType::ItemPurchased { item, cost } => builder
                .static_text("Item Purchased: ")
                .l10n(EffectText::Name(item.effect.clone()), _locale)
                .static_text(" (")
                .with_gold_value(format!("{}G", cost))
                .static_text(")"),
            HistoryEventType::ItemUsed { item_effect } => builder
                .static_text("Item Used: ")
                .l10n(EffectText::Name(item_effect.clone()), _locale),
            HistoryEventType::UpgradeSelected { upgrade } => builder
                .static_text("Upgrade Selected: ")
                .l10n(UpgradeKindText::Name(&upgrade.kind), _locale),
            HistoryEventType::UpgradePurchased { upgrade, cost } => builder
                .static_text("Upgrade Purchased: ")
                .l10n(UpgradeKindText::Name(&upgrade.kind), _locale)
                .static_text(" (")
                .with_gold_value(format!("{}G", cost))
                .static_text(")"),
            HistoryEventType::ContractPurchased { contract, cost } => builder
                .static_text("Contract Purchased: ")
                .l10n(ContractText::Risk(&contract.risk), _locale)
                .static_text(" / ")
                .l10n(ContractText::Reward(&contract.reward), _locale)
                .static_text(" (")
                .with_gold_value(format!("{}G", cost))
                .static_text(")"),
            HistoryEventType::GameOver => builder.static_text("Game Over"),
        }
    }
}

impl HistoryEventType {}
