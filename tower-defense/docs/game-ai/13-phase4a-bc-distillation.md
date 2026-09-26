# Phase 4A: Canonical BC and Selective Teacher Distillation

Status: complete. Gate A passed; Gate B positive by its frozen definition, but the effect is negligible (see Conclusion). Sections marked **Frozen** were written before any Phase 4A data was generated or any model was trained, and are not changed after results are seen.

Phase 4A answers two questions:

1. Can a neural policy that runs without rollouts or search imitate the canonical scripted baseline accurately enough?
2. Does adding a small number of expensive teacher labels improve that policy's terminal clear_rate?

PPO is out of scope. Phase 4A only checks whether the BC checkpoint can initialize the PPO actor.

## Frozen seed split

Game seeds used before Phase 4A are all below 1,000: benchmarks 0-3, teacher development states 0-15, diagnostics 100 and 109, the Phase 3 pilot/held-out/extension/terminal gate 108-131, and legacy PPO training ranges. The legacy PPO validation ranges sit near `u64::MAX`. Teacher scenario pools (10000-10007, 20000-20031, 30000-30063, tests 90000-90027) are a separate domain: they seed scenario forks, not games. Phase 4A therefore uses disjoint ranges starting at 2,000,000. Seeds 108-131 are rejected by the dataset validator.

| split | seeds | use |
|---|---|---|
| `canonical_train` | 2,000,000 - 2,009,999 | canonical BC training (prefixes of the range) |
| `canonical_validation` | 2,100,000 - 2,100,063 | offline BC validation loss/accuracy, epoch selection |
| `teacher_train` | 2,200,000 - 2,200,015 | teacher-controlled label games (16) |
| `development_evaluation` | 2,300,000 - 2,300,127 | terminal evaluation for every model/scale/variant choice |
| `final_evaluation` | 2,400,000 - 2,400,255 | run once, at the end, on the selected models |

Defined in `Phase4Split::range` (`simulator/src/ml/phase4_dataset.rs`). The final evaluation seeds are not used for any model, threshold or hyperparameter choice, and the final evaluation is executed exactly once.

## Frozen success criteria

The score scale is terminal `clear_rate` in percent (0-100). Canonical terminal clear_rate is typically 30-60 per seed, and the Phase 3 teacher's paired improvement was +15.07.

**Gate A - canonical imitation** (final evaluation seeds, canonical BC vs canonical scripted baseline):

- illegal actions = 0 and fallback actions = 0, and
- mean paired terminal clear_rate delta (BC - canonical) >= -2.0.

-2.0 is about 13% of the teacher's improvement and below one stage of progress (each cleared stage adds 2.0 clear_rate points).

**Gate B - teacher distillation** (final evaluation seeds, selected distilled policy vs canonical BC):

- primary: mean paired terminal clear_rate delta (distilled - canonical BC) > 0 positive, = 0 tie, < 0 negative.
- no significance test is part of the gate. SE, a normal-approximation 95% interval and better/worse/tie counts are reported descriptively.

## Frozen model-selection protocol

All selection uses `canonical_validation` (offline metrics) and `development_evaluation` (terminal clear_rate) only.

- Canonical BC data scale: small (128 games), then medium (512 games). Large (2,048 games) only if medium improves on small by at least 0.5 percentage points of validation top-1 accuracy or 5% relative validation NLL.
- Within a scale, the checkpoint is the epoch with the lowest validation NLL.
- The canonical BC used downstream is the scale with the highest development mean terminal clear_rate (ties go to the larger scale).
- Before teacher collection, the selected canonical BC must have 0 illegal actions on the development seeds. Otherwise architecture/data/indexing/mask bugs are fixed first.
- Distillation variants: A (teacher labels, every sample weight 1) and B (teacher-override samples weight 4, agreeing samples weight 1). No other weights are tried. The variant with the higher development mean paired delta vs canonical BC is the one that goes to the final evaluation and Gate B. The other variant is also reported on the final seeds, descriptively only.

