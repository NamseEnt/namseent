use crate::card::{Rank, Suit};

#[derive(Debug, Clone)]
pub enum Upgrade {
    Tower {
        target: TowerUpgradeTarget,
        upgrade: TowerUpgrade,
    },
    ShopSlot {
        extra_slot: usize,
    },
    QuestSlot {
        extra_slot: usize,
    },
    QuestBoardSlot {
        extra_slot: usize,
    },
    Reroll {
        extra_reroll: usize,
    },
}
pub fn merge_or_append_upgrade(upgrades: &mut Vec<Upgrade>, upgrade: Upgrade) {
    match &upgrade {
        Upgrade::Tower {
            target,
            upgrade: tower_upgrade,
        } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::Tower {
                    target: existing_target,
                    upgrade: existing_upgrade,
                } = existing_upgrade
                else {
                    continue;
                };

                if target != existing_target {
                    continue;
                }

                match (tower_upgrade, existing_upgrade) {
                    (
                        TowerUpgrade::DamagePlus { damage },
                        TowerUpgrade::DamagePlus {
                            damage: existing_damage,
                        },
                    ) => {
                        *existing_damage += damage;
                        return;
                    }
                    (
                        TowerUpgrade::DamageMultiplier { multiplier },
                        TowerUpgrade::DamageMultiplier {
                            multiplier: existing_multiplier,
                        },
                    ) => {
                        *existing_multiplier *= multiplier;
                        return;
                    }
                    (
                        TowerUpgrade::SpeedPlus { speed },
                        TowerUpgrade::SpeedPlus {
                            speed: existing_speed,
                        },
                    ) => {
                        *existing_speed += speed;
                        return;
                    }
                    (
                        TowerUpgrade::SpeedMultiplier { multiplier },
                        TowerUpgrade::SpeedMultiplier {
                            multiplier: existing_multiplier,
                        },
                    ) => {
                        *existing_multiplier *= multiplier;
                        return;
                    }
                    (
                        TowerUpgrade::RangePlus { range },
                        TowerUpgrade::RangePlus {
                            range: existing_range,
                        },
                    ) => {
                        *existing_range += range;
                        return;
                    }
                    _ => {}
                }
            }
        }
        Upgrade::ShopSlot { extra_slot } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::ShopSlot {
                    extra_slot: existing_extra_slot,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_slot += extra_slot;
                return;
            }
        }
        Upgrade::QuestSlot { extra_slot } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::QuestSlot {
                    extra_slot: existing_extra_slot,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_slot += extra_slot;
                return;
            }
        }
        Upgrade::QuestBoardSlot { extra_slot } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::QuestBoardSlot {
                    extra_slot: existing_extra_slot,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_slot += extra_slot;
                return;
            }
        }
        Upgrade::Reroll { extra_reroll } => {
            for existing_upgrade in upgrades.iter_mut() {
                let Upgrade::Reroll {
                    extra_reroll: existing_extra_reroll,
                } = existing_upgrade
                else {
                    continue;
                };

                *existing_extra_reroll += extra_reroll;
                return;
            }
        }
    }

    upgrades.push(upgrade);
}
impl Upgrade {
    pub fn name(&self) -> &'static str {
        match self {
            Upgrade::Tower { .. } => "타워 업그레이드",
            Upgrade::ShopSlot { .. } => "상점 슬롯 확장",
            Upgrade::QuestSlot { .. } => "퀘스트 슬롯 확장",
            Upgrade::QuestBoardSlot { .. } => "퀘스트 게시판 슬롯 확장",
            Upgrade::Reroll { .. } => "리롤 횟수 증가가",
        }
    }
    pub fn description(&self) -> String {
        match self {
            Upgrade::Tower { target, upgrade } => {
                let mut description = String::new();
                match target {
                    TowerUpgradeTarget::Rank { rank } => {
                        description.push_str(&format!("{}", rank));
                    }
                    TowerUpgradeTarget::Suit { suit } => {
                        description.push_str(&format!("{}", suit));
                    }
                }
                description.push_str("타워의 ");
                match upgrade {
                    TowerUpgrade::DamagePlus { damage } => {
                        description.push_str(&format!("공격력을 {}만큼 증가시킵니다.", damage));
                    }
                    TowerUpgrade::DamageMultiplier { multiplier } => {
                        description
                            .push_str(&format!("공격력을 {}배 만큼 증가시킵니다.", multiplier));
                    }
                    TowerUpgrade::SpeedPlus { speed } => {
                        description.push_str(&format!("공격 속도를 {}만큼 증가시킵니다.", speed));
                    }
                    TowerUpgrade::SpeedMultiplier { multiplier } => {
                        description
                            .push_str(&format!("공격 속도를 {}배 만큼 증가시킵니다.", multiplier));
                    }
                    TowerUpgrade::RangePlus { range } => {
                        description.push_str(&format!("공격 범위를 {}만큼 증가시킵니다.", range));
                    }
                }
                description
            }
            Upgrade::ShopSlot { .. } => "상점 슬롯을 확장합니다.".to_string(),
            Upgrade::QuestSlot { .. } => "퀘스트 슬롯을 확장합니다.".to_string(),
            Upgrade::QuestBoardSlot { .. } => "퀘스트 게시판 슬롯을 확장합니다.".to_string(),
            Upgrade::Reroll { .. } => "리롤 횟수를 증가시킵니다.".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TowerUpgradeTarget {
    Rank { rank: Rank },
    Suit { suit: Suit },
}
#[derive(Debug, Clone)]
pub enum TowerUpgrade {
    DamagePlus { damage: f32 },
    DamageMultiplier { multiplier: f32 },
    SpeedPlus { speed: f32 },
    SpeedMultiplier { multiplier: f32 },
    RangePlus { range: f32 },
}
