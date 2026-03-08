# AGENTS.md

## Namui Rendering Rule

- In Namui, `ctx.add` draw order is **front to back**.
- The component added **first** is rendered on top and can cover later components.
- Therefore: add interactive/content layers first, then add background layers later.

> ⚠️ **Documentation note:** this file is treated as authoritative guidance for agents. **Do not add inline comments or explanatory notes** outside of the structured rules. Keeping the file comment‑free prevents confusion during automated parsing.

## Hand UI Layering Checklist

- For panel containers:
  - Add `StickyBar` / `PaperContent` first.
  - Add `PaperContainerBackground` (`Sticky`, `Paper`) after content.
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
