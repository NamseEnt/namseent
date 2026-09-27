# 0008: Replace the flattened candidate list with a vectorized joint scorer

- Status: Proposed
- Date: 2026-09-19
- Supersedes: the *implementation* of `decisions/0002-semantic-joint-actions.md`'s
  "compare several (cards, position) pairs together" — not its principle.
- Full analysis: `docs/game-ai/11-candidate-architecture-review.md`

## Decision

Stop materializing one `AgentAction` per `(card_subset, position)` pair and
scoring/rolling out each one individually. Replace it with a dense
`[subset_count, position_count]` joint score matrix (Option B in the linked
review) computed via batched tensor ops over a full, unmasked-by-proposal
position grid, with legality expressed as an elementwise mask rather than by
omitting candidates from a list. No dataset/checkpoint backward compatibility
is preserved; schema versions bump as a breaking change.

## Why

Phase 1 (`docs/game-ai/02-action-contract.md`) and Phase 2
(`docs/game-ai/05-rollout-teacher.md`) benchmarked the current architecture and
found the trade-off is structural: at a realistic `candidate_limit=64`, the
oracle-ranked best `BuildTower` candidate is retained only 34% of the time even
after fixing the prefix-truncation bias; reliable retention needs
`candidate_limit≈512`; and rollout cost scales roughly linearly with
`candidate_limit` (measured ~3.3x wall time for a 4x limit increase). No
candidate-count or ordering fix removes this, because it comes from
materializing tens of thousands of individual `AgentAction`s and legality-BFS
calls per decision — work that a dense matrix and a map-level legality mask
avoid entirely. The full comparison against a factorized/autoregressive
alternative (Option A) is in the linked review; both solve the problem, but B
requires a smaller migration and matches this project's existing "compare pairs
together" language more directly.

## Result

- `position_candidate_limit`/`DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT` and
  `candidate_limit`/`select_candidates_fairly` are retired once the migration
  lands; the Phase 1/2 benchmarks and artifacts stay in the repo as baseline/
  regression evidence for the architecture they measured, not as active
  production code paths.
- Legality masking moves to a map-level computation (occupancy + a
  route-overlap shortcut, with a global min-vertex-cut fallback for genuine
  chokepoints) instead of a per-cell BFS; see the review's Investigation 1.
- Teacher/heuristic scoring moves to dense per-cell grids
  (`nearest_route_grid`, `coverage_grid` per distinct tower range) instead of
  per-candidate route scans; see the review's Investigation 2. Real rollout is
  reserved for a small top-K/top-M pre-filtered set.
- Phase 3 (minimum rollout teacher validation) waits until this migration (or
  an explicit decision to defer it) is resolved, since Phase 3's determinism/
  stability/held-out comparisons should run against the action representation
  the project intends to keep, not the one this decision retires.
