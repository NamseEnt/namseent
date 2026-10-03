# Policy v2: Factorized Policy and Full Action Space

Status: V2 selected policy frozen; Phase R/C runs and artifact review complete. Sections marked **Frozen** were written before the corresponding models were trained and are not changed after results were seen.

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

## Frozen V2-C BuildTower option-only PPO preregistration

Name: `V2-C-option-only` (artifacts: `v2c-option-only-p1`, `v2c-option-only-p2`). This isolates BuildTower `(card subset, tower slot)` proposal learning before any full V2-C run.

### Question and policy decomposition

Question: with position selection and every non-option strategy held at A1′, does learning over all legal BuildTower subset/slot options improve paired `ppo_development` terminal clear_rate over frozen A1′?

- The frozen actor is `v2a1p-ppo-p2/iter-0170`. Its shared/state and candidate encoders, family/kind head, PlaceTower and non-BuildTower candidate probabilities, and all other actor parameters remain unchanged. Only a dedicated BuildTower option scoring head is trainable; the critic may update.
- Card-decision BuildTower options are the unique `(subset_index, hand_slot_index)` pairs with at least one legal position in `PolicyActionSpace`'s dense BuildTower region. `PolicyActionSpace::legal_mask` and `index_to_action` are authoritative for legality and indexing. The selected action uses that pair's highest-ranked legal position from `DenseBuildTowerScoreTable`; no position is sampled or learned. PlaceTower and all other action families retain their A1′ behavior.
- `outside-v1-top8 option` means the pair is absent from the unique `(subset, slot)` projection of the state's eight `DenseBuildTowerScoreTable::top_k_actions`. Store the pair's dense heuristic option rank and the selected position rank for audit.
- The PPO probability is `P_A1′(family) × P_option(subset, slot | BuildTower)` for BuildTower and the original A1′ joint probability for every other action. Thus the BuildTower family probability and every non-BuildTower probability stay fixed. Greedy selection first follows A1′'s greedy family/action; when it chooses BuildTower, only the conditional option is selected by the trainable head. This preserves non-BuildTower greedy decisions.

### Initialization and fixed training conditions

- The first raw-BC audit reported 858/26,072 mismatches, and a subsequent selected-A1′-head audit reported 506/26,072. Both audits incorrectly compared the full BuildTower action, including position, although this ablation deliberately replaces A1′'s position with the dense heuristic-best legal position. The corrected audit compares the BuildTower `(subset, slot)` only. Checkpoint 0 pins deterministic greedy BuildTower option selection to the option containing A1′'s original greedy BuildTower action; its sampled distribution uses a dedicated typed candidate head copied from `v2a1p-ppo-p2/iter-0170`. All non-Build greedy actions map directly from A1′, and every BuildTower position is the dense heuristic-best legal position. After the first optimizer update, greedy BuildTower selection follows the learned option head. This preserves the required checkpoint-zero option behavior while allowing sampled exploration. No new BC fitting, teacher, label smoothing or other tuning is used. The corrected 128-seed audit passed with 0/26,072 option/action-family mismatches; sampled smoke chose outside-top-8 options 7/27 times with 0 illegal/mismatched executions. PPO may start only after both gates pass.
- PPO recipe is fixed: 48 episodes/iteration, γ=1, GAE λ=0.95, reward scale 0.1, option-head Adam 3e-4, critic Adam 3e-4, four epochs, minibatch 256, clip 0.2, max grad norm 0.5, target KL 0.02, no KL penalty, seed 0. Optimize only the conditional option entropy, coefficient 0.01 in phase 1 and 0.003 in phase 2. Do not tune after seeing results.
- Run phase 1 for 75 iterations on `ppo_train` offset 0, with a 50-iteration pilot gate. If valid, continue phase 2 for 200 iterations from phase-1 iteration 75 on offset 1000. Evaluate greedily every five iterations on the existing 128 `ppo_development` seeds. Never use final seeds.

### Required invariants and records

- Before PPO: frozen actor tensors remain unchanged; non-BuildTower greedy actions match A1′ on the development trajectories; BuildTower init greedy option matches A1′; each BuildTower action uses the dense heuristic-best legal position; sampled outside-top-8 option rate is nonzero; each sample is legal in `PolicyActionSpace` and equals the executed action; old/new joint log-probabilities agree; PPO ratio equals the option conditional ratio for BuildTower and 1 for frozen decisions; illegal, mismatch and non-finite counts are zero.
- Every episode records seed, terminal clear_rate, BuildTower decision count, outside-top-8 count/boolean, dense heuristic option rank, selected subset and slot/template. Each iteration records sampled and greedy outside-option rates, option entropy and KL-to-init, dev clear_rate, paired option-only minus A1′ delta, clip fraction, critic EV, cumulative semantic decisions, frozen invariant, position invariant and execution mismatch counts.
- At iteration 50, continue only if sampled outside-top-8 option exploration is at least 1% cumulatively, all invariants hold, and the option head changed. A transient dev decline is not a stop condition. Stop and invalidate on effectively absent outside exploration, any frozen actor or position change, illegal/mismatched execution, or numerical failure.

### Interpretation fixed before results

- **A — option ceiling is a bottleneck:** greedy outside-top-8 `(subset, slot)` options appear and paired development clear_rate improves over A1′ as option choices change.
- **B — option ceiling is unlikely to be a major bottleneck here:** sampled exploration and option-head learning are sufficient, greedy choices return to the existing candidate set, and paired development does not improve over A1′.
- **C — inconclusive:** option exploration/credit assignment fails or any unrelated policy component or technical invariant contaminates the run.
- A/B/C apply only to this isolated proposal experiment. Decide whether full V2-C is justified from its result; the position-only result alone neither proves nor rules out a subset/slot ceiling.

### Option-only results

Run completed as preregistered: phase 1 reached iteration 75, including its 50-iteration pilot; phase 2 reached iteration 200. It used the existing 128 `ppo_development` seeds at each 5-iteration checkpoint and no final seeds.

- **Initialization and isolation passed.** The frozen actor bytes at the A1′ reference checkpoint `v2a1p-ppo-p2/iter-0170`, option-only phase 1 iteration 75, and phase 2 iteration 200 are identical (SHA-256 `a1e0fc22794cfaea3184b5ffb4f413d52d4a02657f49bc14b6674e2a1b338c29`). The dedicated option head changed on all 275 actor updates. The frozen-actor invariant passed on all 275 updates; BuildTower positions continued to use the dense heuristic-best legal position. Across training there were 0 illegal actions, 0 sampled/executed mismatches, and 0 non-finite values. The corrected preflight remained 0/26,072 non-BuildTower/action-family mismatches, with 7/27 sampled outside-top-8 options.
- **Exploration was substantial.** Across 275 updates and 13,200 rollout episodes, the policy sampled outside-v1-top-8 options 87,930 times out of 321,791 BuildTower decisions (27.33%). The option head changed every update. All 41 greedy development checkpoints selected at least some outside-top-8 options: rates ranged from 5.42% to 13.37%, averaging 9.54%. At iteration 200 it selected 353 outside options among 3,087 BuildTower decisions (11.44%). The most frequent outside choice was subset index 15, slot 0 (276 selections); its selected subsets included singleton card IDs 49, 48, 51, 45, and 39, typically at heuristic option rank 15. Thus the learned greedy policy did not return to the old top-8 proposal set.
- **Paired development result was directionally positive but uncertain.** At iteration 200, mean terminal clear_rate was 47.47 for option-only PPO and 47.18 for frozen A1′: paired delta **+0.296 percentage points (SE 0.184; 60 better / 53 worse / 15 ties)**. Its approximate 95% interval is [-0.066, +0.657], so this run does not establish a reliable improvement. Across the 41 repeated checkpoints, 25 deltas were positive and 16 negative; these reuse the same seeds and are correlated, so they are not independent replications.
- **Episode-level outside-use comparison is not diagnostic.** 13,180/13,200 training episodes used at least one outside option. Their mean terminal clear_rate was 48.02, versus 47.30 in the 20 episodes with no outside use. The tiny and highly imbalanced comparison group, along with observational action selection, prevents a useful causal or stable association claim.

**Conclusion: C — performance effect inconclusive, with the ablation technically valid.** Exploration, greedy adoption of new options, and freeze/execution invariants all worked; the unresolved part is whether those new choices improve performance beyond A1′. This C classification reflects the paired development uncertainty, not a technical failure. The observed point estimate alone is insufficient to say the option ceiling was a bottleneck.

The earlier experiment record notes that position-only did not establish a paired development improvement and option-only's paired development estimate was +0.296 (SE 0.184).

