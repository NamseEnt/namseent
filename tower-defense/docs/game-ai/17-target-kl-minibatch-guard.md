# A1 Minibatch-Level Target-KL Guard Experiment

## Scope and preregistered recipe

Question: does checking `target_kl = 0.02` immediately after each actor optimizer minibatch stabilize the weak R1 seed while retaining strong R2 performance?

Only the timing of the actor target-KL check changes. Hidden size remains 64, actor LR `0.0003`, critic LR `0.0003`, clip epsilon `0.2`, update epochs `4`, minibatch size `256`, max grad norm `0.5`, joint entropy `.01` for phase 1 and `.003` for phase 2, target KL `.02`, same BC and critic initialization, same deterministic PPO seed derivation, and the existing R1/R2 seed blocks. No teacher, final seed, architecture, feature, reward, action-space, or other hyperparameter change is included.

The complete raw aligned trajectory is in [17-target-kl-minibatch-guard-trajectory.csv](17-target-kl-minibatch-guard-trajectory.csv). Iterations use the original 75 + 200 schedule: p1 is entropy `.01`; p2 resumes from its iteration-75 actor/critic checkpoint with entropy `.003`, for 275 total training iterations per seed.

## Semantics before and after

Before: each minibatch's PPO approximate KL was computed before its optimizer step. All actor minibatches in the epoch were applied. Only then was their mean compared with target KL; an exceedance skipped later epochs but could not stop any actor step in the current epoch.

After: for every actor optimizer minibatch, the update is applied first. The same minibatch is re-evaluated using its stored rollout old log-probs and the updated actor. If post-step approximate KL is strictly greater than `.02`, no later actor optimizer step is applied in that iteration. The crossing update is retained; there is no rollback. The current and later critic updates continue on the original schedule.

The legacy `early_stopped` artifact key is accepted as an alias on read and serialized as `epoch_target_kl_stopped`; it refers only to the old epoch-level stop and is false for this minibatch guard. New iteration-level counters are:

- `attempted_actor_minibatches`: actor minibatches for which PPO terms were attempted through the stopping minibatch.
- `applied_actor_optimizer_steps`: actor optimizer steps actually applied.
- `target_kl_stop_count`: 0 or 1 for the iteration.
- `target_kl_stop_minibatch_index`: 1-based ordinal among attempted actor minibatches that crossed target.
- `target_kl_stop_post_step_approx_kl`: KL measured after that applied step.
- `max_post_step_approx_kl`: maximum post-step KL over applied actor minibatches in the iteration.
- `critic_optimizer_steps`: critic updates actually applied; these continue through all scheduled minibatches/epochs after an actor stop.

`approx_kl` retains its previous meaning: mean of PPO approximate KL values evaluated before their corresponding optimizer steps. `epochs_completed` counts the complete outer schedule (which continues for critic training), so it must not be interpreted as the number of actor epochs with steps. The actor step counters and stop index give the actor schedule precisely.

## Implementation and focused verification

`PpoLearner::update` now makes the post-step check inside the actor minibatch loop and gates only later actor branches. The critic branch remains outside that gate. The release focused tests passed:

- `target_kl_stop_uses_post_step_minibatch_kl_and_strict_threshold`
- `minibatch_target_kl_stop_keeps_critic_schedule_and_skips_later_actor_steps`

The integration test uses the existing default A1 PPO settings and synthetic transition log-prob perturbation to exercise the guard, not to tune any recipe value. It verifies that a threshold crossing occurs after one applied actor step, no subsequent actor step is applied, all four critic updates still run, a below-target minibatch retains the full actor-step count, and nonfinite skips remain zero.

## Deterministic R1 validation through iteration 30

Validation used the release executable and fresh original R1 initialization, with the existing R1 training seed block. The replay stopped at iteration 30 only for the prescribed technical check. The release binary path and process command line were verified.

Across all 30 iterations, illegal actions, action mismatches, rollout nonfinite values, and update nonfinite skips were zero. On every stopped iteration, `attempted_actor_minibatches = applied_actor_optimizer_steps = target_kl_stop_minibatch_index`; the critic completed all scheduled minibatches and all four epochs. There were 22 target stops in the first 30 iterations. The largest post-step KL was `0.09687` at iteration 22: the guard stopped immediately after observing it, but that single applied step itself overshot target .02 substantially. This is direct evidence relevant to outcome D; full R1/R2 results are still required for the requested final classification.

