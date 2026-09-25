# Adaptive Teacher Budget (Proposed, not implemented)

Status: design only. The production teacher (schema v2, frozen at `2d874b22`) is unchanged, and nothing here was derived from the frozen held-out run (seeds 116-123) or tuned on it.

## Current cost

For a decision with `k` non-baseline S4/1 candidates:

```text
discovery   k x 32 terminal rollouts
validation  (min(k, 3) + 1) x 64 terminal rollouts
```

| k | discovery | validation | total |
|---|---|---|---|
| 1 | 32 | 128 | 160 |
| 2 | 64 | 192 | 256 |
| 3 | 96 | 256 | 352 |
| 9 | 288 | 256 | 544 |
| 10 | 320 | 256 | 576 |

## Development-state replay

To see how much alternative schemes would save, they were replayed offline on the recorded outcomes of the 7 Phase 3N development states (0/6, 1/5, 2/1, 3/1, 4/1, 5/3, 6/1). The data comes from the Phase 3K/3M exhaustive discovery outcomes and the Phase 3N validation fixture. The reconstructed S4/1 sets and discovery top-3 match the fixture exactly for every state. No held-out data was used.

Mean branches per decision and the number of states where the scheme picks the Phase 3N decision:

| scheme | mean branches | matches 3N |
|---|---|---|
| current (32 per candidate, top-3 x 64, Holm) | 567 | 7/7 |
| discovery racing 4->8->16->32 (c = 2.5) + fixed Holm with non-binding futility at 16/32 | 513 | 7/7 |
| current discovery + fixed Holm with non-binding futility at 16/32 | 510 | 7/7 |
| racing (c = 2.0) + group-sequential validation (Lan-DeMets O'Brien-Fleming, Bonferroni alpha/3) | 483 | 6/7 (misses 0/6) |

Findings:

- These 7 states were chosen because they are hard: several candidates are plausibly better than baseline. On states like these, adaptive sampling saves only about 10%, because very little can be eliminated early.
- Group-sequential efficacy stopping with Bonferroni across finalists costs power. It lost the 0/6 decision, which fixed-n Holm finds. Early efficacy stops buy almost nothing here, because the O'Brien-Fleming boundary at n = 16 is about 4.5 standard errors.
- The winner's-curse cases (5/3: discovery top-1 wrong, top-2 truly positive) survive every scheme that keeps three finalists and fresh-seed validation.
- So a 60-140 branch average is only reachable if most production decisions are easy: every candidate is no better than baseline, or `k` is small. That mix must be measured on development seeds before any threshold is fixed (see Protocol).

## Proposed scheme

### 1. Structural savings (no statistical change)

- If `k <= 3`, skip discovery. Discovery only picks the top-3 finalists, so with at most 3 candidates every candidate is a finalist anyway. The selection is identical by construction. This saves `32k` per such decision (k = 2: 256 -> 192).

### 2. Discovery: baseline-paired CRN racing

- Run baseline and every surviving candidate on the same discovery scenarios in rounds with cumulative `n = 4, 8, 16, 32`.
- From `n >= 8`, for each survivor compute the mean paired delta `d` and `se = max(sd, sigma_floor) / sqrt(n)`. Eliminate a candidate if:
  - `d + c * se < 0` (confidently no better than baseline), or
  - `d + c * se` is below the third-largest `d - c * se` among survivors (confidently outside the top 3).
- Stop early when no survivor remains (select baseline and skip validation) or when at most 3 survive (they become the finalists). At `n = 32`, freeze the top 3 by mean.
- Discovery only nominates candidates; it makes no inference claim. Winner's-curse protection comes entirely from the independent validation below, as it does today.
- Constants `c` and `sigma_floor` must be fixed from the development corpus before use. The replay above used `c = 2.5` and `sigma_floor = 0.5` clear-rate points as placeholders.
- Known risk: a true small positive can be eliminated at `n = 8` with probability roughly `Phi(-c)` per candidate. The corpus replay must report this recall loss against the fixed 32-sample discovery.

### 3. Validation: fixed-n Holm with non-binding futility

- Keep the current final analysis: fresh validation seeds, one-sided paired t-test per finalist, Holm at family-wise alpha 0.05 over the original number of finalists, at `n = 64`.
- Add futility looks at `n = 16` and `n = 32`: drop a finalist whose mean paired delta is `<= 0`. If every finalist is dropped, select baseline.
- Futility stopping is non-binding: it can only turn a rejection into a non-rejection, so the family-wise type-I error stays at or below alpha. No naive repeated p-value testing is used.
- Rejected alternatives:
  - Group-sequential efficacy stopping with alpha spending is valid, but in the replay it lost power (0/6) and saved almost nothing. It could be revisited with a Holm-compatible graphical group-sequential procedure (Maurer-Bretz) if the corpus shows many large-effect decisions.
  - Anytime-valid confidence sequences (betting / empirical-Bernstein for the bounded paired delta in [-100, 100]) allow continuous monitoring, but at `n <= 64` they are wider than the fixed-n t-test. They would lose more of the small-effect overrides the teacher exists to find.

## Protocol before any production change

1. Collect a development decision corpus from non-held-out seeds (for example 0-15, excluding nothing already used as a gate). For every decision, log the raw per-scenario terminal clear_rate of every S4/1 candidate and baseline on all 32 discovery and 64 validation seeds.
2. Replay the schemes above on the corpus and report:
   - mean and p90 branches per decision
   - agreement with the fixed-n rule
   - discovery recall loss
   - false-override rate against the fixed-n rule's own decisions
3. Fix `c`, `sigma_floor` and the futility rule from the corpus only. Freeze them, bump `TEACHER_SELECTION_SCHEMA_VERSION`, and validate on a new, untouched seed set. The frozen seeds 116-123 are never used for this.

## Expected budget

The decision mix is taken from the pre-amendment pilot seed 108 (64 decisions, no longer part of any gate). It is used only to describe the mix, not to set any constant. `k` distribution: 15/64 decisions have `k <= 3`, and most have `k = 9-12`. 12/64 were overrides. The current rule averages **536 branches per decision** on this mix.

Per-decision cost under the proposal:

- Hard decision (like the 7 development states): same cost as today minus discovery for `k <= 3`, about 190-610 branches.
- Easy decision, `k <= 3`, every finalist at mean `<= 0` at the first futility look: `(k + 1) x 16` = 32-64 branches.
- Easy decision, `k >= 4`, racing eliminates everything by `n = 8`: `(k + 1) x 8` = 48-160 branches.

The share of easy decisions has not been measured. Using the non-override share (81%) as an optimistic stand-in:

| easy share | expected mean branches | vs 536 today |
|---|---|---|
| 81% | 168 | 3.2x fewer |
| 70% | 216 | 2.5x fewer |
| 50% | 302 | 1.8x fewer |

So this design alone probably lands at about 170-300 branches per decision, not the 60-140 target. Reaching 60-140 needs a cheaper filter before terminal discovery. The 30-80 stretch target is not reachable with terminal-rollout statistics alone. The obvious filter is a learned value or distilled policy used to rank or prune candidates (Phase 4); the Phase 3K/3L lesson that short-horizon scores miss useful candidates rules out the short-horizon score for this.