## V2 final policy selection and evaluation preregistration

### Selected policy (frozen before final evaluation)

**Selected V2 policy: A1′** — `simulator/artifacts/phase4b/v2a1p-ppo-p2/iter-0170` (phase-2 iteration 170, best recorded A1′ `ppo_development` checkpoint). The selected `actor.bin` SHA-256 is `a1e0fc22794cfaea3184b5ffb4f413d52d4a02657f49bc14b6674e2a1b338c29`; its run provenance is recorded in `v2a1p-ppo-p2/ppo.json` (`git_commit d50659cee04238ef857a570ba468083072a2cab9`). This checkpoint is frozen now and will not be replaced after viewing final results.

Selection rationale:

- A0/A0b showed no clear improvement from family factorization alone; A1′ is the strongest policy so far on the existing development seeds.
- A1′ uses normalized actor inputs with the established joint-entropy PPO recipe. The separate A1 per-head entropy attempt drifted excessively and was not selected.
- Position-only and option-only both explored and changed greedy choices, but neither established a reliable paired improvement over A1′. Full V2-C is not proceeding.
- The working hypothesis for this final check is that the main observed gain came from the input/optimization contract, rather than wider action-space proposals. The final result may support or weaken that hypothesis but will not trigger another checkpoint choice.

### One-time V2 final comparison

Before launching, the existing frozen split definition and records were checked: `V2Final` is `4,200,000..=4,200,255`; no V2 final-evaluation artifact or execution log exists, while `final-eval.json` records only the already-consumed `phase4b_final` range `4,100,000..=4,100,255`. The V2 final output path `simulator/artifacts/phase4b/v2-final-eval.json` is absent. The V2 final range is therefore reserved for this single run.

Run one greedy terminal evaluation on all 256 `v2_final` seeds with the existing terminal evaluator semantics (same game config, every policy on every seed, actual terminal state). Compare exactly these four policies:

| name | frozen policy source | artifact SHA-256 |
|---|---|---|
| `canonical` | built-in canonical scripted policy | n/a |
| `bc` | `simulator/artifacts/phase4b/phase4b-init` selected model | `selected-model.bin`: `46c6f9f473b31062526e840ba193f1012b2f36d2ae21d7697989a004a1f57933` |
| `v1_ppo` | Phase 4B run C iteration 200, `simulator/artifacts/phase4b/ppo-long-c/iter-0200` | `actor.bin`: `6710e5106da25cfd00a52422396620c7b152149558c4bd7d2be3d32cf28f37d1` |
| `a1_prime` | the selected A1′ checkpoint above | `actor.bin`: `a1e0fc22794cfaea3184b5ffb4f413d52d4a02657f49bc14b6674e2a1b338c29` |

Primary comparison: `a1_prime - v1_ppo`. Secondary comparisons: `a1_prime - canonical` and `a1_prime - bc`. Position-only and option-only are excluded. Record each policy's mean/SE/median terminal clear_rate, mean stage, full clears, mean decisions/game, terminal completion and safety-cap events, illegal/fallback/post-sampling mutation counts, and action-kind counts. Record paired mean delta, SE, 95% confidence interval, and better/worse/tie counts. The existing evaluator's 512-decision safety cap is a hard failure rather than truncation; report any such failure explicitly and never rerun this seed range.

This selection and seed use are frozen before the final output is generated. No final-seed result will be used to choose another policy, resume training, or tune hyperparameters.

### V2 final results

The one-time evaluation completed on all 256 `v2_final` seeds (4,200,000–4,200,255), using the same greedy terminal runner for all four policies. Every game reached the actual terminal state; the hard 512-decision safety cap was not hit (maximum observed: 357 decisions). There were 0 full clears, illegal actions, fallbacks, or post-sampling mutations for every policy. Per-seed outcomes, final state hashes, and the complete action-kind counts are in `simulator/artifacts/phase4b/v2-final-eval.json`.

| policy | mean clear_rate (SE) | median | mean stage | full clears | mean decisions/game | completed / cap events | illegal / fallback / mutation |
|---|---:|---:|---:|---:|---:|---:|---:|
| canonical | 36.63 (0.29) | 35.23 | 18.72 | 0/256 | 85.0 | 256 / 0 | 0 / 0 / 0 |
| Phase 4B BC | 36.63 (0.29) | 35.24 | 18.70 | 0/256 | 85.0 | 256 / 0 | 0 / 0 / 0 |
| Phase 4B selected PPO (v1) | 41.83 (0.44) | 40.11 | 21.30 | 0/256 | 124.8 | 256 / 0 | 0 / 0 / 0 |
| **A1′ (selected V2)** | **48.06 (0.50)** | **46.84** | **24.37** | **0/256** | **212.4** | **256 / 0** | **0 / 0 / 0** |

Paired deltas use the same 256 seeds. The 95% confidence intervals use the paired standard error and a two-sided Student-t critical value with 255 degrees of freedom.

| comparison | mean delta | SE | 95% CI | median delta | better / worse / tie |
|---|---:|---:|---:|---:|---:|
| **A1′ − Phase 4B PPO (primary)** | **+6.22** | **0.55** | **[+5.15, +7.30]** | **+6.17** | **207 / 49 / 0** |
| A1′ − canonical | +11.43 | 0.52 | [+10.41, +12.44] | +10.45 | 241 / 15 / 0 |
| A1′ − BC | +11.43 | 0.52 | [+10.41, +12.45] | +10.45 | 240 / 16 / 0 |

The A1′ development gain of +10.85 versus canonical was reproduced: the fresh final paired gain is +11.43, 0.58 points higher. The best recorded A1′ development checkpoint was +10.94; the fresh final gain is 0.49 points higher. A1′ beat the v1 PPO in all four consecutive 64-seed blocks, with mean paired deltas +5.17, +7.11, +5.95, and +6.67, so the improvement is not concentrated in one seed block.

The expanded decision behavior also carried over. A1′ averaged 212.4 decisions/game, compared with 124.8 for v1 PPO and 85.0 for canonical. Its development checkpoint at iteration 170 averaged 203.7 decisions/game. Its major action counts remained similar between selected development and fresh final evaluation: rerolls 50.5 to 51.6 per game, continues 75.5 to 80.8, inventory-item uses 11.3 to 11.9, and builds 24.0 to 24.4. This longer decision path coincided with the clear-rate gain on the fresh seeds.

Mean action-kind decisions per game (complete counts and less frequent kinds remain in the JSON artifact):

| action kind | canonical | BC | v1 PPO | A1′ |
|---|---:|---:|---:|---:|
| StartDefense | 18.72 | 18.70 | 21.30 | 24.37 |
| BuildTower | 18.72 | 18.70 | 21.30 | 24.37 |
| PlaceTower | 6.08 | 6.18 | 6.52 | 4.52 |
| Reroll | 5.59 | 5.62 | 26.41 | 51.63 |
| Continue | 11.82 | 11.78 | 15.52 | 80.83 |
| PurchaseShopItem | 9.49 | 9.47 | 13.19 | 11.47 |
| UseInventoryItem | 7.95 | 7.94 | 9.57 | 11.91 |
| SelectTreasure | 2.18 | 2.16 | 2.51 | 2.87 |
| DiscardTreasure | 0.00 | 0.00 | 0.20 | 0.10 |
| SelectCardServiceCard | 2.25 | 2.24 | 4.16 | 0.07 |
| ConfirmCardServiceSelection | 2.22 | 2.21 | 4.09 | 0.06 |
| RemoveTower | 0.00 | 0.00 | 0.00 | 0.25 |

### V2 research conclusion

- V2-A0/A0b: family factorization by itself produced no large, reliable gain over v1.
- A1 per-head entropy caused excessive policy drift and was not selected.
- A1′ combined the normalized actor input contract with the established joint-entropy PPO recipe and was the strongest development policy. Its +10.85 development gain was reproduced as +11.43 on the untouched V2 final seeds, and it beat Phase 4B PPO by +6.22 paired points.
- B/B′ enabled full-position exploration but did not show a reliable gain. The isolated position-only result also did not improve over A1′ beyond uncertainty.
- Option-only PPO did learn to choose outside-top-8 BuildTower options greedily, but its paired development gain remained uncertain.
- **Full V2-C will not be run. Selected V2 policy is A1′.** In the evidence collected here, action-space expansion did not explain the large improvement; the results are more consistent with the input and optimization contract being the main gain. This is evidence from one PPO training seed and one frozen final evaluation, not a causal separation of every A1′ change.