## Frozen training configuration

- Network: the existing `DeepSetsActorCritic` (`hidden_size` 64), the same struct the PPO trainer uses.
- Optimizer: Adam, learning rate 1e-3, batch 64 decisions, 20 epochs, training seed 0, deterministic data order.
- Distillation fine-tuning: start from the selected canonical BC, Adam learning rate 3e-4, batch 64, 10 epochs, final epoch kept (the teacher corpus is too small for a separate validation split).
- Teacher corpus: 16 teacher-controlled terminal games on `teacher_train`, frozen Phase 3 v2 rule (`TEACHER_SELECTION_SCHEMA_VERSION = 2`, production pools). The teacher labels the states its own trajectory visits.

## Dataset schema and provenance

`PHASE4_DATASET_SCHEMA_VERSION = 1`. One gzip JSON file per game seed (`seed-XXXXXXXXXX.json.gz`) holds an `EpisodeRecord`:

- provenance:
  - git commit and dirty paths
  - dataset, observation, catalog, action-wire and environment action schema versions
  - policy candidate set version and candidate encoder version
  - game config version and digest
  - source policy (`canonical`, `teacher` or `learned_policy`)
  - split and split range, the 512-decision guard
  - teacher rule version and teacher scenario pools, when the source is the teacher
- episode:
  - initial and terminal clear_rate, victory, final stage, decision count, final state hash
  - for teacher episodes, the canonical baseline on the same seed
- per decision (`DecisionSample`):
  - game seed, decision index, state hash, decision point, stage and the full `Observation`
  - candidates (action, id, kind, canonical `PolicyActionSpace` index, family rank) and the legal mask
  - full policy-action-space size and legal count
  - chosen index/id/policy-action index and action kind
  - canonical index/id
  - clear_rate before/after, delta, terminated, final terminal clear_rate, victory, source policy
  - optional teacher label

The teacher label stores:

- the canonical and teacher action and the override flag
- the S4/1 proposal ids and every Reroll candidate's low-fidelity proposal score
- the raw terminal clear_rate of every proposal candidate on every discovery scenario, and the discovery means and top 3
- the raw baseline and finalist terminal clear_rate on every validation scenario
- the paired-delta / t / p / Holm statistics

Scenario order follows the pools recorded in the provenance. This is enough to replay adaptive-budget schemes offline.

Storage is resumable. Existing seed files are validated and skipped, files are written to a temporary name and renamed, duplicate seeds are rejected, `--shard-index/--shard-count` split a run, and `ml phase4 merge` combines shard directories while refusing conflicting duplicates or mixed provenance. The loader rejects provenance that does not match the current schema/config contract.

## Policy candidate set and encoder

`POLICY_CANDIDATE_SET_VERSION = 1` (`simulator/src/ml/semantic_candidates.rs`):

- Shop/CardSelection card decisions: every legal non-build semantic action (every Reroll subset, shop purchase, inventory item, treasure discard) plus the dense-order top 8 `BuildTower` pairs from `DenseBuildTowerScoreTable` (full-map legality).
- Other decision points: every legal action, with `PlaceTower` limited to the top 8 in the canonical placement order.

This set always contains the canonical action (the dense top-1 build or top-1 placement) and the teacher's whole S4/1 proposal (top-4 build, top-4 placement, every discrete action, best Reroll). The legacy 64-position proposal that PPO used is not used here.

Each candidate is a small entity set, scored by the existing candidate encoder:

- header row: action kind, decision point, family rank; for placements, route coverage, route distance, neighbor occupancy and position
- tower template row: kind, suit, rank, effective damage, range, interval, cards, splashes
- Reroll: one row per rerolled card (suit, rank, engraving, polish) plus a hand summary row (rerolled fraction, hand size, max rank and suit multiplicity, reroll count, dice)
- shop slot: kind, key, cost, affordability, whether it is the cheapest of its kind
- inventory item key, treasure option key, owned upgrade key
- card-service card; the removed tower's template and position

