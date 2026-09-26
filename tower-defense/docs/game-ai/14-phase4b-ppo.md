# Phase 4B: PPO on the Semantic Action Stack

Status: in progress. Sections marked **Frozen** were written before any Phase 4B data was generated or any model was trained, and are not changed after results are seen.

Phase 4B asks one question: starting from a canonical BC actor, can PPO raise terminal clear_rate above the canonical scripted baseline? The rollout teacher is not used in this phase (no teacher corpus, no distillation, no DAgger).

## Gameplay version boundary

Phase 4B runs on the latest `feat/game-ai-rewrite` gameplay (merge of master #1369-#1371 on top of the Phase 3 gate commit). Relevant core rule changes:

- Tower selection was rewritten (`core/src/game_state/tower_selection.rs`, `get_highest_tower_template`): poker-hand evaluation, suit/rank of the resulting template, and upgrade-dependent straight/flush rules (FourLeafClover, Rabbit, BlackWhite) changed.
- Brush, FountainPen and Tricycle card services now offer 3 seeded random candidate cards (`CardServiceSelectionState::candidate_card_ids`) instead of every matching card.
- BrokenPottery grants a die every 4 rerolls counted per upgrade instead of every 4th global reroll.

The game config digest did not change, so the dataset provenance gained `game_rules_epoch` (`GAME_RULES_EPOCH = 2`; Phase 4A = 1) and `core_tree_hash`. The loader rejects Phase 4A datasets and checkpoints. Phase 3 teacher `Verified` results ([`05-rollout-teacher.md`](05-rollout-teacher.md)) and Phase 4A results ([`13-phase4a-bc-distillation.md`](13-phase4a-bc-distillation.md)) remain valid for the commits and rules they were measured on; they were not re-run on the new rules.

## Integration fixes

- `observation.rs` called the removed `select_tower_build_template`. The `BuildTower` preview now calls `tower_selection::tower_template_for_selection`, the same helper the authoritative `SelectTower` command uses. `observation::tests::build_tower_candidates_match_authoritative_select_tower_for_every_subset` compares every non-empty card subset (7 upgrade sets x 24 random hands of 5-7 cards, shopping and selecting flows, polish and engravings) against executing the same subset through the core command path: kind, suit, rank, damage, effective damage, range, interval, rerolled count, used cards and splashes must be identical.
- The simulator re-implemented the card-service card filter and did not know the 3-candidate rule. It now uses `CardSelectionFilterState::matches` plus `candidate_card_ids` from core. `fork_for_rollout_seed` keeps the RNG seed while a candidate-restricted selection is pending so revealed candidates cannot change.
- Phase 4A was merged (not cherry-picked) to keep its history. Its tracked `artifacts/phase4/` reports were dropped from the index because `artifacts/` is now ignored; they remain on `feat/game-ai-phase4a`.

## Card-service action contract

Phase 4A's learned policy needed a runtime "no-undo" guard: it could toggle an already selected card forever. A guard that rewrites the sampled action is not allowed under PPO (stored action and log-probability would differ from the executed action). The environment action set is now monotone (`ACTION_SCHEMA_VERSION = 8`, `POLICY_CANDIDATE_SET_VERSION = 2`):

- while the current step needs cards, every selectable card that is not yet selected is offered;
- once the step is full, only `ConfirmCardServiceSelection` is offered;
- deselecting (undo) and selecting into a full step (a no-op) are not actions and are rejected if applied.

Canonical play never deselected, so canonical trajectories are unchanged. The learned-policy runner no longer has a guard; evaluation reports `post_sampling_mutations` (sampled candidate != executed action), which is 0 by construction.

## Frozen seed split

Disjoint from every Phase 3 and Phase 4A range (`Phase4Split`, `simulator/src/ml/phase4_dataset.rs`):

| split | seeds | use |
|---|---|---|
| `phase4b_canonical_train` | 3,000,000 - 3,009,999 | fresh canonical BC and critic pretraining data |
| `phase4b_canonical_validation` | 3,100,000 - 3,100,063 | BC epoch selection, critic validation |
| `ppo_train` | 3,200,000 - 3,999,999 | PPO rollouts; iteration `i` uses the next `episodes_per_iteration` seeds, never reused |
| `ppo_development` | 4,000,000 - 4,000,127 | every model, checkpoint and hyperparameter choice |
| `phase4b_final` | 4,100,000 - 4,100,255 | run once, after the checkpoint/recipe is chosen |

## Frozen gates

**BC acceptance** (fresh canonical BC on `ppo_development`, greedy):

- illegal = 0, fallback = 0, post-sampling mutations = 0;
- mean paired terminal clear_rate delta (BC - canonical) >= -2.0.

Initial budget 6 epochs; the epoch with the lowest validation NLL is selected. Passing the gate freezes the checkpoint as `phase4b-init`; no further BC training.

**PPO smoke gates**, in order, before any long run:

1. Rollout correctness on a small rollout: illegal = 0, fallback = 0, sampled != executed = 0, no NaN/Inf, telescoping reward error < 1e-3, terminal/truncated handled.
2. One update: finite policy loss, value loss, entropy, KL; checkpoint save/load; optimizer resume reproduces an uninterrupted run bit-for-bit.
3. Iteration 0: the PPO actor before any update has identical logits to `phase4b-init`, and its greedy development evaluation reproduces the BC evaluation exactly.

**Success signal**: terminal clear_rate on `ppo_development`, greedy policy, compared with canonical and `phase4b-init` on the same seeds (paired). The first target is evidence of PPO > BC ~ canonical. Training-surrogate metrics alone never count as success.

## Frozen PPO formulation

- Actor: the unchanged `DeepSetsActorCritic` actor path, hidden 64, initialized from `phase4b-init`. Decisions come from `semantic_bc::semantic_decision`, the same function the BC evaluator uses (candidate set, legal mask, encoding, action index).
- Critic: a separate `DeepSetsActorCritic` instance (own typed encoder, global + typed value heads), so value learning never moves the actor encoder. Pretrained on canonical trajectories with target `reward_scale * (terminal - current clear_rate)`.
- Reward: `r_t = reward_scale * (clear_rate(s_{t+1}) - clear_rate(s_t))` between consecutive decisions, `reward_scale = 0.1` (a unit change, not shaping). No other reward terms.
- `gamma = 1.0` (the return telescopes to terminal minus current clear_rate), GAE `lambda = 0.95`.
- Synchronous PPO: collect rollouts with the current policy, update, publish new weights, collect again. No asynchronous actor/learner lag.
- Initial recipe: 48 episodes per iteration, clip 0.2, 4 epochs, minibatch 256, actor Adam 1e-5, critic Adam 3e-4, gradient-norm clip 0.5, advantage normalization, early stop when an epoch's mean approximate KL exceeds 0.02, entropy coefficient 0, KL-to-init coefficient 0 (measured every iteration, available as an option).

Recorded every iteration: training terminal clear_rate, policy/value loss, entropy, approximate KL, KL(current || BC init), clip fraction, gradient norms, explained variance, advantage statistics, the sampled share of every action kind, the share of sampled actions that differ from the behavior greedy action, from the canonical action and from the BC-init greedy action (overall and per decision point), and timing split into candidate generation/encoding, actor forward, environment, critic values, init forward, GAE and update.

Decision rule after the pilot:

- clear_rate rises: extend training;
- policy barely changes and entropy is near zero: adjust exploration;
- policy changes and performance collapses: learning rate / KL / clip / critic;
- update time dominates: minibatch size, then re-measure CUDA;
- rollout time dominates: candidate generation/encoding, batched inference across environments;
- learns, then plateaus early: hidden 128/256 comparison.

## Results

All runs are release builds on a 12-thread x86-64 Linux host (31 GB RAM), burn-flex CPU backend. Datasets, checkpoints and run logs live in `simulator/artifacts/phase4b/` (not committed).

### GPU check

A GTX 1080 Ti (CUDA 12.4) was benchmarked on the same BC batches (`semantic_bc::tests::cpu_vs_cuda_bc_step_benchmark`, `--features simulator-cuda`, warm-up excluded):

| batch | train step CPU | train step CUDA | inference CPU | inference CUDA |
|---|---|---|---|---|
| 1 | 2.8 ms | 36.2 ms | 0.42 ms | 4.3 ms |
| 64 | 42.6 ms | 69.8 ms | 8.3 ms | 10.1 ms |
| 256 | 210.9 ms | 239.6 ms | 36.1 ms | 19.2 ms |
| 963 | 1,127 ms | 938 ms | 171 ms | 61.8 ms |

With hidden 64 (about 170k actor parameters) the work is dominated by many small kernels and host-side batch construction, so CUDA does not speed up training at the batch sizes used. Phase 4B trains on the CPU. The CUDA backend stays available behind `simulator-cuda` for a larger model.

### Fresh canonical dataset

Provenance: git `ee6a530f`, clean tree, dataset schema 2, environment action schema 8, candidate set 2, encoder 1, game config digest `3541ca79...`, `game_rules_epoch` 2, core tree `fd78ed5a`.

| dataset | games | decisions | mean decisions/game | mean terminal clear_rate | victories |
|---|---|---|---|---|---|
| `phase4b_canonical_train` first 2,048 | 2,048 | 173,945 | 84.9 | 36.78 | 0 |
| `phase4b_canonical_validation` | 64 | 5,416 | 84.6 | 36.52 | 0 |

Generation took 232 s for 2,048 games on 12 threads (338 MB).

### Fresh canonical BC (`phase4b-init`)

2,048 games, hidden 64, Adam 1e-3, batch 64, budget 6 epochs, 155 s per epoch:

| epoch | train loss | validation NLL | top-1 | action-kind acc. |
|---|---|---|---|---|
| 1 | 0.1388 | 0.0292 | 99.11% | 99.89% |
| 2 | 0.0333 | 0.0178 | 99.17% | 99.98% |
| 3 | 0.0218 | 0.0151 | 99.37% | 100% |
| **4 (selected)** | 0.0209 | **0.0126** | **99.48%** | 100% |
| 5 | 0.0194 | 0.0129 | 99.37% | 100% |
| 6 | 0.0187 | 0.0135 | 99.43% | 100% |

Phase 4A large reached 99.78% top-1 after 16 epochs; 99.48% after 4 epochs is close enough, and the gate below is what decides.

Terminal evaluation on `ppo_development` (128 seeds, greedy):

| policy | mean | median | stage | decisions | illegal | fallback | post-sampling mutations | canonical agreement |
|---|---|---|---|---|---|---|---|---|
| canonical | 36.23 | 35.11 | 18.51 | 84.0 | 0 | 0 | 0 | 100% |
| BC | 36.10 | 35.11 | 18.44 | 83.8 | 0 | 0 | 0 | 99.2% |

BC - canonical: -0.14 (SE 0.13), better/worse/tie 17/12/99. The gate passes; `artifacts/phase4b/phase4b-init` (epoch 4, `selected-model.bin` sha256 `46c6f9f4...`) is frozen as the PPO initialization.

### Critic bootstrap

Canonical trajectories, first 512 training games (42,958 decisions), validation 64 games, target `0.1 * (terminal - current clear_rate)` (target mean about 1.9, standard deviation about 1.1), Adam 1e-3, batch 256, 4 epochs, 21 s per epoch.

| critic | validation MSE | MAE | explained variance | correlation |
|---|---|---|---|---|
| random init, raw inputs | 21,453 | 64.7 | -16,891 | - |
| pretrained, raw inputs (epoch 3) | 0.377 | 0.475 | 0.70 | 0.84 |
| random init, squashed inputs | 5.01 | 1.94 | -2.94 | - |
| **pretrained, squashed inputs (epoch 4)** | **0.156** | **0.305** | **0.877** | **0.937** |

A random critic would have produced advantages hundreds of times larger than the returns at iteration 0. With raw inputs the pretrained critic also broke during the first PPO update: value loss rose from 0.14 to a maximum of 317 within one update and gradient norms reached 80,000, because some shared features are unnormalized (card polish enters as `polish_pct_raw / 1000`, up to 3,000). The critic now squashes its inputs (`sign(x) * ln(1 + |x|)`, `CRITIC_CHECKPOINT_SCHEMA_VERSION = 2`). The actor keeps the raw features its BC initialization was trained on. After the change the PPO update keeps value loss at 0.02-0.05, explained variance at 0.96-0.98, and critic gradient norms below 10. All predictions are finite.

### Smoke gates

- Gate 1 (rollout): every PPO iteration so far has illegal = 0, sampled != executed = 0, truncated = 0, non-finite values = 0, and maximum telescoping error below 1e-3.
- Gate 2 (one update): policy loss, value loss, entropy, KL and KL-to-init are finite; checkpoints save and load. `semantic_ppo::tests::ppo_update_is_finite_and_resume_is_deterministic` checks that an interrupted and resumed run matches an uninterrupted one. The match is bit-exact when run alone; under parallel test load the CPU backend's parallel float reductions can reorder sums, so the test allows 1e-4.
- Gate 3 (iteration 0): the PPO actor before any update matches `phase4b-init` on all 128 development seeds (paired delta 0.00, 0/0/128) and has identical logits.

### Pilot A: frozen recipe (actor Adam 1e-5, entropy 0)

10 iterations (stopped), 48 episodes per iteration:

- approximate KL per iteration about 1e-5 against a target of 0.02, clip fraction 0;
- behavior entropy fell from 0.017 to 0.013, and the share of sampled actions that differ from the greedy action fell from 0.8% to 0.5%;
- TowerPlacement, PreDefenseItem and CardSelection had zero entropy;
- development evaluation at iterations 5 and 10: PPO - BC init 0.00 (0/0/128) and -0.02 (1/1/126).

The policy barely moved and was getting sharper, which is the "policy unchanged, entropy near zero" branch. The next pilot therefore changed exploration only.

### Pilot B: actor Adam 3e-4, entropy coefficient 0.01

Everything else is unchanged (48 episodes per iteration, clip 0.2, 4 epochs, minibatch 256, target KL 0.02, KL-to-init measured only). Development evaluation on 128 `ppo_development` seeds, greedy:

| iteration | PPO mean | PPO - canonical | PPO - BC init | better/worse/tie vs BC | behavior entropy | KL(pi or pi_init) |
|---|---|---|---|---|---|---|
| 0 | 36.10 | -0.14 (0.13) | 0.00 (0.00) | 0/0/128 | 0.017 | 0 |
| 5 | | +0.12 (0.18) | +0.25 (0.13) | 12/9/107 | 0.024 | 0.009 |
| 10 | | +0.23 (0.19) | +0.37 (0.15) | 16/5/107 | 0.026 | 0.033 |
| 15 | 36.47 | +0.24 (0.24) | +0.38 (0.20) | 26/14/88 | 0.029 | 0.089 |
| 20 | 36.48 | +0.24 (0.22) | +0.38 (0.19) | 27/18/83 | 0.032 | 0.076 |
| 25 | 36.69 | +0.45 (0.29) | +0.59 (0.27) | 38/21/69 | 0.070 | 0.285 |
| 30 | 37.17 | +0.93 (0.32) | +1.07 (0.29) | 52/20/56 | 0.179 | 0.416 |

(SE in parentheses. Training-episode clear_rate uses new seeds every iteration and is not comparable across iterations.)

Through iteration 30, every iteration had illegal = 0, sampled != executed = 0, truncation 0, and critic explained variance 0.97-0.99. The sampled action-kind shares stayed stable (reroll 0.065 -> 0.085, every other kind within about 0.01). Policy changes concentrate in Shop, TreasureSelection, card-service and card selection decisions; TowerPlacement and PreDefenseItem are essentially unchanged. The greedy PPO policy buys more (1,295 vs 1,208 purchases on 128 seeds), rerolls more (787 vs 717) and plays further (mean stage 18.96 vs 18.44).

Watch item: behavior entropy and KL to the BC initialization keep growing under the entropy bonus.
