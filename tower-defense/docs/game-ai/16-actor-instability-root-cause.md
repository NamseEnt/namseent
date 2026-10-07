# A1′ Actor Instability: Root-Cause Analysis

**Date:** 2026-10-04  
**Scope:** Existing original R0/R1/R2 artifacts and existing LR-half R1/R2 artifacts; exact diagnostic replay through iteration 30. This is a diagnosis only. No new training recipe, hyperparameter, policy architecture, reward, or action space was tested or changed.

## Executive finding

The strongest supported mechanism is an **R1-specific high-sensitivity update in the shared candidate scorer**. With the LR-half recipe, R1 starts moving away from initialization more sharply than the other runs, while its action entropy stays near its initial low value. By iteration 14 the update-level approximate KL spikes; at iteration 15 the pre-clip actor gradient becomes large in every processed minibatch and is almost entirely in the shared candidate scorer. The learned family head is not the gradient source. R2 at the same LR does not enter this regime.

A frequent one-candidate family, `start_defense`, has the largest observed family-level ratio/KL in modified R1 iteration 15. It is not a rare sampled action: it appears repeatedly across the minibatches, and its advantage magnitudes are not exceptional relative to other families. The evidence therefore points to a seed-conditioned shared conditional-score update, expressed strongly through the `start_defense` family, rather than a single rare high-advantage sample. The measurements do not isolate per-family parameter gradients, so the `start_defense` factor cannot yet be established as the initiating cause.

## 1. Earliest observed divergence

All values below are keyed by the `iteration` field in `ppo.json` (the iteration-0 initialization/evaluation row is excluded). Entropy values are PPO update entropy; action rates and decisions/game are from that iteration's rollout.

| Run | Iter 1 KL-to-init | Iter 1 entropy | Iter 1 actor grad | Iter 5 entropy | Iter 10 entropy | Iter 14 update KL / stop | Iter 15 grad / clip | Iter 15 Continue / Reroll | Iter 15 decisions/game |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Original R1 | 0.0023 | 0.0168 | 0.106 | 0.0380 | 0.1150 | 0.0035 / no | 0.493 / 0.043 | 0.267 / 0.078 | 115.0 |
| Original R2 | 0.0623 | 0.0142 | 0.083 | 0.0677 | 0.1164 | 0.0041 / no | 0.502 / 0.033 | 0.176 / 0.083 | 93.1 |
| LR-half R1 | **0.1780** | 0.0163 | 0.101 | **0.0374** | **0.0356** | **0.0995 / yes** | **16.796 / 0.103** | **0.142 / 0.072** | **90.8** |
| LR-half R2 | 0.0207 | 0.0152 | 0.141 | 0.0354 | 0.0418 | 0.0011 / no | 0.570 / 0.006 | 0.140 / 0.069 | 83.4 |

The first detectable split is already at **iteration 1**: LR-half R1's post-update KL-to-init is 0.178, versus 0.0023 for original R1 and 0.0207 for LR-half R2. The entropy values at iteration 1 are all low, so low absolute entropy alone does not identify the failure. By iteration 5, original R1 and both R2 trajectories have begun growing entropy, while LR-half R1 remains near 0.03–0.04 through iteration 13. The first major update-level KL event is iteration 14 (0.0995, target stop); the first extreme gradient is iteration 15 (mean 16.796, max 63.916). The action mix and decisions/game have not yet collapsed at that point. The reported late-run reductions in Continue/Reroll and decisions/game are downstream changes, not the first signal.

Dev clear rate is evaluated every five iterations. At iteration 15 it is 36.567 for LR-half R1, 32.751 for original R1, and 36.600 for LR-half R2. There is no per-iteration dev observation between those evaluations, so the data cannot time a dev-performance change more finely than that cadence.

The complete iteration 1–40 aligned history, including per-iteration action-kind fractions, dev/rollout clear rate, critic metrics, entropy, KL, clipping, gradients, and stop status, is in [the trajectory CSV](16-actor-instability-root-cause-iteration-1-40.csv).

## 2. Temporal ordering

Observed order in LR-half R1:

1. **Iteration 1:** post-update KL-to-init is already 0.178. Initial rollout entropy is low across all seeds; R1 is not yet distinguished by entropy alone.
2. **Iterations 2–13:** LR-half R1 entropy remains about 0.028–0.038 and its KL-to-init stays roughly 0.05–0.17. Actor gradient is intermittently elevated early (0.952 at iteration 2 and 0.996 at iteration 3), but not yet an extreme sustained spike. Original R1 entropy grows to 0.115 by iteration 10 and 0.199 by iteration 13; LR-half R2 also starts to grow.
3. **Iteration 14:** update approximate KL rises to 0.0995 and target-KL early stop fires. Clip fraction is 0.055 and mean actor gradient is 1.198. KL-to-init remains only 0.0684: the within-update displacement is much larger than the policy's measured displacement from initialization at that checkpoint.
4. **Iterations 15–16:** update approximate KL is 0.1143 then 0.2574; clip fraction is 0.103 then 0.157. Mean pre-clip actor gradient is 16.796 then 14.298. Entropy remains low (0.0463 then 0.0430). The iteration-15 action mix remains close to iteration 14; Continue is 0.142 and Reroll 0.072.
5. **Later:** the policy continues to show repeated target stops and elevated gradients; late action frequencies and decisions/game fall. The new data do not support those late changes as the initiating event.

