# Policy v2: Factorized Policy and Full Action Space

Status: in progress. Sections marked **Frozen** were written before the corresponding v2 models were trained and are not changed after results are seen.

Phase 4B ([`14-phase4b-ppo.md`](14-phase4b-ppo.md)) showed that PPO from a canonical BC initialization beats the canonical baseline (+4.98 terminal clear_rate on the frozen final seeds), but tower placement never changed. The v1 network only scores the heuristic's top-8 placements and top-8 dense builds, although `PolicyActionSpace` already represents the full `subset x hand_slot x position` space. Policy v2 changes the policy representation, not the `AgentAction` contract or the game rules.

## Frozen plan

| stage | change | purpose |
|---|---|---|
| 0 | profile candidate generation, remove obvious duplicate work (30-minute cap) | cheaper rollouts; results must not change |
| A0 | family-factorized probability `P(family) x P(candidate given family)` with v1 features and the v1 candidate set | validate the factorization; A0a: flat-equivalent log-sum-exp family logits must reproduce v1 exactly; A0b: learned family head removes action-multiplicity bias |
| A1 | feature normalization contract for actor and critic, entropy scheme per head | fixed before B and C |
| B | full-position policy (redefined below: PlaceTower and BuildTower positions) | removes the top-8 position ceiling only |
| C | full-build-selection policy: every legal card subset and hand slot, then every legal position | removes the top-8 (subset, slot) proposal |
| D | vectorized environments with batched inference per head, CUDA re-measurement | throughput |

Rules for every stage:

- `AgentAction` and `ACTION_SCHEMA_VERSION` stay; the policy representation version and model/checkpoint schemas change.
- Invariants tested: every sampled action is a legal `PolicyActionSpace` index; sampled action == executed action; the sum of the factor log-probabilities equals the joint log-probability; PPO clips one joint ratio.
- BC gate per stage (new architecture = new BC): paired terminal clear_rate vs canonical >= -2, illegal = 0, fallback = 0, sampled != executed = 0, on `ppo_development`.
- PPO comparison with v1 on the same schedule: 75 iterations at actor Adam 3e-4 and the phase-1 entropy setting, then 200 iterations at the phase-2 setting, 48 games per iteration, the same `ppo_train` seed blocks as v1 (paired), the same critic initialization. Curves are compared on cumulative semantic decisions (primary), cumulative training games and wall-clock time.
- Stage B records, per iteration and in evaluation: the TowerPlacement share differing from the canonical action, placement spatial entropy, the share of placements outside the canonical top 8, and their terminal effect. Stage C records the same for builds.
- Development evaluation uses `ppo_development` (4,000,000-4,000,127), paired with the v1 results. `phase4b_final` has been used and is never reused. A new `v2_final` split (4,200,000-4,200,255) is run once, after the v2 architecture and recipe are frozen.
- Teacher labels are not used.

## Stage 0: candidate generation profile

`perf` is unavailable without root (`perf_event_paranoid`), so `semantic_candidates::profile::candidate_pipeline_profile` times each step single-threaded on 579 canonical decisions (development seeds 4,000,000-4,000,007):

| step | before (ms/decision) | after |
|---|---|---|
| card decisions: `policy_candidates` | 1.58 | 0.30-0.42 |
| of which BuildTower top-k selection | 1.34 | 0.17 |
| card decisions: candidate legality mask | 0.59 | 0.08 |
| other decisions: `policy_candidates` | 0.34 | 0.33 |
| other decisions: candidate legality mask | 0.71 | 0.05 |
| encoding | 0.03 | 0.03 |

Two fixes:

- `DenseBuildTowerScoreTable::top_k_indices` sorted every legal `(subset, slot, position)` triple to keep 8. It now uses `select_nth_unstable_by` with the same total order and sorts only the selected prefix, which gives exactly the same result.
- `semantic_action_is_legal` rebuilt the full legal-action list (and, for builds, the tower placement context) for every candidate. `semantic_actions_are_legal` checks all candidates of one state with a per-state cache; the per-action rule is unchanged.

Equivalence: a development evaluation of canonical, `phase4b-init` and the v1 PPO checkpoint (run C iteration 200) reproduces every final state hash (256/256 for canonical and BC) and the PPO mean (42.3427) exactly.

Single-thread search-free inference (16 development seeds): canonical 0.574 -> 0.447 ms/decision, BC 3.84 -> 1.37, PPO 4.02 -> 1.31.

## Stage A0: family-factorized policy

`simulator/src/ml/policy_v2.rs`, `POLICY_REPRESENTATION_VERSION = 2`:

- `PolicyNet` = the unchanged `DeepSetsActorCritic` candidate scorer plus a family head (`Linear(2H -> H) -> ReLU -> Linear(H -> 20)` over the encoded global state and the typed-entity state). The family of a candidate is its `ActionKind`, stored per candidate in `EncodedDecision::families`.
- `P(candidate) = P(family) x P(candidate | family)`. The conditional is a softmax over the legal candidates of that family. Families without a legal candidate get probability 0.
- `KindMode::LogSumExp`: family logit = log-sum-exp of its candidate logits, which makes the product exactly the flat softmax. `KindMode::Learned`: family logits come from the family head, so a family's probability no longer depends on how many candidates it has (Shop decisions mix up to 31 Reroll subsets with a few purchases).
- BC and PPO consume the joint log-probability `log P(family) + log P(candidate | family)`; PPO clips the single joint ratio and the entropy term is the exact joint entropy (same as v1).
- Checkpoints record `policy_representation_version`. A v1 (flat) checkpoint loads as a `PolicyNet` in `LogSumExp` mode, so every v1 BC/PPO checkpoint remains usable.

### A0a: flat equivalence

- `policy_v2::tests::logsumexp_factorization_equals_the_flat_softmax`: on real decisions, the family distribution sums to 1, `kind + conditional == joint`, and the joint equals the v1 flat log-softmax within 1e-4.
- The v1 checkpoints evaluated through `PolicyNet` (`LogSumExp`) on all 128 development seeds reproduce every final state hash: canonical 128/128, `phase4b-init` 128/128, v1 PPO run C iteration 200 128/128 (means 36.2342 / 36.0953 / 42.3427).

A0a passes.

### A0b: learned family head

BC: `phase4b-init`'s scorer plus a new family head (`--kind-mode learned`), the same 2,048-game canonical dataset, Adam 1e-3, batch 64, budget 6 epochs, lowest validation NLL selected.

The family head reads the encoded state plus, per family, the mean candidate embedding and `log1p(count) / 8` of that family and a family one-hot. A first head that read only the state plateaued at 93.9% family accuracy (validation NLL 0.139 after one epoch) and was aborted.

- BC: validation NLL 0.0121, top-1 99.46%, family accuracy 100%. Gate on `ppo_development`: -0.15 (SE 0.13) vs canonical, illegal 0, fallback 0, post-sampling mutations 0.
- PPO: the v1 schedule (phase 1: 75 iterations, joint entropy 0.01; phase 2: 200 iterations from phase-1 iteration 75, joint entropy 0.003, `ppo_train` seed block offset 1000), the Phase 4B squash-all critic.

Development delta vs canonical at matched cumulative semantic decisions (paired, 128 seeds, SE in parentheses):

| decisions (M) | v1 | A0b |
|---|---|---|
| 0.08 | +0.24 (0.22) | +0.23 (0.22) |
| 0.26 | +2.40 (0.37) | +1.62 (0.39) |
| 0.45 | +3.08 (0.45) | +2.94 (0.47) |
| 0.66 | +4.42 (0.50) | +3.42 (0.48) |
| 0.87 | +5.24 (0.61) | +4.17 (0.50) |
| 1.10 | +5.34 (0.54) | +4.62 (0.51) |
| 1.33 | +5.52 (0.57) | +5.87 (0.66) |

End of schedule: v1 +6.11 at 1.45M decisions (2.95 h), A0b +5.34 at 1.36M decisions (2.63 h; best +5.87 at iteration 195). The learned family head neither helps nor hurts clearly: A0b trails v1 by about one standard error for most of the curve and matches it at the end. The sampled family entropy rises slowly (0.002 at phase-1 iteration 25, 0.02 at iteration 75, about 0.15 late in phase 2). Action-multiplicity bias of the flat softmax was not a major limit of v1.

## Frozen stage A1 specification

Written before any A1 model was trained.

### Input normalization contract

Every numeric input slot of the actor and the critic is kept or passed through `sign(x) * ln(1 + |x|)`. The table was derived once from 21,436 canonical decisions (`phase4b_canonical_train`, first 256 games, `feature_contract::tests::canonical_feature_scale_survey`) and is frozen in `simulator/src/ml/feature_contract.rs`: binary slots and slots with max |x| <= 2 (ratios, coordinates, bounded counts, already log-scaled values) are kept; unbounded slots are squashed.

| input | squashed slots (max abs value on the survey) |
|---|---|
| global features | 21 (1,466), 29 (1,333), 30 (2.1), 56 (6.5), 59 (119,875), 60 (1,333), 77-82 (1,000 each) |
| typed entity numerics | set 0 col 0 (8,000), set 1 col 0 (6,000), set 2 col 1 (2.1), set 3 col 0 (8,000), set 4 col 0 (6,000), set 5 cols 2/4/5 (17.5 / 110 / 50), set 6 col 1 (131,500) and col 3 (62.5), set 9 cols 3/4 (5 / 4) |
| candidate numerics | col 0 (6,000), col 2 (5) |

