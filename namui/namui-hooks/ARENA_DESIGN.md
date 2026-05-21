# Frame Render Arena — Design & Status

The output `RenderingTree` is now allocated from a per-frame bump arena. This
document records the design, the constraint that shaped it, and the remaining
consumer-crate work.

## Why

`examples/alloc_count.rs` showed steady-state rerender allocated 3–4 times per
entity, every allocation escaping into the returned `RenderingTree`. The
`benches/alloc_strategy.rs` prototype showed a bump arena builds an equivalent
tree ~5.7× faster than `Box`/`Vec`.

## The destructor constraint

`bumpalo`'s `reset()` does **not** run destructors. Any arena value that
transitively owns heap memory (`Path`'s command `Vec`, `TextDrawCommand`'s
`String`, …) would leak every frame.

Resolution used here: the arena registers a per-frame **drop list**. Every
arena allocation of a type with a non-trivial `Drop` records a
`(pointer, drop-glue)` pair; `reset_render_arena()` runs all of them before
resetting the bump. So heap-owning payloads are dropped exactly once per frame
and nothing leaks.

## Implemented design

`namui-rendering-tree::arena`:

- A thread-local `bumpalo::Bump` plus a drop list.
- `arena_alloc<T>(value) -> &'static T` — bump-allocates; if `T: Drop`, records
  drop glue.
- `arena_alloc_slice<T>(iter) -> &'static [T]` — for `RenderingTree` slices.
- `reset_render_arena()` — runs the drop list, then `bump.reset()`.

`RenderingTree` keeps **no lifetime parameter**. Its fields changed:

- `Children(Vec<RenderingTree>)` → `Children(&'static [RenderingTree])`
- `Box<RenderingTree>` (special nodes) → `&'static RenderingTree`
- `DrawCommand`'s `Box<…DrawCommand>` → `&'static …DrawCommand`
- `ClipNode.path: Path` → `&'static Path`
- `MouseCursorNode.cursor: Box<MouseCursor>` → `&'static MouseCursor`

All rendering-tree node types are now `Copy`. The `State` (serialization)
derive was removed from them — arena-backed values are not serializable, and
`RenderingTree` is not used as component state.

`namui-hooks`: `World::run` calls `reset_render_arena()` at frame start;
`RtContainer`, `apply_stack`, and the `Component for *DrawCommand` impls
allocate through `arena_alloc` / `arena_alloc_slice`.

The `'static` lifetime is a controlled lie: a tree is valid only until the next
`World::run` on the same thread. `looper::tick` already consumes the tree
within the frame, so the contract holds.

## Result

Steady-state rerender dropped to **1 allocation per entity** (the user's `Path`
command `Vec`); 10,000-entity flat rerender improved from 1.77 ms to 1.37 ms
(-57% versus the original baseline). See `BENCHMARK.md`.

## Consumer crates — status

Stage 3 changed the field types of `RenderingTree` / `SpecialRenderingNode` /
`DrawCommand`. The type *names* and all helper-function signatures
(`translate()`, `RenderingTree::wrap()`, `World::run()`, …) are unchanged, so
code that only *reads* or *passes* trees is unaffected.

Done:

- **Direct construction sites updated** — `namui/src/render/{path,text,image,
  rect}.rs`, `namui/src/lib.rs`, `namui-particle/src/lib.rs` now allocate
  through `arena_alloc` / `arena_alloc_slice`.
- `namui-drawer` only pattern-matches trees (read-only) and needs no change.
- `namui-prebuilt` builds trees through helpers and is unaffected.

Remaining — the renderer/drawer IPC boundary:

`namui` serializes the whole `RenderingTree` with `bincode::encode_to_vec` and
ships the bytes to the drawer process, which decodes them. The `State` derive
(which generated `bincode::Encode` + `Decode`) was removed from the
rendering-tree types because an arena-backed tree cannot implement a plain
`Decode` — deserialization must allocate into the arena.

`bincode::Encode` can still be derived (it encodes through the `&'static`
references). `Decode` needs a hand-written, arena-aware implementation on the
drawer side. Reworking this IPC boundary is the one genuine piece of
architecture work left to build the full application; it is independent of the
namui-hooks rendering performance the benchmark measures.

## Risks

- **Peak memory** — the arena is not freed mid-frame (~1–4 MB held).
- **Lifetime contract** — a `RenderingTree` must not be read after the next
  `World::run`, nor sent to another thread. Enforced by convention, not the
  borrow checker.