Candidate features include the dense heuristic rank of `BuildTower`/`PlaceTower` candidates. The dense ranking is a deterministic function of the current observation. It is the same ranking the canonical policy and the teacher proposal use.

## Progress reward invariant

`r_t = clear_rate(s_{t+1}) - clear_rate(s_t)`, gamma = 1, is checked on every stored episode by `validate_episode` (tolerance 1e-3): consecutive decisions share the boundary value, and the sum of deltas equals terminal minus initial clear_rate. `ml phase4 dataset-report` also reports per-transition and per-action-kind delta statistics. No reward shaping is added.

## Results

All runs are release builds on a 12-thread x86-64 Linux host (31 GB RAM, no GPU, burn-flex CPU backend). Datasets and runs live in `simulator/artifacts/phase4/` (not committed). Reports are committed in `artifacts/phase4/`.

### Canonical dataset

| dataset | games | decisions | mean decisions/game | mean terminal clear_rate | victories |
|---|---|---|---|---|---|
| `canonical_train` first 2,048 | 2,048 | 174,241 | 85.1 | 36.81 | 0 |
| `canonical_validation` | 64 | 5,420 | 84.7 | 36.36 | 0 |

- Generation: about 0.5 s per game, 512 games in 56 s on 12 threads; about 170 KB per game gzip-compressed (345 MB for 2,048 games).
- Rerunning a collection skips and revalidates existing seeds (512 skipped, 0 generated).
- Decision mix (first 512 games): 9,634 build_tower, 9,634 start_defense, 6,211 continue, 4,857 purchase_shop_item, 4,037 use_inventory_item, 3,079 place_tower, 2,865 reroll, 1,176 select_card_service_card, 1,165 confirm_card_service_selection, 1,124 select_treasure, 2 discard_treasure.
- Mean candidate set size 25.5, maximum 78.
- Aliasing: 115 of 43,784 decisions (0.26%) choose a candidate whose encoding is identical to another candidate's, for example two identical inventory items or two identical cards. These choices are equivalent.

### Progress reward invariant (canonical trajectories)

On all 2,048 training episodes:

- maximum telescoping error 0.0
- maximum boundary error 0.0
- 0 decisions with a negative delta

clear_rate never decreases along canonical play, and it only changes on decisions that advance simulation time:

- `PreDefenseItem -> Shop`, `TowerPlacement -> Shop` and `-> TreasureSelection`: exactly +2.0 (one full stage cleared by the defense the decision starts)
- `PreDefenseItem -> DamageResponseItem`: +0.19 to +2.0 (partial stage progress until a damage-response window opens)
- `DamageResponseItem -> Shop`: +0.002 to +1.8 (the rest of the stage)
- transitions into `Terminal`: +0.19 to +2.0

Every Shop, CardSelection, TowerPlacement (build/place), CardServiceSelection and TreasureSelection transition has delta exactly 0. The reward for a build or reroll therefore always arrives later, at the defense-starting decision. This is a credit-assignment fact for Phase 4B, not something to shape.

### BC tiny-overfit gate

`semantic_bc::tests::bc_tiny_overfit_memorizes_canonical_decisions`: 48 canonical decisions (seed 0, candidate sets larger than 1), 60 epochs, batch 16, learning rate 3e-3.

- training loss 2.667 -> 0.0145
- training top-1 100%
- 0 illegal selections

The residual loss comes from the aliased equal-encoding candidates described above. This gate is separate from the PPO `toy_overfit` bandit test.

### Canonical BC training

| scale | games | train samples | selected epoch | validation NLL | top-1 | top-3 | action-kind acc. | train time |
|---|---|---|---|---|---|---|---|---|
| small | 128 | 11,202 | 18 | 0.0356 | 98.93% | 99.61% | 99.65% | 206 s |
| medium | 512 | 43,784 | 17 | 0.0305 | 99.00% | 99.61% | 99.65% | 687 s |
| large | 2,048 | 174,241 | 16 | 0.0136 | 99.78% | 99.98% | 100.00% | 2,768 s |

