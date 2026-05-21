# Bounding-box / Text-measurement Caching Design

How `bounding_box` measurement is cached after the frame-arena refactor, why
the old cache had to go, what replaced it, and the benchmark that verifies it.

## Background

`bounding_box(&RenderingTree) -> Rect` computes the axis-aligned bounds of a
render tree (recursively, applying transforms). Text nodes need skia font
measurement, which is the expensive part.

The old cache was `LruCache<RenderingTree, Rect>` — keyed by the **entire
render tree** by structural `Hash`/`Eq`. The frame-arena refactor made
`RenderingTree` hold `&'static` arena references that dangle after the next
frame's `arena.reset()`, so a cache keyed by the tree became unsound and was
removed. Measured cost: tower-defense rich-text rendering regressed (steady
state 53 µs → 83 µs, see `BENCHMARK.md` / the rich-text comparison).

## Research summary

Nine production frameworks were surveyed (Flutter, egui, Jetpack Compose, Skia,
Blink, Servo, Yoga, Taffy, Slint). Findings:

- **No framework caches a whole render tree by structural key.** The old namui
  approach was not idiomatic.
- **Layout** is cached per stable node identity, keyed by input constraints,
  invalidated by dirty flags (Flutter, Compose, Yoga, Taffy).
- **Text measurement** is cached by a **content key** `(text, font, width…)`
  in a store **independent of the layout tree** (Blink/Servo word cache, egui
  `GalleyCache`, Flutter `TextPainter`). It survives tree rebuilds.
- namui is immediate-mode with a per-frame arena — there is no stable node
  identity across frames, so the content-key approach is the right one.

## Design

### Measurement vs. transform

| | nature | handling |
|---|---|---|
| measurement (text shaping, path bounds) | expensive, **invariant** under translate/rotate/scale | **cache** (content key, persistent heap) |
| transform application (local bounds → world AABB) | cheap, changes every frame | **compute live**, never cached |

A transform matrix must never be part of a cache key — an animation changes it
every frame and the cache would thrash. Cache the *pre-transform local*
measurement (invariant under rotation); re-derive the world AABB each frame by
mapping the local rect through the matrix.

### Caching rules

1. **Cache local measurement only**, keyed by owned content — never by a tree
   or arena reference. Text: the `TextDrawCommand` content. Path: skia's
   `getBounds` (already lazily cached by skia).
2. **Derive transformed / subtree bounds live every frame** — union child
   local bounds, apply the parent matrix. Fast path: translate / axis-aligned
   scale is plain rect arithmetic; only rotation/skew needs 4-corner mapping.
3. **Do not cache subtree bounds** — once the text leaves are cached, the
   union/transform recursion is cheap.
4. **Clip only shrinks bounds** (intersection) — no caching needed.
5. **`getBounds` (conservative) by default**, tight path bounds only on demand.
6. **Invalidation**: the cache lives on the persistent heap; the content key
   makes a stale hit impossible (different input ⇒ different key). A capacity
   bound (LRU) caps memory; a font/DPI epoch can be folded into the key if
   those change.

## Implementation

`namui-rendering-tree`, `bounding_box/draw_command.rs`:

`TextDrawCommand::bounding_box()` now consults a process-wide
`LruCache<TextDrawCommand, Option<Rect<Px>>>` before doing skia measurement.
The key is the owned `TextDrawCommand` (text, font, paint, align, baseline,
max_width, …) — it holds no arena reference, so it survives every
`arena.reset()`. On a miss, `measure_text()` runs the skia path and the result
is inserted.

This is the content-key text-measurement cache from the design. The old
tree-level cache is **not** restored.

## Benchmark (tower-defense rich text, Apple M1)

Workload: a tower-info-popup-sized rich text — 8 Korean blocks/frame, each with
colored style spans and `max_width` wrapping (≈3 lines). `cargo bench`,
`rich_text_benchmark`. Branches: `bench/no-arena-memo`, `bench/arena-no-memo`.

| scenario | no-arena + memo | arena + no-memo | arena + no-memo + text cache |
|---|--:|--:|--:|
| create | 48.9 µs | 89.5 µs | 47.9 µs |
| rerender (stable deps) | 9.1 µs | 83.5 µs | 40.8 µs |
| rerender (changing deps) | 53.6 µs | 82.8 µs | 40.7 µs |

Reading:

- **The text-measurement cache recovers the regression and then some.** Arena
  rerender dropped 83 µs → 41 µs (−51%); create 89 µs → 48 µs.
- **arena + text cache beats the pre-arena world on the memo-miss path**
  (40.7 µs vs 53.6 µs): cheap arena allocation plus a proper content-keyed
  cache outperform the old owned-tree + coarse tree-level cache.
- **Memoization is still worth ~32 µs for the steady state** (9 µs vs 41 µs).
  The text cache only removes skia *measurement*; the per-frame *layout
  assembly* (token→box, line breaking, tree building) remains. `memoized_text`
  skips that entirely.

## Remaining work

1. **Layout-result memoization.** Re-add `memoized_text`, but cache the layout
   output (`Vec<LineBox>`), not the arena tree — rebuild only the cheap tree
   each frame. This is arena-compatible and reclaims the remaining ~32 µs.
2. **Path bounds**: rely on skia's lazily-cached `getBounds`; compute tight
   bounds only on demand.
3. Optionally a line-/word-level text cache for long texts (egui / Blink
   pattern) so partially-changed text reuses unchanged lines.
