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
                    WhatUpgrade::Damage => {
                        "|attack_damage_color|icon<attack_damage:16:16:16:1>공격력이|/attack_damage_color|"
                    }
                    WhatUpgrade::Speed => {
                        "|attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도가|/attack_speed_color|"
                    }
                    WhatUpgrade::Range => {
                        "|attack_range_color|icon<attack_range:16:16:16:1>사거리가|/attack_range_color|"
                    }
                };

                let target_text = match target {
                    TowerUpgradeTarget::Tower(tower_upgrade_target) => match tower_upgrade_target {
                        crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank } => {
                            format!("{rank} 카드")
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit } => {
                            let suit_with_icon = match suit.to_string().as_str() {
                                "♠" => "icon<suit_spades:16:16:16:1>스페이드".to_string(),
                                "♥" => "icon<suit_hearts:16:16:16:1>하트".to_string(),
                                "◆" => "icon<suit_diamonds:16:16:16:1>다이아".to_string(),
                                "♣" => "icon<suit_clubs:16:16:16:1>클럽".to_string(),
                                _ => suit.to_string(),
                            };
                            format!("{suit_with_icon} 카드")
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::TowerKind {
                            tower_kind,
                        } => {
                            // Use tower kind localization
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
                    AddOrMultiply::Add => format!("|B|{how_much:.0}만큼 증가합니다|/B|"),
                    AddOrMultiply::Multiply => format!("|B|{how_much:.1}배 증가합니다|/B|"),
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
                    WhatUpgrade::Damage => {
                        "|attack_damage_color|icon<attack_damage:16:16:16:1>attack damage|/attack_damage_color| increased"
                    }
                    WhatUpgrade::Speed => {
                        "|attack_speed_color|icon<attack_speed:16:16:16:1>attack speed|/attack_speed_color| increased"
                    }
                    WhatUpgrade::Range => {
                        "|attack_range_color|icon<attack_range:16:16:16:1>range|/attack_range_color| increased"
                    }
                };

                let target_text = match target {
                    TowerUpgradeTarget::Tower(tower_upgrade_target) => match tower_upgrade_target {
                        crate::game_state::upgrade::TowerUpgradeTarget::Rank { rank } => {
                            format!("{rank} card")
                        }
                        crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit } => {
                            let suit_with_icon = match suit.to_string().as_str() {
                                "♠" => "icon<suit_spades:16:16:16:1>Spades".to_string(),
                                "♥" => "icon<suit_hearts:16:16:16:1>Hearts".to_string(),
                                "◆" => "icon<suit_diamonds:16:16:16:1>Diamonds".to_string(),
                                "♣" => "icon<suit_clubs:16:16:16:1>Clubs".to_string(),
                                _ => suit.to_string(),
                            };
                            format!("{suit_with_icon} card")
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
                    AddOrMultiply::Add => format!("|B|by {how_much:.0}|/B|"),
                    AddOrMultiply::Multiply => format!("|B|by {how_much:.1}x|/B|"),
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