The next registered study is Phase R followed by Phase C below. Hidden-size comparisons stay on the CPU backend; CUDA benchmarking remains a separate future measurement.

## Phase R and Phase C preregistration

This section is frozen before any Phase R replicate or new-capacity BC/PPO run. The selected policy remains A1′, and `phase4b_final` and `v2_final` are never used again. No new final seeds are used in this research.

### Phase R — independent A1′ PPO training seeds

Question: does the selected A1′ hidden-64 result reproduce across PPO training randomness when its initialization and recipe are held fixed?

- R0 is the existing A1′ run: phase 1 `v2a1p-ppo-p1`, then phase 2 `v2a1p-ppo-p2`; both started from selected `v2a1-bc` actor SHA-256 `0f1f3dc52e1475c8b78220f5bd52e4f121581c63d1bca256f3f395835703e868` and `critic-512-contract/critic.bin` SHA-256 `84ea7b52a613dccea9f5a68f2e5401bdfaab83ff73d68ef8dd7f57a30bef60a8`. Their actor/critic initialization sources and all hyperparameters remain the reference.
- R1 and R2 each start from those same frozen selected BC actor and pretrained critic files, with fresh PPO optimizers. No run starts from the trained A1′ policy checkpoint. Only PPO random seed and fresh game-seed block vary: R1 seed 1, R2 seed 2.
- New, disjoint training seed ranges: R1 `4,300,000..=4,313,199`; R2 `4,400,000..=4,413,199`. Each range contains exactly 13,200 episode seeds for 275 x 48 rollouts. Phase 1 consumes the first 3,600 seeds (75 iterations); phase 2 consumes the remaining 9,600. The ranges do not overlap any registered dataset, training, development, or final split through `v2_final`, nor each other. An explicit absolute `--train-seed-start` input is used; the existing `ppo_train` range is not reused.
- Keep hidden size 64, A1′ normalized actor/critic input contract, learned family policy representation, top-8 candidates, gamma 1, GAE lambda 0.95, reward scale 0.1, actor/critic LR 3e-4, four epochs, minibatch 256, clip 0.2, joint entropy 0.01 in phase 1 and 0.003 in phase 2, KL-to-init coefficient 0 with measurement enabled, target KL 0.02, max grad norm 0.5, advantage normalization, 48 episodes/iteration, and no critic warmup. Use the existing CPU backend for all quality runs.
- Each replicate runs the full schedule: phase 1 75 iterations, then phase 2 200 iterations resumed from phase-1 iteration 75. It is evaluated greedily every five iterations on the same 128 `ppo_development` seeds. Report checkpoints at cumulative iterations 50, 100, 200, and 275 (phase 2 local iterations 25, 125, and 200 where applicable). Low interim performance is not a stop condition; stop only for a technical failure that invalidates training.
- Compare each replicate to canonical and the frozen R0 A1′ policy on identical development seeds. Report mean and SE paired clear-rate deltas, decisions/game, cumulative semantic decisions, KL-to-init, joint entropy, action-kind counts, critic EV, and learning curves. Compare the final 50-iteration mean (cumulative iterations 226–275) as the predeclared plateau summary; do not select a replicate or checkpoint from a single best score.
- R is technically normal if it completes the schedule with finite updates, legal/executed action agreement, and intact saved checkpoints. Performance variance, including a lower score, is an outcome and does not trigger early termination or recipe changes. If technically normal, proceed to Phase C as preregistered.

### Phase C — hidden-size capacity comparison

Question: at matched A1′ experience and PPO recipe, do hidden sizes 128 or 256 reach a higher development plateau than 64?

- Use the same two paired PPO training seed IDs and game-seed blocks as Phase R: hidden-64 R1/R2 are the Phase R runs; hidden-128 and hidden-256 each run seeds 1 and 2 on the corresponding R1/R2 blocks. Reuse across model sizes is intentional pairing; no seed block overlaps any pre-existing split. Keep all quality runs on the same CPU backend.
- Train one new BC model for each size 128 and 256 from scratch on the existing immutable `canonical-train-2048` dataset, validated on `canonical-validation`, without teachers. Use the A1′ BC recipe unchanged: chosen labels, learned family head, normalized inputs, top-8 candidates, Adam LR 0.001, batch 64, six epochs, seed 0, override weight 1. Select the validation-NLL checkpoint under the existing rule. Record validation NLL, top-1, action-kind accuracy, and the greedy paired development gate. Each size must pass the existing gate (canonical delta >= -2, illegal/fallback/post-sampling mutation = 0) before PPO.
- Pretrain a fresh critic per size using the same first 512 episodes of `canonical-train-2048`, the `canonical-validation` set, contract inputs, reward scale 0.1, seed 0, Adam LR 0.001, batch 256, four epochs, and the corresponding hidden size. No critic tuning.
- Use the Phase R PPO recipe, 75 + 200 iterations, checkpoints every five iterations, 128 unchanged development seeds, and identical per-replicate PPO/game seed IDs across sizes. All models start from their own size-matched BC/critic initialization. Do not change the backend during the quality comparison.
- Primary alignment is cumulative semantic decisions; also report episodes and wall time. At each size and replicate record development clear-rate/deltas to canonical and frozen A1′ hidden-64 R0, decisions/game, learning slope, KL-to-init, entropy, critic EV, update/rollout time, and batch-1 inference latency. Plateau is the predeclared mean over cumulative iterations 226–275; the full learning curve remains primary context.
- Run both R1/R2 for each larger size (no result-dependent screening). Do not use a new final split. CUDA measurement, if warranted after the CPU comparison, is an isolated benchmark only and cannot alter/retrain the capacity policies.
- Classify capacity as A only if 128 or 256 improves the plateau and matched-decision curve over hidden-64 across both paired training seeds by more than the Phase R seed variation; B if both larger sizes have similar plateau and no stable improvement despite added compute; C if replicate variation remains too large to separate size effects. In C, recommend whether more PPO replicates would resolve the uncertainty, but do not run them within this preregistration.

### Phase R and C results

All policies below use the same 128 `ppo_development` seeds. The capacity checkpoints at cumulative iterations 50, 100, 200, and 275 were evaluated with a single paired `terminal-eval` invocation per checkpoint across hidden sizes and training seeds. Cumulative semantic decisions are taken from training metadata. Decision-budget values in the table below are linearly interpolated between the stored every-five-iteration development evaluations; they are not additional checkpoint evaluations. The pre-registered 226–275 plateau is summarized by its ten available greedy development evaluations at cumulative 230, 235, …, 275.

#### Phase R: independent hidden-64 PPO seeds

Both new runs started from the same frozen actor/critic initialization recorded above, and completed the full 75 + 200 schedule on CPU. Across their training rollouts there were no illegal actions, sampled/executed mismatches, fallbacks, or non-finite values. Five R2 and one R1 training episodes hit the pre-existing safety cap; all checkpoints and optimizer state were saved, and development evaluations had zero illegal actions, fallbacks, post-sampling mutations, and truncations. This meets the registered technical-validity conditions.

| hidden 64 run | dev clear rate at reported checkpoint | paired delta vs canonical | paired delta vs frozen A1′ | mean clear rate over final plateau |
|---|---:|---:|---:|---:|
| R0, selected A1′ reference (phase-2 iter 170) | 47.18 | +10.94 | — | 44.80 |
| R1, PPO seed 1 | 42.58 | +6.35 | −4.60 (SE 0.77) | 41.50 |
| R2, PPO seed 2 | 48.17 | +11.93 | +0.99 (SE 1.02) | 44.40 |

Observed values: the two independent runs had final development deltas of +6.35 and +11.93 versus canonical and −4.60 and +0.99 versus the selected reference. Their final-plateau means span 2.90 points (sample SD 2.05); including R0, the three plateau means have sample SD 1.80 and range 41.50–44.80.

#### Phase C: hidden-size BC gates

The size-specific BC models used the same 173,945 training and 5,416 validation samples, with no teacher data. Both passed the registered greedy development gate. All four PPO capacity runs completed with zero illegal actions, sampled/executed mismatches, fallbacks, or non-finite values; one H256-R1 training episode hit the pre-existing safety cap. Every scheduled iteration checkpoint was saved.

| hidden size | selected BC epoch | validation NLL | validation top-1 | action-kind accuracy | dev delta vs canonical (SE) | illegal / fallback / mutation |
|---:|---:|---:|---:|---:|---:|---:|
| 128 | 4 | 0.01132 | 99.483% | 100% | −0.20 (0.12) | 0 / 0 / 0 |
| 256 | 6 | 0.01081 | 99.446% | 100% | −0.03 (0.07) | 0 / 0 / 0 |

