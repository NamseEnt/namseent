# Fixed-Seed PPO Policy

The `simulator` feature contains the observation-conditioned PPO policy.
Training, validation, and checkpoint simulation use the same
`GameEnvironment` execution path.

## Seed Contract

Training and validation ranges are inclusive. Seeds are materialized in
ascending order and passed directly to `GameEnvironment`; they are not hashed
or remapped through batch episode indices. The ranges must be disjoint. Their
canonical list digests are stored in the checkpoint and checked on resume.

## Policy Contract

Observations use global features plus typed DeepSets entity sets. Card ownership
and hand/draw/discard composition are exposed with variable cardinality; draw
and discard order is not exposed. Count-aware pooling preserves multiplicity,
and actor and critic receive the same global and typed state. Each legal
candidate is encoded by action kind and target entity features, and the
distribution is normalized only over the current legal candidate set.

The critic receives the state descriptor. PPO uses GAE, normalized advantages,
the clipped policy ratio, entropy regularization, value loss, Adam, and norm
gradient clipping. Training and held-out validation call the same environment
rollout collector.

## CLI

Build or run the new CLI with the `simulator` feature:

```text
cargo run --features simulator --bin td-simulator -- ml train \
  --checkpoint ml_policy_checkpoint.json \
  --train-start 0 --train-end 999 \
  --validation-start 18446744073709550616 \
  --validation-end 18446744073709551615
```

Training and validation report iteration and rollout progress in the terminal.
Rollouts run on the CPU with Rayon. `--threads 0` uses Rayon defaults; pass a
positive `--threads` value to select the worker count explicitly:

```text
cargo run --features simulator --bin td-simulator -- ml train \
  --iterations 10 --threads 16
```

Each PPO progress line reports separate train-rollout, optimization, and
validation-rollout times. Use release builds for throughput measurements;
debug builds substantially distort environment and tensor overhead:

```text
cargo run --release --features simulator --bin td-simulator -- ml train \
  --train-start 0 --train-end 255 \
  --validation-start 18446744073709551360 \
  --validation-end 18446744073709551615 \
  --iterations 1 --ppo-epochs 1 --max-decisions 100 \
  --minibatch-size 256 --threads 16
```

`--minibatch-size` controls the maximum number of rollout steps in each PPO
update. Steps are bucketed by legal-candidate count, so padding cannot assign
probability to illegal actions. Record the three phase times and calculate
episodes per second before changing batch size or model dimensions. The current PPO optimizer still builds one loss
graph per rollout step; minibatch PPO with candidate masking is a separate
optimization and must preserve legal-action probabilities.

For large seed sets, use `--rollout-chunk-size` to bound peak rollout memory:

```text
cargo run --release --features simulator --bin td-simulator -- ml train \
  --train-size 512 --validation-size 256 \
  --rollout-chunk-size 32 --minibatch-size 32
```

The train and validation seeds are still all evaluated. A positive chunk size
streams that many episodes at a time and updates PPO after each train chunk;
this trades some optimizer semantics for bounded memory. `0` preserves the
single full-rollout behavior.

For the WGPU streamed learner, use `--rollout-step-budget` to bound the number
of decision steps retained by one PPO update window. Episodes are kept intact,
so a window may exceed the budget by one received chunk. Use
`--max-queued-steps` to limit producer-ahead work; when it is `0`, the queue
limit defaults to the update-window budget. The learner consumes each chunk
immediately, releases its step permits after the update, and clears the step
payload while retaining episode metadata. `--minibatch-size` remains an
independent GPU activation/gradient memory control and is never changed by
adaptive retry.

To save and resume the full trainer state, use `--run-dir` with
`--resume-auto`:

```text
cargo run --release --features simulator --bin td-simulator -- ml train \
  --run-dir artifacts/ml/overnight \
  --resume-auto \
  --train-size 512 --validation-size 256 \
  --rollout-chunk-size 16
```

The run directory stores model weights, Adam state, trainer metadata, and an
atomic `latest.json`. Re-running the command continues from the latest
completed iteration. Without `--resume-auto`, `--run-dir` still writes trainer
checkpoints but starts a fresh training state.

For unattended overnight training, omit `--iterations`. The command then runs
one iteration at a time, saves the trainer state, and automatically resumes the
next iteration from `latest.json`. Stop it with `Ctrl+C` after the current
iteration reaches its checkpoint boundary:

```text
cargo run --release --features simulator --bin td-simulator -- ml train \
  --run-dir artifacts/ml/overnight \
  --train-size 512 --validation-size 256 \
  --rollout-chunk-size 16 --threads 8
```

Specify `--iterations N` for a finite run of exactly `N` PPO iterations. The
`ml train` command without `--iterations` uses the default auto run directory
and continues until interrupted. `--resume-auto` remains available for finite
runs that should load and update the trainer state in `--run-dir`.

