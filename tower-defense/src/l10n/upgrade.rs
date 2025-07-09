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

pub enum Template {
    TowerUpgrade {
        target: TowerUpgradeTarget,
        what_upgrade: WhatUpgrade,
        add_or_multiply: AddOrMultiply,
        how_much: f32,
    },
}

pub enum Locales {
    KoKR(KoKRLocale),
}

impl Locales {
    pub fn text(&self, template: Template) -> String {
        match self {
            Locales::KoKR(locale) => locale.text(template),
        }
    }
}

pub struct KoKRLocale;
impl KoKRLocale {
    pub fn text(&self, template: Template) -> String {
        match template {
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
                                } => format!("{} 카드", if even { "짝수" } else { "홀수" }),
                                crate::game_state::upgrade::TowerUpgradeTarget::FaceNumber {
                                    face,
                                } => format!("{} 카드", if face { "그림" } else { "숫자" }),
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
}

impl Template {
    pub fn from_kind(kind: &crate::game_state::upgrade::UpgradeKind, is_name: bool) -> Self {
        // 이름/설명 구분은 is_name 플래그로 처리, 실제 매핑은 필요에 따라 구현
        // 예시: 이름은 is_name=true, 설명은 is_name=false
        // 실제 구현에서는 kind의 variant별로 적절한 Template variant를 생성
        match kind {
            // 예시: RankAttackDamagePlus { rank, damage_plus } =>
            crate::game_state::upgrade::UpgradeKind::RankAttackDamagePlus { rank, damage_plus } => {
                if is_name {
                    Template::TowerUpgrade {
                        target: TowerUpgradeTarget::Tower(
                            crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank: *rank },
                        ),
                        what_upgrade: WhatUpgrade::Damage,
                        add_or_multiply: AddOrMultiply::Add,
                        how_much: *damage_plus,
                    }
                } else {
                    Template::TowerUpgrade {
                        target: TowerUpgradeTarget::Tower(
                            crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank: *rank },
                        ),
                        what_upgrade: WhatUpgrade::Damage,
                        add_or_multiply: AddOrMultiply::Add,
                        how_much: *damage_plus,
                    }
                }
            }
            // ...다른 UpgradeKind variant도 동일하게 매핑 필요...
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
