# Checkpoint retraining quickstart

Updated 2026-10-14. Construction exploration and compatible content transfer are implemented. These commands exercise the ordinary production-checkpoint path; short smoke models are not replacements for a performance-qualified production policy.

## Requirements and evidence

| Requirement | Verified behavior | Limits |
| --- | --- | --- |
| Construction beyond scripted proposals | Every legal card-subset/tower choice, full actor PPO and outside-Top8 selections | Opt-in; scripted representative placement; no long-term strength claim |
| Balance/effect changes | Source actor/critic transfer, fresh optimizer, new-configuration training and deterministic resume | Same representation or an explicit supported migration |
| Appended treasures/items/card services | Actual Core integration, transfer, short training, resume and inference for one of each | Experimental fixtures reuse existing semantics, not arbitrary new mechanics |
| Most-selected treasure ablation | Black/White selection fell from 69/69 to 109/202 combined | Preparation plus PPO; repeat/control sensitivity was inconsistent |

See [implementation and integration evidence](26-construction-and-content-retraining-plan.md) and [controlled study results](25-effect-inputs-and-retraining-study.md). Large model/database/raw-run artifacts remain local and ignored; the controlled study [result JSON](effect-adaptation-isolated-20261011-results.json) is checked in.

## Ordinary build and production source

Run from `tower-defense/simulator/`:

```sh
cargo build --release --bin td-simulator
target/release/td-simulator ml phase4 retrain \
  --spec configs/production-all-options.json --threads 1
```

The checked-in [complete spec](../../simulator/configs/production-all-options.json) uses:

- Source: `artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200`.
- Recorded source configuration: `configs/retraining-study-20261011/original.jsonc`, also used as target in this smoke.
- `candidate_mode: "all_build_options"`, `effect_inputs: true`, full actor/critic updates, no teacher preparation.
- Four games, one iteration, actor LR 1e-5, critic LR 3e-4, fresh Adam and a separate output run.

Source checkpoints, parent PPO/BC metadata and historical source Git objects must be available locally. They are not distributed in Git. Spec paths are relative to the spec file, not the shell working directory.

Every legal construction option reaches the model; each has one scripted representative legal cell. Omitting `candidate_mode` retains the source mode. Widening changes probability normalization, so it does not preserve the source action distribution before learning.

The source is Top8/observation 9/entity 6/candidate 1/normalized, hidden 64. Explicit transfer validates provenance, known historical encoder hashes and the supplied source configuration before producing observation 11/entity 8/candidate 2/normalized-effects. No intermediate checkpoint is required for this supported source.

Default catalogs are 37 treasures, 11 items and 16 card services. No experimental-content feature is needed for this path.

### Recorded ordinary end-to-end smoke

The 2026-10-14 run under `artifacts/retraining/production-all-options-20261014/` completed this path before submission cleanup:

- Four games, 881 decisions, 16 accepted actor and critic updates.
- Rollout/update/development time 2.397/13.701/0.765 seconds.
- Outside-Top8 build choices: 74.359% sampled and 83.333% greedy.
- Illegal/mismatch/fallback/truncation counts zero.
- The saved checkpoint ran one game and generated one completed SQLite simulation, 155 events and 17 stage results.

The source actor SHA-256 is `b1a7d7bb655fe49ef4ab3916fb3ea23cb5e0234b43c17a4e991cb1a085088cd4`; critic is `86c4decf135e278f36c1ee108af8fd101e36cdcdc7f2331233fc7deeffc48641`. The configuration digest is `3541ca79abe7ad98cc849242a884b613ddd7fee4f4a3e8ccfab17e6c287f4ed4`.

The checked-in spec was also executed during final PR validation, using a separate output path and training seeds 9,700,000–9,700,003. That run produced 701 decisions, 93.651% sampled outside-Top8 builds, and one completed statistics game with 245 events and 27 stage results. Development selection favored iteration 0; the saved iteration 1 was separately loaded for the smoke. Neither run establishes a performance gain. Built-in development seed 4,000,020 belongs to the previously used development split; it is not a fresh final or independent performance sample. The earlier separate statistics smoke used seed 5,000,000. No final split is needed for a functionality smoke.

