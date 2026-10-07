# A1′ Target-KL Backtracking Experiment

## Preregistered intervention

This experiment changes only how one actor optimizer minibatch that overshoots target KL is handled. The A1′ hidden-64 recipe remains actor and critic LR `.0003`, clip epsilon `.2`, target KL `.02`, four PPO epochs, minibatch 256, max grad norm `.5`, joint entropy `.01` in phase 1 and `.003` in phase 2, with the same initialization, rollout schedule, and deterministic R1/R2 training seed blocks. No final seed or teacher is used; architecture, observations, actions, rewards, and all other PPO settings are unchanged.

For each actor minibatch, preserve actor and optimizer pre-state and try the normal full step first. If the same minibatch's post-step approximate KL exceeds `.02`, restore pre-state and retry the same minibatch gradient with a geometrically halved effective learning rate. The preregistered tested scales are `1, 1/2, 1/4, 1/8, 1/16, 1/32, 1/64, 1/128, 1/256`: eight retries after the full step. Accept the largest tested scale whose post-step KL is `<= .02`; reject the minibatch only if all nine tested scales exceed the limit. A complete reject does not stop the rest of the actor minibatches in that iteration. Critic minibatches always follow the original schedule.

## Optimizer semantics

Implementation inspection uses Burn `burn-optim 0.21.0`. `Adam::step` applies configured weight decay, if present, by transforming the gradient using the current parameter; then `OptimizerAdaptor` applies gradient clipping; Adam updates first and second moments and its per-parameter time from that gradient; finally it computes a normalized direction and applies `parameter - learning_rate * direction`. This repository configures gradient-norm clipping and no weight decay. The optimizer record contains per-parameter moment and step state, and the optimizer supports clone, `to_record`, and `load_record` restoration.

For fixed pre-state and identical gradient, changing only the effective learning rate scales the same Adam parameter delta exactly while leaving the accepted moment and step state the same as a full Adam step. The implementation therefore restores actor and optimizer pre-state before each trial and re-runs the optimizer at `base_lr * scale`; it does not interpolate parameters after a full step. Each retry recomputes the same minibatch loss/gradient from the restored actor and stored transitions. This also preserves the optimizer's normal clipping and weight-decay placement if configuration changes in another supported mode.

## Focused verification plan

Before the deterministic diagnostic, tests will cover: unchanged full-step acceptance and optimizer commit; overshoot followed by a smaller accepted trial, checking parameter and optimizer state against an independent run at that accepted effective LR; full rejection restoring both states while allowing the next actor minibatch; critic schedule invariance; and zero illegal, mismatched, or nonfinite results. The R1 diagnostic stops at iteration 30 for technical checks only. If valid, no tuning occurs before the two-seed 275-iteration run.

## Implementation and focused verification

The backtracking path applies to the full A1′ actor update. It tries the full step followed by the eight preregistered halvings, restoring actor and optimizer before each retry and recomputing the same minibatch gradient from the restored model. It records per-iteration attempted minibatches, full-step accepts, backtracked accepts, complete rejects, retry count, accepted scale histogram, full-step KL samples, accepted post-step KL samples, and pre-clip gradient norms in `UpdateStats`, serialized in each `ppo.json` history record. Existing entropy, pre-step approximate KL, clip fraction, KL-to-init, rollout decisions/game, action frequencies, critic EV/value loss, and invalid counters remain available alongside them.

Release focused tests passed:

- `target_kl_backtracking_matches_scaled_adam_step_and_rejects_atomically`
- `minibatch_target_kl_reject_restores_actor_and_adam_then_keeps_critic_running`

The first verifies full-step byte equality with the no-target-KL A1′ optimizer result, a forced overshoot followed by an accepted smaller step, parameter and Adam record equality with an independent run at the accepted effective LR, complete rollback after all nine scales fail, continued processing of the next actor minibatch, full critic schedule, and zero nonfinite updates. Its synthetic backtracking case is isolated to the test and does not alter the training recipe. The rollout used to construct its transition reported illegal actions, action mismatches, and nonfinite values all zero.

## R1 deterministic technical diagnostic through iteration 30

The release executable was invoked at `tower-defense/simulator/target/release/td-simulator`; `/proc/<pid>/exe` and `ps` confirmed the executable and full command. It used the original R1 seed block, hidden-64 A1′ initialization, and unchanged recipe. This 30-iteration run is retained as the beginning of the R1 phase-1 full run, then resumed from its iteration-30 checkpoint to iteration 75.

