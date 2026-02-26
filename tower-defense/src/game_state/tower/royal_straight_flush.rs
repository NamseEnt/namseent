use super::Tower;
use crate::game_state::GameState;
use crate::game_state::Monster;
use crate::game_state::field_particle::emitter::{
    BlackSmokeSource, spawn_black_smoke_burst, spawn_black_smoke_burst_reversed,
    spawn_black_smoke_dash_trail,
};
use namui::*;
use rand::Rng;

const ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MIN: f32 = 2.5;
const ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MAX: f32 = 3.5;
const ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE: f32 = 2.0;

#[derive(Clone, PartialEq, State)]
pub struct RoyalStraightFlushVisual {
    created_at: Instant,
    clones: Vec<RoyalStraightFlushClone>,
    phase: RoyalStraightFlushPhase,
    target_monster_id: usize,
}

#[derive(Clone, PartialEq, State)]
struct RoyalStraightFlushClone {
    spawn_center_xy: (f32, f32),
    end_center_xy: (f32, f32),
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
enum RoyalStraightFlushPhase {
    Spawning,
    Dashing,
    Returning,
    Finished,
}

impl RoyalStraightFlushVisual {
    const FADE_DURATION: Duration = Duration::from_millis(300);
    const DASH_DURATION: Duration = Duration::from_millis(120);

    fn new(
        created_at: Instant,
        clones: Vec<RoyalStraightFlushClone>,
        target_monster_id: usize,
    ) -> Self {
        Self {
            created_at,
            clones,
            phase: RoyalStraightFlushPhase::Spawning,
            target_monster_id,
        }
    }

    fn phase_at(&self, now: Instant) -> RoyalStraightFlushPhase {
        let elapsed = now - self.created_at;
        if elapsed < Self::FADE_DURATION {
            RoyalStraightFlushPhase::Spawning
        } else if elapsed < Self::FADE_DURATION + Self::DASH_DURATION {
            RoyalStraightFlushPhase::Dashing
        } else if elapsed < Self::FADE_DURATION + Self::DASH_DURATION + Self::FADE_DURATION {
            RoyalStraightFlushPhase::Returning
        } else {
            RoyalStraightFlushPhase::Finished
        }
    }

    fn phase_progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        let (phase_elapsed, phase_duration) = match self.phase_at(now) {
            RoyalStraightFlushPhase::Spawning => (elapsed, Self::FADE_DURATION),
            RoyalStraightFlushPhase::Dashing => {
                (elapsed - Self::FADE_DURATION, Self::DASH_DURATION)
            }
            RoyalStraightFlushPhase::Returning => (
                elapsed - Self::FADE_DURATION - Self::DASH_DURATION,
                Self::FADE_DURATION,
            ),
            RoyalStraightFlushPhase::Finished => return 1.0,
        };
        (phase_elapsed.as_secs_f32() / phase_duration.as_secs_f32()).clamp(0.0, 1.0)
    }

    pub fn original_alpha(&self, now: Instant) -> f32 {
        let t = self.phase_progress(now);
        match self.phase_at(now) {
            RoyalStraightFlushPhase::Spawning => 1.0 - t,
            RoyalStraightFlushPhase::Dashing => 0.0,
            RoyalStraightFlushPhase::Returning => t,
            RoyalStraightFlushPhase::Finished => 1.0,
        }
    }

    pub fn clone_alpha(&self, now: Instant) -> f32 {
        let t = self.phase_progress(now);
        match self.phase_at(now) {
            RoyalStraightFlushPhase::Spawning => t,
            RoyalStraightFlushPhase::Dashing => 1.0,
            RoyalStraightFlushPhase::Returning => 1.0 - t,
            RoyalStraightFlushPhase::Finished => 0.0,
        }
    }