Both fresh critics were trained from the registered 512-episode subset and contract inputs. Hidden 128 reached validation EV 0.880; hidden 256 reached 0.866. Their size-matched critic initializations were used in PPO.

#### Sample-budget learning curves

Each cell is the greedy development clear rate for independent PPO training seeds R1/R2 at the given cumulative semantic-decision budget. Values are interpolated from the stored five-iteration evaluation points. Only two training seeds were run per size, so these curves are descriptive rather than precise estimates of the mean policy quality.

| cumulative semantic decisions | hidden 64 R1 / R2 | hidden 128 R1 / R2 | hidden 256 R1 / R2 |
|---:|---:|---:|---:|
| 200,000 | 34.89 / 34.47 | 38.65 / 37.25 | 36.08 / 33.30 |
| 400,000 | 36.32 / 35.96 | 33.57 / 37.49 | 36.28 / 34.78 |
| 800,000 | 39.80 / 38.62 | 35.94 / 39.70 | 40.85 / 34.63 |
| 1,200,000 | 42.09 / 42.13 | 39.43 / 45.31 | 42.88 / 35.33 |

The trajectories cross substantially. At cumulative iteration 275 the models had consumed different numbers of semantic decisions: hidden 64 R1/R2 2.00M/2.19M, hidden 128 1.68M/1.31M, and hidden 256 2.26M/1.68M. The full learning curves are stored in each run's `ppo.json`; cumulative-checkpoint paired evaluation reports are `a1c-all-dev-cum050.json`, `a1c-all-dev-cum100.json`, `a1c-all-dev-cum200.json`, and `a1c-all-dev-cum275.json` under `simulator/artifacts/phase4b`.

| model | final-plateau dev mean R1 / R2 | cumulative-275 paired delta vs hidden-64 same seed (R1 / R2) | final checkpoint decisions/game R1 / R2 | batch-1 inference ms/decision R1 / R2 | PPO elapsed hours R1 / R2 |
|---|---:|---:|---:|---:|---:|
| hidden 64 | 41.50 / 44.40 | reference | 182.7 / 230.9 | 1.91 / 1.78 | 3.11 / 4.22 |
| hidden 128 | 43.15 / 43.63 | +1.17 (SE 0.70) / −4.42 (SE 1.05) | 175.9 / 142.2 | 2.90 / 3.22 | 5.52 / 4.62 |
| hidden 256 | 43.65 / 34.76 | +1.68 (SE 0.76) / −12.29 (SE 0.94) | 219.4 / 139.4 | 5.52 / 6.60 | 16.15 / 12.98 |

Plateau PPO diagnostics are averaged over the same ten checkpoints (values are R1 / R2):

| hidden size | KL to init | joint entropy | family entropy | critic EV |
|---:|---:|---:|---:|---:|
| 64 | 15.44 / 25.10 | 0.521 / 0.695 | 0.155 / 0.199 | 0.982 / 0.989 |
| 128 | 25.41 / 21.86 | 0.949 / 0.913 | 0.265 / 0.324 | 0.985 / 0.979 |
| 256 | 53.70 / 26.84 | 0.721 / 0.509 | 0.224 / 0.191 | 0.989 / 0.988 |

At cumulative iteration 275, the main greedy action-family counts per game were:

| action kind | A1′ reference | hidden 64 R1 / R2 | hidden 128 R1 / R2 | hidden 256 R1 / R2 |
|---|---:|---:|---:|---:|
| BuildTower | 24.0 | 21.7 / 24.4 | 22.3 / 22.2 | 22.5 / 18.3 |
| Reroll | 50.5 | 43.3 / 59.1 | 27.6 / 21.6 | 57.6 / 12.9 |
| Continue | 75.5 | 82.4 / 102.0 | 64.1 / 48.1 | 77.4 / 64.7 |
| UseInventoryItem | 11.3 | 4.9 / 4.8 | 7.3 / 9.1 | 6.4 / 3.5 |
| PurchaseShopItem | 10.9 | 5.8 / 9.0 | 12.3 / 8.9 | 12.2 / 8.4 |
| RemoveTower | 0.3 | 0.0 / 0.5 | 1.7 / 1.0 | 5.9 / 0.0 |
| PlaceTower | 4.3 | 0.0 / 0.0 | 5.4 / 6.2 | 0.0 / 3.6 |

Recorded paired plateau differences were +1.65 and −0.77 for hidden 128 versus hidden 64, and +1.68 and −12.29 for hidden 256 versus hidden 64 (R1/R2). Hidden-256 plateau means were 43.65 and 34.76, a between-run difference of 8.89 points. Recorded CPU elapsed hours were 16.15 / 12.98 for hidden 256 and 5.52 / 4.62 for hidden 128 (R1/R2); batch-1 inference latency was 5.52 / 6.60 ms versus 2.90 / 3.22 ms. Plateau critic EV means ranged from 0.979 to 0.989. No CUDA benchmark or new final-seed evaluation was run.

### Phase R/C supplemental recorded values

The tables below report recorded values without a new A/B/C classification or research recommendation. Cumulative iteration 275 means phase 1 iteration 75 plus phase 2 iteration 200. “Final-50” uses the ten persisted greedy development evaluations at cumulative iterations 230, 235, …, 275 (the evaluations available within the preregistered 226–275 interval). Development deltas are paired against the same 128 `ppo_development` seeds and canonical policy. Total wall times are the recorded run elapsed hours in the Phase R/C report. The PPO metadata also stores cumulative rollout-plus-update seconds, which exclude some run overhead; rollout/update times below are arithmetic means over the 275 recorded training iterations.

#### A1′ reference and independent hidden-64 runs

The reference row is the selected A1′ checkpoint, phase-2 iteration 170 (`v2a1p-ppo-p2/iter-0170`), which is cumulative iteration 245. For independent runs, “final” is cumulative iteration 275. The action-kind values are greedy development decisions per game at the stated checkpoint, rounded to one decimal; omitted kinds are zero or below 0.1.

| run | PPO seed / training seed block | final dev clear rate | delta vs canonical | delta vs A1′ reference | best dev checkpoint / clear rate | final-50 dev mean (min–max) | cumulative semantic decisions | dev decisions/game | final KL-to-init | final joint entropy | principal action kinds/game: Build / Reroll / Continue / Inventory / Purchase | wall time |
|---|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---|---:|
| A1′ reference R0 | 0 / existing `ppo_train` blocks | 47.18 | +10.94 | 0.00 | phase-2 170 / 47.18 | 44.80 (41.66–47.18) | 1,639,419 at selected checkpoint | 203.7 | 24.68 | 0.856 | 24.0 / 50.5 / 75.5 / 11.3 / 10.9 | 3.60 h full run |
| Independent R1 | 1 / 4,300,000–4,313,199 | 42.58 | +6.35 | −4.60 | cumulative 275 / 42.58 | 41.50 (36.02–45.33) | 2,001,446 | 182.7 | 16.03 | 0.447 | 21.7 / 43.3 / 82.4 / 4.9 / 5.8 | 3.11 h |
| Independent R2 | 2 / 4,400,000–4,413,199 | 48.17 | +11.93 | +0.99 | cumulative 275 / 48.17 | 44.40 (42.26–48.17) | 2,194,669 | 230.9 | 25.75 | 0.693 | 24.4 / 59.1 / 102.0 / 4.8 / 9.0 | 4.22 h |

Independent-run final clear rates span **42.58–48.17** (range 5.59 points). Their final-50 means span **41.50–44.40** (range 2.90 points). The reference’s final-50 mean 44.80 is **0.40 points above** the replicate maximum and **3.30 points above** the replicate minimum; in the ordered three values it is above both replicate means. The reference selected-checkpoint metrics above are taken from the same cumulative-checkpoint evaluation and PPO metadata. The reference run record reports 3.60 h full-schedule elapsed wall time; PPO cumulative rollout-plus-update time through the selected checkpoint is 3.08 h.

#### Hidden-size × PPO training-seed table

BC rows for hidden 64 use the A1′ BC; the size-specific 128/256 values come from their selected BC checkpoints. PPO final and best development deltas are versus canonical. Best checkpoint is the maximum stored greedy development mean across the schedule. Final-50 mean and range use the ten values defined above. Critic EV, KL, entropy, clip fraction, grad norm, and value loss are from the final training iteration; grad norm is the recorded mean actor grad norm. Decisions/game is the greedy development value at cumulative iteration 275. Action-kind values are counts per greedy development game.

