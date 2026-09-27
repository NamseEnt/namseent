# Policy v2: Factorized Policy and Full Action Space

Status: in progress. Sections marked **Frozen** were written before the corresponding v2 models were trained and are not changed after results are seen.

Phase 4B ([`14-phase4b-ppo.md`](14-phase4b-ppo.md)) showed that PPO from a canonical BC initialization beats the canonical baseline (+4.98 terminal clear_rate on the frozen final seeds), but tower placement never changed. The v1 network only scores the heuristic's top-8 placements and top-8 dense builds, although `PolicyActionSpace` already represents the full `subset x hand_slot x position` space. Policy v2 changes the policy representation, not the `AgentAction` contract or the game rules.

## Frozen plan

| stage | change | purpose |
|---|---|---|
| 0 | profile candidate generation, remove obvious duplicate work (30-minute cap) | cheaper rollouts; results must not change |
| A0 | family-factorized probability `P(family) x P(candidate given family)` with v1 features and the v1 candidate set | validate the factorization; A0a: flat-equivalent log-sum-exp family logits must reproduce v1 exactly; A0b: learned family head removes action-multiplicity bias |
| A1 | feature normalization contract for actor and critic, entropy scheme per head | fixed before B and C |
| B | PlaceTower as a dense `slot x 36 x 36` head with the full legal mask; heuristic score is an input channel, not a filter | removes the top-8 placement ceiling |
| C | BuildTower as `subset -> slot -> position` with conditional masks | full dense build space |
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
