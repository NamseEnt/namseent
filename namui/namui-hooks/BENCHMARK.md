# namui-hooks Rendering Benchmark

Measures and records the cost of component creation / rendering logic when
rendering a large number of entities.

## Environment

- CPU: Apple M1
- rustc 1.94.0, `release` profile
- criterion 0.5, `cargo bench --bench entity_benchmark`
- Measured: 2026-05-20

## Benchmark setup (`benches/entity_benchmark.rs`)

- `Entity`: leaf component. Draws a single `PathDrawCommand`.
- `StatefulEntity`: leaf + 1 `state` + 1 `memo`.
- `create/*`: builds a fresh `World` each iteration (first render).
- `rerender/*`: reuses the `World` and renders the same tree again (steady state).
- `flat` / `translated` / `stateful` / `nested` scenarios, 1k / 5k / 10k entities.

## Results

`time` is the criterion median, in µs. Four stages:

- **base** — before any optimization.
- **s1** — hot-path optimizations.
- **s2** — `RtContainer` `Vec` pooling.
- **s3** — frame render arena for the output `RenderingTree`.

### flat

| scenario | count | base | s1 | s2 | s3 | total |
|---|--:|--:|--:|--:|--:|--:|
| create | 1,000 | 551 | 341 | 304 | 280 | -49% |
| create | 5,000 | 2,575 | 1,733 | 1,463 | 1,391 | -46% |
| create | 10,000 | 5,311 | 3,655 | 3,009 | 2,881 | -46% |
| rerender | 1,000 | 306 | 177 | 152 | 128 | -58% |
| rerender | 5,000 | 1,609 | 971 | 801 | 664 | -59% |
| rerender | 10,000 | 3,214 | 1,992 | 1,774 | 1,369 | -57% |

### translated

| scenario | count | base | s1 | s2 | s3 | total |
|---|--:|--:|--:|--:|--:|--:|
| create | 1,000 | 663 | 381 | 349 | 319 | -52% |
| create | 5,000 | 2,983 | 2,038 | 1,702 | 1,551 | -48% |
| create | 10,000 | 5,988 | 4,078 | 3,699 | 3,165 | -47% |
| rerender | 1,000 | 400 | 208 | 219 | 155 | -61% |
| rerender | 5,000 | 2,005 | 1,097 | 1,014 | 782 | -61% |
| rerender | 10,000 | 4,263 | 2,232 | 2,099 | 1,615 | -62% |

### stateful

| scenario | count | base | s1 | s2 | s3 | total |
|---|--:|--:|--:|--:|--:|--:|
| create | 1,000 | 820 | 623 | 560 | 505 | -38% |
| create | 5,000 | 4,203 | 3,015 | 2,685 | 2,490 | -41% |
| create | 10,000 | 8,791 | 5,888 | 5,669 | 5,205 | -41% |
| rerender | 1,000 | 417 | 221 | 180 | 150 | -64% |
| rerender | 5,000 | 1,884 | 1,063 | 991 | 764 | -59% |
| rerender | 10,000 | 4,166 | 2,215 | 1,971 | 1,682 | -60% |

### nested

| scenario | leaves | base | s1 | s2 | s3 | total |
|---|--:|--:|--:|--:|--:|--:|
| create | 1,000 | 1,465 | 759 | 729 | 620 | -58% |
| create | 10,000 | 14,115 | 7,331 | 7,977 | 6,588 | -53% |
| rerender | 1,000 | 624 | 316 | 316 | 243 | -61% |
| rerender | 10,000 | 7,480 | 3,415 | 3,276 | 2,893 | -61% |

For 10,000 entities, flat rerender went from 3.21 ms to 1.37 ms (-57% total).

## Allocation profile (`examples/alloc_count.rs`)

Allocations counted inside `World::run`, per entity, steady-state rerender:

| scenario | base/s1 | s2 (pooled) | s3 (arena) |
|---|--:|--:|--:|
| flat | 3.0 | 2.0 | 1.0 |
| translated | 4.0 | 3.0 | 1.0 |
| stateful | 3.0 | 2.0 | 1.0 |
| nested | 4.3 | 2.3 | 1.0 |

Stage 3 brings every scenario down to a single allocation per entity — the
`Path` command `Vec` built by the user's component code. Everything else (the
child-buffer `Vec`, `Box<PathDrawCommand>`, the `Box<RenderingTree>` of each
transform node) is now allocated from a per-frame bump arena.

## Applied improvements

### Stage 1 — hot-path optimizations

1. `RtContainer`: `boxcar::Vec` → `RefCell<Vec>`.
2. `full_stack` redesigned as arena indices — `translate()` no longer clones.
3. atomics → `Cell` on `Composer`/`Instance`/`ComponentCtx`.
4. Merged composer child maps (one `component_child_map`).
5. Generation-based render tracking — `remove_unused_guys` skips its O(n) scan.
6. `record_used_sig_ids`: `FrozenVec<Box<SigId>>` → `RefCell<Vec<SigId>>`.

### Stage 2 — RtContainer Vec pooling

7. `World` pools `Vec<RenderingTree>` child buffers; leaf containers recycle them.

### Stage 3 — frame render arena

8. **The output `RenderingTree` is allocated in a per-thread frame bump arena**
   (`namui-rendering-tree`'s `arena` module). `RenderingTree` keeps no lifetime
   parameter — its `Box<_>` / `Vec<_>` fields became `&'static _` / `&'static
   [_]` backed by a thread-local `bumpalo::Bump` that is `reset()` at the start
   of every `World::run`. `RenderingTree` is now `Copy`.

   `bumpalo`'s `reset()` does not run destructors, so any arena value that owns
   heap memory (`PathDrawCommand`'s `Path`, etc.) is registered in a per-frame
   drop list and dropped explicitly just before the reset — no leaks.

   The tree from one frame is invalidated by the next `World::run`; callers must
   consume it within the frame (the existing `looper::tick` usage already does).

## Not applied

### Compile-time type-name hashing

`incremental_component` hashes `type_name` at runtime. A compile-time
`Component` associated const is impossible because `core::any::type_name` is not
a `const fn` on stable Rust, and a runtime cache is not cheaper than the direct
hash.

## Verification

- `cargo test`: 45 passed, 1 failed. The failing `interval_should_work` **also
  fails on the original branch before these changes** — a pre-existing defect,
  unrelated to this work. Event-handling tests (`mouse_event`, which exercise
  `bounding_box` and `xy_in` over the arena tree) pass.

## Consumer crates

Stage 3 changed the field types of `RenderingTree` / `SpecialRenderingNode` /
`DrawCommand`. The construction sites in `namui`, `namui-prebuilt`,
`namui-particle` were updated to allocate through the arena, and `namui-drawer`
now decodes the IPC byte stream into the arena via a hand-written, round-trip
tested `bincode::Decode`. All consumer crates build. See `ARENA_DESIGN.md`.