Validation curves (epoch: train loss / validation NLL / validation top-1):

- small: 1: 0.916/0.277/91.3%, 5: 0.068/0.064/98.1%, 10: 0.055/0.048/98.7%, 18: 0.043/0.036/98.9%, 20: 0.054/0.049/98.9%
- medium: 1: 0.346/0.068/98.1%, 5: 0.042/0.048/98.6%, 10: 0.039/0.038/98.8%, 17: 0.034/0.031/99.0%, 20: 0.033/0.034/99.0%
- large: 1: 0.126/0.037/99.0%, 5: 0.015/0.020/99.2%, 10: 0.013/0.018/99.2%, 16: 0.013/0.014/99.8%, 20: 0.013/0.015/99.7%

Scale rule: medium improved validation top-1 by only 0.07 points, but it lowered validation NLL by 14% relative (at least 5% is required), so large was trained as the frozen rule requires.

Validation top-1 by target action kind (large; small in parentheses):

- 100%: build_tower, place_tower, start_defense, continue, use_inventory_item, confirm_card_service_selection
- reroll 100% (94.8%)
- select_treasure 100% (92.8%)
- purchase_shop_item 99.3% (96.7%)
- select_card_service_card 94.2% (93.5%)

Small's reroll errors were all reroll -> build_tower kind errors (19 of 366). The remaining large errors are card-service card choices, mostly among equal-encoding duplicate cards. By decision point (large): CardServiceSelection 97.1%, Shop 99.8%, every other point 100%. Illegal selections: 0. There are no masked candidates in the data, because every stored candidate is legal.

### Canonical BC terminal evaluation (development seeds 2,300,000-2,300,127)

| policy | mean clear_rate | median | mean stage | decisions | illegal | fallback | no-undo guard | canonical agreement |
|---|---|---|---|---|---|---|---|---|
| canonical | 36.33 | 35.06 | 18.58 | 83.0 | 0 | 0 | 0 | 100% |
| BC small | 36.30 | 34.79 | 18.55 | 82.8 | 0 | 0 | 2 | 98.8% |
| BC medium | 36.21 | 34.58 | 18.53 | 82.4 | 0 | 0 | 2 | 98.9% |
| BC large | 36.35 | 35.06 | 18.59 | 83.1 | 0 | 0 | 1 | 99.9% |

Paired delta vs canonical (mean, SE, better/worse/tie):

- small: -0.03, 0.12, 31/24/73
- medium: -0.12, 0.11, 29/26/73
- large: +0.02, 0.03, 4/4/120

No victories in any arm.

The selected canonical BC is **large** (highest development mean). A first run of the evaluation stopped at the 512-decision guard: BC small repeatedly toggled the same card-service card (`select_card_service_card` on an already selected card deselects it). The learned-policy runner now never deselects a card-service card, and every decision where this rule changed the greedy choice is counted as a guard intervention. It is not a canonical fallback. Canonical play never deselects.

### Teacher corpus

Collection: 16 teacher-controlled terminal games on `teacher_train` (seeds 2,200,000-2,200,015), frozen v2 rule, production pools, 12 threads.

