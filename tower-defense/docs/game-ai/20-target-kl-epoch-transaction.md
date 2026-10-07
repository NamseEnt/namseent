# A1′ Target-KL Epoch Transaction Experiment

## Question and registered recipe

This experiment asks whether a target-KL transaction around a complete PPO actor epoch can reject catastrophic policy movement while preserving useful updates. The only recipe change from hidden-64 A1′ is the actor epoch commit/rollback boundary. Actor and critic learning rates remain `.0003`, clip epsilon `.2`, target KL `.02`, minibatch 256, four update epochs, max grad norm `.5`, and entropy `.01` in phase 1 / `.003` in phase 2. Training uses development seeds R1 and R2 only. No final seed, teacher, or changes to architecture, features, actions, rewards, or PPO hyperparameters are included.

## Approximate-KL definition

For rollout actor sample `i`, let `old_log_prob_i` be the stored behavior actor log-probability of the selected action and `new_log_prob_i` be that selected action's log-probability under the actor being measured. The implementation computes:

```text
d_i = new_log_prob_i - old_log_prob_i
ratio_i = exp(d_i)
approx_kl_i = (ratio_i - 1) - d_i
approx_kl = mean_i(approx_kl_i)
```

The sample value is the nonnegative estimator `exp(d) - 1 - d`; the aggregate is its arithmetic mean over actor samples. In the full semantic actor, the selected joint log-probability includes the masked family probability plus the selected candidate's family-conditional probability. These log-probabilities are added, not averaged. Where an action has a spatial cell choice, its selected conditional cell log-probability is also added to the joint selected-action log-probability. Thus each rollout transition contributes one joint-action `d_i` and one KL estimate.

The epoch gate does **not** average minibatch KLs. At an epoch boundary it evaluates every actor transition from the iteration's rollout against the same epoch-final actor and the transition's stored behavior log-probability. Evaluation is chunked for tensor memory, but all chunks use the identical final actor. The mean uses all `N` rollout actor samples with equal weight. Samplewise KL and log-ratio p50/p90/p95/p99/max are recorded; percentiles use the nearest-rank index `ceil((N-1) * p)` after sorting.

The separately recorded minibatch values are the post-optimizer-step approximate KLs measured on each individual actor minibatch against its stored old log-probabilities. They describe local sensitivity only and never feed the transaction decision.

## Transaction semantics

At each actor epoch start, the learner clones the actor and its complete Adam optimizer state. It then performs the ordinary A1′ minibatch updates across that epoch. At the boundary it measures full-rollout final-policy approximate KL:

- `KL <= .02`: commit actor and optimizer state, then attempt the next actor epoch.
- `KL > .02`: restore the epoch-start actor and Adam state, retain earlier committed epochs, and stop the iteration's remaining actor epochs. Critic minibatches continue on their original schedule.

The measured attempted minibatch count and gradient diagnostics remain available for audit; actor parameters, Adam moments/steps, and committed actor-update accumulators are restored on rejection. Iteration records include actor epochs attempted/committed, rejected epoch index, each attempted epoch's aggregate and samplewise KL/log-ratio summaries, and isolated minibatch KL samples.

## Focused tests and R1 short diagnostic

Four release library tests passed:

- `epoch_transaction_accepts_full_rollout_kl_and_matches_original_update` forces accepted epochs under the configured threshold and verifies full sample count, next-epoch progress, and byte-identical actor and Adam records versus an otherwise identical ungated A1′ update.
- `epoch_transaction_gate_uses_final_full_rollout_kl_not_minibatch_average` uses four minibatches and a threshold between their post-step KL mean and the full-rollout final-actor KL; the transaction follows the full-rollout measurement and its sample count covers the complete supplied rollout.
- `epoch_transaction_reject_restores_actor_and_adam_and_keeps_critic_schedule` forces a first-epoch overshoot and verifies exact actor and Adam restoration, no later actor epoch, and completion of the original critic schedule.
- `epoch_transaction_keeps_prior_commit_when_later_epoch_rejects` sets the synthetic threshold between consecutive epoch KLs and verifies the first epoch remains committed when the second rolls back.

