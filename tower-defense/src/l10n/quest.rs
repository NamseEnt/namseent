use super::{Language, Locale, LocalizedText};

#[derive(Debug, Clone)]
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
        suit: String,
        count: usize,
    },
    BuildTowerSuit {
        suit: String,
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
}

impl QuestText {
    pub(super) fn to_korean(&self) -> String {
        match self {
            QuestText::BuildTowerRankNew { rank, count } => {
                format!("{rank}타워를 {count}개 새로 건설하세요.")
            }
            QuestText::BuildTowerRank {
                rank,
                count,
                current_count,
            } => format!("{rank}타워를 {count}개 소유하세요. ({current_count}/{count})"),
            QuestText::BuildTowerSuitNew { suit, count } => {
                format!("{suit}타워를 {count}개 새로 건설하세요.")
            }
            QuestText::BuildTowerSuit {
                suit,
                count,
                current_count,
            } => format!("{suit}타워를 {count}개 소유하세요. ({current_count}/{count})"),
            QuestText::BuildTowerHandNew { hand, count } => {
                format!("{hand}타워를 {count}개 새로 건설하세요.")
            }
            QuestText::BuildTowerHand {
                hand,
                count,
                current_count,
            } => format!("{hand}타워를 {count}개 소유하세요. ({current_count}/{count})"),
            QuestText::ClearBossRoundWithoutItems => {
                "아이템을 사용하지않고 보스라운드 클리어".to_string()
            }
            QuestText::DealDamageWithItems { damage } => {
                format!("아이템을 사용해 {damage}피해 입히기")
            }
            QuestText::BuildTowersWithoutReroll { count } => {
                format!("리롤하지않고 타워 {count}개 만들기")
            }
            QuestText::UseReroll { count } => format!("리롤 {count}회 사용하기"),
            QuestText::SpendGold { gold } => format!("{gold}골드 사용하기"),
            QuestText::EarnGold { gold } => format!("{gold}골드 획득하기"),
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            QuestText::BuildTowerRankNew { rank, count } => {
                format!("Build {count} new {rank} towers.")
            }
            QuestText::BuildTowerRank {
                rank,
                count,
                current_count,
            } => format!("Own {count} {rank} towers. ({current_count}/{count})"),
            QuestText::BuildTowerSuitNew { suit, count } => {
                format!("Build {count} new {suit} towers.")
            }
            QuestText::BuildTowerSuit {
                suit,
                count,
                current_count,
            } => format!("Own {count} {suit} towers. ({current_count}/{count})"),
            QuestText::BuildTowerHandNew { hand, count } => {
                format!("Build {count} new {hand} towers.")
            }
            QuestText::BuildTowerHand {
                hand,
                count,
                current_count,
            } => format!("Own {count} {hand} towers. ({current_count}/{count})"),
            QuestText::ClearBossRoundWithoutItems => {
                "Clear the boss round without using items".to_string()
            }
            QuestText::DealDamageWithItems { damage } => {
                format!("Deal {damage} damage using items")
            }
            QuestText::BuildTowersWithoutReroll { count } => {
                format!("Build {count} towers without rerolling")
            }
            QuestText::UseReroll { count } => format!("Use reroll {count} times"),
            QuestText::SpendGold { gold } => format!("Spend {gold} gold"),
            QuestText::EarnGold { gold } => format!("Gain {gold} gold"),
        }
    }
}

impl LocalizedText for QuestText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum QuestRewardText {
    Money { amount: usize },
    Item,
    Upgrade,
}

impl QuestRewardText {
    pub(super) fn to_korean(&self) -> String {
        match self {
            QuestRewardText::Money { amount } => format!("${amount} 골드"),
            QuestRewardText::Item => "아이템".to_string(),
            QuestRewardText::Upgrade => "업그레이드".to_string(),
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            QuestRewardText::Money { amount } => format!("${amount} Gold"),
            QuestRewardText::Item => "Item".to_string(),
            QuestRewardText::Upgrade => "Upgrade".to_string(),
        }
    }
}

impl LocalizedText for QuestRewardText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}