- 2,291 teacher decisions, 0 forced (empty-proposal) decisions, 449 overrides (19.6%).
- 1,405,184 terminal rollouts (613 per decision), 12,389 s wall-clock (774 s per game, 5.4 s per decision), 9.3 MB of data including every raw scenario outcome.
- Real-environment steps skip the policy trace, so each rollout fork no longer copies a growing trace. This made collection about 1.5x faster than the Phase 3 gate (8,776 s for 8 games) without changing any selection. The teacher's baseline equals the canonical action at every decision; the collector checks this.
- Paired terminal clear_rate on the same seeds (teacher - canonical): mean +14.78, SE 1.99, median +12.75, 16 / 0 / 0. Canonical mean 37.42, teacher mean 52.20. This agrees with the Phase 3 gate (+15.07).
- Per-seed deltas: +8.18, +13.83, +10.19, +26.00, +11.10, +15.28, +25.55, +10.51, +4.00, +11.68, +22.24, +2.37, +23.36, +22.39, +23.34, +6.49. No victories.
- Teacher trajectories: 143 decisions per game on average (canonical 85). The progress-reward invariant holds (telescoping error 0, no negative deltas).
- Bug fixed while reading the corpus: a zero-variance validation sample has an infinite t statistic, which JSON writes as `null`. `CandidateValidationStat` now restores it from the sign of the mean delta, as `paired_one_sided_t_test` defines it. Writing and the teacher rule are unchanged.

### Teacher override and disagreement analysis

`ml phase4 teacher-analysis` (`artifacts/phase4/teacher-analysis.json`). The validation delta is the selected finalist's mean paired terminal delta on the 64 validation scenarios.

By the canonical action's kind (how often the teacher replaced it):

| canonical kind | opportunities | overrides | override rate | mean override validation delta |
|---|---|---|---|---|
| build_tower | 538 | 172 | 32.0% | +0.50 |
| start_defense | 505 | 86 | 17.0% | +0.15 |
| use_inventory_item | 249 | 72 | 28.9% | +0.19 |
| purchase_shop_item | 309 | 71 | 23.0% | +1.22 |
| reroll | 119 | 20 | 16.8% | +0.92 |
| select_treasure | 51 | 18 | 35.3% | +2.00 |
| place_tower | 101 | 9 | 8.9% | +0.15 |
| select_card_service_card | 56 | 1 | 1.8% | +0.07 |
| continue | 307 | 0 | 0% | - |
| confirm_card_service_selection | 56 | 0 | 0% | - |

By the teacher's selected kind (opportunities = decisions whose S4/1 proposal contained the kind):

| selected kind | opportunities | overrides | rate | mean validation delta |
|---|---|---|---|---|
| reroll | 943 | 176 | 18.7% | +0.69 |
| remove_tower | 606 | 82 | 13.5% | +0.12 |
| continue | 249 | 72 | 28.9% | +0.19 |
| build_tower | 966 | 56 | 5.8% | +0.74 |
| use_inventory_item | 1,825 | 24 | 1.3% | +0.61 |
| select_treasure | 51 | 18 | 35.3% | +2.00 |
| purchase_shop_item | 108 | 11 | 10.2% | +1.31 |
| discard_treasure | 1,607 | 3 | 0.2% | +0.24 |
| place_tower | 101 | 3 | 3.0% | +0.38 |
| start_defense | 101 | 3 | 3.0% | +0.08 |
| select_card_service_card | 112 | 1 | 0.9% | +0.07 |

Top override pairs (canonical -> teacher):

| pair | overrides |
|---|---|
| build_tower -> reroll | 130 |
| start_defense -> remove_tower | 79 |
| use_inventory_item -> continue | 72 |
| purchase_shop_item -> reroll | 35 |
| build_tower -> build_tower (different pair) | 30 |
| purchase_shop_item -> build_tower | 20 |
| select_treasure -> select_treasure | 18 |
| purchase_shop_item -> purchase_shop_item | 11 |
| reroll -> reroll | 11 |
| build_tower -> use_inventory_item | 10 |

By decision point:

| decision point | decisions | overrides | override rate |
|---|---|---|---|
| Shop | 688 | 165 | 24.0% |
| CardSelection | 278 | 98 | 35.3% |
| TowerPlacement | 606 | 95 | 15.7% |
| DamageResponseItem | 249 | 72 | 28.9% |
| TreasureSelection | 51 | 18 | 35.3% |
| CardServiceSelection | 112 | 1 | 0.9% |
| PreDefenseItem | 307 | 0 | 0% |

