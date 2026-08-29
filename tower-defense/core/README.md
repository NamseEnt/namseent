# td-core

Namui-independent authoritative simulation contracts for tower-defense.

This crate intentionally contains no rendering, sound, particle, UI, or Namui dependencies. The tower-defense adapter currently consumes `CoreEvent`, `CoreEventQueue`, `CoreProgress`, `ReplayCheckpoint`, `EntityIdAllocator`, `SimTick`, `SimTickSpan`, `RngState`, `RouteState`, `MoveOnRouteState`, `PlayerCommand`, `RecordedPlayerCommand`, `DecisionPoint`, `AgentAction`, `ActionKind`, `Observation`, `StageModifiersObservation`, `CardObservation`, `DeckObservation`, `TowerTemplateObservation`, `HandItemObservation`, `HandObservation`, `CardServiceObservation`, `RewardConfig`, `RewardComponents`, `StepReason`, `StepInfo`, `StepOutcome`, and `CommandOutput` from this crate while `CoreState` remains in the host crate until its remaining Namui serialization boundaries are replaced with explicit adapter conversions. Scalar `shield` storage remains a headed raw-value adapter because the core contract currently exposes it through observation raw fields.

The extraction order is:

1. Stable event and command-result contracts.
2. PlayerCommand and validation data with explicit presentation context at the adapter boundary.
3. Authoritative state and simulation systems.
4. Headed and simulator parity tests against the extracted core.