All other slots are kept. The v1 actor read these values raw (card polish enters as `polish_pct_raw / 1000`), which is what broke the raw-input critic in Phase 4B.

The A1 critic uses this contract too (instead of squashing every slot), so actor and critic share one input definition.

### Entropy scheme

The entropy bonus is computed per head and normalized by the head's maximum entropy:

- family head: `H(P(family)) / ln(number of families with a legal candidate)`;
- candidate head: `sum_f P(f) * H(P(candidate | f)) / ln(number of legal candidates in f)`;
- a head with at most one legal choice contributes 0;
- loss term: `-(c_family * mean normalized family entropy + c_candidate * mean normalized candidate entropy)`.

Coefficients: phase 1 `c_family = c_candidate = 0.02`, phase 2 `0.006`. The normalization divides the gradient by `ln n` (about 2-3.5 for typical head sizes), so doubling the v1 coefficients (0.01 / 0.003 on the joint entropy) keeps a comparable push; in stage B the placement head (`ln 1296 = 7.2`) will not dominate. This scheme and these coefficients stay fixed for stages B and C.

### A1 training

- BC from scratch (seeded initialization), family head as in A0b, normalized inputs, the same data, optimizer and 6-epoch budget; gate as above.
- Critic pretrained with the contract (512 games, 4 epochs, as in Phase 4B).
- PPO on the v1 schedule (phase 1: 75 iterations; phase 2: 200 iterations from phase-1 iteration 75), same seed blocks.

### A1 results

- BC (from scratch, learned family head, normalized inputs): epoch 4 selected, validation NLL 0.0107, top-1 99.61%, family accuracy 100%. Gate: -0.18 (SE 0.12) vs canonical, illegal 0, fallback 0, post-sampling mutations 0.
- Contract critic: validation MSE 0.147, explained variance 0.884 (squash-all critic: 0.156 / 0.877).
- PPO phase 1 with the normalized per-head entropy (0.02 / 0.02): **stopped post hoc at iteration 24 after severe policy drift; not used for model comparison.**

| iteration | vs canonical | KL to init | normalized family entropy |
|---|---|---|---|
| 5 | +0.06 (0.25) | 0.27 | 0.03 |
| 10 | -0.85 (0.36) | 0.55 | 0.05 |
| 15 | -1.08 (0.39) | 1.13 | 0.11 |
| 20 | -3.67 (0.42), better/worse 17/111 | 3.90 | 0.22 |

At iteration 20, 70% of `DamageResponseItem` decisions differed from canonical and development games took 123 decisions (canonical 84): the policy started using items it used to skip. Illegal and mismatch counts stayed 0, the update KL stayed below the target and the critic explained variance stayed about 0.98, so this was not numerical instability. A0b at the same budget had KL to init 0.02 and family entropy about 0.001.

Interpretation: with one coefficient on normalized entropies, the relative regularization strength on a small head (2-3 legal families, `ln n` about 0.7-1.1) is much larger than under the v1 joint entropy, and it randomizes the family choice. The assumption above that doubled coefficients keep a comparable push was wrong for the family head.

## Frozen stage A1' specification

Written after the A1 PPO failure and before any A1' PPO run. A1' isolates the input normalization contract; A1' vs A0b differs only in the normalized inputs (actor and critic).

- Actor: the A1 BC (`v2a1-bc`, normalized inputs, learned family head), unchanged.
- Critic: the A1 contract critic (`critic-512-contract`), unchanged.
- Entropy: the v1/A0b joint entropy (`--entropy-scheme joint`), 0.01 in phase 1 and 0.003 in phase 2.
- Everything else as A0b: 75 + 200 iterations, actor Adam 3e-4, the same seed blocks (phase-2 offset 1000).

Interpretation, fixed in advance: A1' above A0b means the normalization helps, A1' about A0b means no clear effect, A1' below A0b means it hurts. If A1' reproduces A0b-level performance, the entropy objective is not tuned further and stage B follows. Stage B then uses the joint entropy too; how the position head enters the entropy term is frozen in the stage B specification before any stage-B PPO run.

### A1' results

PPO from `v2a1-bc` and `critic-512-contract`, joint entropy 0.01 / 0.003, the A0b schedule. Development delta vs canonical at matched cumulative semantic decisions:

| decisions (M) | v1 | A0b | A1' |
|---|---|---|---|
| 0.08 | +0.24 (0.22) | +0.23 (0.22) | +1.44 (0.39) |
| 0.26 | +2.40 (0.37) | +1.62 (0.39) | +4.19 (0.55) |
| 0.45 | +3.08 (0.45) | +2.94 (0.47) | +0.21 (0.54) |
| 0.66 | +4.42 (0.50) | +3.42 (0.48) | +4.55 (0.56) |
| 0.87 | +5.24 (0.61) | +4.17 (0.50) | +6.64 (0.67) |
| 1.10 | +5.34 (0.54) | +4.62 (0.51) | +7.19 (0.69) |
| 1.33 | +5.52 (0.57) | +5.87 (0.66) | +6.13 (0.71) |
| 1.45 | +6.11 (0.64) | | +8.15 (0.77) |