The ML backend is the CPU `burn-flex` backend. Use the physical CPU core count
for `--threads` when running long training jobs.

The WGPU learner reuses the default checkpoint when its seed schedule, reward
configuration, model size, and ML contract match. An incompatible or damaged
checkpoint is reported and replaced through the scripted behavior bootstrap;
it is never silently treated as a fresh random run. The default CLI
`damage_progress_weight` is `0.0`, matching `RewardConfig`, so dense damage
shaping must be explicitly enabled for an ablation.

In auto-resume mode, the checkpoint's best validation score and best model are
also carried across the one-iteration WGPU process boundaries. A later
iteration that regresses on held-out validation therefore rolls back to the
previous process-wide best instead of overwriting the artifact with a weaker
policy. The saved current iteration is also advanced across those boundaries;
this changes the rollout exploration seed on each restart instead of replaying
the same stochastic batch indefinitely.

The WGPU path also enforces a minimum validation clear-rate floor of `10%`.
Candidates below this floor cannot become the best model, and an existing
checkpoint below the floor is treated as an invalid bootstrap artifact. If a
run never reaches the floor, it fails without writing a new weak checkpoint;
this prevents silent degradation of the policy artifact.

`--wgpu-stream-rollout` consumes episode chunks concurrently with rollout
production. With a positive `--rollout-step-budget`, advantages are normalized
within each complete-episode window and PPO updates it before the next window
is admitted. This bounds retained CPU trajectory memory while preserving GAE
episode boundaries; choose the window and minibatch sizes together when
comparing validation results.

BC bootstrap prioritizes card selection, treasure, card-service, and inventory
actions over frequent `Continue` samples without increasing the number of
batches. PPO also gives a bounded emphasis to positive advantages from the
highest-clear-rate episodes in the current rollout. Stale trajectories from
older iterations are not replayed directly because their old policy
probabilities no longer match PPO's on-policy ratio; auto-resume instead
carries the best model and advances the exploration iteration. The seeds of
above-average training episodes are also stored in WGPU checkpoint metadata;
the next rollout replaces at most one eighth of its seed slots with those
seeds, preserving rollout size while increasing exposure to high-clear-rate
trajectories.

Use `--resume PATH` to continue from a current-schema checkpoint. Resume requires
the same environment contract and exact seed ranges. Use
`--require-improvement` to reject a resumed run unless its held-out clear rate
is strictly greater than the previous best.

Validation reloads both files and can persist a provenance report:

```text
cargo run --features simulator --bin td-simulator -- ml validate \
  --checkpoint ml_policy_checkpoint.json \
  --run-id run-1 --output artifacts/ml/run-1-validation.json
```

The JSON checkpoint stores contract, hyperparameters, seed schedule, and best
validation metrics. The matching `.mpk` file stores the Burn model record.
The report stores episode returns, termination reasons, full-clear count,
partial-clear aggregates, stage quantiles, truncation breakdown, throughput,
seed digest, config/schema provenance, checkpoint SHA-256, and git revision.
Full clear count is compared first; mean partial progress is compared second;
fewer truncations are used as the final tie-break.

After each PPO iteration, a validation regression does not become the next
iteration's rollout policy. The trainer restores the best validation model and
its Adam state, while the monotonic optimizer-step counter remains in the
trainer metadata. Trainer checkpoints store current and best model/optimizer
records separately. The trainer checkpoint schema must be incremented when
this resumable state changes.

## Contract and version matrix

The following values are part of the reproducibility contract:

| Contract           | Current source of truth                                        | Change requires                                               |
| ------------------ | -------------------------------------------------------------- | ------------------------------------------------------------- |
| Environment        | `ENVIRONMENT_VERSION` in `src/simulator/environment.rs`        | environment observation or transition semantics change        |
| Action             | `ACTION_SCHEMA_VERSION` in `src/simulator/environment.rs`      | `AgentAction`, legal-action ordering, or FSM semantics change |
| Trajectory         | `TRAJECTORY_SCHEMA_VERSION` in `src/simulator/trajectory.rs`   | trajectory serialization changes                              |
| Dataset            | `DATASET_SCHEMA_VERSION` in `src/simulator/ml/contract.rs`     | training dataset format changes                               |
| Features           | `FEATURE_SCHEMA_VERSION` in `src/simulator/ml/contract.rs`     | feature meaning, count, or normalization changes              |
| Policy checkpoint  | `NEURAL_CHECKPOINT_SCHEMA_VERSION` and `POLICY_SCHEMA_VERSION` | model/checkpoint metadata changes                             |
| Entity encoder     | `ENTITY_ENCODER_SCHEMA_VERSION` in `src/simulator/ml/model.rs` | typed entity or pooling input changes                         |
| Trainer checkpoint | `TRAINER_CHECKPOINT_SCHEMA_VERSION`                            | resumable model/optimizer state changes                       |
| Game configuration | `config_version` and `config_digest`                           | authoritative game balance/configuration changes              |
| RNG                | `RNG_ALGORITHM_VERSION`                                        | gameplay random sequence semantics change                     |