| Diagnostic measure | Result |
|---|---:|
| Attempted actor minibatches | 2,216 |
| Full-step accepts | 2,180 (98.38%) |
| Backtracked accepts | 3 (0.14%) |
| Complete rejects | 33 (1.49%) |
| Secondary retry trials | 271 |
| Accepted scales | `1.0: 2,180`; `0.5: 2`; `0.03125: 1` |
| Full-step proposed KL, mean / max | `.004460 / .096290` |
| Accepted post-step KL, max | `.019992` |
| Critic steps / scheduled minibatches | `2,216 / 2,216` |
| Illegal / mismatch / nonfinite rollout / nonfinite update | `0 / 0 / 0 / 0` |
| Mean decisions/game | `92.77` |
| Mean update entropy / clip fraction | `.2293 / .03522` |
| Mean / max pre-clip actor gradient norm | `.4842 / 3.3256` |

The cap was technically valid: every accepted actor step met target KL, complete rejects restored state and did not stop later actor steps, the critic schedule was unaffected, and all validity counters remained zero. The registered full run therefore proceeded without tuning. Development-score behavior during this diagnostic is not used for intervention selection.

## Full stress test

Both R1 and R2 completed 275 iterations (phase 1: 75; phase 2: 200). Original A1′, minibatch-guard, and hard-reject controls below are the already completed runs recorded in [the prior experiment](18-target-kl-reject-on-overshoot.md); none was retrained. The full backtracking trace contains 550 rows in [19-target-kl-backtracking-trajectory.csv](19-target-kl-backtracking-trajectory.csv). All score deltas are percentage points versus canonical. Last-50 dev metrics use the ten scheduled evaluations at global iterations 226–275; other last-50 metrics average the 50 training iterations in that window.

### Performance and late-window metrics

| Policy / seed | Final delta | Best delta | Last-50 dev mean / min / max (n=10) | Decisions/game | KL-to-init | Entropy | Critic EV / value loss |
|---|---:|---:|---:|---:|---:|---:|---:|
| Original A1′ R1 | +6.348 | +11.510 | +5.265 / -0.218 / +9.094 | 131.25 | 15.3621 | .5270 | .9816 / .0212 |
| Minibatch guard R1 | +3.626 | +9.344 | +7.576 / +3.626 / +9.344 | 175.53 | 20.0956 | .5786 | .9859 / .0119 |
| Hard reject R1 | +9.771 | +10.103 | +3.888 / -1.038 / +10.103 | 154.09 | 14.1513 | .6185 | .9826 / .0146 |
| Backtracking R1 | +8.280 | +10.164 | +8.381 / +6.762 / +10.164 | 228.91 | 25.0804 | .6918 | .9890 / .0097 |
| Original A1′ R2 | +11.934 | +11.934 | +8.166 / +6.028 / +11.934 | 228.71 | 24.9702 | .6969 | .9888 / .0102 |
| Minibatch guard R2 | +9.278 | +9.731 | +8.888 / +8.234 / +9.731 | 126.01 | 12.8019 | .6089 | .9805 / .0176 |
| Hard reject R2 | +9.603 | +9.603 | +8.889 / +8.269 / +9.603 | 120.92 | 10.7327 | .4146 | .9836 / .0161 |
| Backtracking R2 | +3.032 | +3.032 | +2.587 / +2.253 / +3.032 | 94.30 | .2510 | .0172 | .9822 / .0138 |

Backtracking final-score seed spread is `5.248` points, close to Original A1′ `5.587` and the non-rollback guard `5.652`, and far above hard reject's `.168`. R1 improved over Original A1′ by `1.932` points and retained a stronger late-window mean/min than every control. R2 ended `8.902` points below Original A1′ and `6.571` below hard reject; its last-window entropy and KL-to-init also show severe actor-update suppression. The intervention did not preserve the strong seed peak.

### Last-50 policy, gradient, and critic trajectory

| Policy / seed | Clip fraction | Pre-clip grad norm mean (range across iterations) | Critic EV (range) | Value loss (range) |
|---|---:|---:|---:|---:|
| Original A1′ R1 | .1286 | 5.0535 | .9816 | .0212 |
| Minibatch guard R1 | .0708 | 1.1609 | .9859 | .0119 |
| Hard reject R1 | .0643 | .7523 | .9826 | .0146 |
| Backtracking R1 | .0809 | .8811 (.8063–.9987) | .9890 (.9824–.9929) | .0097 (.0064–.0160) |
| Original A1′ R2 | .0813 | .9368 | .9888 | .0102 |
| Minibatch guard R2 | .0664 | .7887 | .9805 | .0176 |
| Hard reject R2 | .0396 | .5382 | .9836 | .0161 |
| Backtracking R2 | .2000 | 34.7468 (26.0519–91.0501) | .9822 (.9765–.9859) | .0138 (.0112–.0193) |

