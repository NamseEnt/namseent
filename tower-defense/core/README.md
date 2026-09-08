# td-core

Namui-independent authoritative simulation contracts for tower-defense.

Both the tower-defense app adapter and simulator consume `CoreState` and its command/event contracts from this crate. Namui-specific rendering, sound, UI, and persistence adapters remain in the app crate.

The extraction order is:

1. Stable event and command-result contracts.
2. PlayerCommand and validation data with explicit presentation context at the adapter boundary.
3. Authoritative state and simulation systems.
4. Headed and simulator parity tests against the extracted core.
