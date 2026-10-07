# A1 Target-KL Reject-on-Overshoot Experiment

## Question and fixed recipe

This experiment tests whether rejecting the actor optimizer step that first exceeds `target_kl = 0.02` controls the single-step overshoot observed with the minibatch target-KL guard. It changes only the actor-step commit semantics. Hidden size is 64. The registered A1 recipe remains actor and critic LR `.0003`, clip epsilon `.2`, target KL `.02`, four PPO epochs, minibatch 256, max grad norm `.5`, joint entropy `.01` for phase 1 and `.003` for phase 2, the same A1 BC and critic initialization, and the existing R1/R2 deterministic training seed blocks. No final seed, teacher, architecture, feature, action, reward, or hyperparameter change was used.

Full per-iteration data for Original A1, minibatch guard without rollback, and reject-on-overshoot is in [18-target-kl-reject-trajectory.csv](18-target-kl-reject-trajectory.csv). It contains 1,680 rows: 275 iterations for each of six comparison runs, plus the separate 30-iteration R1 technical validation. The run metadata records base commit `e188088471068044aa9f68b8865b7375cd1c342a`; the reject implementation was committed after the runs as `8bbf02fc6f4b99c8e707b83523d5ab9f1d77aaeb`. The release binary used for every run came from the exact source later committed there, and that source was not changed during the runs.

## Implementation semantics

Before each proposed actor minibatch update, the learner snapshots the actor module and the active actor optimizer. For Full A1 this is the entire actor plus its Adam `OptimizerAdaptor`; its clone includes the per-parameter Adam moment records and step state. The candidate optimizer step is applied, then the candidate policy's approximate KL is evaluated against the same minibatch's stored old log-probs.

- If post-step KL is `<= .02`, the actor and Adam update are retained and counted as accepted/applied.
- If post-step KL is `> .02`, the actor module and active optimizer are restored from their pre-step snapshots. The rejected step is not counted as applied. Actor updates stop for the rest of that iteration; critic updates continue through the existing full schedule.

PositionOnly and BuildOptionOnly paths also snapshot and restore their active actor parameters and optimizer state. The legacy `applied_actor_optimizer_steps` counter now equals accepted steps. New `accepted_actor_optimizer_steps`, `rejected_actor_optimizer_steps`, `rejected_step_minibatch_index`, `rejected_step_proposed_post_step_approx_kl`, `max_accepted_post_step_approx_kl`, `max_proposed_post_step_approx_kl`, `rejected_actor_grad_norm`, and `rejected_shared_scorer_grad_norm` fields make commit/reject outcomes explicit. The earlier target-KL stop fields remain aliases for the rejection that stopped the iteration. `max_post_step_approx_kl` remains an alias for the maximum accepted post-step KL.

## Focused verification

Release build and focused test passed:

```text
cargo check --release --bin td-simulator
cargo build --release --bin td-simulator
cargo test --release --lib minibatch_target_kl_reject_restores_actor_and_adam_then_keeps_critic_running -- --nocapture
1 passed; 0 failed
```

The test exercises an accepted update and a forced rejected update after Adam moments have been populated. Accepted updates change actor parameters and optimizer serialization. For rejection, the actor module serialization and canonicalized per-parameter Adam record serialization are exactly equal to their pre-proposal snapshots; accepted/applied count stays zero, rejected count increments, the proposed KL is recorded, and later actor minibatches are skipped. A following iteration accepts updates from that exact restored state. Critic updates continue for every scheduled minibatch, and rollout/update invalid counters are zero.

## Deterministic R1 validation through iteration 30

The validation used the release binary, original R1 seed block, original A1 initialization, and unchanged `.0003 / .2 / .02 / .01` recipe. It reached 30 iterations with no performance stopping rule.

| Measure | Result |
|---|---:|
| Rejected iterations / 30 | 17 |
| Attempted / accepted / rejected actor steps | 1,553 / 1,536 / 17 |
| Maximum accepted post-step KL | `.019647` |
| Rejected proposed KL, min / mean / max | `.020115 / .031542 / .055570` |
| Critic optimizer steps / scheduled minibatches | 2,076 / 2,076 |
| Illegal / mismatch / nonfinite rollout / nonfinite update skips | 0 / 0 / 0 / 0 |

The exact actor and optimizer rollback is asserted in the focused test; the subsequent accepted update from the restored state also passed. The validation was technically valid, so the registered full experiment proceeded without recipe changes.

## Full stress test: performance

All final and best dev deltas are percentage points versus canonical. Last-50 dev values summarize the ten scheduled evaluations in global iterations 226275; last-50 rollout clear values use all 50 training rollouts.

| Policy / seed | Final dev delta | Best dev delta | Last-50 dev mean / min / max (n=10) | Last-50 rollout clear mean / min / max |
|---|---:|---:|---:|---:|
| Original A1 R1 | +6.348 | +11.510 | +5.265 / -0.218 / +9.094 | 41.707 / 35.469 / 48.909 |
| Minibatch guard R1 | +3.626 | +9.344 | +7.576 / +3.626 / +9.344 | 46.993 / 42.621 / 50.157 |
| Reject R1 | +9.771 | +10.103 | +3.888 / -1.038 / +10.103 | 47.545 / 44.720 / 51.197 |
| Original A1 R2 | +11.934 | +11.934 | +8.166 / +6.028 / +11.934 | 48.643 / 45.372 / 52.314 |
| Minibatch guard R2 | +9.278 | +9.731 | +8.888 / +8.234 / +9.731 | 45.434 / 41.079 / 48.215 |
| Reject R2 | +9.603 | +9.603 | +8.889 / +8.269 / +9.603 | 45.149 / 41.879 / 47.918 |