By stage (the clear_rate buckets are the same partition, because clear_rate before a decision is 2 x (stage - 1)):

| stages | 1-5 | 6-10 | 11-15 | 16-20 | 21-25 | 26-30 | 31-35 |
|---|---|---|---|---|---|---|---|
| override rate | 13.4% | 13.6% | 24.1% | 20.9% | 26.5% | 17.0% | 24.5% |
| mean override delta | +1.09 | +0.78 | +0.62 | +0.37 | +0.34 | +0.20 | +0.32 |

Validation strength:

- 283 of 449 overrides have a selected-candidate validation delta in [0, 0.5); 85 in [0.5, 1); 52 in [1, 2); 24 in [2, 4); 5 in [4, 8).
- 366 overrides have Holm-adjusted p < 0.001.
- Where the teacher kept canonical, the best finalist's delta was below 0 in 406 decisions, in [0, 0.5) in 1,368, in [0.5, 1) in 63, and in [1, 2) in 5.

Most teacher value comes from many small per-decision gains, concentrated in reroll (instead of building or buying), removing towers before a defense, skipping inventory use in damage-response windows, and treasure choice. Per-override deltas shrink in later stages.

Canonical BC (large) on the same 2,291 teacher states:

- it differs from canonical in only 7 decisions (0.3%)
- on the 449 override states it picks the canonical action 449 times and the teacher action 0 times
- the BC/canonical disagreements are purchase_shop_item (6) and select_card_service_card (1)

The BC policy has no teacher behaviour to begin with.

### Distillation

Both variants start from canonical BC large and fine-tune on the 2,291 teacher-labelled decisions with target = teacher action (Adam 3e-4, batch 64, 10 epochs, final epoch; about 21 s each). Variant A uses weight 1 for every sample. Variant B uses weight 4 for override samples.

Offline:

| model | teacher-corpus top-1 | NLL | override samples: predicts teacher / predicts canonical | agreeing samples top-1 | canonical-validation top-1 |
|---|---|---|---|---|---|
| canonical BC large | 80.0% | 19.45 | 0.0% / 100% | 99.5% | 99.8% |
| distilled A (1x) | 78.9% | 1.12 | 3.8% / 88.2% | 97.2% | 96.4% |
| distilled B (4x) | 73.1% | 1.34 | 15.6% / 73.3% | 87.1% | 85.3% |

The canonical BC is extremely confident on override states (NLL 19.4). Under the frozen budget of 10 epochs at 3e-4, the fine-tuning loss plateaus near 1.14 (A) and 2.14 (B, weighted). Neither variant fits most overrides, and B trades agreement accuracy for them.

Development evaluation (128 seeds), paired delta vs canonical BC:

- A: +0.11, SE 0.18, 48 / 31 / 49
- B: -3.22, SE 0.34, 23 / 105 / 0; decisions per game rose to 127.6

**Variant A was selected** for Gate B.

### Final paired terminal evaluation (final seeds 2,400,000-2,400,255, run once)

`artifacts/phase4/final-eval.json`, 256 seeds, every episode to the actual terminal state, 44.5 s on 12 threads.

| policy | mean clear_rate | median | mean stage | victories | decisions | illegal | fallback | no-undo guard | canonical agreement |
|---|---|---|---|---|---|---|---|---|---|
| canonical scripted | 36.59 | 35.24 | 18.71 | 0 | 85.4 | 0 | 0 | 0 | 100% |
| canonical BC (large) | 36.60 | 35.18 | 18.72 | 0 | 85.5 | 0 | 0 | 5 | 99.8% |
| distilled A (selected) | 36.71 | 35.25 | 18.76 | 0 | 88.8 | 0 | 0 | 8 | 94.6% |
| distilled B (descriptive) | 33.78 | 33.27 | 17.29 | 0 | 131.1 | 0 | 0 | 12 | 57.4% |