    pub fn clone_positions(&self, now: Instant) -> impl Iterator<Item = (f32, f32)> + '_ {
        let phase = self.phase_at(now);
        let eased_t = ease_out_cubic(self.phase_progress(now));
        self.clones.iter().map(move |clone| match phase {
            RoyalStraightFlushPhase::Spawning => clone.spawn_center_xy,
            RoyalStraightFlushPhase::Dashing => {
                lerp_xy(clone.spawn_center_xy, clone.end_center_xy, eased_t)
            }
            RoyalStraightFlushPhase::Returning | RoyalStraightFlushPhase::Finished => {
                clone.end_center_xy
            }
        })
    }

    fn tick(
        &mut self,
        now: Instant,
        tower_center_xy: (f32, f32),
        monsters: &[Monster],
        black_smoke_sources: &mut Vec<BlackSmokeSource>,
    ) {
        let next_phase = self.phase_at(now);
        if next_phase == self.phase {
            return;
        }

        if next_phase == RoyalStraightFlushPhase::Dashing {
            if let Some(monster) = monsters.iter().find(|m| m.id() == self.target_monster_id) {
                let target = monster.center_xy_tile();
                let target_xy = (target.x, target.y);
                for clone in &mut self.clones {
                    let dx = target_xy.0 - clone.spawn_center_xy.0;
                    let dy = target_xy.1 - clone.spawn_center_xy.1;
                    let length = (dx * dx + dy * dy).sqrt().max(0.001);
                    clone.end_center_xy = (
                        target_xy.0
                            + (dx / length) * ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE,
                        target_xy.1
                            + (dy / length) * ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE,
                    );

                    spawn_black_smoke_dash_trail(clone.spawn_center_xy, clone.end_center_xy, now);
                }
            }
            for clone in &self.clones {
                spawn_penetration_effect_dummy(clone.spawn_center_xy, now);
            }
        }

        if next_phase == RoyalStraightFlushPhase::Returning {
            spawn_black_smoke_burst(black_smoke_sources, tower_center_xy, now);
            for clone in &self.clones {
                spawn_black_smoke_burst_reversed(black_smoke_sources, clone.end_center_xy, now);
            }
        }

        self.phase = next_phase;
    }

    fn is_finished(&self, now: Instant) -> bool {
        self.phase_at(now) == RoyalStraightFlushPhase::Finished
    }
}

pub fn royal_straight_flush_hit_delay() -> Duration {
    RoyalStraightFlushVisual::FADE_DURATION + RoyalStraightFlushVisual::DASH_DURATION
}

impl Tower {
    pub fn spawn_royal_straight_flush_visual(
        &mut self,
        target_xy: (f32, f32),
        target_monster_id: usize,
        now: Instant,
        black_smoke_sources: &mut Vec<BlackSmokeSource>,
    ) {
        let tower_center = self.center_xy_f32();
        let tower_center_xy = (tower_center.x, tower_center.y);
        let clones = generate_royal_straight_flush_clones(target_xy);

        spawn_black_smoke_burst_reversed(black_smoke_sources, tower_center_xy, now);
        for clone in &clones {
            spawn_black_smoke_burst(black_smoke_sources, clone.spawn_center_xy, now);
        }

        self.royal_straight_flush_visual = Some(RoyalStraightFlushVisual::new(
            now,
            clones,
            target_monster_id,
        ));
    }

    pub fn royal_straight_flush_visual(&self) -> Option<&RoyalStraightFlushVisual> {
        self.royal_straight_flush_visual.as_ref()
    }

    pub fn has_royal_straight_flush_visual(&self) -> bool {
        self.royal_straight_flush_visual.is_some()
    }
}

pub fn tick_royal_straight_flush_visuals(game_state: &mut GameState, now: Instant) {
    let monsters = &game_state.monsters;
    let black_smoke_sources = &mut game_state.black_smoke_sources;
    for tower in game_state.towers.iter_mut() {
        let tower_center = tower.center_xy_f32();
        let tower_center_xy = (tower_center.x, tower_center.y);

        if let Some(visual) = tower.royal_straight_flush_visual.as_mut() {
            visual.tick(now, tower_center_xy, monsters, black_smoke_sources);
            if visual.is_finished(now) {
                tower.royal_straight_flush_visual = None;
            }
        }
    }
}

fn generate_royal_straight_flush_clones(target_xy: (f32, f32)) -> Vec<RoyalStraightFlushClone> {
    let mut rng = rand::thread_rng();
    let angle1 = rng.gen_range(0.0..std::f32::consts::TAU);
    let separation = rng.gen_range(
        std::f32::consts::FRAC_PI_3..(std::f32::consts::TAU - std::f32::consts::FRAC_PI_3),
    );
    let angle2 = (angle1 + separation) % std::f32::consts::TAU;

    [angle1, angle2]
        .into_iter()
        .map(|angle| {
            let radius = rng.gen_range(
                ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MIN
                    ..ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MAX,
            );
            let spawn_center_xy = (
                target_xy.0 + angle.cos() * radius,
                target_xy.1 + angle.sin() * radius,
            );

            let dx = target_xy.0 - spawn_center_xy.0;
            let dy = target_xy.1 - spawn_center_xy.1;
            let length = (dx * dx + dy * dy).sqrt().max(0.001);
            let end_center_xy = (
                target_xy.0 + (dx / length) * ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE,
                target_xy.1 + (dy / length) * ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE,
            );

            RoyalStraightFlushClone {
                spawn_center_xy,
                end_center_xy,
            }
        })
        .collect()
}

fn lerp_xy(a: (f32, f32), b: (f32, f32), t: f32) -> (f32, f32) {
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn spawn_penetration_effect_dummy(_xy: (f32, f32), _now: Instant) {}
