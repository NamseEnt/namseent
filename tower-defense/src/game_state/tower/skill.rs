use super::*;
use crate::{SimTick, SimTickSpan};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct TowerSkillTemplate {
    pub kind: TowerSkillKind,
    pub cooldown: SimTickSpan,
    pub duration: SimTickSpan,
}
impl TowerSkillTemplate {
    pub fn new_passive(kind: TowerSkillKind) -> Self {
        Self {
            kind,
            cooldown: SimTickSpan::from_millis_ceil(1_000),
            duration: SimTickSpan::from_millis_ceil(1_000),
        }
    }
}

#[derive(Clone, PartialEq, State)]
pub struct TowerSkill {
    pub last_used_at: SimTick,
    pub template: TowerSkillTemplate,
}

impl TowerSkill {
    pub fn new(template: TowerSkillTemplate, sim_tick: SimTick) -> Self {
        Self {
            last_used_at: sim_tick,
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
}

impl TowerStatusEffectKind {
    pub fn affects_damage(&self) -> bool {
        matches!(
            self,
            TowerStatusEffectKind::DamageMul { .. } | TowerStatusEffectKind::DamageAdd { .. }
        )
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub enum TowerStatusEffectEnd {
    Time { end_at: SimTick },
    NeverEnd,
}

pub fn remove_tower_finished_status_effects(game_state: &mut GameState, sim_tick: SimTick) {
    let upgrade_revision = game_state.upgrade_state.revision;
    let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();

    for tower in game_state.towers.iter_mut() {
        let mut removed_damage_effect = false;
        tower.status_effects.retain(|e| {
            let keep = match e.end_at {
                TowerStatusEffectEnd::Time { end_at } => sim_tick < end_at,
                TowerStatusEffectEnd::NeverEnd => true,
            };
            if !keep && e.kind.affects_damage() {
                removed_damage_effect = true;
            }
            keep
        });

        if removed_damage_effect {
            tower.refresh_cached_upgrade_damage(upgrade_revision, &upgrade_bonuses);
        }
    }
}

pub fn activate_tower_skills(game_state: &mut GameState, sim_tick: SimTick) {
    let mut activated_skills = vec![];

    for tower in game_state.towers.iter_mut() {
        for skill in tower.skills.iter_mut() {
            if sim_tick < skill.last_used_at + skill.cooldown {
                continue;
            }

            skill.last_used_at = sim_tick;
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

        let upgrade_revision = game_state.upgrade_state.revision;
        let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();

        let mut on_nearby_towers = |range_radius: f32, effect: TowerStatusEffect| {
            for tower in game_state.towers.iter_mut() {
                if caster_xy.distance(tower.center_xy_f32()) <= range_radius {
                    let affects_damage = effect.kind.affects_damage();
                    tower.status_effects.push(effect.clone());
                    if affects_damage {
                        tower.refresh_cached_upgrade_damage(upgrade_revision, &upgrade_bonuses);
                    }
                }
            }
        };

        let mut on_nearby_monsters = |range_radius: f32, effect: MonsterStatusEffect| {
            for monster in game_state.monsters.iter_mut() {
                if caster_xy.distance(monster.center_xy_tile()) <= range_radius {
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
                            end_at: sim_tick + skill.duration,
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
                            end_at: sim_tick + skill.duration,
                        },
                    },
                );
            }
            TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => {
                on_nearby_monsters(
                    range_radius,
                    MonsterStatusEffect {
                        kind: MonsterStatusEffectKind::SpeedMul { mul },
                        end_at: sim_tick + skill.duration,
                    },
                );
            }
            TowerSkillKind::MoneyIncomeAdd { .. } => {}
            TowerSkillKind::TopCardBonus { bonus_damage, .. } => {
                if bonus_damage > 0
                    && let Some(tower) = game_state
                        .towers
                        .iter_mut()
                        .find(|tower| tower.id == tower_id)
                {
                    let effect = TowerStatusEffect {
                        kind: TowerStatusEffectKind::DamageAdd {
                            add: bonus_damage as f32,
                        },
                        end_at: TowerStatusEffectEnd::Time {
                            end_at: sim_tick + skill.duration,
                        },
                    };
                    tower.status_effects.push(effect.clone());
                    tower.refresh_cached_upgrade_damage(upgrade_revision, &upgrade_bonuses);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SimTick, SimTickSpan};

    use crate::game_state::effect::tests_support::make_test_state;

    #[test]
    fn top_card_bonus_activates_damage_add_status_effect() {
        let mut game_state = make_test_state();
        let now = SimTick::from_ticks(120);

        let mut tower = Tower::new(
            &TowerTemplate::new(TowerKind::RubberCone, Suit::Spades, Rank::Ace),
            MapCoord::new(0, 0),
            now,
        );

        tower.skills.push(TowerSkill::new(
            TowerSkillTemplate {
                kind: TowerSkillKind::TopCardBonus {
                    rank: Rank::Ace,
                    bonus_damage: 15,
                },
                cooldown: SimTickSpan::from_millis_ceil(1_000),
                duration: SimTickSpan::from_millis_ceil(1_000),
            },
            SimTick::ZERO,
        ));

        game_state.towers.place_tower(tower);
        activate_tower_skills(&mut game_state, now);

        let tower = game_state.towers.iter().next().expect("tower should exist");
        assert!(tower.status_effects.iter().any(|effect| {
            matches!(
                effect.kind,
                TowerStatusEffectKind::DamageAdd { add }
                if add == 15.0_f32
            )
        }));
    }

    #[test]
    fn top_card_bonus_updates_cached_upgrade_damage() {
        let mut game_state = make_test_state();
        let now = SimTick::from_ticks(120);
        let bonus_damage = 15;

        let mut tower = Tower::new(
            &TowerTemplate::new(TowerKind::RubberCone, Suit::Spades, Rank::Ace),
            MapCoord::new(0, 0),
            now,
        );

        tower.skills.push(TowerSkill::new(
            TowerSkillTemplate {
                kind: TowerSkillKind::TopCardBonus {
                    rank: Rank::Ace,
                    bonus_damage,
                },
                cooldown: SimTickSpan::from_millis_ceil(1_000),
                duration: SimTickSpan::from_millis_ceil(1_000),
            },
            SimTick::ZERO,
        ));

        game_state.towers.place_tower(tower);
        activate_tower_skills(&mut game_state, now);

        let tower = game_state.towers.iter().next().expect("tower should exist");
        assert_eq!(tower.cached_upgrade_damage(), bonus_damage as f32);
    }
}
