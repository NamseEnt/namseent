# 11: Candidate Architecture Review

Status: Proposed (blocks Phase 3 until resolved)

## Why this review exists

Phase 1 and Phase 2 benchmarked the current "flattened candidate list" architecture
(`docs/game-ai/02-action-contract.md`, `artifacts/benchmarks/phase1-candidate-recall.json`,
`artifacts/benchmarks/phase2-candidate-limit-bias.json`) and found a structural, not
incidental, trade-off:

- With `position_candidate_limit=64`, `candidate_limit=64` (a realistic smoke-scale
  teacher budget), the fixed, fairness-corrected selection (`select_candidates_fairly`)
  still only retains the oracle-ranked best `BuildTower` candidate 34% of the time.
- Retaining it reliably (100%) requires `candidate_limit≈512`.
- Rollout cost scales close to linearly with `candidate_limit`: measured ~3.3x wall
  time for a 4x limit increase (64→256, same seed/scenario/horizon budget).

So under the current architecture, "afford enough candidates to find good actions"
and "keep rollout/inference cheap" pull directly against each other, and no choice
of `candidate_limit` or candidate ordering removes that tension — it is a property
of materializing every `(card_subset, position)` pair as a distinct `AgentAction`
and scoring/rolling out each one individually. This document evaluates whether the
flattened-candidate representation itself should be replaced, per the user's
request, before Phase 3 proceeds.

Two costs, both currently paid per legal-action-generation call, motivate the two
special investigations below:

1. Legality masking currently calls `TowerPlacementContext::can_place_at` per map
   cell, and each call runs a fresh BFS-based connectivity check
   (`routes_exist_with_extra_blockers`) for each of 6 travel-point segments. For a
   35x35 grid of candidate top-left corners that is up to 1225 x 6 = 7350 BFS
   traversals just to build the position list for one decision (before any card
   subset is even considered, since the position list is subset-independent).
2. Teacher/heuristic scoring (`rank_build_tower_actions_by_heuristic`) computes
   route coverage and nearest-route-distance per `AgentAction` by scanning the
   route's coordinate list per candidate. With up to ~185,000 candidate
   `BuildTower` actions observed in Phase 2 samples, that is up to ~185,000 x
   route_length distance computations for a single decision's ranking.

## Current architecture (C): flattened candidate list

**Shape.** `semantic_card_actions` enumerates every non-empty card subset
(`2^hand_size - 1`, up to 31 for a 5-card hand) in ascending subset-mask order;
for each it emits one `Reroll` and up to `position_candidate_limit` `BuildTower`
actions (one per surviving position, itself pre-sorted by nearest-route-distance
and truncated). The result is flattened into a single `Vec<LegalAction>`, each
entry a fully materialized `AgentAction::BuildTower { card_ids, hand_slot_index,
left, top }`. `teacher.rs::evaluate_semantic_candidates` further truncates/reorders
that flat list via `candidate_limit` (`select_candidates_fairly` as of Phase 2).

**Model.** `DeepSetsActorCritic` (`simulator/src/ml/model.rs`) scores each
candidate independently: `candidate_features(observation, action)` builds an
11+`ActionKind::COUNT`-wide feature vector per `AgentAction`, `forward_logits`
concatenates a shared state encoding with each action encoding and outputs one
scalar logit per candidate, and the policy distribution is a softmax over
however many candidates were materialized for that decision.

**Why this is expensive at the scale this game actually has.** The action space
per `BuildTower` decision is `card_subset_count x position_count`, i.e. up to
31 x 1225 ≈ 38,000 pairs for a normal hand (Phase 2 measured up to ~185,000 when
`position_candidate_limit` was temporarily larger during sampling). Representing
every pair as a heap-allocated `AgentAction` plus a `Vec<f32>` feature vector, and
scoring each with a full forward pass through the shared MLP, is asymptotically
the wrong shape for a highly regular, mostly-independent (subset, position) grid.

