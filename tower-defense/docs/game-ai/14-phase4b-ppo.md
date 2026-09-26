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

Recorded below as they are produced.
