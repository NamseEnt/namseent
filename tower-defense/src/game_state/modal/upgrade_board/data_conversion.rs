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
    pub description: UpgradeInfoDescription,
}

#[derive(Clone, State)]
pub enum UpgradeInfoDescription {
    Single(UpgradeBoardText),
    PrefixSuffix {
        prefix: UpgradeBoardText,
        suffix: UpgradeBoardText,
    },
}

impl UpgradeInfoDescription {
    pub fn key(&self) -> String {
        match self {
            UpgradeInfoDescription::Single(text) => format!("{:?}", text),
            UpgradeInfoDescription::PrefixSuffix { prefix, suffix } => {
                format!("{:?}:{:?}", prefix, suffix)
            }
        }
    }
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
    _text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    if state.gold_earn_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::GoldEarnPlus,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::GoldEarnPlus {
                amount: state.gold_earn_plus,
            }),
        });
    }

    if state.shop_slot_expand != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShopSlotExpansion,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::ShopSlotExpand {
                amount: state.shop_slot_expand,
            }),
        });
    }

    if state.reroll_chance_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::RerollCountPlus,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::RerollChancePlus {
                amount: state.reroll_chance_plus,
            }),
        });
    }

    if state.shop_item_price_minus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShopItemPriceMinus,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::ShopItemPriceMinus {
                amount: state.shop_item_price_minus,
            }),
        });
    }

    if state.shop_refresh_chance_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShopRefreshPlus,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::ShopRefreshChancePlus {
                amount: state.shop_refresh_chance_plus,
            }),
        });
    }

    if state.shorten_straight_flush_to_4_cards {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::ShortenStraightFlushTo4Cards,
            description: UpgradeInfoDescription::Single(
                UpgradeBoardText::ShortenStraightFlushTo4Cards,
            ),
        });
    }

    if state.skip_rank_for_straight {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::SkipRankForStraight,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::SkipRankForStraight),
        });
    }

    if state.treat_suits_as_same {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::TreatSuitsAsSame,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::TreatSuitsAsSame),
        });
    }
}

fn add_tower_select_upgrades(
    state: &UpgradeState,
    _text: &crate::l10n::TextManager,
    infos: &mut Vec<UpgradeInfo>,
) {
    for (target, tower_upgrade_state) in &state.tower_select_upgrade_states {
        let target_prefix = match target {
            TowerSelectUpgradeTarget::LowCard => UpgradeBoardText::TowerSelectLowCard {
                amount: LOW_CARD_COUNT,
            },
            TowerSelectUpgradeTarget::NoReroll => UpgradeBoardText::TowerSelectNoReroll,
            TowerSelectUpgradeTarget::Reroll => UpgradeBoardText::TowerSelectReroll,
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
            let suffix = UpgradeBoardText::DamageMultiplier {
                amount: tower_upgrade_state.damage_multiplier,
            };
            infos.push(UpgradeInfo {
                upgrade_kind,
                description: UpgradeInfoDescription::PrefixSuffix {
                    prefix: target_prefix.clone(),
                    suffix,
                },
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
            TowerUpgradeTarget::Rank { rank } => UpgradeBoardText::TowerUpgradeRank {
                name: rank.to_string(),
            },
            TowerUpgradeTarget::Suit { suit } => UpgradeBoardText::TowerUpgradeSuit {
                name: suit.to_string(),
            },
            TowerUpgradeTarget::TowerKind { tower_kind } => UpgradeBoardText::TowerUpgradeKind {
                name: text.tower(tower_kind.to_text()).to_string(),
            },
            TowerUpgradeTarget::EvenOdd { even } => {
                let name = if *even { "짝수" } else { "홀수" };
                UpgradeBoardText::TowerUpgradeEvenOdd {
                    name: name.to_string(),
                }
            }
            TowerUpgradeTarget::FaceNumber { face } => {
                let name = if *face { "그림" } else { "숫자" };
                UpgradeBoardText::TowerUpgradeFaceNumber {
                    name: name.to_string(),
                }
            }
        };

        // 각 타워 업그레이드 상태를 개별 UpgradeKind로 변환
        add_tower_damage_upgrades(target, tower_upgrade_state, &target_prefix, text, infos);
    }
}

fn add_tower_damage_upgrades(
    target: &TowerUpgradeTarget,
    tower_upgrade_state: &TowerUpgradeState,
    target_prefix: &UpgradeBoardText,
    _text: &crate::l10n::TextManager,
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
        let suffix = UpgradeBoardText::DamageMultiplier {
            amount: tower_upgrade_state.damage_multiplier,
        };
        infos.push(UpgradeInfo {
            upgrade_kind,
            description: UpgradeInfoDescription::PrefixSuffix {
                prefix: target_prefix.clone(),
                suffix,
            },
        });
    }
}
