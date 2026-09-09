mod monster_hp_bar;
mod monster_kind;
mod monster_template;
mod render;
pub mod skill;

use crate::{
    Damage, FixedRatio, Health, MapCoordF32, MonsterId, WorldCoord, WorldVec,
    game_state::{monster::render::MonsterAnimation, projectile::ProjectileTargetIndicator},
    route::MoveOnRoute,
};
#[cfg(test)]
use crate::{SimTick, route::Route};
pub use monster_kind::MonsterKind;
pub use monster_template::MonsterTemplate;
use namui::*;
pub(crate) use render::RenderMonsterPose;
pub use render::{monster_animation_tick, monster_wh};
pub use skill::{MonsterSkill, MonsterSkillTemplate, MonsterStatusEffect, MonsterStatusEffectKind};
#[cfg(test)]
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

#[derive(Clone, PartialEq)]
pub(crate) struct MonsterPresentationState {
    animation: MonsterAnimation,
    projectile_target_indicator: ProjectileTargetIndicator,
}

impl Monster {
    #[cfg(test)]
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

    pub(crate) fn presentation_state(&self) -> MonsterPresentationState {
        MonsterPresentationState {
            animation: self.animation.clone(),
            projectile_target_indicator: self.projectile_target_indicator,
        }
    }

    pub(crate) fn restore_presentation_state(&mut self, state: MonsterPresentationState) {
        self.animation = state.animation;
        self.projectile_target_indicator = state.projectile_target_indicator;
    }

    pub(crate) fn to_core_state(&self) -> td_core::MonsterState {
        td_core::MonsterState {
            id: self.id.raw(),
            move_on_route: self.move_on_route.to_core_state(),
            kind: self.kind.to_core_raw(),
            hp_raw: self.hp.raw(),
            max_hp_raw: self.max_hp.raw(),
            stage_progress_counted: self.stage_progress_counted,
            skills: self
                .skills
                .iter()
                .map(MonsterSkill::to_core_skill)
                .collect(),
            status_effects: self
                .status_effects
                .iter()
                .map(MonsterStatusEffect::to_core_status_effect)
                .collect(),
            damage_raw: self.damage.raw(),
            reward: self.reward,
        }
    }

