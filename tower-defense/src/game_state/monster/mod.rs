mod monster_hp_bar;
mod monster_kind;
mod monster_template;
mod move_monsters;
mod render;
pub mod skill;

use crate::{
    Damage, FixedRatio, Health, MapCoordF32, MonsterId, SimTick, WorldCoord, WorldVec,
    game_state::{monster::render::MonsterAnimation, projectile::ProjectileTargetIndicator},
    route::{MoveOnRoute, Route},
};
pub use monster_kind::MonsterKind;
pub use monster_template::MonsterTemplate;
pub use move_monsters::{move_monsters, resolve_base_damage};
use namui::*;
pub use render::{monster_animation_tick, monster_wh};
#[allow(unused_imports)]
pub use skill::{
    MonsterSkill, MonsterSkillTemplate, MonsterStatusEffect, MonsterStatusEffectKind,
    activate_monster_skills, remove_monster_finished_status_effects,
};
use std::sync::Arc;

const MONSTER_HP_BAR_HEIGHT: Px = px(4.);

#[derive(State, Clone)]
pub struct Monster {
    id: MonsterId,
    pub move_on_route: MoveOnRoute,
    pub kind: MonsterKind,
    pub projectile_target_indicator: ProjectileTargetIndicator,
    pub hp: Health,
    pub max_hp: Health,
    pub stage_progress_counted: bool,
    #[cfg(feature = "debug-tools")]
    pub base_max_hp: Health,
    pub skills: Vec<MonsterSkill>,
    pub status_effects: Vec<MonsterStatusEffect>,
    pub damage: Damage,
    pub reward: usize,
    pub animation: MonsterAnimation,
}
impl Monster {
    pub(crate) fn new_with_id(
        template: &MonsterTemplate,
        route: Arc<Route>,
        sim_tick: SimTick,
        health_multipliers: &crate::RatioProduct,
        id: MonsterId,
    ) -> Self {
        let adjusted_max_hp = template.max_hp.scaled_by_product(health_multipliers);
        Self {
            id,
            move_on_route: MoveOnRoute::new(route, template.velocity),
            kind: template.kind,
            projectile_target_indicator: ProjectileTargetIndicator::from_id(id),
            hp: adjusted_max_hp,
            max_hp: adjusted_max_hp,
            stage_progress_counted: false,
            #[cfg(feature = "debug-tools")]
            base_max_hp: adjusted_max_hp,
            skills: template
                .skills
                .iter()
                .map(|&t| MonsterSkill::new(t, sim_tick))
                .collect(),
            status_effects: vec![],
            damage: template.damage,
            reward: template.reward,
            animation: MonsterAnimation::new(),
        }
    }
    pub fn get_damage(&mut self, damage: Damage) {
        if self.dead()
            || self.status_effects.iter().any(|status_effect| {
                matches!(status_effect.kind, MonsterStatusEffectKind::Invincible)
            })
        {
            return;
        }

        self.hp = self.hp.saturating_sub(Health::from_raw(damage.raw()));
    }
    pub fn heal(&mut self, amount: Health) {
        if self.dead() {
            return;
        }

        self.hp = self.hp.saturating_add(amount);
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
    pub fn get_damage_to_user(&self) -> Damage {
        // weaken or strengthen the damage
        self.damage
    }

    pub fn dead(&self) -> bool {
        self.hp.is_zero()
    }

    pub fn xy(&self) -> MapCoordF32 {
        self.move_on_route.xy()
    }

    pub fn world_xy(&self) -> WorldCoord {
        self.move_on_route.world_xy()
    }

    /// 몬스터의 중심점 (타일 단위) - 프로젝타일/레이저 유도용
    pub fn center_xy_tile(&self) -> MapCoordF32 {
        self.center_world_xy().as_map_coord_f32()
    }

    pub fn center_world_xy(&self) -> WorldCoord {
        self.world_xy()
            + WorldVec::new(
                crate::world::WORLD_UNITS_PER_TILE / 2,
                crate::world::WORLD_UNITS_PER_TILE / 2,
            )
    }

    pub fn id(&self) -> MonsterId {
        self.id
    }
    pub fn get_speed_multiplier(&self) -> FixedRatio {
        let is_immune_to_slow = self.status_effects.iter().any(|status_effect| {
            matches!(status_effect.kind, MonsterStatusEffectKind::ImmuneToSlow)
        });
        let factors = self.status_effects.iter().filter_map(|status_effect| {
            let MonsterStatusEffectKind::SpeedMul { mul } = status_effect.kind else {
                return None;
            };
            if is_immune_to_slow && mul < FixedRatio::ONE {
                return None;
            }
            Some(mul)
        });
        FixedRatio::from_raw(
            crate::RatioProduct::one()
                .with_all(factors)
                .apply_raw(FixedRatio::ONE.raw()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monster_with_speed_effects(multipliers: impl IntoIterator<Item = FixedRatio>) -> Monster {
        let game_state = crate::game_state::create_game_state_with_seed(19);
        let template = MonsterTemplate::new(MonsterKind::Mob01, &game_state.config);
        let mut monster = Monster::new_with_id(
            &template,
            game_state.route,
            SimTick::ZERO,
            &crate::RatioProduct::one(),
            crate::MonsterId::from_raw(1),
        );
        monster.status_effects = multipliers
            .into_iter()
            .map(|mul| MonsterStatusEffect {
                kind: MonsterStatusEffectKind::SpeedMul { mul },
                end_at: SimTick::from_ticks(100),
            })
            .collect();
        monster
    }

    #[test]
    fn speed_multiplier_is_independent_of_status_insertion_order() {
        let factors = [
            FixedRatio::from_raw(1_234_567),
            FixedRatio::from_raw(765_433),
            FixedRatio::from_raw(1_000_003),
        ];
        let forward = monster_with_speed_effects(factors);
        let reversed = monster_with_speed_effects(factors.into_iter().rev());

        assert_eq!(
            forward.get_speed_multiplier(),
            reversed.get_speed_multiplier()
        );
    }
}
