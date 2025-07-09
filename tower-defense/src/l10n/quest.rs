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
    pub fn to_korean(&self) -> String {
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
}