| hidden | PPO seed / block | BC validation NLL | BC top-1 | BC gate delta | PPO final dev delta | PPO best dev delta / iteration | final-50 mean (min–max) | cumulative semantic decisions | rollout / update sec per iteration | total wall time | final KL / entropy | critic EV / clip fraction / grad norm / value loss | decisions/game | key action kinds/game: Build / Reroll / Continue / Inventory / Purchase |
|---:|---|---:|---:|---:|---:|---|---:|---:|---:|---:|---|---|---:|---|
| 64 | 1 / 4,300,000–4,313,199 | 0.0107 | 99.61% | −0.18 | +6.35 | +9.09 / 250 | 41.50 (36.02–45.33) | 2,001,446 | 3.99 / 34.70 | 3.11 h | 16.03 / 0.447 | 0.983 / 0.106 / 5.348 / 0.0172 | 182.7 | 21.7 / 43.3 / 82.4 / 4.9 / 5.8 |
| 64 | 2 / 4,400,000–4,413,199 | 0.0107 | 99.61% | −0.18 | +11.93 | +11.93 / 275 | 44.40 (42.26–48.17) | 2,194,669 | 4.40 / 49.03 | 4.22 h | 25.75 / 0.693 | 0.992 / 0.080 / 0.886 / 0.0079 | 230.9 | 24.4 / 59.1 / 102.0 / 4.8 / 9.0 |
| 128 | 1 / 4,300,000–4,313,199 | 0.01132 | 99.483% | −0.20 | +7.51 | +8.36 / 255 | 43.15 (41.46–44.60) | 1,676,539 | 5.21 / 64.94 | 5.52 h | 26.16 / 0.898 | 0.985 / 0.130 / 0.945 / 0.0081 | 175.9 | 22.3 / 27.6 / 64.1 / 7.3 / 12.3 |
| 128 | 2 / 4,400,000–4,413,199 | 0.01132 | 99.483% | −0.20 | +7.51 | +9.89 / 270 | 43.63 (40.66–46.13) | 1,312,162 | 4.40 / 54.00 | 4.62 h | 23.23 / 0.875 | 0.979 / 0.135 / 0.921 / 0.0144 | 142.2 | 22.2 / 21.6 / 48.1 / 9.1 / 8.9 |
| 256 | 1 / 4,300,000–4,313,199 | 0.01081 | 99.446% | −0.03 | +8.03 | +9.27 / 260 | 43.65 (40.85–45.50) | 2,256,308 | 14.56 / 192.63 | 16.15 h | 57.75 / 0.676 | 0.990 / 0.117 / 0.948 / 0.0088 | 219.4 | 22.5 / 57.6 / 77.4 / 6.4 / 12.2 |
| 256 | 2 / 4,400,000–4,413,199 | 0.01081 | 99.446% | −0.03 | −0.36 | +3.09 / 165 | 34.76 (33.86–35.90) | 1,680,291 | 11.42 / 155.10 | 12.98 h | 30.47 / 0.483 | 0.990 / 0.081 / 0.610 / 0.0043 | 139.4 | 18.3 / 12.9 / 64.7 / 3.5 / 8.4 |

The reported PPO run seed is 1 or 2. For each size, seed 1 uses game-seed block 4,300,000–4,313,199 and seed 2 uses 4,400,000–4,413,199; phase 2 begins at offsets +3,600 within those blocks. BC gate deltas are paired 128-seed development comparisons against canonical. Hidden-64 best dev iteration and plateau values are measured on the same saved R1/R2 histories as the larger models.

#### Paired hidden-size differences at the same training seed

These are differences in the final cumulative-275 paired development clear-rate delta versus canonical; equivalently they are the same-seed clear-rate differences between model sizes on the shared seed list.

| training seed | 128 − 64 | 256 − 64 |
|---|---:|---:|
| 1 (block starts 4,300,000) | +1.17 | +1.68 |
| 2 (block starts 4,400,000) | −4.42 | −12.29 |

#### Hidden-256 same-iteration trajectory: R1 and R2

R1 is the higher-scoring final hidden-256 run in the common cumulative-275 dev evaluation; R2 is the lower-scoring run. Each row is the same cumulative iteration for both seeds. Metrics are stored rollout clear-rate, post-update KL-to-init, joint entropy, clip fraction, mean actor grad norm, critic explained variance/value loss, training decisions/game, and selected training-rollout action-kind shares. Action shares are shown as `Reroll / Continue / BuildTower / PlaceTower / RemoveTower` percentages. Dev values are greedy 128-seed clear rates. Values are shown to two or three decimals, so rounded differences may not subtract exactly.

| cumulative iter | dev clear rate R1 / R2 | training rollout clear rate R1 / R2 | KL R1 / R2 | entropy R1 / R2 | clip fraction R1 / R2 | grad norm R1 / R2 | critic EV R1 / R2 | value loss R1 / R2 | decisions/game R1 / R2 | action-kind shares R1 / R2 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 25 | 36.74 / 36.67 | 36.81 / 37.34 | 0.6 / 1.0 | 0.102 / 0.191 | 0.016 / 0.025 | 0.21 / 0.39 | 0.984 / 0.981 | 0.0121 / 0.0100 | 89.5 / 90.6 | .07/.14/.21/.07/.00 / .07/.13/.21/.06/.00 |
| 50 | 36.32 / 32.94 | 40.08 / 37.67 | 12.1 / 7.7 | 0.447 / 0.383 | 0.076 / 0.041 | 0.44 / 0.29 | 0.987 / 0.984 | 0.0078 / 0.0082 | 118.9 / 99.1 | .10/.24/.17/.05/.00 / .08/.21/.19/.05/.00 |
| 75 | 36.81 / 36.11 | 39.69 / 39.04 | 31.3 / 11.3 | 0.824 / 0.551 | 0.115 / 0.062 | 0.70 / 0.37 | 0.976 / 0.983 | 0.0171 / 0.0090 | 141.6 / 109.4 | .17/.23/.14/.05/.05 / .11/.20/.18/.05/.00 |
| 100 | 39.18 / 33.82 | 41.87 / 38.82 | 41.5 / 16.1 | 0.857 / 0.570 | 0.090 / 0.074 | 0.60 / 0.43 | 0.982 / 0.984 | 0.0181 / 0.0080 | 158.2 / 113.9 | .26/.20/.13/.04/.05 / .13/.24/.17/.05/.00 |
| 125 | 40.09 / 33.94 | 42.81 / 42.11 | 45.8 / 19.4 | 0.710 / 0.482 | 0.105 / 0.079 | 0.72 / 0.42 | 0.983 / 0.986 | 0.0120 / 0.0075 | 180.7 / 147.4 | .28/.26/.12/.01/.04 / .11/.33/.15/.04/.00 |
| 150 | 39.95 / 36.11 | 44.37 / 41.75 | 41.9 / 20.1 | 0.726 / 0.545 | 0.109 / 0.074 | 0.65 / 0.51 | 0.988 / 0.988 | 0.0072 / 0.0062 | 187.1 / 148.5 | .25/.32/.12/.00/.03 / .11/.33/.14/.04/.00 |
| 175 | 42.21 / 35.45 | 42.80 / 40.69 | 52.1 / 20.3 | 0.862 / 0.589 | 0.144 / 0.072 | 0.77 / 0.50 | 0.990 / 0.983 | 0.0055 / 0.0079 | 218.0 / 131.8 | .24/.37/.10/.00/.04 / .10/.32/.16/.05/.00 |
| 200 | 43.04 / 37.04 | 40.10 / 41.16 | 55.1 / 24.5 | 0.932 / 0.490 | 0.140 / 0.083 | 1.04 / 0.52 | 0.984 / 0.989 | 0.0128 / 0.0056 | 186.3 / 145.5 | .26/.32/.11/.00/.05 / .10/.37/.14/.04/.00 |
| 225 | 40.85 / 33.96 | 46.65 / 38.80 | 48.2 / 25.9 | 0.613 / 0.523 | 0.122 / 0.086 | 0.80 / 0.58 | 0.992 / 0.989 | 0.0045 / 0.0048 | 236.4 / 133.1 | .23/.39/.10/.00/.03 / .11/.36/.15/.04/.00 |
| 230 | 42.79 / 33.91 | 44.74 / 38.98 | 49.4 / 26.2 | 0.697 / 0.503 | 0.126 / 0.075 | 0.87 / 0.53 | 0.990 / 0.989 | 0.0055 / 0.0043 | 217.9 / 152.4 | .23/.38/.10/.00/.04 / .10/.41/.13/.03/.00 |
| 235 | 40.85 / 35.08 | 41.16 / 39.84 | 50.4 / 23.9 | 0.690 / 0.547 | 0.114 / 0.084 | 0.81 / 0.59 | 0.990 / 0.988 | 0.0068 / 0.0052 | 206.8 / 138.3 | .24/.37/.10/.01/.04 / .10/.37/.15/.04/.00 |
| 240 | 44.18 / 34.41 | 44.24 / 39.77 | 51.3 / 24.1 | 0.721 / 0.532 | 0.127 / 0.092 | 0.94 / 0.59 | 0.991 / 0.989 | 0.0070 / 0.0046 | 219.0 / 139.4 | .27/.34/.10/.00/.03 / .10/.37/.15/.04/.00 |
| 245 | 43.53 / 34.23 | 45.80 / 39.59 | 53.5 / 23.9 | 0.771 / 0.590 | 0.128 / 0.097 | 1.11 / 0.63 | 0.990 / 0.987 | 0.0069 / 0.0053 | 219.6 / 134.2 | .27/.32/.11/.00/.04 / .09/.34/.15/.04/.00 |
| 250 | 43.81 / 34.56 | 44.32 / 40.92 | 54.6 / 27.5 | 0.805 / 0.478 | 0.132 / 0.090 | 1.04 / 0.65 | 0.987 / 0.989 | 0.0105 / 0.0056 | 203.2 / 155.2 | .27/.29/.11/.01/.05 / .09/.42/.13/.04/.00 |
| 255 | 44.59 / 33.86 | 42.49 / 39.26 | 56.5 / 26.9 | 0.735 / 0.511 | 0.131 / 0.077 | 1.01 / 0.55 | 0.985 / 0.988 | 0.0090 / 0.0042 | 201.7 / 143.3 | .27/.30/.11/.00/.04 / .10/.39/.14/.04/.00 |
| 260 | 45.50 / 34.55 | 45.62 / 40.30 | 53.0 / 27.9 | 0.639 / 0.510 | 0.116 / 0.084 | 0.92 / 0.72 | 0.988 / 0.985 | 0.0067 / 0.0073 | 215.3 / 155.7 | .26/.32/.11/.00/.04 / .09/.42/.13/.04/.00 |
| 265 | 43.54 / 35.90 | 44.47 / 40.64 | 56.2 / 25.9 | 0.796 / 0.469 | 0.143 / 0.075 | 1.08 / 0.60 | 0.988 / 0.989 | 0.0081 / 0.0049 | 206.1 / 148.2 | .26/.32/.11/.00/.05 / .10/.39/.14/.04/.00 |
| 270 | 43.48 / 35.22 | 46.15 / 41.10 | 54.2 / 31.7 | 0.654 / 0.447 | 0.113 / 0.091 | 0.93 / 0.67 | 0.990 / 0.987 | 0.0059 / 0.0051 | 219.5 / 161.4 | .26/.33/.11/.00/.02 / .09/.44/.13/.03/.00 |
| 275 | 44.26 / 35.88 | 46.42 / 39.43 | 57.8 / 30.5 | 0.676 / 0.483 | 0.117 / 0.081 | 0.95 / 0.61 | 0.990 / 0.990 | 0.0088 / 0.0043 | 237.2 / 150.4 | .26/.34/.10/.00/.04 / .10/.42/.13/.04/.00 |

