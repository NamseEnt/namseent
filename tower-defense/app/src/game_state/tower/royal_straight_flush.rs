use crate::game_state::field_particle::emitter::{
    BlackSmokeSource, spawn_black_smoke_burst, spawn_black_smoke_burst_reversed,
    spawn_black_smoke_dash_trail, spawn_black_smoke_puff_burst, spawn_red_slash_marks,
    spawn_yellow_explosion_burst,
};
use crate::game_state::{
    GameState, PresentationEvent, PresentationEventQueue, SoundCue, SoundVolume,
};
use crate::{MonsterId, SimRenderTime, SimTick, SimTickSpan};
use namui::*;

const ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MIN: f32 = 2.5;
const ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MAX: f32 = 3.5;
const ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE: f32 = 2.0;

#[derive(Clone, PartialEq, State)]
pub struct RoyalStraightFlushVisual {
    created_at: SimTick,
    clones: Vec<RoyalStraightFlushClone>,
    phase: RoyalStraightFlushPhase,
    target_monster_id: MonsterId,
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
    const FADE_DURATION: SimTickSpan = SimTickSpan::from_millis_ceil(300);
    const DASH_DURATION: SimTickSpan = SimTickSpan::from_millis_ceil(120);

    fn new(
        created_at: SimTick,
        clones: Vec<RoyalStraightFlushClone>,
        target_monster_id: MonsterId,
    ) -> Self {
        Self {
            created_at,
            clones,
            phase: RoyalStraightFlushPhase::Spawning,
            target_monster_id,
        }
    }

