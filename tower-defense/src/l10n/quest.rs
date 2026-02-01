use super::{rich_text_helpers::RichTextHelpers, Language, Locale, LocalizedText};
use crate::{card::Suit, theme::typography::TypographyBuilder, *};

#[derive(Debug, Clone, State)]
pub enum QuestText {
    BuildTowerRankNew {
        rank: String,
        count: usize,
    },
    BuildTowerRank {
        rank: String,
        count: usize,
        current_count: usize,
    },
    BuildTowerSuitNew {
        suit: Suit,
        count: usize,
    },
    BuildTowerSuit {
        suit: Suit,
        count: usize,
        current_count: usize,
    },
    BuildTowerHandNew {
        hand: String,
        count: usize,
    },
    BuildTowerHand {
        hand: String,
        count: usize,
        current_count: usize,
    },
    ClearBossRoundWithoutItems,
    DealDamageWithItems {
        damage: usize,
    },
    BuildTowersWithoutReroll {
        count: usize,
    },
    UseReroll {
        count: usize,
    },
    SpendGold {
        gold: usize,
    },
    EarnGold {
        gold: usize,
    },
    IncreaseAttackSpeed {
        speed: usize,
    },
    IncreaseAttackRange {
        range: usize,
    },
}

impl QuestText {
    fn apply_korean<'a>(&self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            QuestText::BuildTowerRankNew { rank, count } => builder.text(format!(
                "{rank}타워를 {count}개 새로 건설하세요."
            )),
            QuestText::BuildTowerRank {
                rank,
                count,
                current_count,
            } => builder.text(format!(
                "{rank}타워를 {count}개 소유하세요. ({current_count}/{count})"
            )),
            QuestText::BuildTowerSuitNew { suit, count } => {
                builder.text(format!("{:?}타워를 {count}개 새로 건설하세요.", suit))
            }
            QuestText::BuildTowerSuit {
                suit,
                count,
                current_count,
            } => builder.text(format!(
                "{:?}타워를 {count}개 소유하세요. ({current_count}/{count})",
                suit
            )),
            QuestText::BuildTowerHandNew { hand, count } => {
                builder.text(format!("{hand}타워를 {count}개 새로 건설하세요."))
            }
            QuestText::BuildTowerHand {
                hand,
                count,
                current_count,
            } => builder.text(format!(
                "{hand}타워를 {count}개 소유하세요. ({current_count}/{count})"
            )),
            QuestText::ClearBossRoundWithoutItems => {
                builder.text("아이템을 사용하지않고 보스라운드 클리어")
            }
            QuestText::DealDamageWithItems { damage } => builder
                .text("아이템을 사용해 ")
                .with_attack_damage_icon(format!("{damage}"))
                .text(" 피해 입히기"),
            QuestText::BuildTowersWithoutReroll { count } => {
                builder.text(format!("리롤하지않고 타워 {count}개 만들기"))
            }
            QuestText::UseReroll { count } => builder.text(format!("리롤 {count}회 사용하기")),
            QuestText::SpendGold { gold } => builder
                .with_gold_icon(format!("{gold}"))
                .text(" 사용하기"),
            QuestText::EarnGold { gold } => builder
                .with_gold_icon(format!("{gold}"))
                .text(" 획득하기"),
            QuestText::IncreaseAttackSpeed { speed } => builder
                .with_attack_speed_icon(format!("{speed}"))
                .text(" 증가시키기"),
            QuestText::IncreaseAttackRange { range } => builder
                .with_attack_range_icon(format!("{range}"))
                .text(" 증가시키기"),
        }
    }

    fn apply_english<'a>(&self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            QuestText::BuildTowerRankNew { rank, count } => {
                builder.text(format!("Build {count} new {rank} towers."))
            }
            QuestText::BuildTowerRank {
                rank,
                count,
                current_count,
            } => builder.text(format!(
                "Own {count} {rank} towers. ({current_count}/{count})"
            )),
            QuestText::BuildTowerSuitNew { suit, count } => {
                builder.text(format!("Build {count} new {:?} towers.", suit))
            }
            QuestText::BuildTowerSuit {
                suit,
                count,
                current_count,
            } => builder.text(format!(
                "Own {count} {:?} towers. ({current_count}/{count})",
                suit
            )),
            QuestText::BuildTowerHandNew { hand, count } => {
                builder.text(format!("Build {count} new {hand} towers."))
            }
            QuestText::BuildTowerHand {
                hand,
                count,
                current_count,
            } => builder.text(format!(
                "Own {count} {hand} towers. ({current_count}/{count})"
            )),
            QuestText::ClearBossRoundWithoutItems => {
                builder.text("Clear the boss round without using items")
            }
            QuestText::DealDamageWithItems { damage } => builder
                .text("Deal ")
                .with_attack_damage_icon(format!("{damage}"))
                .text(" damage using items"),
            QuestText::BuildTowersWithoutReroll { count } => {
                builder.text(format!("Build {count} towers without rerolling"))
            }
            QuestText::UseReroll { count } => builder.text(format!("Use reroll {count} times")),
            QuestText::SpendGold { gold } => builder
                .text("Spend ")
                .with_gold_icon(format!("{gold}")),
            QuestText::EarnGold { gold } => builder
                .text("Gain ")
                .with_gold_icon(format!("{gold}")),
            QuestText::IncreaseAttackSpeed { speed } => builder
                .text("Increase attack speed by ")
                .with_attack_speed_icon(format!("{speed}")),
            QuestText::IncreaseAttackRange { range } => builder
                .text("Increase attack range by ")
                .with_attack_range_icon(format!("{range}")),
        }
    }
}

impl LocalizedText for QuestText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

#[derive(Debug, Clone, State)]
pub enum QuestRewardText {
    Money { amount: usize },
    Item,
    Upgrade,
}

impl QuestRewardText {
    fn apply_korean<'a>(&self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            QuestRewardText::Money { amount } => builder
                .with_gold_icon(format!("{amount}"))
                .space()
                .text("골드"),
            QuestRewardText::Item => builder.text("아이템"),
            QuestRewardText::Upgrade => builder.text("업그레이드"),
        }
    }

    fn apply_english<'a>(&self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            QuestRewardText::Money { amount } => builder
                .with_gold_icon(format!("{amount}"))
                .space()
                .text("Gold"),
            QuestRewardText::Item => builder.text("Item"),
            QuestRewardText::Upgrade => builder.text("Upgrade"),
        }
    }
}

impl LocalizedText for QuestRewardText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}
