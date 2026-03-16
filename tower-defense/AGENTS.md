# AGENTS.md

## Namui Rendering Rule

- In Namui, `ctx.add` draw order is **front to back**.
- The component added **first** is rendered on top and can cover later components.
- Therefore: add interactive/content layers first, then add background layers later.

> ⚠️ **Documentation note:** this file is treated as authoritative guidance for agents. **Do not add inline comments or explanatory notes** outside of the structured rules. Keeping the file comment‑free prevents confusion during automated parsing.

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
