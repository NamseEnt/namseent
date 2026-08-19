use crate::game_state::fast_forward::FastForwardMultiplier;
use crate::{PresentationDelta, SimTickSpan};
use namui::*;

pub const MAX_STEPS_PER_FRAME: u32 = 32;
pub const MAX_BACKLOG_TICKS: u64 = 240;

const SIM_TICKS_PER_SECOND: u128 = 60;
const NANOS_PER_SECOND: u128 = 1_000_000_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, State)]
pub struct ScheduleReport {
    pub executed_steps: u32,
    pub backlog_ticks: u64,
    pub fractional_units: u64,
    pub discarded_units: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, State)]
pub struct FixedTickScheduler {
    accumulator_units: u64,
    discarded_units: u64,
}

impl FixedTickScheduler {
    pub fn advance(
        &mut self,
        real_dt: PresentationDelta,
        multiplier: FastForwardMultiplier,
    ) -> ScheduleReport {
        let scaled_units = (real_dt.as_nanos() as u128)
            .saturating_mul(multiplier.time_scale().get() as u128)
            .saturating_mul(SIM_TICKS_PER_SECOND)
            .min(u64::MAX as u128) as u64;
        self.accumulator_units = self.accumulator_units.saturating_add(scaled_units);

        let max_backlog_units = MAX_BACKLOG_TICKS.saturating_mul(NANOS_PER_SECOND as u64);
        let mut discarded_units = 0;
        if self.accumulator_units > max_backlog_units {
            discarded_units = self.accumulator_units - max_backlog_units;
            self.accumulator_units = max_backlog_units;
            self.discarded_units = self.discarded_units.saturating_add(discarded_units);
        }

        let available_steps = self.accumulator_units / NANOS_PER_SECOND as u64;
        let executed_steps = available_steps.min(MAX_STEPS_PER_FRAME as u64) as u32;
        self.accumulator_units -= executed_steps as u64 * NANOS_PER_SECOND as u64;

        ScheduleReport {
            executed_steps,
            backlog_ticks: self.accumulator_units / NANOS_PER_SECOND as u64,
            fractional_units: self.accumulator_units % NANOS_PER_SECOND as u64,
            discarded_units,
        }
    }

    pub const fn discarded_units(&self) -> u64 {
        self.discarded_units
    }

    pub const fn backlog(&self) -> SimTickSpan {
        SimTickSpan::from_ticks(self.accumulator_units / NANOS_PER_SECOND as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::fast_forward::FastForwardMultiplier;

    fn run_cadence(frame_count: u32, multiplier: FastForwardMultiplier) -> u64 {
        let total_nanos = 10_000_000_000u64;
        let frame_nanos = total_nanos / frame_count as u64;
        let remainder = total_nanos % frame_count as u64;
        let mut scheduler = FixedTickScheduler::default();
        let mut steps = 0;

        for frame in 0..frame_count {
            let nanos = frame_nanos + u64::from(frame < remainder as u32);
            let report = scheduler.advance(PresentationDelta::from_nanos(nanos), multiplier);
            steps += report.executed_steps as u64;
        }

        while scheduler.backlog().ticks() > 0 {
            let report = scheduler.advance(PresentationDelta::ZERO, multiplier);
            steps += report.executed_steps as u64;
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
        let report = scheduler.advance(
            PresentationDelta::from_nanos(10_000_000_000),
            FastForwardMultiplier::X8,
        );

        assert_eq!(report.executed_steps, MAX_STEPS_PER_FRAME);
        assert_eq!(
            report.backlog_ticks,
            MAX_BACKLOG_TICKS - MAX_STEPS_PER_FRAME as u64
        );
        assert!(report.discarded_units > 0);
        assert_eq!(scheduler.discarded_units(), report.discarded_units);
    }

    #[test]
    fn thirty_fps_x8_does_not_hit_the_per_frame_step_cap() {
        let mut scheduler = FixedTickScheduler::default();
        let mut steps = 0;
        for frame in 0..30 {
            let nanos = 1_000_000_000 / 30 + u64::from(frame < 10);
            steps += scheduler
                .advance(
                    PresentationDelta::from_nanos(nanos),
                    FastForwardMultiplier::X8,
                )
                .executed_steps;
        }

        assert_eq!(steps, 480);
    }
}