Through iteration 30, the guard tracked the original R1 recipe and did not stop for a performance gate. It was technically valid, so the same checkpoint was continued to the full phase-1 budget.

## R1 phase 1 through iteration 75 (interim)

The 75-iteration phase completed with zero technical invariant failures. There were 30 iterations with a minibatch-level target stop. Across phase 1, the actor attempted/applied `3,956/3,956` optimizer steps; critic optimizer steps were `5,096`, so critic updates continued after every actor stop. Maximum post-step approximate KL was `0.09687`.

Compared with original R1 at selected early iterations:

| Iteration | Metric | Original R1 | Guarded R1 |
|---:|---|---:|---:|
| 14 | entropy | 0.216 | 0.055 |
| 14 | KL-to-init | 3.556 | 0.139 |
| 14 | Continue frequency | 0.285 | 0.143 |
| 14 | decisions/game | 113.4 | 89.1 |
| 22 | update mean pre-step KL | 0.00379 | 0.00736 |
| 22 | max post-step KL | not recorded | 0.09687 |
| 30 | dev delta vs canonical | -5.523 | +0.767 |
| 30 | critic explained variance / value loss | 0.951 / 0.0434 | 0.976 / 0.0197 |
| 40 | entropy | 0.404 | 0.249 |
| 40 | Continue / Reroll / BuildTower | 0.247 / 0.121 / 0.163 | 0.146 / 0.035 / 0.222 |
| 75 | dev delta vs canonical | -1.457 | +2.792 |

At iteration 75, guarded R1 had mean pre-step KL 0.00414 and KL-to-init 3.170, versus original R1 0.00635 and 12.401. Phase-1 dev and action-frequency results are interim only; final judgment uses both phases and both seeds.

## Full stress test results

Both guarded seeds completed the registered 275 iterations: phase 1 ran 75 iterations at entropy `.01`; phase 2 resumed the phase-1 iteration-75 checkpoint for 200 iterations at entropy `.003`. Baselines are the existing original A1 R1/R2 artifacts. No baseline was retrained.

### Early R1 trajectory, iterations 140

The first guarded R1 target crossing was iteration 3, actor minibatch 16, post-step KL `.05024`. The first clear policy-trajectory separation is visible by rollout iteration 14: entropy was `.0552` versus original `.2144`, KL-to-init `.139` versus `3.556`, Continue frequency `.143` versus `.285`, and decisions/game `89.29` versus `113.40`.

| Global iteration | Metric | Original R1 | Guarded R1 |
|---:|---|---:|---:|
| 14 | entropy / KL-to-init | .2144 / 3.556 | .0552 / .139 |
| 14 | Continue / Reroll / BuildTower | .285 / .076 / .175 | .143 / .065 / .216 |
| 14 | decisions/game | 113.40 | 89.29 |
| 14 | critic EV / value loss | .9779 / .0165 | .9763 / .0183 |
| 22 | pre-update mean KL / max post-step KL | .00379 / not recorded | .00736 / .09687 |
| 22 | entropy / KL-to-init | .2530 / 5.453 | .1041 / .785 |
| 22 | Continue / Reroll / BuildTower | .309 / .089 / .162 | .148 / .044 / .216 |
| 30 | dev delta vs canonical | -5.523 | +.767 |
| 30 | critic EV / value loss | .9511 / .0434 | .9757 / .0197 |
| 40 | entropy / KL-to-init | .4098 / 7.605 | .2652 / 1.402 |
| 40 | Continue / Reroll / BuildTower | .247 / .121 / .163 | .146 / .035 / .222 |
| 40 | critic EV / value loss | .9672 / .0311 | .9811 / .0182 |

This is an ordering of observed rollout and update metrics, not a causal attribution. The critic remained in its normal high-EV, low-value-loss range and did not show an earlier failure. Across the full run, the maximum recorded actor grad norm was much lower in guarded R1's final-50 window than in original R1; the shared candidate scorer was previously measured at 99.9986% of actor gradient squared norm. This run recorded total actor grad norms, not a new per-parameter-group decomposition.

### Final and late-window performance

Dev deltas are percentage points versus canonical. The final and best values are across the 275-iteration development-evaluation history. Last-50 dev statistics use the ten scheduled evaluations within iterations 226-275; rollout statistics use all 50 per-iteration training rollouts.

