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

- Numeric configuration changes.
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

## Validation status

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

## Adaptation status

Checkpoint transfer and continued training after supported configuration or
content changes are implemented. The Black/White ablation showed no change in
treasure selection after retraining, so it does not establish behavioral
adaptation. Teaching a policy to respond to changed effects, and evaluating that
adaptation against a scratch-trained control with independent runs, remain
follow-up research. No experiment-specific effect toggle or result-generation
script is part of the simulator.
