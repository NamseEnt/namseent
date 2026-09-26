# Phase 4A: Canonical BC and Selective Teacher Distillation

Status: in progress. Sections marked **Frozen** were written before any Phase 4A data was generated or any model was trained, and are not changed after results are seen.

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

(Teacher corpus, distillation and final results below.)