The R1 30-iteration technical diagnostic was retained as the opening of the full R1 phase 1 run. It attempted 117 actor epochs and committed 115; global iterations 4 / 19 rejected actor epoch 3 / 2 at full-rollout KL `.02645` / `.04207`. It processed all `2,164/2,164` scheduled critic minibatches, had zero nonfinite update skips, and logged zero illegal actions, action mismatches, or nonfinite rollout values. The byte-level tests and diagnostic showed valid rollback and critic invariants, so the registered recipe proceeded unchanged to the full runs. This was a technical check, not a performance gate.

## Full stress test and comparisons

Both development seeds completed 275 iterations (phase 1: 75; phase 2: 200). The R1 technical diagnostic is the first 30 iterations of its full run. Original A1′ and the three minibatch interventions are existing controls from [the earlier backtracking report](19-target-kl-backtracking.md); none was retrained. Deltas are percentage points versus canonical. Last-50 development statistics average the ten scheduled evaluations at global iterations 226–275. Other last-50 measures average the final 50 training iterations.

The complete epoch-level trace has 2,200 rows in [20-target-kl-epoch-transaction-trajectory.csv](20-target-kl-epoch-transaction-trajectory.csv). It records each scheduled actor epoch as committed, rejected, or not attempted after a rejection, and retains all per-iteration minibatch post-step KL and pre-clip actor gradient samples, action fractions, critic statistics, and validity counters.

### Five-method comparison

| Method / seed | Final dev delta | Best dev delta | Last-50 dev mean / min / max (n=10) | Decisions/game | Entropy | KL-to-init | Critic EV / value loss |
|---|---:|---:|---:|---:|---:|---:|---:|
| Original A1′ R1 | +6.348 | +11.510 | +5.265 / -0.218 / +9.094 | 131.25 | .5270 | 15.3621 | .9816 / .0212 |
| Minibatch guard R1 | +3.626 | +9.344 | +7.576 / +3.626 / +9.344 | 175.53 | .5786 | 20.0956 | .9859 / .0119 |
| Hard reject R1 | +9.771 | +10.103 | +3.888 / -1.038 / +10.103 | 154.09 | .6185 | 14.1513 | .9826 / .0146 |
| Backtracking R1 | +8.280 | +10.164 | +8.381 / +6.762 / +10.164 | 228.91 | .6918 | 25.0804 | .9890 / .0097 |
| **Epoch transaction R1** | **+7.408** | **+7.408** | **+4.091 / -0.471 / +7.408** | **186.27** | **.7837** | **19.3913** | **.9883 / .0095** |
| Original A1′ R2 | +11.934 | +11.934 | +8.166 / +6.028 / +11.934 | 228.71 | .6969 | 24.9702 | .9888 / .0102 |
| Minibatch guard R2 | +9.278 | +9.731 | +8.888 / +8.234 / +9.731 | 126.01 | .6089 | 12.8019 | .9805 / .0176 |
| Hard reject R2 | +9.603 | +9.603 | +8.889 / +8.269 / +9.603 | 120.92 | .4146 | 10.7327 | .9836 / .0161 |
| Backtracking R2 | +3.032 | +3.032 | +2.587 / +2.253 / +3.032 | 94.30 | .0172 | .2510 | .9822 / .0138 |
| **Epoch transaction R2** | **+11.934** | **+11.934** | **+8.166 / +6.028 / +11.934** | **228.71** | **.6969** | **24.9702** | **.9888 / .0102** |

Primary contrasts are epoch-transaction R1 minus original R1 = **+1.060 points** and epoch-transaction R2 minus original R2 = **0.000 points**. R2's trajectory and reported late-window measures reproduce Original A1′; every one of its 1,100 actor epochs committed.

For context, the Original A1′ minibatch-level target-KL stop counted 79 stops on R1 and 1 on R2. The epoch transaction instead rejected 2 whole actor epochs on R1 and 0 on R2, with no minibatch rejects.

### Epoch transactions, KL distributions, and minibatch comparison