**Observation:** by cumulative iteration 15, KL (0.070/0.583) and entropy (0.033/0.077) already differed while dev clear rates differed by 0.20 points (36.35/36.55); clip fractions were 0.014/0.015. At iteration 25, dev means were 36.74/36.67. At iteration 40, dev means were 36.40/32.42 and training rollout means were 37.19/36.07; KL was 3.56/7.59, entropy 0.213/0.363, clip fraction 0.036/0.059, and decisions/game 90.4/96.6. Critic EV at iteration 40 was 0.982/0.985 and value loss 0.0094/0.0077. Thus KL/entropy separation appears in the stored comparisons before the displayed larger dev-rate difference; by iteration 40, clear-rate, rollout, and several update/action metrics differ in the same checkpoint. These are timing observations, not a cause claim.

#### A1′ late-game distributions

Development uses the selected A1′ policy on the 128 `ppo_development` seeds from `a1r-dev-cum275.json`; fresh final uses the same frozen selected actor on 256 `v2_final` seeds from `v2-final-eval.json`. Quantiles use nearest-rank order statistics. The episode artifact records `final_stage` and `terminal_clear_rate`; it does not contain a separate explicit cause-of-termination field. “Stage distribution” below therefore counts the recorded final stage, not an inferred failure cause.

| split | episodes | mean stage | median stage | p75 / p90 / p95 / p99 / max stage | clear-rate p75 / p90 / p95 / p99 / max | full clears |
|---|---:|---:|---:|---|---|---:|
| development | 128 | 23.95 | 22.5 | 27 / 29 / 30 / 36 / 38 | 52.735 / 57.094 / 60.000 / 72.000 / 74.769 | 0/128 |
| fresh final | 256 | 24.37 | 24 | 27 / 30 / 32 / 37 / 37 | 53.821 / 59.201 / 62.672 / 72.154 / 73.231 | 0/256 |

The ten highest development final-stage episodes are: 4,000,040 (stage 38, clear-rate 74.7694), 4,000,034 (36, 72.0000), 4,000,099 (33, 65.1672), 4,000,098 (31, 61.6673), 4,000,074 (31, 60.6690), 4,000,010 (30, 60.0000), 4,000,029 (30, 60.0000), 4,000,078 (30, 60.0000), 4,000,055 (30, 59.7965), and 4,000,088 (30, 59.0241). The ten highest fresh-final final-stage episodes are: 4,200,123 (stage 37, clear-rate 73.2311), 4,200,081 (37, 72.1554), 4,200,024 (37, 72.1544), 4,200,091 (35, 69.0946), 4,200,019 (33, 65.6673), 4,200,041 (32, 64.0000), 4,200,241 (32, 63.6688), 4,200,015 (32, 63.1714), 4,200,154 (32, 63.1700), and 4,200,254 (32, 63.0035).

Recorded development final-stage counts are: stage 16: 1, 17: 3, 18: 2, 19: 6, 20: 11, 21: 5, 22: 36, 23: 7, 24: 4, 25: 7, 26: 8, 27: 18, 28: 4, 29: 5, 30: 6, 31: 2, 33: 1, 36: 1, 38: 1. Fresh final counts are: stage 15: 1, 16: 1, 17: 7, 18: 4, 19: 8, 20: 26, 21: 10, 22: 44, 23: 21, 24: 20, 25: 13, 26: 16, 27: 30, 28: 15, 29: 14, 30: 12, 31: 1, 32: 8, 33: 1, 35: 1, 37: 3. Both artifacts record **zero full clears**. They do not break out a final-stage-specific failure event beyond these final-stage counts.

#### Existing experiment summary

Values below are copied from the historical development/final artifacts and the experiment records above. “Best/final dev delta” is versus canonical unless the cell names another comparator. An em dash means no corresponding result was recorded in the artifacts inspected for this summary. “Action space” states whether the action candidates/representation were changed. Technical status describes completion/invariants as recorded, not performance.

