# Core Separation Phase 0 Baseline

## Scope

This document records the pre-refactor behavior used to detect gameplay regressions during core separation. Phase 0 does not extract `GameCore`, move `GameState` fields, split effect events, or remove visual ticks.

## Verification Results

- `cargo test`: 292 passed.
- `cargo test --features simulator`: 477 passed.
- `cargo check --tests --features simulator`: passed.
- `cargo check --features simulator`: passed.
- Replay-focused tests: 7 passed.
- Headed/headless shared-step test: passed.
- Fast-forward authoritative-hash test: passed.
- Baseline determinism test: passed.

The simulator feature currently emits four pre-existing dead-code warnings in the ML CLI/PPO modules. They did not cause test or compilation failures.

## Baseline Artifact

The deterministic random-legal report is stored at `artifacts/phase0/random-baseline.json`. It covers seeds `0` through `3` with a maximum of 256 decisions per seed. Its SHA-256 is:

`cd2d58872f3897ef771ee7efdf827a858e9132b629d1b60b6e5020156880d6f4`

Metadata and command provenance are stored in `artifacts/phase0/manifest.json`.

## Current Tick Behavior

`advance_simulation_tick` advances exactly one `SimTick`, then runs `tick_logic` and `tick_world_visuals`. The headed scheduler may subsequently run presentation work and flush effect events.

`tick_headless` calls `advance_simulation_tick` with `PresentationInstant::zero()`, then clears black-smoke sources and effect events. Therefore the current headless path still mutates world visual state during the simulation step even though its presentation queues are cleared afterward. This is recorded behavior, not a Phase 0 fix.

The simulator environment maps applicable agent decisions to `PlayerCommand`, applies the command, and advances automatically until a decision point, terminal state, or configured limit. This cadence differs from a UI callback, so future UI/simulator equivalence checks must compare at the shared `PlayerCommand` boundary and at an equal `sim_tick`.

## Known Gaps

- The baseline CLI does not emit config digest or per-step authoritative hashes.
- A direct UI-originated versus simulator-originated `PlayerCommand` hash harness is still required.
- `AgentAction` includes decision-context actions that are not `PlayerCommand` and must not be compared as if they were the same command sequence.
