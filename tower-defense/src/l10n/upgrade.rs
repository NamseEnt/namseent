use super::{Locale, Language, LocalizedText};

#[derive(Debug, Clone, Copy)]
pub enum TowerUpgradeTarget {
    Tower(crate::game_state::upgrade::TowerUpgradeTarget),
    TowerSelect(crate::game_state::upgrade::TowerSelectUpgradeTarget),
}

#[derive(Debug, Clone, Copy)]
pub enum WhatUpgrade {
    Damage,
    Speed,
    Range,
}

#[derive(Debug, Clone, Copy)]
pub enum AddOrMultiply {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub enum Template {
    TowerUpgrade {
        target: TowerUpgradeTarget,
        what_upgrade: WhatUpgrade,
        add_or_multiply: AddOrMultiply,
        how_much: f32,
    },
}

impl LocalizedText for Template {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl Template {
    pub(super) fn to_korean(&self) -> String {
        match self {
            Template::TowerUpgrade {
                target,
                what_upgrade,
                add_or_multiply,
                how_much,
            } => {
                format!(
                    "{} 타워의 {} {}",
                    match target {
                        TowerUpgradeTarget::Tower(tower_upgrade_target) =>
                            match tower_upgrade_target {
                                crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank } =>
                                    format!("{rank} 카드"),
                                crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit } =>
                                    format!("{suit} 카드"),
                                crate::game_state::upgrade::TowerUpgradeTarget::TowerKind {
                                    tower_kind,
                                } => tower_kind.to_string(),
                                crate::game_state::upgrade::TowerUpgradeTarget::EvenOdd {
                                    even,
                                } => format!("{} 카드", if *even { "짝수" } else { "홀수" }),
                                crate::game_state::upgrade::TowerUpgradeTarget::FaceNumber {
                                    face,
                                } => format!("{} 카드", if *face { "그림" } else { "숫자" }),
                            },
                        TowerUpgradeTarget::TowerSelect(tower_select_upgrade_target) =>
                            match tower_select_upgrade_target {
                                crate::game_state::upgrade::TowerSelectUpgradeTarget::LowCard =>
                                    "3장 이하로 만든".to_string(),
                                crate::game_state::upgrade::TowerSelectUpgradeTarget::NoReroll =>
                                    "리롤 안하고 만든".to_string(),
                                crate::game_state::upgrade::TowerSelectUpgradeTarget::Reroll =>
                                    "리롤하고 만든".to_string(),
                            },
                    },
                    match what_upgrade {
                        WhatUpgrade::Damage => "공격력이",
                        WhatUpgrade::Speed => "공격 속도가",
                        WhatUpgrade::Range => "사거리가",
                    },
                    match add_or_multiply {
                        AddOrMultiply::Add => format!("{how_much:.0}만큼 증가합니다"),
                        AddOrMultiply::Multiply => format!("{how_much:.1}배 증가합니다"),
                    }
                )
            }
        }
    }
    
    pub(super) fn to_english(&self) -> String {
        match self {
            Template::TowerUpgrade {
                target,
                what_upgrade,
                add_or_multiply,
                how_much,
            } => {
                format!(
                    "{} towers {} by {}",
                    match target {
                        TowerUpgradeTarget::Tower(tower_upgrade_target) =>
                            match tower_upgrade_target {
                                crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank } =>
                                    format!("{rank} card"),
                                crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit } =>
                                    format!("{suit} card"),
                                crate::game_state::upgrade::TowerUpgradeTarget::TowerKind {
                                    tower_kind,
                                } => tower_kind.to_string(),
                                crate::game_state::upgrade::TowerUpgradeTarget::EvenOdd {
                                    even,
                                } => format!("{} card", if *even { "even" } else { "odd" }),
                                crate::game_state::upgrade::TowerUpgradeTarget::FaceNumber {
                                    face,
                                } => format!("{} card", if *face { "face" } else { "number" }),
                            },
                        TowerUpgradeTarget::TowerSelect(tower_select_upgrade_target) =>
                            match tower_select_upgrade_target {
                                crate::game_state::upgrade::TowerSelectUpgradeTarget::LowCard =>
                                    "built with 3 or fewer cards".to_string(),
                                crate::game_state::upgrade::TowerSelectUpgradeTarget::NoReroll =>
                                    "built without reroll".to_string(),
                                crate::game_state::upgrade::TowerSelectUpgradeTarget::Reroll =>
                                    "built with reroll".to_string(),
                            },
                    },
                    match what_upgrade {
                        WhatUpgrade::Damage => "attack damage increased",
                        WhatUpgrade::Speed => "attack speed increased",
                        WhatUpgrade::Range => "range increased",
                    },
                    match add_or_multiply {
                        AddOrMultiply::Add => format!("{how_much:.0}"),
                        AddOrMultiply::Multiply => format!("{how_much:.1}x"),
                    }
                )
            }
        }
    }

    pub fn from_kind(kind: &crate::game_state::upgrade::UpgradeKind, _is_name: bool) -> Self {
        match kind {
            crate::game_state::upgrade::UpgradeKind::RankAttackDamagePlus { rank, damage_plus } => {
                Template::TowerUpgrade {
                    target: TowerUpgradeTarget::Tower(
                        crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank: *rank },
                    ),
                    what_upgrade: WhatUpgrade::Damage,
                    add_or_multiply: AddOrMultiply::Add,
                    how_much: *damage_plus,
                }
            }
            _ => Template::TowerUpgrade {
                target: TowerUpgradeTarget::Tower(
                    crate::game_state::upgrade::TowerUpgradeTarget::Rank {
                        rank: crate::card::Rank::Ace,
                    },
                ),
                what_upgrade: WhatUpgrade::Damage,
                add_or_multiply: AddOrMultiply::Add,
                how_much: 0.0,
            },
        }
    }
}