End of schedule: A1' +10.85 (SE 0.80, better/worse 119/9) at 1.93M decisions (3.60 h), best +10.94 at phase-2 iteration 170. The development mean terminal clear_rate is 47.09 (canonical 36.23, A0b 41.58).

- The curve is much noisier than A0b. Phase 1 reached +4.19 at iteration 55, fell to -2.76 at iteration 70 and phase 2 reached -3.18 at iteration 10 before recovering; from phase-2 iteration 100 on every evaluation is between +5.4 and +10.9.
- The policy moves much further from its initialization: KL to init 13 at phase-1 iteration 65 (A0b 1.6) and 27 at the end (A0b 16); joint entropy 0.91 at the end (A0b 0.76).
- It found a different game plan. Development games take 233 decisions (A0b 119, canonical 84). At phase-2 iteration 200 the training rollouts spend 31% of decisions on rerolls (A0b 21%), 25% on continue (12%) and none on card service selections (A0b 4%); `DamageResponseItem` decisions quadruple (1,900 vs 456 per iteration) and 79% of them differ from canonical.
- Compared on matched decisions A1' is ahead of v1 and A0b from about 0.8M decisions on; per training game it is further ahead because its games are longer.

Interpretation (the rule fixed above): A1' is above A0b, so the input normalization contract helps. This is one training seed and the curve is noisy, so the size of the gain is uncertain; the direction is consistent over the last 100 iterations. The entropy objective is not tuned further; stage B follows.

## Frozen stage B specification (full-position policy)

Written before any stage-B model was trained. The stage was redefined after checking the game structure: the main tower of every build is placed inside the `BuildTower` macro (about 17 per game), and `PlaceTower` decisions only place extra towers (about 5 per game), so a PlaceTower-only stage would leave most placements capped.

### Candidate set (`CandidateMode::FullPosition`)

- Card decisions: the v1 top-8 `BuildTower` joint actions `(subset, hand slot, position)` are projected to `(subset, hand slot)` pairs, deduplicated in v1 order. Each pair is one option and allows every legal position.
- Tower placement decisions: the v1 top-8 `PlaceTower` actions are projected to their hand slots the same way.
- Every other candidate is identical to v1. `full_position_options_are_the_projection_of_the_v1_top8` pins this.

The option set is exactly what v1 could reach, so stage B changes only the position. Stage C widens the options.

### Policy

`P(a) = P(family) x P(option | family) x P(position | option)`.

- The option is scored like any candidate, with the v1 encoding of its heuristic-best position.
- The position head scores every legal cell of the chosen option: `MLP([encoded state, typed state, option embedding, cell features])`.
- Cell features: heuristic rank percentile within the option, heuristic top-1 and top-8 flags, route coverage for the tower's range, distance to the route, neighboring occupancy, x, y. All are in [0, 1].
- The heuristic order is an input, never a filter.
- `POLICY_REPRESENTATION_VERSION = 3`. Stage-A checkpoints load with a seeded, untrained cell head.
- PPO stores and clips the joint log-probability. The KL-to-init monitor and penalty include the position head.

### Training

- BC:
  - warm-started from the A1 BC (scorer and family head; the cell head starts untrained), normalized inputs;
  - samples are rebuilt by replaying the canonical dataset games from their seeds (cell legality needs the environment); every replayed state must match its recorded hash;
  - the target cell of a canonical placement is the option's heuristic-best cell;
  - same optimizer and 6-epoch budget; same BC gate.
- Critic: the A1 contract critic.
- PPO: the A1' recipe with the same seed blocks: 75 iterations, then 200 iterations from phase-1 iteration 75 (`ppo_train` offset 1000), actor Adam 3e-4, 48 games per iteration.

### Entropy rule (frozen before any stage-B PPO run)

Written after the A1' result and the B BC gate, before any stage-B PPO run; it replaces the normalized per-head entropy written above.

