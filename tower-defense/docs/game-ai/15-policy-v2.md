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

BC: `phase4b-init`'s scorer plus a new family head (`--kind-mode learned`), the same 2,048-game canonical dataset, Adam 1e-3, batch 64, budget 6 epochs, lowest validation NLL selected. Results below.

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
- PPO stores and clips the joint log-probability. The position head's normalized entropy `H / ln(cells)` uses `c_candidate`. The KL-to-init monitor and penalty include the position head.

### Training

- BC:
  - warm-started from the A1 BC (scorer and family head; the cell head starts untrained), normalized inputs;
  - samples are rebuilt by replaying the canonical dataset games from their seeds (cell legality needs the environment); every replayed state must match its recorded hash;
  - the target cell of a canonical placement is the option's heuristic-best cell;
  - same optimizer and 6-epoch budget; same BC gate.
- Critic: the A1 contract critic.
- PPO: the A1 schedule, entropy scheme and coefficients, same seed blocks.

### Recorded, per training iteration and per development evaluation

- `BuildTower` and `PlaceTower` separately:
  - share of executed positions outside the state's v1 top-8 actions;
  - share differing from the canonical action;
  - mean heuristic rank percentile of the chosen cell within its option;
  - mean Manhattan distance from the option's heuristic-best cell;
  - cell-head entropy.
- Mean terminal clear_rate of development episodes with at least one position outside the v1 top 8, and of episodes with none.
- Development performance vs v1 PPO and vs stage A at matched cumulative decisions.

Interpretation, fixed in advance:

- B beats A and a substantial share of placements leave the v1 top 8: evidence that the position ceiling limited PPO.
- B chooses almost only v1 top-8 positions and performs like A: the position-ceiling hypothesis is weakened, and stage C (the (subset, slot) proposal) becomes the main suspect.
