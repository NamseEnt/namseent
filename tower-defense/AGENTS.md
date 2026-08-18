# AGENTS.md

## Namui Rendering Rule

- In Namui, `ctx.add` draw order is **front to back**.
- The component added **first** is rendered on top and can cover later components.
- Therefore: add interactive/content layers first, then add background layers later.

> ⚠️ **Documentation note:** this file is treated as authoritative guidance for agents. **Do not add inline comments or explanatory notes** outside of the structured rules. Keeping the file comment‑free prevents confusion during automated parsing.

## Codex Linux Sandbox Workaround Rule

- On this Ubuntu 24.04 host, the workspace sandbox is known to fail during bubblewrap loopback initialization with `bwrap: loopback: Failed RTM_NEWADDR: Operation not permitted`.
- Treat that exact failure as a host AppArmor/unprivileged-user-namespace incompatibility, not as a repository permission, command, or patch-syntax problem.
- For `exec_command` calls in this repository, request command-scoped escalated execution on the first attempt; do not probe the default sandbox first.
- Do not retry a command in the default sandbox after this exact error.
- Do not invoke the sandboxed `apply_patch` path on this host because it has no escalation channel and deterministically fails. When file edits are required and higher-priority instructions permit, apply the same unified diff through an escalation-capable patch command such as `git apply`, then inspect `git diff`.
- Keep every escalation narrowly scoped and include the required user-facing justification.

## Hand UI Layering Checklist

- For panel containers:
  - Add `StickyBar` / `PaperContent` first.
  - Add `PaperContainerBackground` (`Sticky`, `Paper`) after content **and after any interactive overlays** (e.g., tooltips/hover regions) so the background renders behind them.
- For action areas:
  - Add buttons/text first.
  - Add sticky paper background last.

## Quick Review Rule

Before committing UI changes that use `ctx.add`, verify:

1. Foreground components are added earlier.
2. Background components are added later.
3. Hover/click targets are not hidden by later-added visual layers.

## Table Layout Rule

- When implementing Tower Defense UI with `table`, prefer non-clipping layout helpers.
- Use `table::ratio_no_clip` instead of `table::ratio`.
- Use `table::fixed_no_clip` instead of `table::fixed`.
- Use `table::padding_no_clip` instead of `table::padding` where applicable.

## Namui Ctx Ownership Rule

- Treat `ctx` as move-only inside render closures.
- If a helper like `table::padding_no_clip(...)(wh, ctx)` consumes `ctx`, do not call `ctx.add(...)` after it in the same closure.
- When both layout rendering and extra drawing are needed in one area, split them into separate phases:
  - Phase 1: `ctx.compose(|ctx| { ...table/layout call... });`
  - Phase 2: `ctx.add(...)` for additional overlays/backgrounds.
- Avoid passing `ctx` into nested calls and then reusing the same `ctx` variable unless ownership is clearly preserved.
- Before committing, verify there is no `E0382` (`borrow of moved value`) around `ctx` in changed UI code.

## Text and Localization Rule

- When adding or changing user-facing text in Tower Defense, follow the API structure under `src/l10n/` so multilingual support is straightforward.
- Avoid hardcoding display text directly in UI components when a localization path exists via the `l10n` APIs.
- When rendering text in Tower Defense UI, use `memoized_text()`.
- Keep text rendering consistent with existing usage patterns where `memoized_text()` receives stable memoization keys and renders through the typography builder.

## Namui State Tracking Rule

- `ctx.track_eq` compares its target on every render and clones the target when the comparison reports a change.
- Do not derive field-by-field `PartialEq` for a large tracked state containing collections when its owner updates frequently.
- For such state, maintain a lightweight wrapping revision and either track that revision directly or implement `PartialEq` using only the revision.
- Increment the revision on every mutation that changes rendered data, including load and merge paths.
- Route collection mutations through methods that also update the revision; avoid direct mutation that can leave the revision stale.

## Game State Ownership and Sparse Event Rule

- Keep gameplay-owned data in `GameState` when its lifetime and mutation authority follow the game session.
- Avoid mirroring the same data in another `Atom` and synchronizing it through pending batches or per-frame snapshots.
- Trigger sparse gameplay events such as discoveries at authoritative `GameStateAction` or mutation points instead of observing the whole `GameState`, which changes every tick.
- Use a separate `Atom` only for state with genuinely independent ownership or lifetime, not solely to notify another copy of gameplay state.

## Persistent State and Headless Rule

- Load persistent gameplay metadata before rendering consumers that require it.
- If runtime mutations can occur before an asynchronous load completes, merge them with loaded data instead of overwriting either side.
- Cache and deduplicate values in memory, mark them dirty only on real changes, and avoid issuing KV writes for duplicate observations.
- Serialize or coalesce asynchronous KV writes so an older request cannot complete after and overwrite a newer snapshot.
- Check headless or simulator mode at the authoritative side-effect entry point and skip discovery persistence and other runtime-only side effects there.

## Native Namui KV Test Rule

- Keep cache, merge, revision, and deduplication logic separable from `namui::system::kv_store` so it can be tested without the Namui runtime.
- Native test binaries that reference Namui KV may fail to link on `_kv_store_get` or `_kv_store_put` when the runtime FFI is unavailable.
- In that environment, run `cargo check --tests` for compile coverage and report the linker limitation explicitly; do not report the runtime tests as passed.