| policy / experiment | best / final dev delta | fresh final result | action-space change | directly recorded observations | technical status |
|---|---|---|---|---|---|
| canonical | 0 / 0 reference | clear-rate 36.63, stage 18.72, full clears 0/256 | no | 84.5 decisions/game in Phase4B final; 85.0 in V2 final | completed baseline |
| Phase4B BC | −0.15 gate | V2 final clear-rate 36.63, stage 18.70, full clears 0/256 | no | 99.5% top-1 in Phase4B training report; final actions close to canonical | completed |
| Phase4B PPO (v1) | +6.11 final checkpoint; best +6.11 in run C | V2 final clear-rate 41.83, stage 21.30, full clears 0/256; paired A1′ delta +6.22 | no | Phase4B final clear-rate 41.63 on prior final split | completed; zero final-eval illegal/fallback/mutation |
| A0 / A0a | exact v1 factorized equivalence | — | factorized family × candidate representation; candidate set unchanged | final state hashes matched v1 on 128 dev seeds | equivalence checks recorded as passing |
| A0b | +5.34 final; best +5.87 | — | learned family head; candidate set unchanged | 1.36M decisions, 2.63 h; final development mean 41.58 | completed |
| A1 per-head entropy | best +0.06 at iter 5; −3.67 at iter 20 | — | no | stopped after phase-1 iteration 24; KL-to-init reached 3.90 by iter 20 | stopped post hoc; excluded from model comparison |
| A1′ | +10.85 final; best +10.94 at phase-2 iter 170 | V2 final clear-rate 48.06, stage 24.37, full clears 0/256; paired +11.43 vs canonical and +6.22 vs v1 PPO | no | selected actor; final decisions/game 212.4 | completed; final eval had zero illegal/fallback/mutation |
| position-only (B/B′ records) | B′ best +0.65; final −19.07; B′ final mean clear-rate 17.16 | — | full legal positions in the trained cell head; position-only variant froze non-position actor | B run stopped at iter 58 with cell entropy 0; B′ sampled outside-top-8 but greedy dev used none; B′ RemoveTower 1,684 and Reroll 3,348 across 128 games | B stopped post hoc; B′ completed, no illegal/mismatch reported |
| option-only | +0.296 vs frozen A1′ at iter 200 (SE 0.184) | — | full BuildTower subset/slot proposal options; position and non-BuildTower choices frozen | 27.33% sampled outside-top-8 options; 11.44% greedy at iter 200; frozen actor digest unchanged | completed; recorded invariants passed |
| hidden-64 independent R1/R2 | +6.35 / +11.93 final vs canonical; best +9.09 / +11.93 | — | unchanged A1′ action candidates | final dev 42.58 / 48.17; 2.00M / 2.19M semantic decisions | completed schedules; recorded execution/finite checks passed |
| hidden-128 R1/R2 | +7.51 / +7.51 final; best +8.36 / +9.89 | — | unchanged A1′ action candidates | final dev 43.75 / 43.74; 1.68M / 1.31M decisions | completed schedules; recorded execution/finite checks passed |
| hidden-256 R1/R2 | +8.03 / −0.36 final; best +9.27 / +3.09 | — | unchanged A1′ action candidates | final dev 44.26 / 35.88; 2.26M / 1.68M decisions | completed schedules; recorded execution/finite checks passed |

### Source artifacts and value definitions

Phase R run records: `simulator/artifacts/phase4b/a1r-r{1,2}-p{1,2}/ppo.json`; selected reference: `v2a1p-ppo-p2/ppo.json`; paired evaluations: `a1r-dev-cum275.json`. Capacity BC metadata: `a1c-bc-h{128,256}/bc.json`; PPO histories: `a1c-h{128,256}-r{1,2}-p{1,2}/ppo.json`; common paired evaluation: `a1c-all-dev-cum275.json`; BC gates: `a1c-bc-h{128,256}-dev.json`. A1′ development and fresh-final episode data: `a1r-dev-cum275.json` and `v2-final-eval.json`. These artifacts are locally present under the ignored simulator artifact directory and are not included in the documentation commit.

## Actor stability stress test (pre-registered)

Pre-registration date: 2026-10-03. No modified-recipe training had started when this section was recorded.

### Existing hidden-64 trajectory evidence

The existing R0/R1/R2 PPO histories were aligned by cumulative iteration (phase 1 iterations 1–75, followed by phase 2 iterations 76–275). The first clear actor-trajectory separation for R1 is at cumulative iteration 15: R1's post-update KL-to-initial-policy was 3.83, versus 0.46 for R0 and 1.53 for R2. At that point greedy development clear rate was 32.75 for R1, 37.68 for R0, and 36.46 for R2; training rollout clear rate was 39.93 / 38.89 / 38.25 (R1/R0/R2), so rollout return did not yet mirror the greedy development gap. R1's main action-kind counts per training game were Continue 30.7, Reroll 8.9, BuildTower 20.3, UseInventoryItem 8.4, and PurchaseShopItem 11.2, versus 14.0 / 7.3 / 19.8 / 8.6 / 10.9 for R0 and 16.4 / 7.8 / 19.5 / 8.1 / 9.7 for R2. At cumulative iteration 20, R1 KL-to-init was 5.59 versus 0.91 / 2.88, and Continue counts were 38.7 versus 13.5 / 16.8; this confirms a policy-distribution split before the later instability.

The late R1 actor updates also show unusually large update signals and repeated target-KL early stops. At cumulative iterations 200, 225, 245, 250, and 275, approximate KL was 0.0216, 0.0319, 0.0230, 0.1610, and 0.0460; the update stopped early at each checkpoint (1, 1, 1, 1, and 2 epochs completed). Corresponding mean actor grad norms were 1.34, 5.38, 4.29, 6.62, and 5.35. At iteration 275 R0/R1/R2 grad norms were 1.06 / 5.35 / 0.89; R0 and R2 completed all four epochs without early stopping. Across these points, critic explained variance remained approximately 0.980–0.992. This is evidence to directly constrain actor update magnitude; it does not establish causality.

### Locked intervention and run contract

- Change exactly one existing actor-stability variable: `actor_learning_rate` from 0.0003 to **0.00015** (one half of the A1′ value).
- Keep `clip_epsilon=0.2`, `target_kl=0.02`, critic learning rate 0.0003, four epochs, minibatch 256, max grad norm 0.5, and the existing A1′ optimizer/normalization settings.
- Keep hidden size 64, A1′ representation and actions, reward, critic, and all other architecture and training settings unchanged.
- Run existing PPO seeds R1 (seed 1; train seed start 4,300,000) and R2 (seed 2; train seed start 4,400,000), each for phase 1: 75 iterations at entropy coefficient 0.01, then phase 2: 200 iterations at entropy coefficient 0.003, resuming the new phase-1 actor, critic, and optimizer states. Total budget: 275 cumulative iterations per seed. Do not retrain R0 or evaluate a final seed.
- Use the existing 128-seed `ppo_development` evaluation schedule. At cumulative iteration 75 inspect technical validity only. Continue to 275 unless a pre-registered technical stop condition occurs: non-finite values, illegal/mismatch actions, optimizer failure, unchanged policy, or experiment-contract violation. Do not stop based on performance.
- Compare each modified run with its existing same-seed baseline. Report final and best dev delta versus canonical; last-50 dev mean/min/max; cumulative semantic decisions; per-update KL and target-KL stops; KL-to-init; entropy; clip fraction; actor grad norm; decisions/game; action-kind distribution; critic metrics; and final/late-window between-seed spread.

This is a development stability test, not final evaluation. No additional hyperparameter values or experiment branches are authorized by this pre-registration.

## Actor stability stress test results

Completed on 2026-10-03. The preregistered half-actor-LR recipe completed all 275 cumulative iterations for both existing training seeds. No baseline was retrained. Raw PPO histories remain in the ignored `simulator/artifacts/phase4b` directory and are not included in the documentation commit.

### 1. First actor trajectory separation

The original A1′ R1 trajectory first separated clearly at cumulative iteration 15, as recorded in the preregistration above: KL-to-initial was 3.834 for R1 versus 0.460 for R0 and 1.526 for R2; greedy dev clear rates were 32.751 / 37.680 / 36.458 (R1/R0/R2). The modified runs show that halving actor LR did not make the R1 actor trajectory stable: at iteration 15 its approximate update KL was already 0.1143, target-KL stopped after one epoch, mean actor grad norm was 16.796, and entropy was 0.0463. At the same point modified R2 had KL 0.0015, completed all four epochs, grad norm 0.570, and entropy 0.0436. The seeds still produced sharply different update dynamics under the same recipe.

### 2. Preregistered stability variable

The only changed variable was actor learning rate, 0.0003 → 0.00015. Clip epsilon remained 0.2, target KL 0.02, critic LR 0.0003, hidden64, epochs4, minibatch256, entropy coefficients 0.01 for cumulative iterations 1–75 and 0.003 for 76–275. The paired runs used existing seeds R1/1 and R2/2 and their existing training seed blocks. No architecture, representation, action, reward, critic, or final-seed evaluation change was made.

### 3–5. Same-seed comparisons and performance spread

Canonical development clear rate was 36.234232 in the paired evaluation histories. The best checkpoint is the highest recorded PPO dev clear-rate checkpoint; the final is cumulative iteration 275. The last-50 performance window uses the ten scheduled dev evaluations at cumulative iterations 230–275, inclusive. “Semantic decisions” is the sum of rollout transitions across all 275 iterations.

