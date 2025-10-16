use super::*;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct TowerSkillTemplate {
    pub kind: TowerSkillKind,
    pub cooldown: Duration,
    pub duration: Duration,
}
impl TowerSkillTemplate {
    pub fn new_passive(kind: TowerSkillKind) -> Self {
        Self {
            kind,
            cooldown: Duration::from_secs(1),
            duration: Duration::from_secs(1),
        }
    }
}

#[derive(Clone, PartialEq, State)]
pub struct TowerSkill {
    pub last_used_at: Instant,
    pub template: TowerSkillTemplate,
}

impl TowerSkill {
    pub fn new(template: TowerSkillTemplate, now: Instant) -> Self {
        Self {
            last_used_at: now,
            template,
        }
    }
}

impl Deref for TowerSkill {
    type Target = TowerSkillTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Clone, Copy, PartialEq, Debug, State)]
pub enum TowerSkillKind {
    NearbyTowerDamageMul { mul: f32, range_radius: f32 },
    NearbyTowerDamageAdd { add: f32, range_radius: f32 },
    NearbyTowerAttackSpeedAdd { add: f32, range_radius: f32 },
    NearbyTowerAttackSpeedMul { mul: f32, range_radius: f32 },
    NearbyTowerAttackRangeAdd { add: f32, range_radius: f32 },
    NearbyMonsterSpeedMul { mul: f32, range_radius: f32 },
    MoneyIncomeAdd { add: u32 },
    TopCardBonus { rank: Rank, bonus_damage: usize },
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct TowerStatusEffect {
    pub kind: TowerStatusEffectKind,
    pub end_at: TowerStatusEffectEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, State)]
pub enum TowerStatusEffectKind {
    DamageMul { mul: f32 },
    DamageAdd { add: f32 },
    AttackSpeedMul { mul: f32 },
    AttackSpeedAdd { add: f32 },
    AttackRangeAdd { add: f32 },
}

#[derive(Debug, Clone, PartialEq, State)]
pub enum TowerStatusEffectEnd {
    Time { end_at: Instant },
    NeverEnd,
}

pub fn remove_tower_finished_status_effects(game_state: &mut GameState, now: Instant) {
    for tower in game_state.towers.iter_mut() {
        tower.status_effects.retain(|e| match e.end_at {
            TowerStatusEffectEnd::Time { end_at } => now < end_at,
            TowerStatusEffectEnd::NeverEnd => true,
        });
    }
}

pub fn activate_tower_skills(game_state: &mut GameState, now: Instant) {
    let mut activated_skills = vec![];

    for tower in game_state.towers.iter_mut() {
        for skill in tower.skills.iter_mut() {
            if now < skill.last_used_at + skill.cooldown {
                continue;
            }

            skill.last_used_at = now;
            activated_skills.push((tower.id, skill.template));
        }
    }

    for (tower_id, skill) in activated_skills {
        let caster_xy = game_state
            .towers
            .iter()
            .find(|m| m.id == tower_id)
            .unwrap()
            .center_xy_f32();

        let mut on_nearby_towers = |range_radius: f32, effect: TowerStatusEffect| {
            for tower in game_state.towers.iter_mut() {
                if caster_xy.distance(tower.center_xy_f32()) <= range_radius {
                    tower.status_effects.push(effect.clone());
                }
            }
        };

        let mut on_nearby_monsters = |range_radius: f32, effect: MonsterStatusEffect| {
            for monster in game_state.monsters.iter_mut() {
                if caster_xy.distance(monster.xy()) <= range_radius {
                    monster.status_effects.push(effect.clone());
                }
            }
        };
        match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { mul, range_radius } => {
                on_nearby_towers(
                    range_radius,
                    TowerStatusEffect {
                        kind: TowerStatusEffectKind::DamageMul { mul },
                        end_at: TowerStatusEffectEnd::Time {
                            end_at: now + skill.duration,
                        },
                    },
                );
            }
            TowerSkillKind::NearbyTowerDamageAdd { add, range_radius } => {
                on_nearby_towers(
                    range_radius,
                    TowerStatusEffect {
                        kind: TowerStatusEffectKind::DamageAdd { add },
                        end_at: TowerStatusEffectEnd::Time {
                            end_at: now + skill.duration,
                        },
                    },
                );
            }
            TowerSkillKind::NearbyTowerAttackSpeedAdd { add, range_radius } => {
                on_nearby_towers(
                    range_radius,
                    TowerStatusEffect {
                        kind: TowerStatusEffectKind::AttackSpeedAdd { add },
                        end_at: TowerStatusEffectEnd::Time {
                            end_at: now + skill.duration,
                        },
                    },
                );
            }
            TowerSkillKind::NearbyTowerAttackSpeedMul { mul, range_radius } => {
                on_nearby_towers(
                    range_radius,
                    TowerStatusEffect {
                        kind: TowerStatusEffectKind::AttackSpeedMul { mul },
                        end_at: TowerStatusEffectEnd::Time {
                            end_at: now + skill.duration,
                        },
                    },
                );
            }
            TowerSkillKind::NearbyTowerAttackRangeAdd { add, range_radius } => {
                on_nearby_towers(
                    range_radius,
                    TowerStatusEffect {
                        kind: TowerStatusEffectKind::AttackRangeAdd { add },
                        end_at: TowerStatusEffectEnd::Time {
                            end_at: now + skill.duration,
                        },
                    },
                );
            }
            TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => {
                on_nearby_monsters(
                    range_radius,
                    MonsterStatusEffect {
                        kind: MonsterStatusEffectKind::SpeedMul { mul },
                        end_at: now + skill.duration,
                    },
                );
            }
            TowerSkillKind::MoneyIncomeAdd { .. } => {}
            TowerSkillKind::TopCardBonus { .. } => {}
        }
    }
}