## Option A: Factorized / autoregressive policy

```text
P(action_type | state)
  x P(card_subset | state, action_type)
  x P(position | state, action_type, card_subset)
```

`card_subset` is a small, bounded categorical (≤ 2^hand_size - 1, currently ≤ 31;
even a much larger future hand size of 10 cards is only 1023). `position` is
conditioned on the *resulting tower template* (kind, range, damage — a cheap,
already-computed O(1) lookup per subset via `select_tower_build_template`) and
produced as a dense 1225-way categorical over map cells (35x35 valid top-left
corners), masked by legality, with no candidate proposal at all: every legal cell
gets a logit from a single spatial forward pass (e.g. a small CNN or an MLP over
per-cell map features), not a materialized `AgentAction`.

Sampling is ancestral (sample subset, then sample position conditioned on that
subset), and `log_prob`/entropy are the standard sum-of-components used by
autoregressive/factorized policies elsewhere in deep RL (AlphaStar, OpenAI Five):
`log P(subset) + log P(position | subset)`, entropy likewise additive. This is
exactly the "factorized representation, not greedy card-first" principle already
stated in `decisions/0002-semantic-joint-actions.md` — the chain rule makes this
exact, not an approximation, as long as training uses the true joint objective
(matching action log-probs from sampled subset and sampled position, not two
independently-trained heads).

## Option B: Vectorized joint scorer

Same action space as A, but instead of two sequential sampling stages, compute a
dense `[subset_count, position_count]` score matrix directly: a subset embedding
`[S, D]` and a position/map embedding `[P, D]` combined via a batched
bilinear/MLP operation (matrix multiply, not a Rust loop over materialized
structs) into `[S, P]` logits in one shot, masked by the same legality mask
(broadcast across all `S` subsets, since — see below — the mask is
subset-independent), then a single softmax over the flattened `S x P` matrix.

This is functionally the same joint distribution as A (the chain rule and a joint
softmax over the same support represent identical distributions), but computed
as one dense tensor op instead of an autoregressive two-stage sample, and it
matches `decisions/0002`'s literal language more closely: it "compares several
(cards, position) pairs together" in one explicit joint object, rather than
committing to a subset before ever seeing position information.

**Is the dense matrix actually a burden?** `S x P` is at most ~31 x 1225 ≈ 38,000
for the current 5-card hand, and would stay under ~1023 x 1225 ≈ 1.25M even for a
hypothetical 10-card hand. Both are small by ML standards — comparable to, or
smaller than, a single hidden layer's activation count in a modest MLP, and
several orders of magnitude below what routine CPU or GPU batched matrix ops
handle without strain. This is not a scale where GPU batching or sparsity tricks
are needed; it is a scale where *not* vectorizing (materializing 38,000 Rust
structs and running 38,000 separate forward passes) is the actual cost driver
Phase 1/2 measured.

## Option C recap (do-nothing baseline)

Already implemented and benchmarked (Phase 1/2). Kept here only as the
comparison baseline; not proposed as the final design.

## Comparison