    fn phase_at(&self, sim_tick: SimTick) -> RoyalStraightFlushPhase {
        let elapsed = sim_tick - self.created_at;
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

    fn phase_progress(&self, sim_tick: SimTick) -> f32 {
        let elapsed = sim_tick - self.created_at;
        let (phase_elapsed, phase_duration) = match self.phase_at(sim_tick) {
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

    pub fn original_alpha(&self, sim_tick: SimTick) -> f32 {
        let t = self.phase_progress(sim_tick);
        match self.phase_at(sim_tick) {
            RoyalStraightFlushPhase::Spawning => 1.0 - t,
            RoyalStraightFlushPhase::Dashing => 0.0,
            RoyalStraightFlushPhase::Returning => t,
            RoyalStraightFlushPhase::Finished => 1.0,
        }
    }

    pub fn original_alpha_at(&self, render_time: SimRenderTime) -> f32 {
        let (phase, progress) = self.phase_and_progress_at(render_time);
        match phase {
            RoyalStraightFlushPhase::Spawning => 1.0 - progress,
            RoyalStraightFlushPhase::Dashing => 0.0,
            RoyalStraightFlushPhase::Returning => progress,
            RoyalStraightFlushPhase::Finished => 1.0,
        }
    }

    pub fn clone_alpha(&self, sim_tick: SimTick) -> f32 {
        let t = self.phase_progress(sim_tick);
        match self.phase_at(sim_tick) {
            RoyalStraightFlushPhase::Spawning => t,
            RoyalStraightFlushPhase::Dashing => 1.0,
            RoyalStraightFlushPhase::Returning => 1.0 - t,
            RoyalStraightFlushPhase::Finished => 0.0,
        }
    }

    pub fn clone_alpha_at(&self, render_time: SimRenderTime) -> f32 {
        let (phase, progress) = self.phase_and_progress_at(render_time);
        match phase {
            RoyalStraightFlushPhase::Spawning => progress,
            RoyalStraightFlushPhase::Dashing => 1.0,
            RoyalStraightFlushPhase::Returning => 1.0 - progress,
            RoyalStraightFlushPhase::Finished => 0.0,
        }
    }

    pub fn clone_positions(&self, sim_tick: SimTick) -> impl Iterator<Item = (f32, f32)> + '_ {
        let phase = self.phase_at(sim_tick);
        let eased_t = ease_out_cubic(self.phase_progress(sim_tick));
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

    pub fn clone_positions_at(
        &self,
        render_time: SimRenderTime,
    ) -> impl Iterator<Item = (f32, f32)> + '_ {
        let (phase, progress) = self.phase_and_progress_at(render_time);
        let eased_t = ease_out_cubic(progress);
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

    fn phase_and_progress_at(&self, render_time: SimRenderTime) -> (RoyalStraightFlushPhase, f32) {
        let elapsed_ticks =
            (render_time.tick - self.created_at).ticks() as f32 + render_time.alpha.as_f32();
        let fade_ticks = Self::FADE_DURATION.ticks() as f32;
        let dash_ticks = Self::DASH_DURATION.ticks() as f32;
        let (phase, phase_elapsed, phase_duration) = if elapsed_ticks < fade_ticks {
            (RoyalStraightFlushPhase::Spawning, elapsed_ticks, fade_ticks)
        } else if elapsed_ticks < fade_ticks + dash_ticks {
            (
                RoyalStraightFlushPhase::Dashing,
                elapsed_ticks - fade_ticks,
                dash_ticks,
            )
        } else if elapsed_ticks < fade_ticks + dash_ticks + fade_ticks {
            (
                RoyalStraightFlushPhase::Returning,
                elapsed_ticks - fade_ticks - dash_ticks,
                fade_ticks,
            )
        } else {
            (RoyalStraightFlushPhase::Finished, 1.0, 1.0)
        };
        (phase, (phase_elapsed / phase_duration).clamp(0.0, 1.0))
    }

    fn tick(
        &mut self,
        presentation_events: &mut PresentationEventQueue,
        sim_tick: SimTick,
        presentation_instant: crate::PresentationInstant,
        tower_center_xy: (f32, f32),
        target_xy: Option<(f32, f32)>,
        black_smoke_sources: &mut Vec<BlackSmokeSource>,
    ) {
        let next_phase = self.phase_at(sim_tick);
        if next_phase == self.phase {
            return;
        }

        if next_phase == RoyalStraightFlushPhase::Dashing
            && let Some(target_xy) = target_xy
        {
            for clone in &mut self.clones {
                clone.end_center_xy = compute_pass_through_xy(clone.spawn_center_xy, target_xy);

                spawn_black_smoke_dash_trail(
                    clone.spawn_center_xy,
                    clone.end_center_xy,
                    presentation_instant.as_namui(),
                );
                spawn_red_slash_marks(
                    clone.spawn_center_xy,
                    target_xy,
                    presentation_instant.as_namui(),
                );
            }
            spawn_yellow_explosion_burst(target_xy, presentation_instant.as_namui());
        }

        if next_phase == RoyalStraightFlushPhase::Returning {
            spawn_black_smoke_burst(
                black_smoke_sources,
                tower_center_xy,
                presentation_instant.as_namui(),
            );
            spawn_black_smoke_puff_burst(tower_center_xy, presentation_instant.as_namui());
            for clone in &self.clones {
                spawn_black_smoke_burst_reversed(
                    black_smoke_sources,
                    clone.end_center_xy,
                    presentation_instant.as_namui(),
                );
                spawn_black_smoke_puff_burst(clone.end_center_xy, presentation_instant.as_namui());
                presentation_events.push(PresentationEvent::PlaySoundCue {
                    cue: SoundCue::Wind,
                    position: Some([clone.end_center_xy.0, clone.end_center_xy.1]),
                    volume: SoundVolume::Minimum,
                    max_duration_ms: None,
                });
            }
            presentation_events.push(PresentationEvent::PlaySoundCue {
                cue: SoundCue::Wind,
                position: Some([tower_center_xy.0, tower_center_xy.1]),
                volume: SoundVolume::Minimum,
                max_duration_ms: None,
            });
        }

        self.phase = next_phase;
    }

    fn is_finished(&self, sim_tick: SimTick) -> bool {
        self.phase_at(sim_tick) == RoyalStraightFlushPhase::Finished
    }
}

pub fn royal_straight_flush_hit_delay() -> SimTickSpan {
    RoyalStraightFlushVisual::FADE_DURATION + RoyalStraightFlushVisual::DASH_DURATION
}

pub(crate) struct RoyalStraightFlushVisualParams {
    pub(crate) tower_id: crate::TowerId,
    pub(crate) tower_center_xy: (f32, f32),
    pub(crate) target_xy: (f32, f32),
    pub(crate) target_monster_id: MonsterId,
    pub(crate) sim_tick: SimTick,
    pub(crate) presentation_instant: crate::PresentationInstant,
}

pub(crate) fn spawn_royal_straight_flush_visual(
    presentation_metadata: &mut crate::game_state::presentation_metadata::PresentationMetadataStore,
    presentation_events: &mut PresentationEventQueue,
    params: RoyalStraightFlushVisualParams,
    black_smoke_sources: &mut Vec<BlackSmokeSource>,
) {
    let clones = generate_royal_straight_flush_clones(
        params.target_xy,
        params.tower_id.raw().wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ params.sim_tick.ticks()
            ^ params.target_monster_id.raw(),
    );

    spawn_black_smoke_burst_reversed(
        black_smoke_sources,
        params.tower_center_xy,
        params.presentation_instant.as_namui(),
    );
    spawn_black_smoke_puff_burst(
        params.tower_center_xy,
        params.presentation_instant.as_namui(),
    );
    presentation_events.push(PresentationEvent::PlaySoundCue {
        cue: SoundCue::Wind,
        position: Some([params.tower_center_xy.0, params.tower_center_xy.1]),
        volume: SoundVolume::Minimum,
        max_duration_ms: None,
    });
    for clone in &clones {
        spawn_black_smoke_burst(
            black_smoke_sources,
            clone.spawn_center_xy,
            params.presentation_instant.as_namui(),
        );
        spawn_black_smoke_puff_burst(
            clone.spawn_center_xy,
            params.presentation_instant.as_namui(),
        );
        presentation_events.push(PresentationEvent::PlaySoundCue {
            cue: SoundCue::Wind,
            position: Some([clone.spawn_center_xy.0, clone.spawn_center_xy.1]),
            volume: SoundVolume::Minimum,
            max_duration_ms: None,
        });
    }

    if let Some(tower) = presentation_metadata.tower_mut(params.tower_id) {
        tower.royal_straight_flush_visual = Some(RoyalStraightFlushVisual::new(
            params.sim_tick,
            clones,
            params.target_monster_id,
        ));
    }
}

pub fn tick_royal_straight_flush_visuals(
    game_state: &mut GameState,
    sim_tick: SimTick,
    presentation_instant: crate::PresentationInstant,
    black_smoke_sources: &mut Vec<BlackSmokeSource>,
) {
    let raw_snapshot = game_state.raw_render_snapshot();
    let tower_positions = raw_snapshot
        .towers
        .iter()
        .map(|tower| (tower.id, tower.left_top))
        .collect::<std::collections::HashMap<_, _>>();
    let monster_positions = raw_snapshot
        .monsters
        .iter()
        .map(|monster| (monster.id, monster.position))
        .collect::<std::collections::HashMap<_, _>>();
    let presentation_events = &mut game_state.pending_presentation_events;
    for tower in &mut game_state.presentation_metadata.towers {
        if tower.royal_straight_flush_visual.is_none() {
            continue;
        }
        let Some([left, top]) = tower_positions.get(&tower.id.raw()).copied() else {
            tower.royal_straight_flush_visual = None;
            continue;
        };
        let tower_center_xy = (left as f32 + 1.5, top as f32 + 1.5);
        let visual = tower.royal_straight_flush_visual.as_mut().unwrap();
        let target_xy = monster_positions
            .get(&visual.target_monster_id.raw())
            .map(|position| {
                (
                    position[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32 + 0.5,
                    position[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32 + 0.5,
                )
            });
        visual.tick(
            presentation_events,
            sim_tick,
            presentation_instant,
            tower_center_xy,
            target_xy,
            black_smoke_sources,
        );
        if visual.is_finished(sim_tick) {
            tower.royal_straight_flush_visual = None;
        }
    }
}

fn generate_royal_straight_flush_clones(
    target_xy: (f32, f32),
    key: u64,
) -> Vec<RoyalStraightFlushClone> {
    let unit = |value: u64| -> f32 {
        (value.wrapping_mul(0xA24B_AED4_963E_E407) >> 40) as f32 / (1u64 << 24) as f32
    };
    let angle1 = unit(key) * std::f32::consts::TAU;
    let separation = std::f32::consts::FRAC_PI_3
        + unit(key.rotate_left(17)) * (std::f32::consts::TAU - 2.0 * std::f32::consts::FRAC_PI_3);
    let angle2 = (angle1 + separation) % std::f32::consts::TAU;

    [angle1, angle2]
        .into_iter()
        .map(|angle| {
            let radius = ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MIN
                + unit(key.rotate_left(31) ^ angle.to_bits() as u64)
                    * (ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MAX
                        - ROYAL_STRAIGHT_FLUSH_CLONE_SPAWN_RADIUS_MIN);
            let spawn_center_xy = (
                target_xy.0 + angle.cos() * radius,
                target_xy.1 + angle.sin() * radius,
            );
            let end_center_xy = compute_pass_through_xy(spawn_center_xy, target_xy);

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

fn compute_pass_through_xy(from: (f32, f32), target: (f32, f32)) -> (f32, f32) {
    let dx = target.0 - from.0;
    let dy = target.1 - from.1;
    let length = (dx * dx + dy * dy).sqrt().max(0.001);
    (
        target.0 + (dx / length) * ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE,
        target.1 + (dy / length) * ROYAL_STRAIGHT_FLUSH_CLONE_PASS_THROUGH_DISTANCE,
    )
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
