#[cfg(test)]
use crate::SimTickSpan;
use crate::game_state::fast_forward::FastForwardMultiplier;
use crate::game_state::render_snapshot::{
    MonsterRenderSample, RenderSnapshotHistory, SpatialProjectileRenderSample, WorldRenderSnapshot,
};
use crate::time::INTERPOLATION_UNITS_PER_TICK;
use crate::{PresentationDelta, SimRenderTime};
use namui::*;

pub const MAX_STEPS_PER_FRAME: u32 = 32;
pub const MAX_BACKLOG_TICKS: u64 = 240;

const SIM_TICKS_PER_SECOND: u128 = 60;
const NANOS_PER_SECOND: u128 = INTERPOLATION_UNITS_PER_TICK as u128;

/// Fixed simulation ticks executed while advancing one presentation frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, State)]
pub struct ScheduleReport {
    pub executed_ticks: u32,
    pub backlog_ticks: u64,
    pub fractional_units: u64,
    pub discarded_units: u64,
}

#[derive(Clone, Default, State)]
pub struct FixedTickScheduler {
    accumulator_units: u64,
    discarded_units: u64,
    render_history: RenderSnapshotHistory,
    revision: u64,
}

impl FixedTickScheduler {
    /// Accumulates one presentation-frame delta and reports fixed ticks ready
    /// for execution.
    pub fn advance_frame(
        &mut self,
        real_dt: PresentationDelta,
        multiplier: FastForwardMultiplier,
    ) -> ScheduleReport {
        self.revision = self.revision.wrapping_add(1);
        let discarded_units = self.accumulate_frame_units(real_dt, multiplier);
        let available_steps = self.accumulator_units / NANOS_PER_SECOND as u64;
        let executed_ticks = available_steps.min(MAX_STEPS_PER_FRAME as u64) as u32;
        self.accumulator_units -= executed_ticks as u64 * NANOS_PER_SECOND as u64;

        ScheduleReport {
            executed_ticks,
            backlog_ticks: self.accumulator_units / NANOS_PER_SECOND as u64,
            fractional_units: self.accumulator_units % NANOS_PER_SECOND as u64,
            discarded_units,
        }
    }

    pub(crate) fn discard_blocked_frame(
        &mut self,
        real_dt: PresentationDelta,
        multiplier: FastForwardMultiplier,
    ) -> ScheduleReport {
        self.revision = self.revision.wrapping_add(1);
        let incoming_units = self.scaled_units(real_dt, multiplier);
        let discarded_units = self.accumulator_units.saturating_add(incoming_units);
        self.accumulator_units = 0;
        self.discarded_units = self.discarded_units.saturating_add(discarded_units);
        ScheduleReport {
            executed_ticks: 0,
            backlog_ticks: 0,
            fractional_units: 0,
            discarded_units,
        }
    }

    fn accumulate_frame_units(
        &mut self,
        real_dt: PresentationDelta,
        multiplier: FastForwardMultiplier,
    ) -> u64 {
        let scaled_units = self.scaled_units(real_dt, multiplier);
        self.accumulator_units = self.accumulator_units.saturating_add(scaled_units);

        let max_backlog_units = MAX_BACKLOG_TICKS.saturating_mul(NANOS_PER_SECOND as u64);
        let mut discarded_units = 0;
        if self.accumulator_units > max_backlog_units {
            discarded_units = self.accumulator_units - max_backlog_units;
            self.accumulator_units = max_backlog_units;
            self.discarded_units = self.discarded_units.saturating_add(discarded_units);
        }
        discarded_units
    }

    fn scaled_units(&self, real_dt: PresentationDelta, multiplier: FastForwardMultiplier) -> u64 {
        (real_dt.as_nanos() as u128)
            .saturating_mul(multiplier.time_scale().get() as u128)
            .saturating_mul(SIM_TICKS_PER_SECOND)
            .min(u64::MAX as u128) as u64
    }

    pub(crate) fn rebase_render_snapshot(&mut self, snapshot: WorldRenderSnapshot) {
        self.render_history.rebase(snapshot);
        self.revision = self.revision.wrapping_add(1);
    }

    pub(crate) fn commit_render_snapshot(&mut self, snapshot: WorldRenderSnapshot) {
        self.render_history.commit(snapshot);
        self.revision = self.revision.wrapping_add(1);
    }