| Dimension | A: Factorized/autoregressive | B: Vectorized joint scorer | C: Flattened candidate list (current) |
| --- | --- | --- | --- |
| Represents every legal semantic action | Yes, exactly (chain rule) | Yes, exactly (same support, one softmax) | Yes, but only up to `position_candidate_limit`/`candidate_limit` truncation — Phase 1/2 showed this is lossy in practice |
| Card/position interaction | Position head conditioned on subset-derived template (kind/range/damage); expressive, but only through that conditioning channel | Full `[S,P]` interaction term (bilinear/MLP over the outer product); most direct interaction modeling | Modeled per-pair via `candidate_features`, but each pair is independent work — no shared computation across pairs at all |
| BC training | Two cross-entropy terms (subset, position \| subset) | One cross-entropy over the flattened `S x P` matrix | One cross-entropy over whatever candidate list was generated (biased by proposal truncation per Phase 1/2) |
| PPO log_prob / entropy | Sum of per-stage log_prob/entropy (standard, more moving parts) | Single categorical log_prob/entropy over the flattened matrix (same math as today, different index space) | Single categorical log_prob/entropy over the materialized list (today's code) |
| Legality masking | Elementwise `-inf` on the 1225-cell position logits (subset-independent mask, computed once) | Elementwise `-inf` broadcast across all `S` rows of the `[S,P]` matrix (same mask, same one-time cost) | Legality is baked into which `AgentAction`s get materialized — cheap per-action, but the *generation* step still needs the same map-wide check |
| Teacher action generation | Vectorized map-level heuristic (see below) selects/ranks without materializing pairs; only a handful of top pairs need real rollout | Same vectorized heuristic; dense score matrix doubles as the ranking surface | Requires materializing pairs first, then either heuristic-scoring or rolling out each one individually |
| Candidates needing real rollout | O(small K) — heuristic pre-filter picks top subsets/positions before any rollout | O(small K) — same pre-filter, read off the dense matrix | O(candidate_limit) — measured to need ≈512 for reliable quality, with cost scaling ~linearly |
| Inference complexity per decision | 1 subset head forward pass (~S) + 1 position head forward pass (~1225 cells), both dense | 1 dense matmul producing `S x P` logits (~38k) | Up to `candidate_limit` (Phase 2: needs ~512 for quality) separate `AgentAction` constructions + feature extractions + forward passes |
| CPU/GPU batch efficiency | Two small dense ops per env step; batches cleanly across parallel envs | One small dense op per env step; batches cleanly, simplest to batch (single matrix shape per step) | Ragged: candidate count varies per decision/state, complicating batched inference; today handled via `PaddedEntityBatch` padding overhead |
| Expected throughput | High — no per-pair Rust struct/feature allocation; dominated by tensor op cost (µs-scale for these sizes) | High — same, single tensor op is simpler to reason about than two | Bounded by `candidate_limit` x per-candidate cost; Phase 1's own fix already showed per-decision `legal_actions()` waste dominates wall time even before this issue |
| Implementation complexity | High: new spatial/CNN-style position head, per-stage log_prob/entropy bookkeeping, subset→template conditioning plumbing, autoregressive sampling code path in both BC and PPO | Medium: one new joint-scoring head, index-based (not `AgentAction`-based) categorical distribution, mask broadcasting; log_prob/entropy math unchanged from today's categorical-over-N-things | None (already built), but all of Phase 1/2's mitigations are permanent maintenance burden |
| Code deletable if adopted | `semantic_card_actions`/`semantic_build_actions`/`semantic_legal_positions` truncation logic, `select_candidates_fairly`, `candidate_features`, per-candidate `PaddedEntityBatch` construction for `BuildTower` | Same as A | N/A |
| New failure modes | Exposure bias: position head trained on teacher-chosen subsets may see an out-of-distribution conditioning context at inference if the subset head's distribution shifts (standard autoregressive RL risk, usually mild at this action-space size) | Dense matrix growing unexpectedly if hand size upgrades push `S` far beyond current assumptions (mitigated: still bounded, see above); mask-broadcast bugs would silently legalize illegal positions for every subset at once (must be tested directly) | Already observed: prefix-truncation bias (Phase 2), position-ordering coverage bias (Phase 1) — both are failure modes of the current design, not hypothetical |

## Special investigation 1: full-map legality mask without per-cell BFS

`can_place_at(left, top)` is two independent checks:

1. **Local, O(1)**: `placement_coords` — bounds, "not a travel point", "not already
   occupied" for the tower's fixed 2x2 footprint. Already cheap; the current
   `Vec::contains` scans over `occupied`/`blockers` could become a boolean
   36x36 grid or `HashSet` for O(1) membership, a small independent win
   regardless of which architecture is chosen.
2. **Global, currently O(BFS) per query**: does adding this 2x2 footprint as a
   blocker still leave every consecutive `TRAVEL_POINTS` pair connected
   (`routes_exist_with_extra_blockers`)? This is the actual cost driver.

**Exact, zero-semantics-change optimization (no algorithm change needed):**
`CoreState.route` already holds a currently-valid path through every travel
point (`RouteState.map_coords`), recomputed whenever a tower is placed/removed.
If a candidate footprint's 4 cells do not intersect `route.map_coords`, that
same path is still a valid witness of connectivity after adding the candidate —
no BFS is needed, the candidate is legal by construction. A route through 7
travel points on this map is a thin path, almost certainly well under 200 cells
against 1225 total candidate cells, so this alone is expected to eliminate the
large majority of the 7350 BFS calls per decision, with no change to what counts
as legal.

**For the remaining route-overlapping candidates**, a single global check per
decision avoids per-candidate BFS entirely in the common case: run one
unit-vertex-capacity max-flow between each consecutive travel-point pair (a
~1300-node graph, trivially fast — well under the cost of even one of today's
7350 BFS calls). By the max-flow/min-cut theorem, if the computed min vertex cut
between that pair is ≥ 5, no single tower's 4-cell footprint can disconnect it,
so *every* candidate overlapping that path segment is automatically legal too,
still with zero BFS. Only when a real chokepoint exists (min cut ≤ 4) does a
route-overlapping candidate need an actual connectivity re-check — a rare,
structurally-identified case, not the default path. This is the "cached
connectivity" the user asked about; articulation-point/low-link analysis is the
right tool specifically for that narrow fallback, not for the bulk of cells.

This fix is orthogonal to the A/B/C choice — it should happen either way — but
it matters most for A/B, where the position mask needs to cover all 1225 cells
every decision with no candidate-count escape valve at all (there is no
`position_candidate_limit` to fall back on once positions are a dense logit
vector instead of a proposal list).

## Special investigation 2: vectorized teacher/heuristic scoring

`rank_build_tower_actions_by_heuristic` currently computes, per materialized
`AgentAction`: `covered_route` (count of route cells within the resulting
tower's range of `(left, top)`) and `nearest_route` (min distance to any route
cell) by scanning the full route coordinate list per candidate.

Both quantities are actually properties of *(position, range)*, and `range` only
takes one of 9 distinct values (`tower_range_raw`: `4e6, 5e6, 6e6, 7e6, 9e6,
11e6, 14e6, 15e6` — one per poker-hand category), independent of the specific
cards in a subset (only the resulting damage varies further within a range
class). This means both quantities can be precomputed once per decision as dense
map-sized grids instead of once per candidate:

- `nearest_route_grid[x, y]`: one multi-source BFS/distance-transform seeded
  from every route cell simultaneously — O(map_size), not O(route_length x
  map_size) — answers `nearest_route` for every cell in O(1) thereafter.
- `coverage_grid[range][x, y]`: for each of the ≤9 distinct range values, a
  disk-radius coverage count over the route-occupancy grid (a bounded
  convolution/integral-image computation) — O(map_size) per range value, O(1)
  per lookup thereafter.

`damage_raw` is already O(1) per subset (a template field, not recomputed per
position). So full-map heuristic ranking drops from O(candidates x
route_length) — up to ~185,000 x (tens to ~150) observed in Phase 2 — to
O(map_size x distinct_ranges + map_size) ≈ O(1225 x 9), independent of how many
card subsets exist. This is the natural representation for both A and B (it
produces exactly the dense per-cell values a spatial position head or a `[S,P]`
score matrix needs) and is not usable as-is under C, which still needs to copy
these values onto ~38,000 individually materialized `AgentAction`s.

For genuine rollout-based teacher labels (not just the heuristic proxy), this
vectorized ranking becomes a pre-filter: rollout only the top-K subsets and
top-M positions the vectorized heuristic ranks highest (e.g. K=3, M=3 → 9 real
rollouts instead of hundreds), rather than rolling out every retained candidate
after a `candidate_limit` truncation. Teacher and policy would then share the
same factorized/dense action representation end to end, per the user's
suggestion.

## Recommendation: Option B, vectorized joint scorer

Both A and B solve the two problems this review was asked to solve (no
per-candidate `AgentAction` materialization, no per-cell BFS) and are equally
expressive (the chain rule and a joint softmax over the same support are the
same distribution). The choice between them is an engineering judgment, and B
wins on the concrete numbers for this game:

1. **Scale doesn't need factorization's main advantage.** Autoregressive
   factorization earns its complexity when the joint space is too large to
   score densely (AlphaStar's action space is enormous). Here `S x P` tops out
   in the tens of thousands to low millions even under generous future hand-size
   growth — small enough that one dense matmul is not a meaningful cost, so
   factorization's benefit (avoiding a large dense score tensor) doesn't apply.
2. **Closer to the already-accepted design.** `decisions/0002` calls for
   "comparing several (cards, position) pairs together," which a joint score
   matrix does directly; an autoregressive sampler never forms that explicit
   comparison.
3. **Smaller, lower-risk migration.** B keeps the exact same categorical
   `log_prob`/entropy math this codebase already uses for PPO/BC — only the
   index space changes (dense `[subset_index, position_index]` instead of a
   materialized `Vec<AgentAction>`). A requires new per-stage log_prob/entropy
   bookkeeping and a new autoregressive sampling path in both BC and PPO.
4. **One less new failure mode.** A introduces exposure bias between the two
   stages (a known, if usually mild, autoregressive-RL risk); B has no
   analogous cross-stage conditioning to get wrong.

Option A remains the right fallback if hand-size upgrades or future action-space
extensions ever push `S x P` into a regime where a dense matrix stops being
cheap; nothing in this recommendation forecloses migrating from B to A later,
since both share the same underlying map-level features (Investigation 2) and
legality mask (Investigation 1).

## Migration plan (design only — not yet implemented)

**Representation changes**

- Replace `AgentAction::BuildTower { card_ids, hand_slot_index, left, top }` as
  the *policy-facing* unit with an index pair `(subset_index, position_index)`
  resolved against two small deterministic enumerations: `legal_card_subsets(hand)`
  (≤ 2^hand_size - 1, same subset-mask order as today, order no longer matters
  for bias since nothing truncates it) and `legal_positions()` (all cells passing
  the O(1) local check — no position proposal/limit at all, since the position
  head is dense over the full masked grid). `to_player_command` resolves an
  index pair back to the concrete `left, top, hand_slot_index` the same way it
  resolves `card_ids` today.
- `Reroll` keeps its existing `card_ids`-only shape (it has no position
  dimension); it becomes one more row alongside the `[S, P]` matrix or a
  separate small action-type head per the factorized breakdown in Option A/B's
  shared `action_type` stage.
- New map-level feature tensors (Investigation 1/2 outputs): occupancy grid,
  legality mask, `nearest_route_grid`, `coverage_grid[range]`. These become
  first-class observation features (`03-observation-contract.md` already scopes
  "map, route, wave, shop, resource, tower state" as policy input — this is a
  concrete instantiation of that, not a new category).

**Schema/version bumps**

Per `decisions/0002`/`00-goals`'s own rule ("변경 규칙: schema 의미가 바뀌면 기존
버전을 재사용하지 않는다") and the user's explicit release from backward
compatibility: bump `ACTION_WIRE_SCHEMA_VERSION`, `OBSERVATION_SCHEMA_VERSION`,
`ACTION_SCHEMA_VERSION`, `ENVIRONMENT_VERSION`, `FEATURE_SCHEMA_VERSION`,
`TRAJECTORY_SCHEMA_VERSION`, `ENTITY_ENCODER_SCHEMA_VERSION`, and
`TEACHER_SCORE_SCHEMA_VERSION`. No migration/compat shim for existing
datasets or checkpoints — they are regenerated, not converted, consistent with
the stable-identity change's precedent (`2a4967a8`).

**Code to remove**

- `semantic_legal_positions`'s `position_limit`/truncation path and
  `DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT` (replaced by the full masked
  grid).
- `select_candidates_fairly`, `candidate_group_key`, and
  `RolloutTeacherConfig::candidate_limit` truncation in
  `evaluate_semantic_candidates` (replaced by the top-K/top-M rollout pre-filter
  from Investigation 2). Keep the Phase 1/2 benchmarks and their artifacts as
  historical baseline/regression evidence per the user's instruction — do not
  delete `phase1_candidate_recall_report`/`phase2_candidate_limit_bias_report`.
- `candidate_features`'s per-`AgentAction` feature extraction for `BuildTower`
  and the corresponding `PaddedEntityBatch` construction path for build
  candidates (replaced by the dense map-level tensors).
- `DeepSetsActorCritic`'s candidate-scoring heads (`action_input`,
  `action_hidden`, `actor_hidden`/`actor_output`, `candidate_fusion`) for the
  `BuildTower` action type specifically, replaced by the new joint-scorer head;
  other action types (Reroll, shop, inventory, etc.) can likely keep the
  existing per-candidate scorer, since their candidate counts were never the
  problem.

**New tests/benchmarks**

- Unit: `(subset_index, position_index) -> to_player_command` round-trips to the
  same command the old `(card_ids, left, top)` triple would have produced, for a
  representative fixture set (reuse the same edge/blocked/legal fixture style
  from the atomicity test cleanup).
- Unit: full-map legality mask matches `can_place_at` exactly for every cell on
  a set of representative maps/occupancy states (a differential test, same
  spirit as `can_place_at_matches_actual_placement_outcome_for_sampled_map_cells`,
  but exhaustive now that it's cheap — no sampling needed).
- Unit: `coverage_grid`/`nearest_route_grid` match the current per-candidate
  `covered_route`/`nearest_route` computation exactly, for the same
  representative states used in Phase 1/2.
- Benchmark: re-run Phase 1's recall methodology with "recall" against the new
  representation redefined as "does the dense mask/matrix ever exclude a legal
  action" — should be trivially 100% by construction, so this becomes a
  regression guard, not a discovery benchmark.
- Benchmark: release-mode decisions/sec and legal-action-generation wall time,
  same methodology as the Phase 0/1 benchmarks, to confirm the expected
  throughput win over the `candidate_limit` sweep in
  `phase2-candidate-limit-bias.json`.
- Benchmark: teacher heuristic-ranking wall time before/after vectorization
  (Investigation 2), using the same decision-point sampling as Phase 1/2's
  benchmarks, reporting a deterministic work counter (route/coverage lookups
  performed) alongside wall-clock.

**Implementation order**

1. Investigation 1's fixes (route-overlap shortcut, then the global min-cut
   fallback) — pure optimization, no representation change, safe to land and
   verify independently with the differential test above.
2. Investigation 2's vectorized grids — also representation-independent;
   verify against the current per-candidate heuristic values before anything
   else depends on them.
3. New observation/feature tensors built from (1) and (2).
4. New joint-scorer model head (Option B) consuming those tensors; keep the
   existing per-candidate scorer live for non-`BuildTower` action types.
5. New BC/PPO loss wiring for the dense `[S, P]` categorical (log_prob/entropy
   unchanged in form, new index space).
6. Teacher: vectorized ranking + top-K/top-M rollout pre-filter, replacing
   `select_candidates_fairly`.
7. Dataset/schema version bumps; regenerate smoke datasets; re-run BC/PPO smoke
   tests against the new representation.
8. Remove the dead candidate-list code listed above once the new path is
   verified end to end.

Only after this migration (or an explicit decision to defer it) should Phase 3
minimum-rollout-teacher validation proceed, since Phase 3's determinism/horizon/
seed-stability/held-out comparisons would otherwise be run against an action
representation this review recommends replacing.
