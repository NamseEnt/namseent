# Construction exploration and content adaptation

## Scope

The October 9 follow-up adds two requirements to batch simulation and existing
`all-stats` reporting: learn construction outside scripted proposals, and reuse
checkpoints after content changes. Balance optimization and human-player modeling
remain deferred.

Construction means choosing cards and the resulting tower option. Placement
means choosing its map cell. `all_build_options` enumerates **every legal
(card subset, tower option)** pair and uses the scripted best legal cell for each.
It does not discard construction options by heuristic score. The complete PPO
actor, including treasure/shop/card-service choices, stays trainable. This differs
from the older `build_option_a1_marginal` experiment, which froze other choices.

The model sees its existing contextual observation: owned/hand/draw/discard cards,
owned treasures, inventory, towers, monsters, shop and candidate card identities.
The wider candidate set reuses existing tensors and actor/critic weights, with
fresh Adam state. Widening changes probability normalization and can change
behavior before training; it does not promise initial policy equivalence or an
immediate strength improvement.

## Use

Run from `tower-defense/simulator/`:

```sh
cargo build --release --bin td-simulator
target/release/td-simulator ml phase4 retrain --spec retrain.json --threads 6
```

Add `"candidate_mode": "all_build_options"` to the [retraining spec](22-numeric-retraining.md).
Omit it to retain the source checkpoint's candidate mode. A saved run resumes
with the same mode and configuration; changing either starts a separate run.
The result remains usable by `simulate --checkpoint RUN/iter-NNNN --all-stats`.

## Supported content changes

- Numeric configuration changes and the authoritative
  `treasures.black_white_enabled` rule switch.
- Core behavior changes with the same observation/action representation, through
  an explicit new retraining run. Ordinary evaluation and resume retain their
  rules/configuration checks.
- Appended treasure, item and card-service kinds with stable existing raw IDs,
  registered engine behavior and vocabulary, and the same input/action schema.
  New checkpoints store the catalog and encoder fingerprints. The shared entity
  embedding already has 4096 rows; catalog extensions do not resize parameters.
- Historical checkpoints require their original configuration and source Git
  objects. The Top8-to-all-options adapter pins both source and target candidate
  file hashes. Convert historical checkpoints to the new contract before adding
  content; arbitrary historical encoder differences are not auto-approved.

Changing existing IDs, numeric field meanings, observation/action structure or
model dimensions requires a specific representation migration. Newly invented
mechanics must first be implemented in the authoritative core. The current actor
uses IDs and a limited effect representation; richer splash/status/trigger inputs
remain planned work. Checkpoint reuse does not mean the policy understands a new
mechanic without experience.

## Treasure ablation protocol

- Source: latest production checkpoint
  `artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200` (iteration 200).
  Later tiny retraining checkpoints are technical smoke tests.
- Probe: development seeds 4000000–4000127. `black_white` was selected most often:
  **63 selections / 63 offers**. Lock the target before training.
- Remove its black/red suit-equivalence effect with
  `"treasures": {"black_white_enabled": false}` in JSONC. Keep the kind collectible,
  its rarity, reward draw procedure and inventory slot consumption unchanged.
  The rendered game and simulator share this rule in the authoritative core.
- Use the source's original balance configuration for both arms, isolating the
  treasure change from subsequent default enemy-HP changes.
- Paired arms: disabled-effect retraining and unchanged-effect retraining control.
  Each uses source actor/critic, fresh Adam, PPO seed 83, training seeds starting
  at 5200000, 48 rollouts/iteration, 2 epochs/iteration, 20 iterations (960
  rollouts per arm). Keep the source learning rates, entropy and KL recipe.
- Evaluate every 5 iterations on 128 development games. Production checkpoint
  selection uses mean progress. For the behavioral experiment compare **iteration
  20** in both arms at the same rollout budget; never select by treasure rate.
- Validation seeds 6000000–6000255 are reserved before evaluation, disjoint from
  training/development and the earlier final sets. Evaluate source, control and
  retrained policies under both original and disabled-effect configurations.
- Report picks/offers, first-treasure picks/offers, progress, illegal/fallback
  counts and paired game-cluster bootstrap intervals. Raw pick counts alone are
  affected by how long a policy survives and which treasures it sees.

A training/evaluation interruption preserves the last complete checkpoint.
The original 512-decision cap rejected a normal stage-43 control episode;
it was raised to 10000 and both arms resumed from completed checkpoints.
Aborted iteration work is excluded from recorded budgets. An audit found two
truncated, bootstrapped training rollouts in the disabled arm and one in the
control arm during the early cap phase: respectively 958 and 959 of the 960
rollouts reached terminal. All final validation games reached terminal. Treat
this run as an exploratory diagnostic, not a fixed-cap efficiency experiment.

## Validation status

- The authoritative Black/White disable test passes and matches the tower
  outcome without that treasure, while retaining its inventory identity.