Training and inference reject incompatible environment, action, trajectory,
feature, configuration, RNG, and checkpoint contracts. A configuration digest
override is explicit and does not bypass schema or RNG validation. Existing
checkpoint files are not migrated across feature or action semantics; a new
contract requires a new training run.

The current policy combines fixed-size global features with variable-cardinality
typed DeepSets for cards, towers, monsters, shop entries, inventory, upgrades,
and route entities. The actor scores the current legal candidate set, while the
authoritative game rules remain in `GameEnvironment` and `PlayerCommand`.
Owned treasure upgrades are represented through the owned-upgrade entity set and
their effect on the observation; card-service changes are visible through the
deck, draw, discard, and card-service state. Card engravings and tower template
inputs are exposed on both state entities and applicable candidates, allowing
the actor to learn tower/enchantment and deck/treasure interactions without
assigning a speculative immediate value to a treasure choice.

## Reproducible final evaluation

Run at least three independent training runs after all P0-P2 gates pass. Keep
the same held-out seed schedule, decision budget, configuration, and checkpoint
schema for every run. Save each validation JSON with its run ID, seed digest,
git revision, checkpoint path, checkpoint SHA-256, and config digest. Compare
paired seed results using full clears, mean partial progress, and truncation in
that order; inspect stage p25/p50/p75/p90 and catastrophic regressions before
declaring an improvement.

## Action and reward contract

Card selection is represented as a simulator-local FSM:

1. begin reroll or tower selection;
2. select or deselect individual hand cards;
3. confirm or cancel the selection;
4. commit through the existing `PlayerCommand`.

Tower placement is split into tower selection and legal coordinate selection.
The placement coordinate candidates continue to use the authoritative route
validation logic. Defense item windows are exposed before defense and after
survivable damage ticks; fatal damage does not open a response window.

The default reward configuration preserves the original scales:

- terminal win: `+1.0`;
- terminal loss: `-1.0`;
- escaped HP penalty denominator: `50.0`;
- player damage penalty denominator: `60.0`.

Clear rate and full clear are the primary training objective. Immediate
selection and placement signals are intentionally bounded and secondary:
tower quality and route-length changes may help exploration, but stage damage
and damage near effective towers are evaluated through authoritative delayed
combat metrics. Treasure selection has no standalone immediate value; its
long-term effect is learned from later stage and terminal outcomes. Any new
shaping term must be evaluated by ablation and remain small relative to the
terminal clear outcome.

Potential-based progress shaping is opt-in. Its component is:

$$
r_{potential}=w\left(\gamma\Phi(s')-\Phi(s)\right)
$$

where `potential_weight` defaults to `0.0`, so the default policy objective is
unchanged. Terminal handling and truncation semantics must be tested whenever
the potential configuration is enabled. The reward configuration is stored in
the neural checkpoint and incompatible configurations are rejected.

For GAE, terminal transitions do not bootstrap. Truncated transitions bootstrap
from the next value estimate. A decision-limit truncation is recorded on the
last transition with `StepReason::MaxDecisions`; a no-progress cycle has its
own reason.

Normal cancel and deselect actions have no direct penalty. A sparse
`no_progress_cycle_penalty` is applied exactly once on a confirmed
`NoProgressCycle`. Terminal and cycle transitions do not bootstrap GAE;
`MaxTicks` and `MaxDecisions` truncations do.

Rollout diagnostics retain named selection counts, immediate inverse actions,
selected-set revisits, Begin→Cancel repetitions, cycle penalty totals,
reason-based bootstrap counts, and reward-component sums in PPO iteration
history. Compact iteration logs use named metrics instead of opaque action
slots.

To evaluate a checkpoint across explicit simulation seeds, use:

```text
cargo run --features simulator --bin td-simulator -- simulate \
  --checkpoint ml_policy_checkpoint.json \
  --samples 1000 \
  --db sim_results.db
```

`simulate` always loads `ml_policy_checkpoint.json` by default. Use
`--checkpoint PATH` only when evaluating a different checkpoint. There is no
heuristic fallback; a missing or invalid checkpoint is an error.

Simulation rejects a config digest mismatch by default. Pass
`--allow-checkpoint-config-change` to allow only that digest change; schema,
environment, action, and RNG contract fields remain strict. The decision is
stored with each simulation's SQLite provenance.
