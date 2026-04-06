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
            upgrade_kind: UpgradeKind::Magnet,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::Magnet {
                amount: state.gold_earn_plus,
            }),
        });
    }

    if state.shop_slot_expand != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::Backpack,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::ShopSlotExpand {
                amount: state.shop_slot_expand,
            }),
        });
    }

    if state.dice_chance_plus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::DiceBundle,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::RerollChancePlus {
                amount: state.dice_chance_plus,
            }),
        });
    }

    if state.shop_item_price_minus != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::EnergyDrink,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::ShopItemPriceMinus {
                amount: state.shop_item_price_minus,
            }),
        });
    }

    if state.removed_number_rank_count != 0 {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::Eraser,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::Eraser {
                amount: state.removed_number_rank_count,
            }),
        });
    }

    if state.shorten_straight_flush_to_4_cards {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::FourLeafClover,
            description: UpgradeInfoDescription::Single(
                UpgradeBoardText::ShortenStraightFlushTo4Cards,
            ),
        });
    }

    if state.skip_rank_for_straight {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::Rabbit,
            description: UpgradeInfoDescription::Single(UpgradeBoardText::SkipRankForStraight),
        });
    }

    if state.treat_suits_as_same {
        infos.push(UpgradeInfo {
            upgrade_kind: UpgradeKind::BlackWhite,
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
                TowerSelectUpgradeTarget::LowCard => UpgradeKind::Spoon {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
                TowerSelectUpgradeTarget::NoReroll => UpgradeKind::PerfectPottery {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
                TowerSelectUpgradeTarget::Reroll => UpgradeKind::BrokenPottery {
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
            TowerUpgradeTarget::Suit { suit } => UpgradeBoardText::TowerUpgradeSuit {
                name: suit.to_string(),
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
            TowerUpgradeTarget::Suit { suit } => match suit {
                crate::card::Suit::Diamonds => UpgradeKind::CainSword {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
                crate::card::Suit::Spades => UpgradeKind::LongSword {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
                crate::card::Suit::Hearts => UpgradeKind::Mace {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
                crate::card::Suit::Clubs => UpgradeKind::ClubSword {
                    damage_multiplier: tower_upgrade_state.damage_multiplier,
                },
            },
            TowerUpgradeTarget::EvenOdd { even } => {
                if *even {
                    UpgradeKind::PairChopsticks {
                        damage_multiplier: tower_upgrade_state.damage_multiplier,
                    }
                } else {
                    UpgradeKind::SingleChopstick {
                        damage_multiplier: tower_upgrade_state.damage_multiplier,
                    }
                }
            }
            TowerUpgradeTarget::FaceNumber { face } => {
                if *face {
                    UpgradeKind::Brush {
                        damage_multiplier: tower_upgrade_state.damage_multiplier,
                    }
                } else {
                    UpgradeKind::FountainPen {
                        damage_multiplier: tower_upgrade_state.damage_multiplier,
                    }
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