- All legal construction pairs, scripted representative placement and removal
  of the frozen A1 probability projection are checked together.
- Weight reuse, fresh optimizer state, deterministic resume and all-options
  transfer/resume are covered by the retraining workflow test.
- Statistics tests cover trace/streaming equivalence, per-run reports and
  exclusion of incomplete records. Four development games match the existing
  terminal evaluator's progress values within float serialization precision.
- Final serial simulator library suite: **416 passed, 8 failed, 20 ignored**.
  All 8 failures also reproduce on `origin/master` (`acb2cabc`),
  whose suite has 28 failures; the card-service legality fixes remove the other
  20. One BC roundtrip/resume precision test also failed once in parallel, then
  passed in isolation and in the serial suite. Existing failures include five stale
  enemy-HP expectations, two PPO fixture assumptions and a teacher candidate fixture. Targeted new tests pass separately.
- An initial statistics smoke accidentally reused four V2 final seeds
  (4200000–4200003). It was excluded from training, model selection and ablation
  analysis, and repeated on development seeds. No new held-out claim uses those
  historical final seeds.

## Experiment results

The requested treasure-selection adaptation **was not demonstrated**. On 256
fresh validation seeds in each environment, every policy selected `black_white`
whenever offered:

| Policy | Original effect picks/offers | Disabled effect picks/offers | First-stage picks/offers | Mean conditional selection probability |
| --- | --- | --- | --- | --- |
| Source | 136/136 | 136/136 | 47/47 | 99.85% |
| Unchanged-effect control, iteration 20 | 133/133 | 133/133 | 47/47 | 99.12% |
| Disabled-effect retraining, iteration 20 | 146/146 | 146/146 | 47/47 | 99.93% |

The first treasure occurs at stage 1, with the same 47 opportunities across
policies and environments; this comparison does not depend on later survival.
Probabilities are conditional on the SelectTreasure action family, rather than
unconditional joint action probabilities. First-opportunity Wilson 95% intervals
are [92.44%, 100%]. A unanimous empirical bootstrap degenerates at zero change;
it does not establish equal underlying preferences.

| Validation environment | Source mean progress | Control mean progress | Retrained mean progress |
| --- | ---: | ---: | ---: |
| Original effect | 49.22 | 48.69 | 53.17 |
| Disabled effect | 45.70 | 45.87 | 50.47 |

Disabled-effect retraining improved progress by 4.77 points over the source
(paired standard error 0.62), while treasure selection stayed at 100%. Progress
is not win probability; all policies recorded zero victories under this balance.
Illegal actions, fallback actions and post-sampling mutations were zero in all
validation games. Near-saturated source probabilities suggest exploration and
credit assignment as follow-up investigation; the experiment does not identify
the root cause. No independent training repeats or scratch-training comparison
were run, so faster adaptation is unverified.

The separate all-construction smoke resumed successfully across two iterations
and four rollouts, sampling outside Top8 in 69.05% and 86.84% of construction
choices. On four development games its selected initialization scored 41.35
progress versus the source's 46.77. This verifies exploration and checkpoint
reuse, not a stronger production policy. The treasure experiment retained Top8
to isolate the content change; all-options construction remains opt-in.

[Machine-readable results and artifact hashes](results/2026-10-09-treasure-ablation.json)
include rollout budgets, truncations, PPO settings, source identity and test
status. Large raw evaluation files and model weights remain local artifacts.

## Reproduce the comparison

The local experiment directory is
`artifacts/retraining/treasure-ablation-20261009/`. It contains the original and
disabled JSONC configurations, both retraining specs and all evaluation inputs.
With the source artifacts available, run each spec using the command above.
From `tower-defense/simulator/`, evaluate each environment (replace `original`
with `disabled` for the second invocation):

```sh
target/release/td-simulator ml phase4 \
  --config artifacts/retraining/treasure-ablation-20261009/original.jsonc \
  terminal-eval --split retraining-validation --count 256 \
  --allow-checkpoint-config-change \
  --policy source=artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200 \
  --policy control=artifacts/retraining/treasure-ablation-20261009/control-run/iter-0020 \
  --policy retrained=artifacts/retraining/treasure-ablation-20261009/disabled-run/iter-0020 \
  --compare retrained:source --compare retrained:control \
  --output artifacts/retraining/treasure-ablation-20261009/validation-original.json
python3 scripts/summarize_treasure_ablation.py \
  --baseline artifacts/retraining/treasure-ablation-20261009/baseline.json \
  --original artifacts/retraining/treasure-ablation-20261009/validation-original.json \
  --disabled artifacts/retraining/treasure-ablation-20261009/validation-disabled.json \
  --output artifacts/retraining/treasure-ablation-20261009/selection-summary.json
```

The summary script reproduces selection/progress aggregates and intervals. The
checked-in result additionally records training metadata and artifact hashes.
