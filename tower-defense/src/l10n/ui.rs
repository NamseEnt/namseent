use super::{Language, Locale, LocalizedText, rich_text_helpers::RichTextHelpers};
use crate::theme::typography::TypographyBuilder;
use crate::*;

#[derive(Debug, Clone, Copy, State)]
pub enum TopBarText {
    Run,
    Stage,
    Quest,
    Refresh,
    Accepted,
    Use,
    Settings,
    Shop,
    UseTower,
    Encyclopedia,
}

#[derive(Debug, Clone, Copy, State)]
pub enum EncyclopediaText {
    Title,
    Items,
    CardServices,
    Treasures,
    Undiscovered,
}

#[derive(Debug, Clone, Copy, State)]
pub struct EncyclopediaProgressText {
    pub discovered: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Copy, State)]
pub struct EncyclopediaCompletionText {
    pub percentage: usize,
}

impl LocalizedText for EncyclopediaProgressText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean | Language::English => {
                builder.text(format!("{}/{}", self.discovered, self.total));
            }
        }
    }
}

impl LocalizedText for EncyclopediaCompletionText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.text(format!("전체 수집률 {}%", self.percentage));
            }
            Language::English => {
                builder.text(format!("Collection {}%", self.percentage));
            }
        }
    }
}

impl LocalizedText for EncyclopediaText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match (locale.language, self) {
            (Language::Korean, Self::Title) => "백과사전",
            (Language::Korean, Self::Items) => "아이템",
            (Language::Korean, Self::CardServices) => "카드 서비스",
            (Language::Korean, Self::Treasures) => "보물",
            (Language::Korean, Self::Undiscovered) => "아직 발견하지 않은 항목입니다",
            (Language::English, Self::Title) => "Encyclopedia",
            (Language::English, Self::Items) => "Items",
            (Language::English, Self::CardServices) => "Card Services",
            (Language::English, Self::Treasures) => "Treasures",
            (Language::English, Self::Undiscovered) => "This entry has not been discovered yet",
        });
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum ResultModalText {
    Title,
    RestartButton,
    MaxPerfectClearLabel,
    TotalGoldLabel,
    TotalDamageLabel,
    CardRerollCountLabel,
    ShopRerollCountLabel,
    NoTowerDamage,
}

#[derive(Debug, Clone, Copy, State)]
pub enum TowerInfoPopupText {
    DamageLabel,
    AttackSpeedLabel,
    RangeLabel,
    TotalDamageLabel,
    RemoveButton,
}

impl LocalizedText for TopBarText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl TopBarText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            TopBarText::Run => "런",
            TopBarText::Stage => "스테이지",
            TopBarText::Quest => "퀘스트",
            TopBarText::Refresh => "새로고침",
            TopBarText::Accepted => "수락됨",
            TopBarText::Use => "사용",
            TopBarText::Settings => "설정",
            TopBarText::Shop => "상점",
            TopBarText::UseTower => "타워 사용",
            TopBarText::Encyclopedia => "백과사전",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            TopBarText::Run => "Run",
            TopBarText::Stage => "Stage",
            TopBarText::Quest => "Quest",
            TopBarText::Refresh => "Refresh",
            TopBarText::Accepted => "Accepted",
            TopBarText::Use => "Use",
            TopBarText::Settings => "Settings",
            TopBarText::Shop => "Shop",
            TopBarText::UseTower => "Use Tower",
            TopBarText::Encyclopedia => "Encyclopedia",
        }
    }
}

impl LocalizedText for ResultModalText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl ResultModalText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            ResultModalText::Title => "게임 결과",
            ResultModalText::RestartButton => "다시하기",
            ResultModalText::MaxPerfectClearLabel => "최대 연속 퍼펙트 클리어",
            ResultModalText::TotalGoldLabel => "총 획득 골드",
            ResultModalText::TotalDamageLabel => "총 데미지",
            ResultModalText::CardRerollCountLabel => "카드 리롤 횟수",
            ResultModalText::ShopRerollCountLabel => "상점 리롤 횟수",
            ResultModalText::NoTowerDamage => "타워 기여 기록이 없습니다",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            ResultModalText::Title => "Game Result",
            ResultModalText::RestartButton => "Restart",
            ResultModalText::MaxPerfectClearLabel => "Max Perfect Streak",
            ResultModalText::TotalGoldLabel => "Total Gold Earned",
            ResultModalText::TotalDamageLabel => "Total Damage",
            ResultModalText::CardRerollCountLabel => "Card Reroll Count",
            ResultModalText::ShopRerollCountLabel => "Shop Reroll Count",
            ResultModalText::NoTowerDamage => "No tower contribution recorded",
        }
    }
}