| Seed | Actor epochs attempted | Committed | Rejected | Rejection rate | Rejected global iteration / epoch / final-rollout KL | Critic steps / scheduled minibatches |
|---|---:|---:|---:|---:|---|---:|
| R1 | 1,097 | 1,095 | 2 | 0.18% | 4 / 3 / .026452; 19 / 2 / .042065 | 29,040 / 29,040 |
| R2 | 1,100 | 1,100 | 0 | 0% | none | 34,848 / 34,848 |

Every rejected transaction restored the epoch-start actor and Adam state; subsequent actor epochs stopped in that iteration. All critic minibatches ran. The aggregate full-rollout KLs include every attempted epoch, including the two rejected R1 epochs:

| Seed / measure | n | Mean | p50 | p90 | p95 | p99 | Max |
|---|---:|---:|---:|---:|---:|---:|---:|
| R1 full-rollout epoch-final KL | 1,097 | .005711 | .005419 | .008328 | .009762 | .013474 | .042065 |
| R1 isolated minibatch post-step KL | 28,985 | .005278 | .004882 | .008730 | .010332 | .015458 | .186447 |
| R2 full-rollout epoch-final KL | 1,100 | .006696 | .006413 | .010318 | .011318 | .016486 | .019838 |
| R2 isolated minibatch post-step KL | 34,848 | .006511 | .005901 | .011007 | .013057 | .019201 | .234491 |

The local minibatch KL means are close to the epoch aggregate means, while their maxima are substantially larger. R1's full-rollout gate separated the two epochs above `.02`; the largest R2 full-rollout epoch KL remained `.019838`, although an isolated minibatch reached `.234491`. The gate therefore retained R2's strong path without minibatch rejection. The measures have different sample units: each minibatch value is a local post-step measurement; each epoch value re-evaluates the entire iteration rollout under one common epoch-final actor.

For the full-rollout samplewise distributions, the median across epochs of `(sample KL p50, p90, p95, p99)` was R1 `(.000141, .010500, .024110, .086608)` and R2 `(.000137, .011906, .028055, .103302)`. The largest samplewise KL across all epoch-final evaluations was `58.599` for R1 and `54.187` for R2. Median across epochs of sample log-ratio `(p50, p90, p95, p99)` was R1 `(0, .068640, .128217, .283254)` and R2 `(0, .069196, .135238, .307684)`; the maximum sample log-ratio was `4.155` and `4.082`. Each epoch's own p50/p90/p95/p99/max summaries are in the CSV.

### Last-50 entropy, KL, gradients, action mix, and critic

The performance table reports last-50 mean entropy, KL-to-init, critic explained variance (EV), and value loss. Last-50 mean clip fraction and actor gradient norms were R1 `.07061`, committed-step mean `.82577`, attempted pre-clip sample mean/max `.82627 / 11.92057`; R2 `.08133`, `.93676`, and `.93593 / 12.18375`. The gradient sample vectors preserve values from rejected attempts for audit, while the committed-step mean excludes rolled-back epochs.

| Action fraction (mean over final 50 rollouts) | R1 | R2 |
|---|---:|---:|
| Build tower | .1291 | .1083 |
| Confirm card service selection | .0007 | .0063 |
| Continue | .3208 | .3819 |
| Discard treasure | .0002 | .0011 |
| Place tower | .0381 | .0007 |
| Purchase shop item | .0665 | .0539 |
| Remove tower | .0163 | .0216 |
| Reroll | .2159 | .2539 |
| Select card service card | .0008 | .0063 |
| Select treasure | .0153 | .0126 |
| Start defense | .1291 | .1083 |
| Use inventory item | .0672 | .0450 |

Across all 550 training iterations, both seeds had zero illegal actions, action mismatches, nonfinite rollout values, fallback actions, and nonfinite update skips. R1 and R2 critic schedules completed all `29,040/29,040` and `34,848/34,848` minibatches, respectively.

## Classification and selection bias

**A — success on these development seeds.** R1's final development delta improved by `1.060` points over Original A1′, with two high aggregate-KL epochs rolled back. R2 had zero epoch rejections and exactly retained the Original A1′ strong trajectory. No minibatch actor updates were rejected, and no R2 epoch starvation occurred.

R1/R2 are algorithm-development seeds and are subject to selection bias. This result does not establish a baseline. No final seed was used, and no prospective validation on new seeds was run.