Primary reject-minus-original final deltas were `+3.423` for R1 and `-2.331` for R2. Secondary reject-minus-guard final deltas were `+6.145` for R1 and `+.325` for R2. The final seed spread fell from `5.587` points for Original A1 and `5.652` for the non-rollback guard to `.168` for reject-on-overshoot. The last-50 dev-mean spread, however, was `5.001` for reject, compared with `2.901` original and `1.311` guard. R1's final score improved, but its last-50 mean and minimum were worse than both comparators; R2's last-50 mean and minimum were better than original while its final delta remained `2.331` points below original.

## Accepted and rejected steps

| Seed | Attempted actor steps | Accepted | Rejected | Rejected iterations | Rejected / attempted | Accepted steps per iteration, min / mean / max | Iterations with zero accepted actor steps |
|---|---:|---:|---:|---:|---:|---:|---:|
| R1 | 23,348 | 23,280 | 68 | 68 / 275 (24.7%) | 0.29% | 2 / 84.65 / 132 | 0 |
| R2 | 15,254 | 15,081 | 173 | 173 / 275 (62.9%) | 1.13% | 2 / 54.84 / 100 | 0 |

Reject counts by entropy phase were R1 `36/75` in phase 1 and `32/200` in phase 2; R2 `60/75` and `113/200`. Each iteration has at most one rejected step, after which later actor updates stop. Even in the most rejection-heavy R2 phase, every iteration retained at least two accepted actor steps. Thus rejects were frequent by iteration, especially for R2, but not most of the actor proposals that were actually attempted; no iteration had zero accepted steps.

| Rejected proposed post-step KL | R1 | R2 |
|---|---:|---:|
| Count | 68 | 173 |
| Min | .02012 | .02002 |
| Median | .02504 | .02541 |
| Mean | .02888 | .03409 |
| P95 | .05251 | .06188 |
| Max | .06735 (iteration 188) | .49571 (iteration 219) |
| Max accepted post-step KL | .01965 (iteration 6) | .01983 (iteration 90) |

All accepted actor steps in both full runs stayed at or below `.02`. The highest overshoot proposals were rejected and restored. At R1's maximum rejected proposal, total/shared-scorer pre-clip grad norms were `1.7843 / 1.7832`; at R2's maximum-KL rejection they were `.6705 / .6705`. Across rejected steps, mean shared-scorer grad norm was `.94` for R1 and `.64` for R2. Earlier root-cause instrumentation measured the shared candidate scorer at 99.9986% of actor gradient squared norm; the rejected-step measurements remain consistent with that concentration.

## Last-50 trajectory: policy, actor, and critic

| Run | Update entropy | KL-to-init | Clip fraction | Mean actor grad norm | Mean per-iteration max actor grad norm | Critic EV / value loss |
|---|---:|---:|---:|---:|---:|---:|
| Original R1 | .5270 | 15.3621 | .1286 | 5.0535 | 18.1312 | .9816 / .0212 |
| Guard R1 | .5786 | 20.0956 | .0708 | 1.1609 | 2.7559 | .9859 / .0119 |
| Reject R1 | .6185 | 14.1513 | .0643 | .7523 | 2.0172 | .9826 / .0146 |
| Original R2 | .6969 | 24.9702 | .0813 | .9368 | 3.0802 | .9888 / .0102 |
| Guard R2 | .6089 | 12.8019 | .0664 | .7887 | 2.0418 | .9805 / .0176 |
| Reject R2 | .4146 | 10.7327 | .0396 | .5382 | 1.6874 | .9836 / .0161 |

| Run | Decisions/game | Continue | Reroll | BuildTower |
|---|---:|---:|---:|---:|
| Original R1 | 131.25 | .2653 | .3013 | .1672 |
| Guard R1 | 175.53 | .3293 | .1748 | .1372 |
| Reject R1 | 154.09 | .3024 | .1367 | .1571 |
| Original R2 | 228.71 | .3819 | .2539 | .1083 |
| Guard R2 | 126.01 | .1456 | .1872 | .1834 |
| Reject R2 | 120.92 | .1499 | .1652 | .1899 |

Critic EV remained high throughout; no critic failure preceded actor issues. Across the 550 full reject iterations, critic optimizer steps equaled scheduled minibatches (R1 `25,904/25,904`; R2 `23,500/23,500`). Illegal actions, action mismatches, nonfinite rollout values, and nonfinite update skips were all zero.

## Classification

**B  stable but overconstrained trade-off (closest fit).** The reject semantics enforce the minibatch KL bound on every accepted step, R1's final dev delta improves, the final cross-seed spread shrinks sharply, and no iteration loses all accepted actor updates. But the late-window R1 mean/min regress, R2's final score falls `2.331` points versus Original A1, and R2 triggers a stop in `62.9%` of iterations. This is not outcome D: rejected steps were only `0.29%` of attempted R1 proposals and `1.13%` of R2 proposals, with at least two accepted actor steps in every iteration. The result does not satisfy A's late-window stability condition.

No further tuning or experiment was performed.
