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
    IncreaseAttackSpeed {
        speed: usize,
    },
    IncreaseAttackRange {
        range: usize,
    },
}

impl QuestText {
    /// Suit을 한국어와 아이콘으로 변환하는 헬퍼 함수
    fn suit_to_korean_with_icon(suit: &str) -> String {
        match suit {
            "♠" => "icon<suit_spades:16:16:16:1>스페이드".to_string(),
            "♥" => "icon<suit_hearts:16:16:16:1>하트".to_string(),
            "◆" => "icon<suit_diamonds:16:16:16:1>다이아".to_string(),
            "♣" => "icon<suit_clubs:16:16:16:1>클럽".to_string(),
            _ => suit.to_string(),
        }
    }

    /// Suit을 영어와 아이콘으로 변환하는 헬퍼 함수
    fn suit_to_english_with_icon(suit: &str) -> String {
        match suit {
            "♠" => "icon<suit_spades:16:16:16:1>Spades".to_string(),
            "♥" => "icon<suit_hearts:16:16:16:1>Hearts".to_string(),
            "◆" => "icon<suit_diamonds:16:16:16:1>Diamonds".to_string(),
            "♣" => "icon<suit_clubs:16:16:16:1>Clubs".to_string(),
            _ => suit.to_string(),
        }
    }

    pub(super) fn to_korean(&self) -> String {
        match self {
            QuestText::BuildTowerRankNew { rank, count } => {
                format!("{}타워를 {}개 새로 건설하세요.", rank, count)
            }
            QuestText::BuildTowerRank {
                rank,
                count,
                current_count,
            } => format!(
                "{}타워를 {}개 소유하세요. ({}/{})",
                rank, count, current_count, count
            ),
            QuestText::BuildTowerSuitNew { suit, count } => {
                let suit_text = Self::suit_to_korean_with_icon(suit);
                format!("{}타워를 {}개 새로 건설하세요.", suit_text, count)
            }
            QuestText::BuildTowerSuit {
                suit,
                count,
                current_count,
            } => {
                let suit_text = Self::suit_to_korean_with_icon(suit);
                format!(
                    "{}타워를 {}개 소유하세요. ({}/{})",
                    suit_text, count, current_count, count
                )
            }
            QuestText::BuildTowerHandNew { hand, count } => {
                format!("{}타워를 {}개 새로 건설하세요.", hand, count)
            }
            QuestText::BuildTowerHand {
                hand,
                count,
                current_count,
            } => format!(
                "{}타워를 {}개 소유하세요. ({}/{})",
                hand, count, current_count, count
            ),
            QuestText::ClearBossRoundWithoutItems => {
                "아이템을 사용하지않고 보스라운드 클리어".to_string()
            }
            QuestText::DealDamageWithItems { damage } => {
                format!(
                    "아이템을 사용해 |attack_damage_color|icon<attack_damage:16:16:16:1>{}|/attack_damage_color|피해 입히기",
                    damage
                )
            }
            QuestText::BuildTowersWithoutReroll { count } => {
                format!("리롤하지않고 타워 {}개 만들기", count)
            }
            QuestText::UseReroll { count } => format!("리롤 {}회 사용하기", count),
            QuestText::SpendGold { gold } => {
                format!(
                    "|gold_color|icon<gold:16:16:16:1>{}골드|/gold_color| 사용하기",
                    gold
                )
            }
            QuestText::EarnGold { gold } => {
                format!(
                    "|gold_color|icon<gold:16:16:16:1>{}골드|/gold_color| 획득하기",
                    gold
                )
            }
            QuestText::IncreaseAttackSpeed { speed } => {
                format!(
                    "|attack_speed_color|icon<attack_speed:16:16:16:1>공격속도를 {}|/attack_speed_color| 증가시키기",
                    speed
                )
            }
            QuestText::IncreaseAttackRange { range } => {
                format!(
                    "|attack_range_color|icon<attack_range:16:16:16:1>사거리를 {}|/attack_range_color| 증가시키기",
                    range
                )
            }
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            QuestText::BuildTowerRankNew { rank, count } => {
                format!("Build {} new {} towers.", count, rank)
            }
            QuestText::BuildTowerRank {
                rank,
                count,
                current_count,
            } => format!(
                "Own {} {} towers. ({}/{})",
                count, rank, current_count, count
            ),
            QuestText::BuildTowerSuitNew { suit, count } => {
                let suit_text = Self::suit_to_english_with_icon(suit);
                format!("Build {} new {} towers.", count, suit_text)
            }
            QuestText::BuildTowerSuit {
                suit,
                count,
                current_count,
            } => {
                let suit_text = Self::suit_to_english_with_icon(suit);
                format!(
                    "Own {} {} towers. ({}/{})",
                    count, suit_text, current_count, count
                )
            }
            QuestText::BuildTowerHandNew { hand, count } => {
                format!("Build {} new {} towers.", count, hand)
            }
            QuestText::BuildTowerHand {
                hand,
                count,
                current_count,
            } => format!(
                "Own {} {} towers. ({}/{})",
                count, hand, current_count, count
            ),
            QuestText::ClearBossRoundWithoutItems => {
                "Clear the boss round without using items".to_string()
            }
            QuestText::DealDamageWithItems { damage } => {
                format!(
                    "Deal |attack_damage_color|icon<attack_damage:16:16:16:1>{}|/attack_damage_color| damage using items",
                    damage
                )
            }
            QuestText::BuildTowersWithoutReroll { count } => {
                format!("Build {} towers without rerolling", count)
            }
            QuestText::UseReroll { count } => format!("Use reroll {} times", count),
            QuestText::SpendGold { gold } => {
                format!(
                    "Spend |gold_color|icon<gold:16:16:16:1>{}|/gold_color| gold",
                    gold
                )
            }
            QuestText::EarnGold { gold } => {
                format!(
                    "Gain |gold_color|icon<gold:16:16:16:1>{}|/gold_color| gold",
                    gold
                )
            }
            QuestText::IncreaseAttackSpeed { speed } => {
                format!(
                    "Increase |attack_speed_color|icon<attack_speed:16:16:16:1>attack speed|/attack_speed_color| by {}",
                    speed
                )
            }
            QuestText::IncreaseAttackRange { range } => {
                format!(
                    "Increase |attack_range_color|icon<attack_range:16:16:16:1>attack range|/attack_range_color| by {}",
                    range
                )
            }
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
            QuestRewardText::Money { amount } => {
                format!(
                    "|gold_color|icon<gold:16:16:16:1>${}|/gold_color| 골드",
                    amount
                )
            }
            QuestRewardText::Item => "아이템".to_string(),
            QuestRewardText::Upgrade => "업그레이드".to_string(),
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            QuestRewardText::Money { amount } => {
                format!(
                    "|gold_color|icon<gold:16:16:16:1>${}|/gold_color| Gold",
                    amount
                )
            }
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