| comparison | mean | SE | 95% CI (normal) | median | better / worse / tie |
|---|---|---|---|---|---|
| BC - canonical | +0.01 | 0.04 | [-0.06, +0.09] | 0.00 | 7 / 6 / 243 |
| distilled A - BC | +0.11 | 0.14 | [-0.16, +0.39] | 0.00 | 88 / 55 / 113 |
| distilled A - canonical | +0.13 | 0.13 | [-0.14, +0.39] | 0.00 | 87 / 57 / 112 |
| distilled B - BC | -2.82 | 0.32 | [-3.44, -2.20] | -2.27 | 62 / 193 / 1 |
| distilled B - canonical | -2.81 | 0.31 | [-3.42, -2.19] | -2.27 | 61 / 194 / 1 |

- **Gate A: PASSED.** 0 illegal and 0 fallback actions, and a mean paired delta of +0.01 against a threshold of -2.0.
- **Gate B: POSITIVE by the frozen definition** (mean +0.11 > 0). The effect is negligible: its interval includes 0, and it is under 1% of the teacher's own +14.8.
- Distilled A's behavior changes are mostly `use_inventory_item -> continue` in damage-response windows (continue 3,810 vs 3,067 for BC) and some treasure discards. It adopted almost none of the teacher's reroll overrides (1,506 vs 1,500 rerolls) and none of its tower removals.
- Distilled B is clearly worse. It learned to skip inventory items broadly (759 uses vs 2,027), which leaves it in long runs of damage-response windows (16,551 continues) and loses about 1.4 stages.

### Search-free inference latency (1 CPU thread)

`artifacts/phase4/latency-1thread.json`, 16 development seeds, `--threads 1` (Rayon and burn-flex on one thread):

| policy | ms / decision | network forward ms | decisions / s | episode wall time |
|---|---|---|---|---|
| canonical scripted | 0.565 | - | 1,771 | 0.087 s |
| canonical BC | 3.62 | 0.34 | 276 | 0.34 s |
| distilled A | 3.66 | 0.33 | 273 | 0.35 s |
| Phase 3 teacher (12 threads) | about 5,400 | - | about 0.19 | about 774 s |

The learned policy is about 1,500 times faster per decision than the teacher on 12 threads and needs no rollouts. About 90% of its per-decision time is outside the network: candidate generation (which includes the dense build table the canonical policy also computes), the per-candidate legality check, and encoding. This has not been profiled or optimized yet.

### PPO compatibility (checked, not trained)

- The BC network is the unchanged `DeepSetsActorCritic`. `semantic_bc::tests::bc_actor_weights_load_into_ppo_model_with_separate_value_head` checks that its full-precision record loads into a fresh PPO-trainable (autodiff) model with identical actor outputs and parameter count.
- BC only trains the actor path. The critic head (`typed_critic_hidden/output`) keeps its initialization and can be trained or re-initialized separately.
- Before the BC checkpoint can be used, `ml/rollout.rs` must change to use the Phase 4 candidate set (`semantic_candidates::policy_candidates`), the new candidate encoder and the masked group log-probabilities. Today it scores `semantic_legal_actions_with_position_limit(64)` with the legacy candidate rows, which is a different action set and encoding.
- The learned-policy runner's no-undo card-service rule must be kept.
- Parameters must be materialized at creation (`new_materialized_model`), or save/resume is not reproducible.

## Tests

New tests (all pass in debug):

- `phase4_dataset`:
  - frozen splits are disjoint and exclude the Phase 3 seeds
  - canonical episodes match the terminal-gate baseline (hash, clear_rate, decisions) and telescope
  - episode serialization and provenance roundtrip
  - stored candidates' `PolicyActionSpace` indices roundtrip and are legal
  - corrupted episodes (illegal chosen action, broken delta, reserved seed) are rejected
  - teacher raw-outcome roundtrip, including an infinite t statistic
- `semantic_candidates`:
  - the canonical action is always in the candidate set, and every candidate is legal and validly encoded along canonical episodes
  - Reroll candidates encode card identity
