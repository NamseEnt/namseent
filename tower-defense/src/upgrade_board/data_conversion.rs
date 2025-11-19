use crate::{
    game_state::upgrade::{
        LOW_CARD_COUNT, TowerSelectUpgradeTarget, TowerUpgradeState, TowerUpgradeTarget,
        UpgradeKind, UpgradeState,
    },
    l10n::upgrade_board::UpgradeBoardText,
    *,
};

#[derive(Clone, State)]
pub struct UpgradeInfo {
    pub upgrade_kind: UpgradeKind,
    pub description: String,
}

pub fn get_upgrade_infos(
    state: &UpgradeState,
    text: &crate::l10n::TextManager,
) -> Vec<UpgradeInfo> {
    let mut infos = vec![];

    // 기본 업그레이드들 추가
    add_basic_upgrades(state, text, &mut infos);

    // 타워 선택 업그레이드들 추가
    add_tower_select_upgrades(state, text, &mut infos);

    // 타워 업그레이드들 추가
    add_tower_upgrades(state, text, &mut infos);

    infos
}

fn add_basic_upgrades(
    state: &UpgradeState,
    text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    if state.gold_earn_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::GoldEarnPlus,
            description: text.upgrade_board(UpgradeBoardText::GoldEarnPlus {
                amount: state.gold_earn_plus,
            }),
        });
    }

    if state.shop_slot_expand != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShopSlotExpansion,
            description: text.upgrade_board(UpgradeBoardText::ShopSlotExpand {
                amount: state.shop_slot_expand,
            }),
        });
    }

    if state.reroll_chance_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::RerollCountPlus,
            description: text.upgrade_board(UpgradeBoardText::RerollChancePlus {
                amount: state.reroll_chance_plus,
            }),
        });
    }

    if state.shop_item_price_minus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShopItemPriceMinus,
            description: text.upgrade_board(UpgradeBoardText::ShopItemPriceMinus {
                amount: state.shop_item_price_minus,
            }),
        });
    }

    if state.shop_refresh_chance_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShopRefreshPlus,
            description: text.upgrade_board(UpgradeBoardText::ShopRefreshChancePlus {
                amount: state.shop_refresh_chance_plus,
            }),
        });
    }

    if state.shorten_straight_flush_to_4_cards {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShortenStraightFlushTo4Cards,
            description: text.upgrade_board(UpgradeBoardText::ShortenStraightFlushTo4Cards),
        });
    }

    if state.skip_rank_for_straight {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::SkipRankForStraight,
            description: text.upgrade_board(UpgradeBoardText::SkipRankForStraight),
        });
    }

    if state.treat_suits_as_same {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::TreatSuitsAsSame,
            description: text.upgrade_board(UpgradeBoardText::TreatSuitsAsSame),
        });
    }
}

