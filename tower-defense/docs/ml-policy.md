# Managed Semantic Policy

`td-simulator` trains and uses a single policy family: the semantic-action
policy (policy-v2 "A1′": Top8 candidates, normalized inputs, learned kind head,
PPO with a transactional target-KL update). Training, simulation and balance
all run through `GameEnvironment`.

## Commands

| Command | Behavior |
| --- | --- |
| `ml train [--iterations N] [--config FILE] [--threads N]` | Resume, transfer or bootstrap, then run PPO. Default is 20 iterations per call. |
| `ml status` | Print the active policy and provenance. |
| `simulate` | Run games with the active policy and record statistics. |
| `balance` | Tune stage enemy HP using the active policy. |

`--checkpoint DIR` on `simulate` and `balance` overrides the active policy.

## Artifact layout

Everything lives under `simulator/artifacts/ml/`:

```text
active.json                    selected policy manifest
runs/run-NNN/bootstrap/        BC and critic bootstrap (train data is deleted after use)
runs/run-NNN/ppo/              ppo.json and iter-NNNN/ (iterations 0, best and latest kept)
```

`active.json` points at the iteration with the best development mean terminal
clear rate. It is republished after every iteration.

## `ml train` decision

1. No run exists (or the last bootstrap was interrupted): bootstrap a new run
   from canonical expert games (behavior cloning plus critic pretraining),
   then PPO.
2. The latest run matches the current game config: resume it.
3. Numeric config changes (costs, damage, HP, and similar): start a new run
   warm-started from the best checkpoint of the previous run.
4. Structural changes (new item, treasure or card service): the old policy
   cannot be carried over, so a new run is bootstrapped from scratch.

## Reset

```sh
rm -rf simulator/artifacts/ml
```

## Memory

PPO collects 48 episodes per iteration; peak RSS is about 6 GB.