    pub(crate) fn from_core_state(state: td_core::MonsterState) -> Option<Self> {
        Some(Self {
            id: MonsterId::from_raw(state.id),
            move_on_route: MoveOnRoute::from_core_state(state.move_on_route)?,
            kind: MonsterKind::from_core_raw(state.kind)?,
            projectile_target_indicator: ProjectileTargetIndicator::from_id(MonsterId::from_raw(
                state.id,
            )),
            hp: Health::from_raw(state.hp_raw),
            max_hp: Health::from_raw(state.max_hp_raw),
            stage_progress_counted: state.stage_progress_counted,
            #[cfg(feature = "debug-tools")]
            base_max_hp: Health::from_raw(state.max_hp_raw),
            skills: state
                .skills
                .into_iter()
                .map(MonsterSkill::from_core_skill)
                .collect(),
            status_effects: state
                .status_effects
                .into_iter()
                .map(MonsterStatusEffect::from_core_status_effect)
                .collect(),
            damage: Damage::from_raw(state.damage_raw),
            reward: state.reward,
            animation: MonsterAnimation::new(),
        })
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
            game_state.route.clone(),
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

    #[test]
    fn monster_status_effect_snapshot_round_trips_without_host_types() {
        let effects = [
            MonsterStatusEffect {
                kind: MonsterStatusEffectKind::SpeedMul {
                    mul: FixedRatio::from_raw(750_000),
                },
                end_at: SimTick::from_ticks(12),
            },
            MonsterStatusEffect {
                kind: MonsterStatusEffectKind::Invincible,
                end_at: SimTick::from_ticks(24),
            },
            MonsterStatusEffect {
                kind: MonsterStatusEffectKind::ImmuneToSlow,
                end_at: SimTick::from_ticks(36),
            },
        ];

        let snapshots = effects
            .iter()
            .map(MonsterStatusEffect::to_core_status_effect)
            .collect::<Vec<_>>();
        let restored = snapshots
            .into_iter()
            .map(MonsterStatusEffect::from_core_status_effect)
            .collect::<Vec<_>>();

        assert_eq!(restored[0].end_at, effects[0].end_at);
        assert_eq!(restored[1].end_at, effects[1].end_at);
        assert_eq!(restored[2].end_at, effects[2].end_at);
        assert!(matches!(
            restored[0].kind,
            MonsterStatusEffectKind::SpeedMul { mul }
                if mul == FixedRatio::from_raw(750_000)
        ));
        assert!(matches!(
            restored[1].kind,
            MonsterStatusEffectKind::Invincible
        ));
        assert!(matches!(
            restored[2].kind,
            MonsterStatusEffectKind::ImmuneToSlow
        ));
    }

    #[test]
    fn headed_monster_exports_authoritative_snapshot() {
        let game_state = crate::game_state::create_game_state_with_seed(19);
        let template = MonsterTemplate::new(MonsterKind::Mob01, &game_state.config);
        let monster = Monster::new_with_id(
            &template,
            game_state.route.clone(),
            SimTick::from_ticks(7),
            &crate::RatioProduct::one(),
            MonsterId::from_raw(9),
        );

        let snapshot = monster.to_core_state();
        assert_eq!(snapshot.id, 9);
        assert_eq!(snapshot.kind, MonsterKind::Mob01.to_core_raw());
        assert_eq!(snapshot.hp_raw, snapshot.max_hp_raw);
        assert_eq!(
            snapshot.move_on_route.route,
            game_state.route.to_core_state()
        );
        assert_eq!(snapshot.damage_raw, template.damage.raw());
        assert_eq!(snapshot.reward, template.reward);

        let restored = Monster::from_core_state(snapshot).expect("valid monster snapshot");
        assert_eq!(restored.id(), MonsterId::from_raw(9));
        assert_eq!(restored.kind, MonsterKind::Mob01);
        assert_eq!(restored.hp, template.max_hp);
        assert_eq!(restored.damage, template.damage);
        assert_eq!(restored.reward, template.reward);

        let mut state = game_state;
        state.monsters.push(restored);
        let snapshots = state.presentation_projection().monster_snapshots();
        let presentation = state.monsters[0].presentation_state();
        assert_eq!(snapshots, vec![state.monsters[0].to_core_state()]);
        assert!(
            state
                .presentation_projection_mut()
                .restore_monster_snapshots(snapshots)
        );
        assert_eq!(state.monsters[0].id(), MonsterId::from_raw(9));
        assert!(state.monsters[0].presentation_state() == presentation);

        let mut invalid = state.presentation_projection().monster_snapshots();
        invalid[0].kind = u8::MAX;
        assert!(
            !state
                .presentation_projection_mut()
                .restore_monster_snapshots(invalid)
        );
        assert_eq!(state.monsters[0].id(), MonsterId::from_raw(9));

        let mut duplicate = state.presentation_projection().monster_snapshots();
        duplicate.push(duplicate[0].clone());
        assert!(
            !state
                .presentation_projection_mut()
                .restore_monster_snapshots(duplicate)
        );
        assert_eq!(state.monsters.len(), 1);
        assert_eq!(state.monsters[0].id(), MonsterId::from_raw(9));
    }

    #[test]
    fn monster_snapshot_restore_preserves_legacy_bincode_bytes() {
        let game_state = crate::game_state::create_game_state_with_seed(29);
        let template = MonsterTemplate::new(MonsterKind::Mob01, &game_state.config);
        let monster = Monster::new_with_id(
            &template,
            game_state.route.clone(),
            SimTick::from_ticks(11),
            &crate::RatioProduct::one(),
            MonsterId::from_raw(13),
        );
        let original = namui::bincode::encode_to_vec(&monster, namui::bincode::config::standard())
            .expect("monster encoding");
        let restored =
            Monster::from_core_state(monster.to_core_state()).expect("valid monster snapshot");
        let snapshot = namui::bincode::encode_to_vec(&restored, namui::bincode::config::standard())
            .expect("restored monster encoding");

        assert_eq!(original, snapshot);
    }
}