| Run | Final dev / delta vs canonical | Best dev / delta vs canonical (iter) | Last-50 dev mean / min / max | Semantic decisions | Modified − original final / best delta / late mean |
|---|---:|---:|---:|---:|---:|
| Original R1, LR .0003 | 42.581750 / +6.347518 | 47.743966 / +11.509734 (215) | 41.499217 / 36.016116 / 45.328614 | 2,001,446 | — |
| Modified R1, LR .00015 | 39.702608 / +3.468377 | 39.702608 / +3.468377 (275) | 39.240538 / 38.967661 / 39.702608 | 1,241,430 | −2.879141 / −8.041357 / −2.258679 |
| Original R2, LR .0003 | 48.168695 / +11.934463 | 48.168695 / +11.934463 (275) | 44.399764 / 42.262532 / 48.168695 | 2,194,669 | — |
| Modified R2, LR .00015 | 44.024749 / +7.790517 | 44.024749 / +7.790517 (275) | 38.902162 / 36.294021 / 44.024749 | 1,385,206 | −4.143946 / −4.143946 / −5.497602 |

The between-seed final spread fell from 5.586945 to 4.322141 (−1.264804; 22.6%). The spread between last-50 dev means fell from 2.900547 to 0.338376 (−2.562171; 88.3%). This compression came with lower performance in both seeds: the weak R1 run declined by 2.879 final dev points, while the strong R2 run declined by 4.144. The modified R1’s late mean/min/max is tightly clustered because it plateaued at a lower level; that is not recovery of the weak seed.

### 6. KL, entropy, clipping, gradient, loss, decisions, and critic trajectory

The table gives scheduled snapshots (cumulative iterations 15, 75, 150, 225, 275) from the stored histories. Approx-KL and target-KL stop/epoch count are per-update values. KL-init is measured after the actor update. Entropy, clip fraction, actor loss, grad norm, and critic EV are update/rollout fields from that iteration. Dev clear is the greedy PPO development evaluation when scheduled; rollout clear is the sampled training rollout.

| Run | Iter | Dev / rollout clear | Approx KL; stop (epochs) | KL-init | Entropy | Clip frac | Actor grad norm | Actor loss | Decisions/game | Critic EV |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Original R1 | 15 | 32.751 / 39.930 | .0046; no (4) | 3.834 | .1994 | .0426 | .493 | −.00193 | 115.00 | .9750 |
| Original R1 | 75 | 34.777 / 39.457 | .0064; no (4) | 12.401 | .5920 | .0756 | .687 | −.00053 | 136.44 | .9778 |
| Original R1 | 150 | 43.183 / 43.793 | .0060; no (4) | 15.326 | .7952 | .0739 | .795 | −.00569 | 167.19 | .9851 |
| Original R1 | 225 | 45.133 / 43.534 | .0319; yes (1) | 13.027 | .4959 | .1205 | 5.383 | .03384 | 133.38 | .9856 |
| Original R1 | 275 | 42.582 / 48.909 | .0460; yes (2) | 16.029 | .4468 | .1062 | 5.348 | .01000 | 195.23 | .9829 |
| Modified R1 | 15 | 36.567 / 37.814 | .1143; yes (1) | .073 | .0463 | .1031 | 16.796 | .04351 | 90.83 | .9816 |
| Modified R1 | 75 | 36.852 / 37.764 | .0222; yes (2) | .140 | .0602 | .1419 | 15.463 | .00107 | 91.60 | .9783 |
| Modified R1 | 150 | 38.348 / 39.042 | .0984; yes (1) | 1.235 | .0265 | .3183 | 27.459 | .03790 | 95.08 | .9722 |
| Modified R1 | 225 | 39.226 / 41.022 | .0937; yes (1) | 1.820 | .0159 | .4104 | 52.573 | .04225 | 104.29 | .9805 |
| Modified R1 | 275 | 39.703 / 40.569 | .0879; yes (1) | 1.073 | .0105 | .5021 | 72.867 | .04153 | 102.71 | .9747 |
| Original R2 | 15 | 36.458 / 38.250 | .0037; no (4) | 1.526 | .1610 | .0328 | .502 | −.00242 | 93.15 | .9750 |
| Original R2 | 75 | 35.024 / 39.965 | .0035; no (4) | 9.839 | .7101 | .0401 | .559 | −.00191 | 125.10 | .9762 |
| Original R2 | 150 | 40.205 / 45.562 | .0070; no (4) | 15.015 | .6945 | .0696 | .794 | −.00113 | 166.48 | .9827 |
| Original R2 | 225 | 44.231 / 47.664 | .0085; no (4) | 23.575 | .7786 | .0864 | 1.028 | −.00372 | 197.88 | .9880 |
| Original R2 | 275 | 48.169 / 52.314 | .0073; no (4) | 25.749 | .6934 | .0798 | .886 | −.00272 | 260.62 | .9920 |
| Modified R2 | 15 | 36.600 / 37.486 | .0015; no (4) | .085 | .0436 | .0105 | .570 | −.00228 | 83.40 | .9740 |
| Modified R2 | 75 | 37.732 / 38.305 | .0036; no (4) | 1.408 | .2302 | .0391 | .560 | −.00359 | 90.60 | .9721 |
| Modified R2 | 150 | 38.322 / 39.848 | .0065; no (4) | 3.158 | .2637 | .0332 | .593 | −.00313 | 97.67 | .9784 |
| Modified R2 | 225 | 38.411 / 41.725 | .0036; no (4) | 7.481 | .3529 | .0334 | .937 | −.00309 | 115.88 | .9822 |
| Modified R2 | 275 | 44.025 / 44.780 | .0036; no (4) | 9.257 | .3573 | .0431 | .990 | −.00208 | 133.00 | .9802 |

Across all 275 updates, original R1 had 79 target-KL stops and modified R1 had 257; R2 had 1 original stop and 0 modified stops. There were zero non-finite optimizer skips in all four runs. Over cumulative iterations 226–275, mean approximate KL / entropy / clip fraction / actor grad norm were: original R1 .052864 / .526989 / .128580 / 5.053477; modified R1 .105627 / .016124 / .456028 / 58.655260; original R2 .007635 / .696900 / .081331 / .936761; modified R2 .003604 / .351569 / .043024 / 1.005588. The intervention smoothed R2 updates, but it amplified R1’s repeated target-KL stops, entropy collapse, clipping, and actor gradients.

Final critic value loss / explained variance were original R1 .017163 / .982908 and modified R1 .033648 / .974721; original R2 .007912 / .992025 and modified R2 .016576 / .980174. These stayed finite, but critic fit was weaker in both modified runs. This does not change that the preregistered intervention targeted the actor.

### 7. Final greedy action-kind distribution

Counts below are PPO greedy development action-kind counts divided by 128 evaluation episodes (actions/game). They show the changed policy mix, especially the collapse in `continue` and `reroll` for modified R1 and the reduced `continue` rate for modified R2.

| Run | Build tower | Continue | Reroll | Inventory item | Shop purchase | Place tower | Remove tower |
|---|---:|---:|---:|---:|---:|---:|---:|
| Original R1 | 21.695 | 82.414 | 43.312 | 4.922 | 5.773 | 0.000 | 0.000 |
| Modified R1 | 20.234 | 14.062 | 16.180 | 9.375 | 10.305 | 6.320 | 0.047 |
| Original R2 | 24.438 | 102.000 | 59.086 | 4.805 | 8.953 | 0.031 | 0.469 |
| Modified R2 | 22.336 | 59.344 | 15.578 | 10.625 | 11.195 | 6.734 | 0.047 |

### 8. Technical invariants

Both modified seeds completed 75 phase-1 and 200 phase-2 iterations (275 actor updates and 13,200 training episodes per seed), using the release simulator and the phase-1 actor, critic, and optimizer states for phase 2. At every recorded rollout, illegal actions, action mismatches, and non-finite rollout values were zero; optimizer non-finite skips were zero; every iteration recorded an actor update. Both policies moved from initialization (final KL-to-init 1.0735 for R1 and 9.2574 for R2). The requested 75-iteration technical check passed for each run. High but finite R1 actor gradient norms and frequent target-KL stops were logged as outcome metrics, not technical stop conditions.

### 9. Conclusion

**B. Trade-off.** The final and last-50 between-seed spreads narrowed, but neither seed improved: R1 lost 2.879 final dev points versus its baseline, and R2 lost 4.144. The strong seed’s best/final score was meaningfully lower, while the weak seed did not improve. Halving actor LR therefore does not meet the preregistered stability goal. No new research direction is proposed here.

### Raw artifact references

- Original: `simulator/artifacts/phase4b/a1r-r{1,2}-p{1,2}/ppo.json`
- Modified: `simulator/artifacts/phase4b/a1s-lrhalf-r{1,2}-p{1,2}/ppo.json`
- Modified checkpoint directories: `simulator/artifacts/phase4b/a1s-lrhalf-r{1,2}-p{1,2}/iter-*`
