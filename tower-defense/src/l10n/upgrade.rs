use crate::icon::{Icon, IconKind};
// --- Rich text 헬퍼 함수 (upgrade 전용) ---
fn suit_icon(suit: &crate::card::Suit) -> String {
    Icon::new(IconKind::Suit { suit: *suit }).as_tag()
}
fn attack_damage_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackDamage);
    format!(
        "|attack_damage_color|{}{value}|/attack_damage_color|",
        icon.as_tag()
    )
}
fn attack_speed_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackSpeed);
    format!(
        "|attack_speed_color|{}{value}|/attack_speed_color|",
        icon.as_tag()
    )
}
fn attack_range_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackRange);
    format!(
        "|attack_range_color|{}{value}|/attack_range_color|",
        icon.as_tag()
    )
}
use super::{Language, Locale, LocalizedText};

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
                let upgrade_text = match what_upgrade {
                    WhatUpgrade::Damage => attack_damage_icon("공격력"),
                    WhatUpgrade::Speed => attack_speed_icon("공격 속도"),
                    WhatUpgrade::Range => attack_range_icon("사거리"),
                };

                let target_text = match target {
                    TowerUpgradeTarget::Tower(tower_upgrade_target) => match tower_upgrade_target {
                        crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank } => {
                            format!("|purple|{rank}|/purple| 카드")
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit } => {
                            format!("{} 카드", suit_icon(suit))
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::TowerKind {
                            tower_kind,
                        } => {
                            let tower_text = tower_kind.to_text();
                            match tower_text {
                                crate::l10n::tower::TowerKindText::Barricade => "바리케이드",
                                crate::l10n::tower::TowerKindText::High => "하이카드",
                                crate::l10n::tower::TowerKindText::OnePair => "원페어",
                                crate::l10n::tower::TowerKindText::TwoPair => "투페어",
                                crate::l10n::tower::TowerKindText::ThreeOfAKind => "트리플",
                                crate::l10n::tower::TowerKindText::Straight => "스트레이트",
                                crate::l10n::tower::TowerKindText::Flush => "플러쉬",
                                crate::l10n::tower::TowerKindText::FullHouse => "풀하우스",
                                crate::l10n::tower::TowerKindText::FourOfAKind => "포카드",
                                crate::l10n::tower::TowerKindText::StraightFlush => {
                                    "스트레이트 플러쉬"
                                }
                                crate::l10n::tower::TowerKindText::RoyalFlush => "로열 플러쉬",
                            }
                            .to_string()
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::EvenOdd { even } => {
                            format!("{} 카드", if *even { "짝수" } else { "홀수" })
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::FaceNumber { face } => {
                            format!("{} 카드", if *face { "그림" } else { "숫자" })
                        }
                    },
                    TowerUpgradeTarget::TowerSelect(tower_select_upgrade_target) => {
                        match tower_select_upgrade_target {
                            crate::game_state::upgrade::TowerSelectUpgradeTarget::LowCard => {
                                "3장 이하로 만든".to_string()
                            }
                            crate::game_state::upgrade::TowerSelectUpgradeTarget::NoReroll => {
                                "리롤 안하고 만든".to_string()
                            }
                            crate::game_state::upgrade::TowerSelectUpgradeTarget::Reroll => {
                                "리롤하고 만든".to_string()
                            }
                        }
                    }
                };

                let amount_text = match add_or_multiply {
                    AddOrMultiply::Add => format!("|green|+{how_much:.0}|/green|만큼 증가합니다"),
                    AddOrMultiply::Multiply => {
                        format!("|green|×{how_much:.1}|/green|배 증가합니다")
                    }
                };

                format!("{target_text} 타워의 {upgrade_text} {amount_text}")
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
                let upgrade_text = match what_upgrade {
                    WhatUpgrade::Damage => attack_damage_icon("attack damage"),
                    WhatUpgrade::Speed => attack_speed_icon("attack speed"),
                    WhatUpgrade::Range => attack_range_icon("range"),
                };

                let target_text = match target {
                    TowerUpgradeTarget::Tower(tower_upgrade_target) => match tower_upgrade_target {
                        crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank } => {
                            format!("|purple|{rank}|/purple| card")
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit } => {
                            format!("{} card", suit_icon(suit))
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::TowerKind {
                            tower_kind,
                        } => {
                            // Use tower kind localization
                            let tower_text = tower_kind.to_text();
                            match tower_text {
                                crate::l10n::tower::TowerKindText::Barricade => "Barricade",
                                crate::l10n::tower::TowerKindText::High => "High",
                                crate::l10n::tower::TowerKindText::OnePair => "One Pair",
                                crate::l10n::tower::TowerKindText::TwoPair => "Two Pair",
                                crate::l10n::tower::TowerKindText::ThreeOfAKind => {
                                    "Three of a Kind"
                                }
                                crate::l10n::tower::TowerKindText::Straight => "Straight",
                                crate::l10n::tower::TowerKindText::Flush => "Flush",
                                crate::l10n::tower::TowerKindText::FullHouse => "Full House",
                                crate::l10n::tower::TowerKindText::FourOfAKind => "Four of a Kind",
                                crate::l10n::tower::TowerKindText::StraightFlush => {
                                    "Straight Flush"
                                }
                                crate::l10n::tower::TowerKindText::RoyalFlush => "Royal Flush",
                            }
                            .to_string()
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::EvenOdd { even } => {
                            format!("{} card", if *even { "even" } else { "odd" })
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::FaceNumber { face } => {
                            format!("{} card", if *face { "face" } else { "number" })
                        }
                    },
                    TowerUpgradeTarget::TowerSelect(tower_select_upgrade_target) => {
                        match tower_select_upgrade_target {
                            crate::game_state::upgrade::TowerSelectUpgradeTarget::LowCard => {
                                "built with 3 or fewer cards".to_string()
                            }
                            crate::game_state::upgrade::TowerSelectUpgradeTarget::NoReroll => {
                                "built without reroll".to_string()
                            }
                            crate::game_state::upgrade::TowerSelectUpgradeTarget::Reroll => {
                                "built with reroll".to_string()
                            }
                        }
                    }
                };

                let amount_text = match add_or_multiply {
                    AddOrMultiply::Add => format!("increased by |green|+{how_much:.0}|/green|"),
                    AddOrMultiply::Multiply => {
                        format!("increased by |green|×{how_much:.1}|/green|")
                    }
                };

                format!("{target_text} towers {upgrade_text} {amount_text}")
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
