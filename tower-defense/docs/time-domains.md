# Time domains

## Simulation domain

- `SimTick` is the authoritative 60 Hz simulation position.
- `SimTickSpan` is a duration in simulation ticks. Gameplay deadlines and intervals use this type.
- A simulation step advances `SimTick` by exactly one. Spawn, cooldown, status effects, timed attacks, monster movement, and projectile movement are simulation work.
- Fast-forward changes the number of fixed steps executed by the scheduler; it never changes the length of a step.

## Presentation domain

- `PresentationInstant` and `PresentationDelta` represent monotonic wall-clock presentation time.
- The root render captures one `PresentationInstant` per render frame and passes that value to presentation consumers.
- Notifications, hover/long-press state, menu transitions, sound/particle playback, and screen/UI animation use presentation time.
- Presentation time must not be used to decide gameplay outcomes or simulation deadlines.
- The Namui particle trait still accepts a raw `namui::Instant`; its adapter is presentation-only and is a remaining migration boundary.
- A few standalone UI components still capture `PresentationInstant` locally because their component API does not yet carry the root frame value; these captures are presentation-only and are tracked for follow-up consolidation.

## World and screen animation classification

| Animation or timer | Domain | Reason |
| --- | --- | --- |
| Monster spawn, movement, attack, status effect | `SimTick` / `SimTickSpan` | Changes authoritative gameplay state |
| Tower cooldown, skill duration, tower attack squash | `SimTick` / `SimTickSpan` | Must remain deterministic under fast-forward |
| Spatial/timed/laser attack lifetime and Royal Flush phases | `SimTick` / `SimTickSpan` | World attack sequence follows simulation steps |
| Base hit/spawn squash | `SimTick` / `SimTickSpan` | Triggered by authoritative world events |
| Field particles, trails, damage text, sound playback | `PresentationInstant` / `PresentationDelta` | Cosmetic playback must not accelerate with fast-forward |
| Card-service notification, hover, long press, menu transition | `PresentationInstant` / `PresentationDelta` | Screen UI follows real elapsed time |

## Scheduler policy

- The scheduler stores an integer accumulator in nanosecond-times-tick units; it does not use a floating-point accumulator.
- Each frame adds `real_dt * FastForwardMultiplier * 60` to the accumulator and executes zero or more 1-tick steps.
- At most 32 steps execute in one frame. Backlog is retained for later frames.
- Backlog is capped at 240 ticks. Time above that cap is discarded intentionally and reported in `ScheduleReport::discarded_units` and `GameState::sim_scheduler_report()`; cumulative discarded units remain available through `GameState::sim_scheduler_discarded_units()`. `ScheduleReport::executed_ticks` counts fixed simulation ticks, not presentation frames.
- The latest backlog, fractional accumulator, executed-step count, and discarded amount remain observable for diagnostics.

## Fixed-tick render interpolation

- `FixedTickScheduler` exposes the previous and current lightweight world render snapshots together with the fractional accumulator as `SimRenderTime { tick, alpha }`.
- `tick` identifies the previous snapshot and `alpha` is the clamped progress toward the current snapshot. It is never used by gameplay, targeting, collision, damage, or HP calculations.
- Snapshots contain only render state for entity IDs, world position, direction, continuity revision, and visual state. A new ID starts at its current position; an ID absent from the current snapshot is not rendered.
- A changed continuity revision (route reset, teleport, restore, or other discontinuity) snaps to the current position instead of interpolating.

## World and screen VFX classification

- Simulation-coupled world animation uses `SimRenderTime`: monster and spatial projectile poses, tower/base spring poses, and Royal Straight Flush world phases.
- World-space cosmetic playback remains presentation-only: death corpse/soul events, field particles, projectile trails, damage text, and sound use `PresentationInstant`/`PresentationDelta` and do not affect authoritative state.
- Screen UI VFX and interaction animation—notifications, hover, long press, menus, and camera/UI transitions—also use `PresentationInstant`/`PresentationDelta`.
- The raw `namui::Instant` particle adapter remains on the presentation side of this boundary.
