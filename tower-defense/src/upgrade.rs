use crate::card::{Rank, Suit};

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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TowerUpgradeTarget {
    Rank { rank: Rank },
    Suit { suit: Suit },
}
pub enum TowerUpgrade {
    DamagePlus { damage: f32 },
    DamageMultiplier { multiplier: f32 },
    SpeedPlus { speed: f32 },
    SpeedMultiplier { multiplier: f32 },
    RangePlus { range: f32 },
}