| Run | Final dev delta | Best dev delta | Last-50 dev mean / min / max (n=10) | Last-50 rollout clear mean / min / max |
|---|---:|---:|---:|---:|
| Original R1 | +6.348 | +11.510 | +5.265 / -0.218 / +9.094 | 41.707 / 35.469 / 48.909 |
| Guarded R1 | +3.626 | +9.344 | +7.576 / +3.626 / +9.344 | 46.993 / 42.621 / 50.157 |
| Original R2 | +11.934 | +11.934 | +8.166 / +6.028 / +11.934 | 48.643 / 45.372 / 52.314 |
| Guarded R2 | +9.278 | +9.731 | +8.888 / +8.234 / +9.731 | 45.434 / 41.079 / 48.215 |

R1 final delta changed by `-2.722` points and R2 by `-2.656` points. The final cross-seed spread changed from `5.587` to `5.652` points (slightly wider). The last-50 dev-mean spread narrowed from `2.901` to `1.311` points, but this did not recover either original final result or the original R2 best result.

### Actor update and stability metrics

| Run | Target-stop iterations | Applied actor steps | Critic steps | Maximum post-step KL (iteration) | Last-50 update entropy | Last-50 clip fraction | Last-50 mean actor grad norm | Last-50 maximum minibatch actor grad norm |
|---|---:|---:|---:|---:|---:|---:|---:|
| Guarded R1 | 117 | 19,183 | 25,296 | .39941 (271) | .5786 | .0708 | 1.1609 | 5.796 |
| Guarded R2 | 74 | 20,635 | 23,228 | .19090 (4) | .6089 | .0664 | .7887 | 11.574 |
| Original R1 | epoch-level only | not recorded with new counters | not recorded with new counters | not recorded | .5270 | .1286 | 5.0535 | 89.477 |
| Original R2 | epoch-level only | not recorded with new counters | not recorded with new counters | not recorded | .6969 | .0813 | .9368 | 12.184 |

The rightmost column is the largest per-minibatch grad norm observed in any iteration within the final-50 window. Guarding reduced R1's final-50 clip fraction and mean grad norm, while guarded R2's entropy, decisions/game, and action mix shifted materially.

### Last-50 action frequencies and critic metrics

Frequencies are means over each iteration's rollout action-kind distribution. Decisions/game is also averaged over all 50 rollouts.

| Run | Decisions/game | Continue | Reroll | BuildTower | Critic EV | Critic value loss |
|---|---:|---:|---:|---:|---:|---:|
| Original R1 | 131.25 | .2653 | .3013 | .1672 | .9816 | .0212 |
| Guarded R1 | 175.53 | .3293 | .1748 | .1372 | .9859 | .0119 |
| Original R2 | 228.71 | .3819 | .2539 | .1083 | .9888 | .0102 |
| Guarded R2 | 126.01 | .1456 | .1872 | .1834 | .9805 | .0176 |

Full action-kind distributions, per-iteration entropy/KL/clip/gradient/critic series, and all validity counters are in the linked CSV. Guarded R1's late-window behavior improved over original R1 on mean dev delta and decisions/game, but final and best dev deltas fell. Guarded R2's late-window mean dev delta rose modestly, while decisions/game fell by about 45% and its final/best dev scores were lower.

### Target-stop and technical invariants

R1 stopped actor updates in 117/275 iterations; R2 did so in 74/275. In every stopped iteration, `attempted_actor_minibatches = applied_actor_optimizer_steps = target_kl_stop_minibatch_index`. The first threshold exceedance ended all later actor minibatches and epochs for that iteration. Critic optimizer steps equaled the full scheduled minibatch count on every iteration, including stopped iterations; critic training continued to completion.

Across all 550 guarded iterations, illegal actions = 0, action mismatches = 0, rollout nonfinite values = 0, and nonfinite update skips = 0. Both phase checkpoints and completed-iteration counts match 75 + 200 per seed. Focused release tests and release build/check passed as recorded above.

The guard does stop later actor updates, but it does not roll back the crossing step. The largest R1 post-step minibatch KL was `.39941` at global iteration 271 (stop minibatch 38); R2 reached `.19090` at iteration 4 (stop minibatch 44). Both are far above target `.02` after a single applied optimizer step. This directly satisfies outcome **D**: one optimizer step can overshoot substantially, so minibatch early stopping alone does not cap the crossing update. The result does not meet A; B-like performance losses are also present, but D is the required primary classification because the observed single-step overshoot is decisive.

No additional recipe or hyperparameter was tested.