    pub(crate) fn discard_scheduled_ticks(&mut self, ticks: u32) -> ScheduleReport {
        self.revision = self.revision.wrapping_add(1);
        let discarded_units = self
            .accumulator_units
            .saturating_add(u64::from(ticks).saturating_mul(NANOS_PER_SECOND as u64));
        self.accumulator_units = 0;
        self.discarded_units = self.discarded_units.saturating_add(discarded_units);
        ScheduleReport {
            executed_ticks: 0,
            backlog_ticks: 0,
            fractional_units: 0,
            discarded_units,
        }
    }

    pub(crate) fn has_render_snapshot(&self) -> bool {
        self.render_history.is_initialized()
    }

    #[cfg(test)]
    pub const fn discarded_units(&self) -> u64 {
        self.discarded_units
    }

    #[cfg(test)]
    pub const fn backlog(&self) -> SimTickSpan {
        SimTickSpan::from_ticks(self.accumulator_units / NANOS_PER_SECOND as u64)
    }

    pub(crate) fn render_frame(&self) -> Option<RenderFrame<'_>> {
        let tick = self
            .render_history
            .previous_tick()
            .or_else(|| self.render_history.current_tick())?;
        Some(RenderFrame {
            history: &self.render_history,
            time: SimRenderTime::new(
                tick,
                crate::InterpolationAlpha::from_fractional_units(self.last_fractional_units()),
            ),
        })
    }

    fn last_fractional_units(&self) -> u64 {
        self.accumulator_units % NANOS_PER_SECOND as u64
    }
}

impl PartialEq for FixedTickScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RenderFrame<'a> {
    history: &'a RenderSnapshotHistory,
    pub(crate) time: SimRenderTime,
}