## Changing balance or compatible content

Copy the spec, set `target_config` to the changed JSONC, and choose a new `run_dir`. Keep `source_config` equal to the source's actual configuration. Train from source weights with fresh optimizers; do not resume an old run under new rules.

For a compatible content kind, implement its authoritative Core behavior and observation/vocabulary mapping, append stable IDs, and preserve existing meanings and model dimensions. Transfer checks catalog prefixes and representation contracts. Registering an ID does not mean the policy understands its value without training.

New effect meanings, action structures, dimensions, ID reorderings or arbitrary encoder changes require an explicit migration and suitable observation/encoder support. Direct checkpoint loading retains strict checks rather than silently treating such changes as compatible.

## Resume the same run

Keep the same spec, target rules, input/candidate contract, seeds and `run_dir`. Increase `iterations` to the desired total and call the same retrain command.

Preserve:

- Run `ppo.json`, migration/configuration metadata.
- Latest complete iteration's `actor.bin`, `critic.bin`, `ppo-actor.json`.
- Both optimizer files, `actor-optimizer.bin` and `critic-optimizer.bin`.

Resume loads the complete actor/critic and both Adam states. It does not rerun source transfer or teacher preparation. New configuration or candidate changes belong in a separate run.

## Checkpoint inference and statistics

After the checked-in one-iteration spec completes:

```sh
target/release/td-simulator simulate \
  --checkpoint artifacts/retraining/production-all-options-pr-validation/run/iter-0001 \
  --config configs/retraining-study-20261011/original.jsonc \
  --samples 1 --seed-start 5000001 --threads 1 \
  --db artifacts/retraining/production-all-options-pr-validation/checkpoint-stats.db \
  --fresh-db --all-stats
```

`simulate` loads the checkpoint's candidate/input mode and records completed games using the existing SQLite statistics. `--fresh-db` replaces the selected output database: use a new path or omit it when appending to a database you want to retain. This command is a smoke, not a fresh held-out performance study.

For terminal policy comparisons, inspect `ml phase4 terminal-eval --help`; final evaluation requires an explicit confirmation and suitable unused reserved seeds. Do not repeat historical final evaluation as a new independent result.

## Experimental content fixtures

Only the integration examples TrainingMelon, ShieldBerry and CardPolisher require:

```sh
cargo build --release --bin td-simulator --features experimental-content
```

They append one treasure, item and service, producing catalogs 38/12/17. Their checkpoints require that catalog and must not be used as ordinary default-content checkpoints. The default-feature preflight against an experimental source was correctly rejected.

Card-service context identifies the pending service and selection conditions. Explicit effect/result inputs cover seven ordinary polish/suit services plus experimental CardPolisher. Other services retain identity/selection context and an unsupported-effect flag; no complete semantic representation is claimed.

## Tests and reproducibility

Run the ordinary regression suites and feature integration suite:

```sh
cargo test --manifest-path ../core/Cargo.toml
cargo test --manifest-path ../core/Cargo.toml --features experimental-content
cargo test --release -- --test-threads=1
cargo test --release --features experimental-content -- --test-threads=1
cargo clippy --all-targets --features experimental-content -- -D warnings
cargo fmt -- --check
```

Development teacher probes and retained-local-checkpoint tests are explicitly ignored. To reproduce one, supply its documented artifacts/environment variables and run the exact test name with `--ignored --exact --nocapture`. Ordinary tests include synthetic Core/encoder/gradient and transfer/resume regressions without requiring local experimental checkpoints.

Submission preparation fixed the debug MessagePack checkpoint decoder's stack overflow with an 8 MiB dedicated loading stack. The original debug resume test passes on ordinary test-worker stacks. Current final counts are recorded in [26](26-construction-and-content-retraining-plan.md), separately from historical experiment validation.