- The entropy bonus is the exact joint entropy of `P(family) x P(option | family) x P(position | option)`: the candidate-level joint entropy (as in A0b and A1') plus, for a spatial step, the raw entropy of the position conditional of the chosen option, `H(position | option)` in nats. By the chain rule this is a single-sample estimate of the full joint entropy.
- Coefficients: 0.01 in phase 1, 0.003 in phase 2, the same as A1'. There is no separate position coefficient and no normalization by `ln(cells)`.
- Rationale: the BC position head is almost deterministic (all 1,604 validation placements on the heuristic-best cell), so without a position term PPO would barely explore positions and a "stays in the v1 top 8" result could not be interpreted. The position conditional can reach `ln(cells)` (up to about 7 nats), so the recorded metrics below are also the drift monitor.

### B BC results

- Warm start from `v2a1-bc`, `--candidate-mode full-position`, 173,945 replayed training samples (every replayed state matched its recorded hash), about 21 GB resident during training.
- Epoch 5 selected: validation NLL 0.0107, top-1 99.54%, family accuracy 100%; the cell head reproduces the heuristic-best cell on all 1,604 validation placements (cell NLL 2e-5).
- Gate on `ppo_development`: -0.03 (SE 0.07) vs canonical, better/worse/tie 12/10/106, illegal 0, fallback 0, post-sampling mutations 0. All 3,087 greedy placements (2,366 BuildTower, 721 PlaceTower) are the option's heuristic-best cell, so no position leaves the v1 top 8 yet.

### B PPO (stopped)

**Stopped post hoc at phase-1 iteration 58: the position hypothesis could not be tested because the position head was saturated; not used for model comparison.**

- Over all 58 iterations the position-head entropy was 0.0000 for both kinds, and every executed BuildTower (about 950 per iteration) and PlaceTower (about 250) position was the option's heuristic-best cell: 0% outside the v1 top 8, mean distance 0.
- The rest of the policy did learn: KL to init 2.18 at iteration 58, 34% of BuildTower actions differed from canonical (all through the (subset, slot) option), development +0.58 (SE 0.21) at iteration 20, +1.31 (0.32) at 40, +1.64 (0.39) at 55.
- Cause: BC trained the position head on a one-hot target (cell NLL 2e-5), so `P(best cell)` is 1 to float precision. Both the entropy gradient and the PPO surrogate gradient of a saturated softmax vanish, so rollouts never sampled another cell.

## Frozen stage B' specification

Written after the stopped B run and before any B' model was trained. B' differs from B in one setting.

- BC cell target with label smoothing `e = 0.02`: `(1 - e)` on the option's heuristic-best cell plus `e` spread uniformly over all of the option's legal cells (`--cell-label-smoothing 0.02`). The loss is `-((1 - e) log P(best cell) + e * mean over legal cells of log P(cell))`. Only the cell term is smoothed; family and option targets are unchanged.
- Everything else is B: warm start from `v2a1-bc`, normalized inputs, learned family head, full-position candidates, replayed samples, 6-epoch budget, same BC gate; the A1 contract critic; the B entropy rule (exact joint entropy including the position conditional, 0.01 then 0.003, no position coefficient); the A1' schedule and seed blocks; the metrics and interpretation below.

### Recorded, per training iteration and per development evaluation

Primary B metric: the share of executed BuildTower and PlaceTower positions outside the state's v1 top-8 actions, together with the terminal clear_rate of episodes with and without such positions. Monitored for drift: KL to init (including the position head), joint entropy, position-head entropy per kind, decisions per game, clip fraction, explained variance.

- `BuildTower` and `PlaceTower` separately:
  - share of executed positions outside the state's v1 top-8 actions;
  - share differing from the canonical action;
  - mean heuristic rank percentile of the chosen cell within its option;
  - mean Manhattan distance from the option's heuristic-best cell;
  - cell-head entropy.
- Mean terminal clear_rate of development episodes with at least one position outside the v1 top 8, and of episodes with none.
- Development performance vs v1 PPO and vs stage A at matched cumulative decisions. The stage-A reference is A1', which shares B's BC lineage, input contract, critic and entropy objective.

Interpretation, fixed in advance:

- B beats A and a substantial share of placements leave the v1 top 8: evidence that the position ceiling limited PPO.
- B chooses almost only v1 top-8 positions and performs like A: the position-ceiling hypothesis is weakened, and stage C (the (subset, slot) proposal) becomes the main suspect.

### B' BC and PPO results

- BC selected epoch 5: validation NLL 0.0165, top-1 99.54%, family accuracy 100%, cell top-1 100% (cell NLL 0.0198). Gate: +0.04 (SE 0.06) vs canonical, better/worse/tie 14/9/105, illegal 0, fallback 0, post-sampling mutations 0.
- PPO completed the frozen 75 + 200 schedule. The phase-1 iteration-75 development delta was +0.44 (SE 0.15), 48/34/46 better/worse/tie. The best development delta was +0.65 (SE 0.23) at phase-2 iteration 15 (0.358M cumulative training decisions). It turned negative by phase-2 iteration 25 and ended at -19.07 (SE 0.40), 0/128/0 better/worse/tie, at 0.972M decisions. Final mean terminal clear_rate was 17.16 vs canonical 36.23; mean decisions were 67.1 vs 84.0. Illegal actions and action mismatches were 0.
- Spatial smoothing removed the saturation: sampled training rollouts had 54.5% outside-v1-top-8 positions at phase-1 iteration 75 (605/1,111; cell entropy 4.34 BuildTower, 4.13 PlaceTower) and 24.4% at phase-2 iteration 200 (223/914; entropy 2.19, 2.23). The greedy final development policy made no outside-top-8 placements, so exploration did not become a greedy preference for those positions.
- The final development policy also collapsed in other choices: across 128 games it chose RemoveTower 1,684 times and Reroll 3,348 times, and its mean terminal clear_rate fell to 17.16. This is a severe performance regression, not a useful B' checkpoint.

Development deltas vs canonical at approximately matched cumulative semantic decisions (nearest recorded evaluation; paired 128-seed reports):

| decisions (M) | B' decisions (M) | B' | A1' decisions (M) | A1' |
|---:|---:|---:|---:|---:|
| 0.08 | 0.083 | +0.50 (0.15) | 0.085 | +1.63 (0.35) |
| 0.26 | 0.260 | +0.51 (0.26) | 0.252 | +4.19 (0.55) |
| 0.45 | 0.444 | -0.71 (0.31) | 0.450 | +0.21 (0.54) |
| 0.66 | 0.645 | -5.07 (0.42) | 0.665 | +4.55 (0.56) |
| 0.87 | 0.872 | -17.79 (0.42) | 0.867 | +6.64 (0.67) |
| 0.97 | 0.972 | -19.07 (0.40) | 0.977 | +3.92 (0.62) |

Interpretation: label smoothing made position exploration possible, but B' did not learn a greedy preference for off-top-8 placements and sharply underperformed both canonical and A1'. This run does not support the position-ceiling hypothesis, but it does not establish that top-8 is not a bottleneck.

### B' attribution check

This check uses only the persisted PPO training records and scheduled `ppo_development` evaluations (128 seeds); it does not use final seeds or run new evaluations. Development performance first turned negative at phase-2 iteration 25 and remained below canonical through iteration 40. Training-rollout clear_rate did not fall monotonically over the same window.

| phase-2 iter | dev delta vs canonical | sampled outside top-8, Build / Place | RemoveTower | Reroll | KL to init | joint H | family H | clip | critic EV |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 20 | +0.44 (0.24) | 27.4% / 19.8% | 0.14% | 9.6% | 0.226 | 0.679 | 0.0035 | 0.687 | 0.972 |
| 25 | -1.79 (0.45) | 22.0% / 16.8% | 0.91% | 10.4% | 0.139 | 0.525 | 0.0047 | 0.687 | 0.965 |
| 26 | — | 25.8% / 19.7% | 8.38% | 11.2% | 0.102 | 0.564 | 0.0014 | 0.720 | 0.917 |
| 30 | -0.50 (0.31) | 19.8% / 14.7% | 4.16% | 13.5% | 0.172 | 0.423 | 0.0039 | 0.742 | 0.968 |
| 35 | -0.71 (0.31) | 28.4% / 22.5% | 7.75% | 15.2% | 0.134 | 0.584 | 0.0123 | 0.759 | 0.954 |
| 40 | -0.94 (0.38) | 21.5% / 14.8% | 7.40% | 15.7% | 0.105 | 0.447 | 0.0003 | 0.767 | 0.960 |

- The first sustained development drop does not line up with a rise in sampled outside-top-8 placements: that share was lower at iterations 25–30 than around the positive iteration-15 to -20 evaluations. Across phase-2 iterations 1–40, the descriptive iteration-level correlation between outside-top-8 share and training clear_rate is +0.04; this is not a per-episode or causal comparison.
- Other action choices shifted in the same period. RemoveTower was near zero through iteration 20, rose to 8.38% at iteration 26, and stayed around 4–8% through iteration 40. Reroll rose from 9.6% at iteration 20 to 15.7% at iteration 40. Continue fell from 13.4% to 10.5%; BuildTower fell from 21.1% to 18.4%; use-inventory-item fell from 9.9% to 8.6%. DamageResponseItem's share also fell (about 9.5% at iteration 20 to 6.8% at iteration 40). The RemoveTower rate has a descriptive iteration-level correlation of -0.67 with training clear_rate over iterations 1–40, but seed-block differences prevent causal interpretation.
- No abrupt KL-to-init jump or entropy spike marks the first development drop: KL-to-init declined from 0.226 at iteration 20 to 0.139 at 25 and 0.102 at 26. Joint entropy also fell from 0.679 to 0.525; family entropy stayed below 0.013. Clip fraction was already high and rose gradually. Critic explained variance dipped to 0.917 at iteration 26, then recovered above 0.95. This points to a broad action-distribution shift rather than a sudden KL explosion; the RemoveTower increase begins at the same time as the first rollout clear_rate dip (iteration 26).
- Position-specific outcome quality cannot be calculated from the saved training records: each iteration stores aggregate clear_rate and aggregate spatial counts, but not each episode's clear_rate grouped by whether it used an outside-top-8 position. In the scheduled greedy development evaluations, all 57 checkpoints (phase 1 and phase 2) had 0% outside-top-8 positions. No intermediate greedy checkpoint adopted such a position.

Attribution classification: **C — conclusion withheld.** The observed decline coincides more closely with RemoveTower and broader action-family changes than with an increase in outside-top-8 sampling, and the iteration-level outside-position rate has no relationship with training clear_rate. However, because the greedy policy never adopted outside-top-8 positions and the per-episode spatial outcome split was not persisted, this run cannot tell whether sampled outside positions were useful, neutral, or harmful. B' is therefore contaminated for a causal claim about the position ceiling; it does not show that opening the full position set caused the collapse, nor does it rule out a position ceiling.


## Frozen V2-B position-only PPO preregistration

Name: `V2-B-position-only` (artifacts: `v2b-position-only-p1`, `v2b-position-only-p2`). Written before implementation, smoke tests, development evaluations, or training for this ablation.

### Question and initialization

Question: with the A1′ non-position strategy fixed, does learning only BuildTower/PlaceTower cell choices over all legal positions improve paired `ppo_development` terminal clear_rate over the A1′ frozen policy?

- The A1′ actor is `v2a1p-ppo-p2/iter-0170` (best recorded A1′ development checkpoint); its scorer, shared/state encoder, family/kind head, and every other actor parameter are immutable.
- Initialize only `cell_input`, `cell_hidden`, and `cell_output` from the selected `v2bp-bc` model (epoch 5; `CandidateMode::FullPosition`; cell smoothing ε=0.02). Do not copy any other B′ parameters.
- Both sources declare representation v3, hidden size 64, learned family mode, normalized inputs, and the same policy/candidate/encoder/game-rules versions. Smoke tests must verify tensor shapes and source digests before composing them.
- To preserve A1′'s non-position policy exactly, evaluate its original Top8 candidate distribution and marginalize it onto the projected BuildTower `(subset, slot)` and PlaceTower `slot` options by summing probability mass over Top8 positions belonging to each option. Non-spatial actions map one-to-one. The sampled joint action probability is this frozen family/option marginal multiplied by the trainable `P(cell | option)`. Do not substitute representative-option logits for the marginal.
- For greedy evaluation, select the projected option containing A1′'s highest-probability original Top8 action, then select the cell with `P(cell | option)`. This keeps the deterministic family/option choice identical to A1′ while allowing only the position choice to change; greedy selection is intentionally distinct from argmax of the summed option marginal. At initialization, the greedy cell must be heuristic-best; sampled rollouts must have outside-top-8 BuildTower and PlaceTower actions; illegal/mismatch must be zero. If these checks fail, stop without training and report the failed correspondence.

### Frozen actor, objective, and PPO budget

- Only the three cell MLP layers listed above receive actor optimizer updates. The critic may update. All other actor parameters and optimizer state remain fixed.
- For each transition, old/new joint log-probability includes the frozen family/option marginal and the cell conditional. Since the frozen term is identical, the PPO ratio must equal `exp(new_cell_log_prob - old_cell_log_prob)` for spatial transitions and 1 for non-spatial transitions. Tests compare both forms numerically.
- Optimize the PPO clipped objective on the joint action log-probability. The only actor entropy bonus is `coefficient * sum(H(cell | option) for spatial decisions) / semantic transition count`, matching the scale of B′'s position term. Frozen family/option entropy is logged separately and has no actor-loss contribution.
- Keep the B′/A1′ recipe: 48 episodes/iteration, γ=1, GAE λ=0.95, reward scale 0.1, actor cell-head Adam 3e-4, critic Adam 3e-4, 4 epochs, minibatch 256, clip 0.2, max grad norm 0.5, target KL 0.02, no KL penalty, seed 0. Entropy coefficient is fixed at 0.01 for phase 1 and 0.003 for phase 2.
- Run phase 1 for 75 iterations on the existing `ppo_train` blocks (offset 0), with a mandatory pilot review at iteration 50. If pilot gates pass, continue to 75, then run phase 2 for 200 iterations from phase-1 iteration 75 on offset 1000. Evaluate greedily every 5 iterations on the existing 128 `ppo_development` seeds. Never use `phase4b_final` seeds.
- At iteration 50, stop only for the preregistered experiment-validity failures below; a temporary development-performance decline is not a stopping condition. Do not tune coefficients or other hyperparameters mid-run.
- Before training, the initialization audit compares greedy family/projected-option choices on A1′-driven trajectories for all 128 existing `ppo_development` seeds (4,000,000–4,000,127). The separate sampled-action smoke uses seeds 8,900,000–8,900,047; these are smoke-only and are not reused in `ppo_train` or evaluation.

### Required records and stop rules

Persist every training episode's seed, terminal clear_rate, BuildTower/PlaceTower position decision counts, outside-top-8 counts by kind, non-heuristic-best count, whether any outside-top-8 cell was used, and each selected cell with its heuristic rank. Each iteration records per-kind sampled and greedy outside-top-8 rates, position entropy and KL-to-init, joint KL, dev clear_rate, critic EV, clip fraction, illegal/mismatch, and frozen-actor invariant result. Development reports pair canonical, frozen A1′, and position-only on exactly the same seeds. Analyze episode-level clear_rate with/without outside cells descriptively, and paired position-only minus A1′ deltas; do not treat either observational split as causal.

Stop immediately and invalidate the run if any frozen non-position actor tensor changes beyond 1e-7 absolute tolerance, a sampled action is illegal or differs from execution, any non-finite value occurs, or saved joint/cell ratio checks fail. At the iteration-50 pilot, also stop if cumulative sampled outside-top-8 rate is below 1% for either BuildTower or PlaceTower; do not proceed by changing hyperparameters. A brief dev decline alone never invalidates the experiment.

### Interpretation fixed before results

- **A — position ceiling was a bottleneck:** the greedy policy adopts outside-top-8 positions and paired development clear_rate improves over frozen A1′ with the gain appearing alongside position-policy change.
- **B — position ceiling is unlikely to be a major bottleneck here:** sampled exploration is adequate, the learned greedy position policy returns to heuristic-best, and paired performance does not improve over A1′.
- **C — inconclusive:** exploration/credit assignment fails or a technical invariant is violated. Do not proceed to V2-C on the basis of an invalid run.

### V2-B-position-only completed result

- Both phases completed from the preserved pilot checkpoint 50: phase 1 ended at iteration 75 and phase 2 at iteration 200. No final-seed split was used. Every trained iteration passed the frozen-actor invariant; the cell head changed on every trained iteration. Illegal actions, action mismatches, and non-finite update skips were all 0.
- Phase 2 sampled outside-top-8 at 29,400 / 234,346 BuildTower decisions (12.55%) and 1,791 / 42,420 PlaceTower decisions (4.22%). Mean per-rollout cell entropy was 1.03 nats for BuildTower and 0.43 for PlaceTower. The position head therefore received substantial sampled exploration and updates.
- Greedy `ppo_development` evaluation at every saved checkpoint (57 checkpoints across the two phases) selected no outside-top-8 positions for either kind. Its clear_rate was unchanged at 47.3289, compared with 47.1779 for the paired frozen A1′ baseline: +0.1510 (SE 0.1653), with 56/52/20 better/worse/tie. This same paired result held at phase-1 iteration 75 and phase-2 iteration 200.
- In phase-2 training episodes, outside-top-8 use was associated with a higher terminal clear_rate: 48.34 over 8,863 episodes that used one or more such positions, versus 45.05 over 737 episodes that did not. The unweighted mean of within-iteration differences was +3.69 (SE 0.40; 156 iterations had both groups). This is descriptive only: chosen positions and the states that expose them are not randomized as an episode-level treatment, so it does not establish a causal benefit.
- **Interpretation: B — under this recipe, position ceiling is unlikely to be a major bottleneck.** Full-position sampled exploration was frequent, the learned greedy policy stayed on heuristic-best cells, and paired development performance did not improve over A1′ beyond noise. This is evidence about this feature/reward/PPO setup, not a general proof that top-8 can never be limiting. The separate BuildTower subset/slot proposal ceiling remains untested; isolate it with an option-only ablation before deciding whether to implement full V2-C.

### Position-only optimization diagnostic

- Before the change, position-only PPO still built autodiff graphs through the frozen scorer, state encoder, family and candidate heads, then discarded their gradients because only the cell head was passed to its optimizer. It also recomputed the frozen scorer context for spatial decisions in every minibatch and PPO epoch. The cell logits were already limited to one selected option per spatial decision, shaped `[spatial decisions in minibatch, max legal cells in that minibatch]` (width at most 1,296); the code did not materialize `option × 1,296` for every transition.
- The update now computes frozen actor metrics and scorer contexts once per rollout batch on the inference backend, then uses those fixed values with the trainable cell head. This removes frozen-path autograd and repeated frozen encoding without changing the cell distribution or joint PPO ratio. The deterministic distribution-equivalence and position-only freeze/ratio tests passed.
- On the same resumed release run, mean update time fell from 63.58 seconds at iterations 41–50 to 27.74 seconds at iterations 51–75; mean rollout time was 8.02 and 8.27 seconds respectively. The 51–75 update time is close to the prior A1′ update baseline of about 28.28 seconds. The 50 checkpoint, including actor, critic and optimizer state, was resumed; no completed iteration was replayed.
