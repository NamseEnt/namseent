use crate::game_state::GameState;
use crate::{SimTick, SimTickSpan};
use namui::*;

use crate::game_state::tower::{Tower, TowerKind};

pub fn tower_animation_tick(game_state: &mut GameState, sim_tick: SimTick) {
    const STIFFNESS: f32 = -1500.0;
    const DAMPING: f32 = -10.0;
    const DELTA_TIME_SECONDS: f32 = 1.0 / 60.0;

    game_state.towers.iter_mut().for_each(|tower| {
        let Tower {
            animation,
            template,
            ..
        } = tower;
        let kind = template.kind;
        if let TowerKind::RubberCone = kind {
            return;
        }

        let delta_time = DELTA_TIME_SECONDS;
        animation.tick_at = sim_tick;

        if sim_tick - animation.transited_at > animation.duration() {
            animation.transition(
                match animation.kind {
                    AnimationKind::Idle1 => AnimationKind::Idle2,
                    AnimationKind::Idle2 => AnimationKind::Idle1,
                    AnimationKind::Attack => AnimationKind::Idle1,
                },
                sim_tick,
            );
        }

        let transit_force_expired = animation
            .transit_force
            .is_some_and(|transit_force| transit_force.end_at < sim_tick);
        let transit_force = animation
            .transit_force
            .map(|transit_force| transit_force.force)
            .unwrap_or(0.0);
        let spring_force = STIFFNESS * animation.y_ratio_offset;
        let damping_force = DAMPING * animation.y_ratio_velocity;
        let acceleration = spring_force + damping_force + transit_force;
        animation.y_ratio_velocity += acceleration * delta_time;
        animation.y_ratio_offset += animation.y_ratio_velocity * delta_time;

        if transit_force_expired {
            animation.transit_force = None;
        }
    });
}

#[derive(Clone, PartialEq, State)]
pub(crate) struct Animation {
    pub(crate) kind: AnimationKind,
    pub(crate) transited_at: SimTick,
    transit_force: Option<TransitForce>,
    pub(crate) tick_at: SimTick,
    pub(crate) y_ratio_offset: f32,
    pub(crate) y_ratio_velocity: f32,
}

impl Animation {
    pub(crate) fn new(sim_tick: SimTick) -> Self {
        Self {
            kind: AnimationKind::Idle1,
            transited_at: sim_tick,
            transit_force: None,
            tick_at: sim_tick,
            y_ratio_offset: 0.0,
            y_ratio_velocity: 0.0,
        }
    }

    pub(crate) fn transition(&mut self, kind: AnimationKind, sim_tick: SimTick) {
        const IDLE_TRANSIT_FORCE: f32 = -100.0;
        const ATTACK_TRANSIT_FORCE: f32 = -500.0;
        const FORCE_DURATION: SimTickSpan = SimTickSpan::from_millis_ceil(33);

        if let AnimationKind::Attack = kind {
            self.transit_force = Some(TransitForce {
                force: ATTACK_TRANSIT_FORCE,
                end_at: sim_tick + FORCE_DURATION,
            });
        } else if let AnimationKind::Attack = self.kind {
        } else {
            self.transit_force = Some(TransitForce {
                force: IDLE_TRANSIT_FORCE,
                end_at: sim_tick + FORCE_DURATION,
            });
        }

        self.kind = kind;
        self.transited_at = sim_tick;
    }

    fn duration(&self) -> SimTickSpan {
        self.kind.duration()
    }
}

#[derive(Clone, Copy, PartialEq, State)]
struct TransitForce {
    force: f32,
    end_at: SimTick,
}

#[derive(Clone, Copy, PartialEq, State)]
pub enum AnimationKind {
    Idle1,
    Idle2,
    Attack,
}

impl AnimationKind {
    fn duration(&self) -> SimTickSpan {
        match self {
            Self::Idle1 => SimTickSpan::from_millis_ceil(1500),
            Self::Idle2 => SimTickSpan::from_millis_ceil(1500),
            Self::Attack => SimTickSpan::from_millis_ceil(333),
        }
    }
}