- `semantic_bc`:
  - BC tiny-overfit
  - a masked candidate is never selected and gets probability exactly 0
  - checkpoint save/load output equality, and resume equals an uninterrupted run
  - deterministic inference on a fixed state
  - BC weights load into the PPO model
- `phase4_eval`:
  - the canonical runner matches the terminal-gate baseline
  - learned-policy terminal evaluation is reproducible, with 0 illegal and 0 fallback actions

Full `cargo test --lib` (debug): 377 passed, 6 failed, 16 ignored. The failures are unrelated to Phase 4A:

- `config::tests::embedded_config_converts_to_core_state_and_back`
- `core::tests::{direct_core_commands_preserve_flow_and_replay_progression, raw_base_damage_syncs_escape_state_when_damage_is_reduced_to_zero, raw_damage_updates_defense_progress_by_applied_hp, step_resolves_base_damage_from_raw_monster_escape}`
- the known `ml::toy_overfit::tests::contextual_bandit_overfits_within_update_budget`: it fails when run alone at the pre-Phase-4 commit `ac27e2a1` too, and it passed in one earlier full-suite run, so it depends on test order

## Known limitations

- Full-clear feasibility is still unknown. No policy won an episode, and the teacher reaches about 52 clear_rate.
- The distilled policy captures almost none of the teacher's advantage. Likely causes:
  - 2,291 labels, with 449 overrides spread across many kinds
  - a canonical-BC starting point that is extremely confident on exactly those states
  - a frozen, short fine-tuning budget
  - a candidate encoding that may not expose what makes a teacher override good (for example the long-term value of a reroll, or which tower to remove)
  None of these was tuned after seeing results, as the protocol requires.
- The teacher labels its own trajectory states (143 decisions per game). A learned policy that does not follow the teacher visits different states, and the labels do not cover them.
- Candidate features include the dense heuristic rank of build/placement candidates, so imitating canonical builds is easy by construction.
- The no-undo card-service rule is a runner-level mask (5-12 interventions per 256 games).
- Learned-policy latency is dominated by non-network work that has not been profiled.
- One seed split and one training seed; no repeated training runs.

## Conclusion and Phase 4B recommendation

1. A search-free neural policy imitates the canonical baseline essentially exactly: 99.8% validation top-1, and +0.01 terminal clear_rate on 256 untouched seeds with 0 illegal actions.
2. Plain supervised fine-tuning on 16 teacher games did not transfer the teacher's +14.8 improvement. The selected variant gained +0.11 (not distinguishable from 0), and override-weighted fine-tuning hurt (-2.8).

Recommended Phase 4B starting configuration:

- Initialize the PPO actor from `simulator/artifacts/phase4/runs/bc-large` (`selected-model.bin`, hidden size 64). Initialize the critic separately and pretrain it on canonical-trajectory returns before any policy update.
- Port `ml/rollout.rs`/`ml/ppo.rs` to the Phase 4 candidate set, candidate encoder, masked log-probabilities and no-undo rule. Verify that greedy PPO rollouts at iteration 0 reproduce the BC terminal clear_rate on the development seeds.
- Reward: exactly `r_t = clear_rate(s_{t+1}) - clear_rate(s_t)` with gamma 1 (or close to 1) and GAE. Reward only arrives on defense-advancing decisions, so build/reroll/remove decisions depend on the value function for credit.
- Keep a KL penalty or trust region toward the BC policy at the start to avoid distill-B-style collapse. Track the per-kind action distribution (continue vs use_inventory_item, reroll, remove_tower) each iteration.
- Train on `canonical_train` seeds beyond 2,048, model selection on `development_evaluation`, and a new untouched final split for Phase 4B. The Phase 4A final seeds have now been used once.
- Keep the teacher as an optional selective labeler for states where the PPO policy and teacher disagree (DAgger-style), using the stored raw outcomes for adaptive-budget replay.