fn add_tower_select_upgrades(
    state: &UpgradeState,
    text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    for (target, tower_upgrade_state) in &state.tower_select_upgrade_states {
        let target_prefix = match target {
            TowerSelectUpgradeTarget::LowCard => {
                text.upgrade_board(UpgradeBoardText::TowerSelectLowCard {
                    amount: LOW_CARD_COUNT,
                })
            }
            TowerSelectUpgradeTarget::NoReroll => {
                text.upgrade_board(UpgradeBoardText::TowerSelectNoReroll)
            }
            TowerSelectUpgradeTarget::Reroll => {
                text.upgrade_board(UpgradeBoardText::TowerSelectReroll)
            }
        };

        // 데미지 배수 업그레이드
        if tower_upgrade_state.damage_multiplier != 1.0 {
            let upgrade_kind = match target {
                TowerSelectUpgradeTarget::LowCard => UpgradeKind::LowCardTowerDamageMultiply {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
                TowerSelectUpgradeTarget::NoReroll => {
                    UpgradeKind::NoRerollTowerAttackDamageMultiply {
                        damage_multiplier: tower_upgrade_state.damage_multiplier,
                    }
                }
                TowerSelectUpgradeTarget::Reroll => UpgradeKind::RerollTowerAttackDamageMultiply {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
            };
            let suffix = text.upgrade_board(UpgradeBoardText::DamageMultiplier {
                amount: tower_upgrade_state.damage_multiplier,
            });
            infos.push(UpgradeInfo {
                upgrade_kind,
                description: format!("{target_prefix} {suffix}"),
            });
        }

        // 사거리 업그레이드
        if tower_upgrade_state.range_plus != 0.0 {
            let upgrade_kind = match target {
                TowerSelectUpgradeTarget::LowCard => UpgradeKind::LowCardTowerAttackRangePlus {
                    range_plus: tower_upgrade_state.range_plus,
                },
                TowerSelectUpgradeTarget::NoReroll => UpgradeKind::NoRerollTowerAttackRangePlus {
                    range_plus: tower_upgrade_state.range_plus,
                },
                TowerSelectUpgradeTarget::Reroll => UpgradeKind::RerollTowerAttackRangePlus {
                    range_plus: tower_upgrade_state.range_plus,
                },
            };
            let suffix = text.upgrade_board(UpgradeBoardText::RangePlus {
                amount: tower_upgrade_state.range_plus,
            });
            infos.push(UpgradeInfo {
                upgrade_kind,
                description: format!("{target_prefix} {suffix}"),
            });
        }
    }
}

fn add_tower_upgrades(
    state: &UpgradeState,
    text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    for (target, tower_upgrade_state) in &state.tower_upgrade_states {
        let target_prefix = match target {
            TowerUpgradeTarget::Rank { rank } => {
                text.upgrade_board(UpgradeBoardText::TowerUpgradeRank {
                    name: rank.to_string(),
                })
            }
            TowerUpgradeTarget::Suit { suit } => {
                text.upgrade_board(UpgradeBoardText::TowerUpgradeSuit {
                    name: suit.to_string(),
                })
            }
            TowerUpgradeTarget::TowerKind { tower_kind } => {
                text.upgrade_board(UpgradeBoardText::TowerUpgradeKind {
                    name: text.tower(tower_kind.to_text()).to_string(),
                })
            }
            TowerUpgradeTarget::EvenOdd { even } => {
                let name = if *even { "짝수" } else { "홀수" };
                text.upgrade_board(UpgradeBoardText::TowerUpgradeEvenOdd {
                    name: name.to_string(),
                })
            }
            TowerUpgradeTarget::FaceNumber { face } => {
                let name = if *face { "그림" } else { "숫자" };
                text.upgrade_board(UpgradeBoardText::TowerUpgradeFaceNumber {
                    name: name.to_string(),
                })
            }
        };

        // 각 타워 업그레이드 상태를 개별 UpgradeKind로 변환
        add_tower_damage_upgrades(target, tower_upgrade_state, &target_prefix, text, infos);
        add_tower_range_upgrades(target, tower_upgrade_state, &target_prefix, text, infos);
    }
}

fn add_tower_damage_upgrades(
    target: &TowerUpgradeTarget,
    tower_upgrade_state: &TowerUpgradeState,
    target_prefix: &str,
    text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    if tower_upgrade_state.damage_multiplier != 1.0 {
        let upgrade_kind = match target {
            TowerUpgradeTarget::Rank { rank } => UpgradeKind::RankAttackDamageMultiply {
                rank: *rank,
                damage_multiplier: tower_upgrade_state.damage_multiplier,
            },
            TowerUpgradeTarget::Suit { suit } => UpgradeKind::SuitAttackDamageMultiply {
                suit: *suit,
                damage_multiplier: tower_upgrade_state.damage_multiplier,
            },
            TowerUpgradeTarget::TowerKind { tower_kind } => UpgradeKind::HandAttackDamageMultiply {
                tower_kind: *tower_kind,
                damage_multiplier: tower_upgrade_state.damage_multiplier,
            },
            TowerUpgradeTarget::EvenOdd { even } => UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even: *even,
                damage_multiplier: tower_upgrade_state.damage_multiplier,
            },
            TowerUpgradeTarget::FaceNumber { face } => {
                UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                    face: *face,
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                }
            }
        };
        let suffix = text.upgrade_board(UpgradeBoardText::DamageMultiplier {
            amount: tower_upgrade_state.damage_multiplier,
        });
        infos.push(UpgradeInfo {
            upgrade_kind,
            description: format!("{target_prefix} {suffix}"),
        });
    }
}

fn add_tower_range_upgrades(
    target: &TowerUpgradeTarget,
    tower_upgrade_state: &TowerUpgradeState,
    target_prefix: &str,
    text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    if tower_upgrade_state.range_plus != 0.0 {
        let upgrade_kind = match target {
            TowerUpgradeTarget::Rank { rank } => UpgradeKind::RankAttackRangePlus {
                rank: *rank,
                range_plus: tower_upgrade_state.range_plus,
            },
            TowerUpgradeTarget::Suit { suit } => UpgradeKind::SuitAttackRangePlus {
                suit: *suit,
                range_plus: tower_upgrade_state.range_plus,
            },
            TowerUpgradeTarget::TowerKind { tower_kind } => UpgradeKind::HandAttackRangePlus {
                tower_kind: *tower_kind,
                range_plus: tower_upgrade_state.range_plus,
            },
            TowerUpgradeTarget::EvenOdd { even } => UpgradeKind::EvenOddTowerAttackRangePlus {
                even: *even,
                range_plus: tower_upgrade_state.range_plus,
            },
            TowerUpgradeTarget::FaceNumber { face } => {
                UpgradeKind::FaceNumberCardTowerAttackRangePlus {
                    face: *face,
                    range_plus: tower_upgrade_state.range_plus,
                }
            }
        };
        let suffix = text.upgrade_board(UpgradeBoardText::RangePlus {
            amount: tower_upgrade_state.range_plus,
        });
        infos.push(UpgradeInfo {
            upgrade_kind,
            description: format!("{target_prefix} {suffix}"),
        });
    }
}