impl RenderFrame<'_> {
    pub(crate) fn current_snapshot(&self) -> Option<&WorldRenderSnapshot> {
        self.history.current()
    }

    pub(crate) fn sample_monster(
        &self,
        id: crate::MonsterId,
        interpolate: bool,
    ) -> Option<MonsterRenderSample<'_>> {
        self.history.sample_monster(id, self.time, interpolate)
    }

    pub(crate) fn sample_projectile(
        &self,
        id: crate::AttackId,
        interpolate: bool,
    ) -> Option<SpatialProjectileRenderSample<'_>> {
        self.history.sample_projectile(id, self.time, interpolate)
    }

    pub(crate) fn sample_tower(&self, id: crate::TowerId, interpolate: bool) -> Option<f32> {
        self.history
            .sample_tower_at(id, self.time.alpha.as_f32(), interpolate)
    }

    pub(crate) fn base_scales(&self, interpolate: bool) -> Option<(Xy<f32>, Xy<f32>)> {
        self.history
            .base_scales_at(self.time.alpha.as_f32(), interpolate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::fast_forward::FastForwardMultiplier;

    fn empty_snapshot(tick: u64) -> WorldRenderSnapshot {
        WorldRenderSnapshot::empty(crate::SimTick::from_ticks(tick))
    }

    fn run_cadence(frame_count: u32, multiplier: FastForwardMultiplier) -> u64 {
        let total_nanos = 10_000_000_000u64;
        let frame_nanos = total_nanos / frame_count as u64;
        let remainder = total_nanos % frame_count as u64;
        let mut scheduler = FixedTickScheduler::default();
        let mut steps = 0;

        for frame in 0..frame_count {
            let nanos = frame_nanos + u64::from(frame < remainder as u32);
            let report = scheduler.advance_frame(PresentationDelta::from_nanos(nanos), multiplier);
            steps += report.executed_ticks as u64;
        }

        while scheduler.backlog().ticks() > 0 {
            let report = scheduler.advance_frame(PresentationDelta::ZERO, multiplier);
            steps += report.executed_ticks as u64;
        }

        steps
    }

    #[test]
    fn cadence_produces_the_same_tick_count_for_the_same_real_time() {
        for multiplier in [
            FastForwardMultiplier::X1,
            FastForwardMultiplier::X2,
            FastForwardMultiplier::X4,
            FastForwardMultiplier::X8,
        ] {
            let expected = 600 * multiplier.time_scale().get() as u64;
            assert_eq!(run_cadence(300, multiplier), expected);
            assert_eq!(run_cadence(600, multiplier), expected);
            assert_eq!(run_cadence(1_440, multiplier), expected);
        }
    }

    #[test]
    fn catch_up_is_capped_but_backlog_and_discard_are_visible() {
        let mut scheduler = FixedTickScheduler::default();
        let report = scheduler.advance_frame(
            PresentationDelta::from_nanos(10_000_000_000),
            FastForwardMultiplier::X8,
        );

        assert_eq!(report.executed_ticks, MAX_STEPS_PER_FRAME);
        assert_eq!(
            report.backlog_ticks,
            MAX_BACKLOG_TICKS - MAX_STEPS_PER_FRAME as u64
        );
        assert!(report.discarded_units > 0);
        assert_eq!(scheduler.discarded_units(), report.discarded_units);
    }

    #[test]
    fn blocked_frames_discard_elapsed_time_without_executing_ticks() {
        let mut scheduler = FixedTickScheduler::default();
        let blocked = scheduler.discard_blocked_frame(
            PresentationDelta::from_nanos(1_000_000_000),
            FastForwardMultiplier::X1,
        );
        assert_eq!(blocked.executed_ticks, 0);
        assert_eq!(blocked.backlog_ticks, 0);

        let resumed = scheduler.advance_frame(PresentationDelta::ZERO, FastForwardMultiplier::X1);
        assert_eq!(resumed.executed_ticks, 0);
        assert_eq!(resumed.backlog_ticks, 0);
    }

    #[test]
    fn blocked_scheduled_ticks_are_discarded_instead_of_replayed() {
        let mut scheduler = FixedTickScheduler::default();
        let report = scheduler.advance_frame(
            PresentationDelta::from_nanos(100_000_000),
            FastForwardMultiplier::X1,
        );
        assert_eq!(report.executed_ticks, 6);

        let discarded = scheduler.discard_scheduled_ticks(4);
        assert_eq!(discarded.executed_ticks, 0);
        assert_eq!(discarded.backlog_ticks, 0);
        assert!(discarded.discarded_units > 0);
        assert_eq!(
            scheduler
                .advance_frame(PresentationDelta::ZERO, FastForwardMultiplier::X1)
                .executed_ticks,
            0
        );
    }

    #[test]
    fn thirty_fps_x8_does_not_hit_the_per_frame_step_cap() {
        let mut scheduler = FixedTickScheduler::default();
        let mut steps = 0;
        for frame in 0..30 {
            let nanos = 1_000_000_000 / 30 + u64::from(frame < 10);
            steps += scheduler
                .advance_frame(
                    PresentationDelta::from_nanos(nanos),
                    FastForwardMultiplier::X8,
                )
                .executed_ticks;
        }

        assert_eq!(steps, 480);
    }

    #[test]
    fn render_alpha_is_the_fractional_accumulator_after_fixed_steps() {
        let mut scheduler = FixedTickScheduler::default();
        scheduler.rebase_render_snapshot(empty_snapshot(0));
        let report = scheduler.advance_frame(
            PresentationDelta::from_nanos(8_333_333),
            FastForwardMultiplier::X1,
        );
        let frame = scheduler.render_frame().unwrap();
        assert_eq!(frame.time.tick, crate::SimTick::ZERO);
        assert_eq!(
            frame.time.alpha.as_f32(),
            report.fractional_units as f32 / 1_000_000_000.0
        );
        assert!(frame.time.alpha.as_f32() >= 0.0);
        assert!(frame.time.alpha.as_f32() <= 1.0);
    }

    #[test]
    fn render_alpha_stays_bounded_at_thirty_sixty_and_one_hundred_forty_four_fps() {
        for frame_nanos in [33_333_333u64, 16_666_667, 6_944_444] {
            let mut scheduler = FixedTickScheduler::default();
            scheduler.rebase_render_snapshot(empty_snapshot(0));
            for _ in 0..120 {
                scheduler.advance_frame(
                    PresentationDelta::from_nanos(frame_nanos),
                    FastForwardMultiplier::X8,
                );
                let frame = scheduler.render_frame().unwrap();
                assert!((0.0..=1.0).contains(&frame.time.alpha.as_f32()));
            }
        }
    }
}