R2 provides a same-LR contrast: through iteration 15 its entropy is 0.0436, update KL 0.0015, clip fraction 0.011, and mean gradient 0.570, with no target stop. The different response under identical LR/config is seed-conditioned; it is not explained by LR alone.

## 3. Factor and action-family measurements

The actor used by these A1′ runs factors action probability into a learned **family/kind probability** and a **conditional candidate probability**. Candidate selection is produced by the shared candidate scorer. For these top-8 candidate runs, the spatial-cell head receives no samples (zero legal spatial cells and zero spatial-head gradient in the inspected iteration-15 replay). BuildTower candidate identity includes its subset/slot/rank tuple. CardService and treasure actions are separate families. `remove_tower` was not sampled in the inspected iteration-15 rollouts.

The table summarizes per-family sample-weighted means over all processed iteration-15 minibatches. “Max batch p99/max ratio” is the largest p99 or maximum reported by any individual minibatch for that family; it is **not** a pooled global percentile. Advantage p99 below is the largest absolute p99/min value seen in any one minibatch. See [the full factor CSV](16-actor-instability-factor-i15.csv) for old/new joint, family, and conditional log-probabilities, deltas, entropies, ratio summaries, clipped fraction, raw/normalized advantage summaries, candidate cardinalities, action IDs, and BuildTower subset/slot/rank counts.

| Run / family | Samples (minibatch appearances) | Mean joint approx-KL | Mean joint ratio | Max batch p99 / max ratio | Clipped fraction | Max batch abs-advantage p99 |
|---|---:|---:|---:|---:|---:|---:|
| Original R1 / Continue | 5,892 | 0.0030 | 1.017 | 1.88 / 1.88 | 0.031 | 0.829 |
| Original R1 / StartDefense | 3,904 | 0.0000 | 1.000 | 1.00 / 1.00 | 0.000 | 0.765 |
| LR-half R1 / **StartDefense** | **924** | **0.3777** | **1.445** | **9.18 / 9.18** | **0.290** | **0.636** |
| LR-half R1 / Continue | 620 | 0.0328 | 1.055 | 3.24 / 3.24 | 0.106 | 0.619 |
| LR-half R1 / PurchaseShopItem | 510 | 0.0206 | 0.976 | 1.03 / 1.03 | 0.033 | 0.656 |
| LR-half R1 / Reroll | 312 | 0.0068 | 1.033 | 1.82 / 1.82 | 0.054 | 0.533 |
| LR-half R1 / SelectTreasure | 106 | 0.0687 | 0.999 | 2.39 / 2.39 | 0.179 | 0.532 |
| LR-half R2 / StartDefense | 3,660 | 0.0000 | 1.000 | 1.00 / 1.00 | 0.000 | 0.714 |
| LR-half R2 / Continue | 2,224 | 0.0000 | 1.000 | 1.00 / 1.00 | 0.000 | 0.757 |
| LR-half R2 / Reroll | 924 | 0.0029 | 1.018 | 1.49 / 1.49 | 0.029 | 0.806 |

StartDefense is a one-candidate family in these masks; in LR-half R1 its mean total legal-candidate count was 18.5. Its frequency (924 sample appearances across 18 minibatches) and moderate advantage range do not fit a “one rare outlier sample” explanation. The same one-candidate family in LR-half R2 had unchanged family probability/ratio in the inspected update. Thus cardinality is a possible sensitivity context, but cardinality alone does not explain the seed contrast.

Factor-specific entropy shows the modified R1 distribution is already concentrated across early iterations. By iteration 15 its rollout family entropy is 0.0191 (vs 0.1715 original R1); its conditional candidate distributions are also low-entropy for many families. The large iteration-15 StartDefense movement is primarily the **family factor**: its conditional candidate choice is deterministic because only one candidate is legal. By comparison, LR-half R2 has rollout family entropy 0.0233 and no comparable StartDefense ratio movement. These values do not establish that family entropy fell before the first KL-to-init split: the iteration-1 KL difference is earlier than any clear entropy divergence.

## 4. Gradient source and minibatch concentration

Iteration-15 pre-clip actor gradient decomposition:

| Run | Processed minibatches | Mean / max total norm | Minibatches >1 / >5 / >10 | Shared scorer mean norm (max) | Shared scorer mean squared-norm share | Family head mean squared-norm share | Spatial head |
|---|---:|---:|---:|---:|---:|---:|---:|
| Original R1 | 88 | 0.493 / 1.072 | 5 / 0 / 0 | 0.485 (1.065) | 96.43% | 3.57% | 0 |
| LR-half R1 | 18 | **16.796 / 63.916** | **18 / 13 / 7** | **16.796 (63.916)** | **99.9986%** | **0.0014%** | 0 |
| LR-half R2 | 64 | 0.570 / 1.677 | 10 / 0 / 0 | 0.570 (1.676) | 99.97% | 0.03% | 0 |

The gradient spike is **not confined to a few minibatches**: all 18 LR-half R1 minibatches exceed norm 1; 13 exceed 5. Global max-norm clipping at 0.5 therefore scales every one of those minibatch gradients, with the shared candidate scorer supplying virtually all of the pre-clip norm. The learned family head is too small to account for the explosion. The spatial cell head is inactive in this candidate mode. In original R1, the same shared scorer is the largest group, but its norm remains ordinary; LR-half R2 likewise has no extreme minibatches.

This decomposition is by parameter group, not by action-family contribution to each parameter gradient. The family diagnostic gives per-family loss and ratio contributions, but it does not run a separate backward pass for every family. Therefore it identifies where the gradient lands (shared scorer), and which family factor moves most (StartDefense), but not a causal family-to-parameter attribution.

## 5. Target-KL implementation semantics

`PpoLearner::update` computes the sampled PPO approximate KL for each minibatch **before that minibatch's optimizer step**, using the old behavior log-probability and current new log-probability. The actor backward pass and optimizer step then execute for every minibatch in the current epoch. Only after the complete epoch does the code divide the accumulated minibatch KL by the number of actor minibatches and compare the epoch mean with the configured target using strict `>`.

When the epoch mean is over target, training skips **subsequent epochs of the current update**. It cannot undo or prevent the optimizer steps already applied in the crossing epoch. `UpdateStats.approx_kl` is the mean over all actor minibatches actually processed across the update, so it can substantially exceed the 0.02 threshold. `early_stopped` records that at least one epoch boundary triggered the stop. This is epoch-level PPO early stopping and is consistent with that intended semantics; it is not a per-minibatch safety cap.

The LR-half R1 iteration-14 update reports approx-KL 0.0995 and early stop after one epoch. That result means the completed epoch already moved past target before the code had a point where it could stop later epochs. The iteration-15 diagnostic similarly has large ratios/gradients within the epoch that completes before the stop.

A focused unit test was added for completed-epoch averaging and strict-threshold behavior: `target_kl_stop_uses_completed_epoch_mean_and_strict_threshold`. It passed in release mode. This test covers the stop-decision helper; source inspection confirms the helper is called after the inner minibatch loop.

## 6. Critic and technical validity

Critic metrics do not lead the divergence in the inspected window. At iterations 1–20, rollout explained variance stays approximately 0.96–0.985 across all runs; value losses remain roughly 0.013–0.035. At iteration 15, LR-half R1 EV is 0.9816 and value loss 0.0185, within the same range as original R1 (EV 0.9750, value loss 0.0176) and LR-half R2 (EV 0.9740, value loss 0.0173). No nonfinite values, illegal actions, or action mismatches were reported in the inspected artifacts/replays. The current evidence does not indicate a critic failure preceding the actor event.

## 7. Deterministic diagnostic replay and implementation

Existing artifacts did not contain action-factor and parameter-group gradient details. Diagnostics were added behind `TOWERDEFENSE_PPO_ACTOR_DIAGNOSTICS`, with no behavior-changing branch when the variable is unset. Three release-binary replays resumed the exact iteration-10 checkpoint state and existing optimizer state, preserving the run configs, deterministic seed derivation, and LR for each run; they stopped at iteration 30:

- Original R1 (LR 0.0003)
- LR-half R1 (LR 0.00015)
- LR-half R2 (LR 0.00015)

The replayed iterations 11–30 exactly match the existing history for rollout clear rate/entropy/family entropy/decisions/transitions, update loss/entropy/KL/clip/grad/epoch/stop fields, and KL-to-init. Thus the instrumentation recorded the same policy trajectory. It did not extend any run to 275 iterations.

## 8. What the evidence supports and what remains unknown

**Most strongly supported:** seed-specific batches and advantages drive a high-sensitivity update in the shared candidate scorer. LR-half R1 is already unusually displaced from initialization after its first update, then remains in a low-entropy regime. Near iteration 14, the update KL surges; by iteration 15, scorer gradients are huge across all minibatches. The learned family head and critic are not the source of the measured gradient spike.

**Not established:** whether early low entropy causes the sensitive scorer updates or is an early symptom of them; whether `start_defense` initiates the update or amplifies a state-dependent scorer change created by other families; whether a specific state/mask-cardinality pattern is decisive; and per-family gradients into shared scorer parameters. The iteration-1 KL split is too early for the available per-iteration entropy aggregates to determine direction of causality.

No intervention is selected or recommended in this report.