impl TowerInfoPopupText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            TowerInfoPopupText::DamageLabel => "데미지",
            TowerInfoPopupText::AttackSpeedLabel => "공격속도",
            TowerInfoPopupText::RangeLabel => "사거리",
            TowerInfoPopupText::TotalDamageLabel => "누적 데미지",
            TowerInfoPopupText::RemoveButton => "철거",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            TowerInfoPopupText::DamageLabel => "Damage",
            TowerInfoPopupText::AttackSpeedLabel => "Attack Speed",
            TowerInfoPopupText::RangeLabel => "Range",
            TowerInfoPopupText::TotalDamageLabel => "Total Damage",
            TowerInfoPopupText::RemoveButton => "Remove",
        }
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum RerollHealthCostDetailText {
    Damage(usize),
}

impl LocalizedText for RerollHealthCostDetailText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl RerollHealthCostDetailText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            RerollHealthCostDetailText::Damage(amount) => builder
                .text("체력을 ")
                .with_health_loss(format!("{}", amount))
                .text(" 잃습니다"),
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            RerollHealthCostDetailText::Damage(amount) => builder
                .text("Lose ")
                .with_health_loss(format!("{}", amount))
                .text(" health"),
        };
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum SettingsText {
    MasterVolume,
    EffectsVolume,
    UiVolume,
    AmbientVolume,
    MusicVolume,
}

impl LocalizedText for SettingsText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl SettingsText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            SettingsText::MasterVolume => "전체 볼륨",
            SettingsText::EffectsVolume => "효과음",
            SettingsText::UiVolume => "UI",
            SettingsText::AmbientVolume => "환경음",
            SettingsText::MusicVolume => "음악",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            SettingsText::MasterVolume => "Master",
            SettingsText::EffectsVolume => "Effects",
            SettingsText::UiVolume => "UI",
            SettingsText::AmbientVolume => "Ambient",
            SettingsText::MusicVolume => "Music",
        }
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum ShopPurchaseBlockReasonText {
    Unavailable,
    AlreadyPurchased,
    NotEnoughGold { required: usize, available: usize },
    PurchasesDisabled,
    NoEngravedCard,
    NotEnoughUnengravedCards { required: usize, available: usize },
}

impl LocalizedText for ShopPurchaseBlockReasonText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        let _ = match locale.language {
            Language::Korean => match self {
                Self::Unavailable => builder.static_text("구매 불가"),
                Self::AlreadyPurchased => builder.static_text("이미 구매한 상품입니다"),
                Self::NotEnoughGold {
                    required,
                    available,
                } => builder.text(format!(
                    "골드가 부족합니다. 필요: {required}, 보유: {available}"
                )),
                Self::PurchasesDisabled => {
                    builder.static_text("현재 상점 구매가 비활성화되어 있습니다")
                }
                Self::NoEngravedCard => builder.static_text("각인된 카드가 없습니다"),
                Self::NotEnoughUnengravedCards {
                    required,
                    available,
                } => builder.text(format!(
                    "각인되지 않은 카드가 부족합니다. 필요: {required}, 가능: {available}"
                )),
            },
            Language::English => match self {
                Self::Unavailable => builder.static_text("Purchase unavailable"),
                Self::AlreadyPurchased => {
                    builder.static_text("This item has already been purchased")
                }
                Self::NotEnoughGold {
                    required,
                    available,
                } => builder.text(format!(
                    "Not enough gold. Required: {required}, available: {available}"
                )),
                Self::PurchasesDisabled => {
                    builder.static_text("Shop purchases are currently disabled")
                }
                Self::NoEngravedCard => builder.static_text("There are no engraved cards"),
                Self::NotEnoughUnengravedCards {
                    required,
                    available,
                } => builder.text(format!(
                    "Not enough unengraved cards. Required: {required}, available: {available}"
                )),
            },
        };
    }
}