The R2 pre-clip gradient norm is highly elevated while only a small fraction of actor minibatches are accepted. Despite that actor instability, critic EV/value loss stayed in the range of the existing controls, and the critic processed every scheduled minibatch: R1 `33,884/33,884`; R2 `19,536/19,536`. Actor update epochs also completed their registered schedule. Illegal actions, action mismatches, nonfinite rollout values, and nonfinite update skips summed to zero. R1 logged one truncated episode over the full run.

### Last-50 action distributions

Each cell is the mean fraction of decisions for that action in the final 50 training rollouts. Action names are kept as emitted by the simulator.

| Policy / seed | Build tower | Confirm card service | Continue | Discard treasure | Place tower | Purchase shop item | Remove tower | Reroll | Select card service card | Select treasure | Start defense | Use inventory item |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Original A1′ R1 | .1672 | .0000 | .2653 | .0047 | .0003 | .0289 | .0091 | .3013 | .0000 | .0201 | .1672 | .0359 |
| Minibatch guard R1 | .1372 | .0038 | .3293 | .0006 | .0430 | .0766 | .0044 | .1748 | .0038 | .0162 | .1372 | .0731 |
| Hard reject R1 | .1571 | .0005 | .3024 | .0007 | .0499 | .0877 | .0035 | .1367 | .0005 | .0185 | .1571 | .0854 |
| Backtracking R1 | .1048 | .0193 | .3221 | .0013 | .0247 | .0629 | .0311 | .2525 | .0196 | .0123 | .1048 | .0448 |
| Original A1′ R2 | .1083 | .0063 | .3819 | .0011 | .0007 | .0539 | .0216 | .2539 | .0063 | .0126 | .1083 | .0450 |
| Minibatch guard R2 | .1834 | .0007 | .1456 | .0008 | .0612 | .1080 | .0009 | .1872 | .0007 | .0215 | .1834 | .1067 |
| Hard reject R2 | .1899 | .0001 | .1499 | .0001 | .0650 | .1072 | .0001 | .1652 | .0001 | .0226 | .1899 | .1100 |
| Backtracking R2 | .2128 | .0106 | .1451 | .0004 | .0684 | .1094 | .0000 | .1038 | .0108 | .0250 | .2128 | .1009 |

### Backtracking outcomes and KL distributions

| Seed | Attempted | Full accept | Backtracked accept | Complete reject | Retry trials | Full / backtracked / reject rate |
|---|---:|---:|---:|---:|---:|---:|
| R1 | 33,884 | 33,744 | 11 | 129 | 1,056 | 99.5868% / .0325% / .3807% |
| R2 | 19,536 | 1,393 | 3,955 | 14,188 | 122,918 | 7.1304% / 20.2447% / 72.6249% |

| Accepted scale | R1 count | R2 count |
|---:|---:|---:|
| 1 | 33,744 | 1,393 |
| 1/2 | 4 | 1,086 |
| 1/4 | 4 | 1,332 |
| 1/8 | 1 | 902 |
| 1/16 | 1 | 371 |
| 1/32 | 1 | 160 |
| 1/64 | 0 | 63 |
| 1/128 | 0 | 32 |
| 1/256 | 0 | 9 |

The scale histogram counts accepted steps only. A minibatch that exhausts all nine scales contributes no accepted scale. Every accepted scale has an accepted post-step approximate KL `<= .02`.

| Seed / distribution | Count | Mean | Median | P95 | Maximum |
|---|---:|---:|---:|---:|---:|
| R1 proposed full-step KL | 33,884 | .006323 | .005966 | .011844 | .096290 |
| R1 accepted post-step KL | 33,755 | .006231 | .005953 | .011702 | .019998 |
| R2 proposed full-step KL | 19,536 | .722943 | .068867 | .254092 | 3,769.529785 |
| R2 accepted post-step KL | 5,348 | .014847 | .017441 | .019877 | .020000 |

Complete reject frequency was `0.3807%` of R1 attempted actor minibatches and `72.6249%` for R2. Unlike hard reject, a failed trial sequence did not stop later actor minibatches: all 53,420 scheduled actor minibatches were attempted. The extreme R2 proposed-KL maximum is retained as measured, not clipped; it explains why even the smallest preregistered scale sometimes failed the constraint. Aggregate per-iteration distributions, accepted-scale histograms, and trial KL samples are preserved in the raw trajectory CSV.

### Classification

**D — training suppression/failure.** Backtracking maintained the trust-region bound for every accepted update and improved R1's late-window stability, but it effectively prevented R2 training: 72.6% of attempted actor minibatches were completely rejected, and the remaining accepted updates had near-zero aggregate KL-to-init and entropy at the end. R2's final score was far below Original A1′ and hard reject. Therefore this run does not meet A, B, or C; its defining outcome is D.

R1/R2 are algorithm-development seeds and are subject to selection bias from prior intervention design. This result does not establish a new baseline. No final seed or prospective validation was run, and no post-result tuning was performed.
