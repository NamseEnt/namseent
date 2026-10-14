# Effect inputs and checkpoint retraining study

This records the 2026-10-11 study. It follows [24](24-construction-and-content-adaptation.md), which found unchanged treasure selection after effect removal. Progress is terminal game progress on a 0–100 scale, not victory probability. The current input contract and usage are documented in [26](26-construction-and-content-retraining-plan.md) and [27](27-retraining-quickstart.md).

## Implementation at the time of the study

- Core observation schema 10 exposes actual item healing, shield, reroll and extra-tower values, owned upgrade and current shop/treasure effects, runtime parameters, and the authoritative `black_white_enabled` flag. Hidden future draw/shop order remains private.
- `effect_inputs.rs` produces variable-length state/candidate effect sets for tower splash triggers, status values and duration, resulting tower templates, and current content. Effects retain their owner and are not silently truncated.
- `raw_effects` and `normalized_effects`, policy representation 4, and zero-initialized residual adapters connect these inputs to actor and critic. Tests cover initial output preservation, changed inputs, gradients, learned output changes and save/load.
- Stable parameter IDs preserve optimizer resume. With effect inputs and temperature 1.7, resumed 1+1 iterations and continuous 2 iterations produced identical actor/critic files.
- Explicit transfer verifies audited historical encoder hashes and provenance. Ordinary checkpoint loading still checks configuration, rules and representation contracts.
- Scratch controls use independently seeded weights of the same architecture, not source weights. Both transfer and scratch start fresh Adam optimizers.

Supported effect-input transfer uses Top8 or all-build-options with full actor PPO. New effect meanings still require authoritative Core implementation and suitable observation/encoder support. Indexed runtime parameters do not automatically describe arbitrary new mechanics. Results apply to the recorded configurations, not every current default balance.

## Treasure preparation

A temperature-3 PPO pilot alone did not resolve saturated Black/White selection. The `content_teacher` preparation evaluates every offered treasure with common resampled future scenarios and scripted continuation. It does not ban a treasure, add an ID-specific reward, or inspect the game's actual hidden future RNG.

Source/target games supply labels for a treasure-specific candidate adapter while base weights are frozen. Full actor/critic PPO follows. Collection games, scenario continuations, semantic decisions and time are reported separately; scratch receives the same preparation budget. A regression test confirms that preparation leaves shop/item/build outputs unchanged. Preparation is not repeated on resume.

An earlier shared-candidate-adapter prototype changed item behavior and missed the progress goal in two of three effect runs. Its 15 development runs are retained in `effect-adaptation-20261011/development-diagnostic.json`; no final seeds were used. The final study separated the treasure adapter.

## Fixed protocol

Source: `artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200`.
Final study: `artifacts/retraining/effect-adaptation-isolated-20261011/`.

| Setting | Value |
| --- | --- |
| Numeric target | Starting gold 100 → 105 |
| Effect target | `black_white_enabled: true` → `false` |
| Initialization | Reused weights / independently seeded scratch |
| Independent runs | 3 per target/initialization, plus 3 unchanged-effect transfer controls |
| PPO budget | 16 games × 4 iterations, 2 update epochs |
| Effect preparation | 16 source + 16 target games; 4 scenarios/candidate; 32 adapter epochs |
| Development seeds | 4,000,000–4,000,031 |
| Development selection | Highest mean progress, latest iteration breaks ties |
| Development goal | Mean progress ≥40 |
| Final seeds | 7,000,000–7,000,127; first use, excluded from development/training |

Final evaluation comprised 1,408 original-effect games, 1,408 disabled-effect games and 1,024 numeric-target games: **3,840 terminal games**. Illegal actions, fallback and post-sampling mutations were zero. No policy won a full game.

Equal budgets refer to direct PPO game count. Episode length, decision count, preparation and execution time are reported separately. Source history was 13,200 PPO games, 2,194,669 decisions and 19,669 seconds; historical BC/teacher costs were unavailable. The zero-shot baseline is the transferred `source-converted/iter-0000` before new PPO updates.

## Final results

[Machine-readable results, budgets and hashes](effect-adaptation-isolated-20261011-results.json) include per-run data, costs, resume evidence and artifact hashes.

| Target | Reuse mean progress (run range) | Scratch mean progress (run range) | Paired difference / 95% CI | Development goal |
| --- | --- | --- | --- | --- |
| Gold 100 → 105 | 48.48 (48.38–48.54) | 31.71 (31.42–32.12) | +16.77 / [15.05, 18.61] | Reuse 3/3; scratch 0/3 |
| Black/White removed | 45.40 (43.98–46.57) | 31.81 (31.67–32.00) | +13.59 / [11.68, 15.48] | Reuse 3/3; scratch 0/3 |

Intervals use 4,000 hierarchical bootstrap resamples of training repeats and common held-out game seeds. Three repeats do not establish superiority across other recipes.

The source already exceeded the progress-40 goal immediately after the change: 48.53 for gold and 44.65 for effect removal. This demonstrates reuse of prior training, not a measured PPO learning-speed multiplier. Effect-transfer changes versus zero-shot were +0.99, +1.92 and −0.68; gameplay did not improve uniformly.

### Treasure selection

Rates are selections divided by offers in the disabled-effect environment.

| Policy | Selected / offered | Selection rate | Unchanged-effect training control |
| --- | --- | --- | --- |
| Source | 69 / 69 | 100.0% | — |
| Disabled-effect transfer 1 | 58 / 67 | 86.6% | 87.9% |
| Disabled-effect transfer 2 | 18 / 70 | 25.7% | 100.0% |
| Disabled-effect transfer 3 | 33 / 65 | 50.8% | 65.7% |

Combined selection was **109/202 = 54.0%**. Each run's paired game bootstrap interval for the decrease versus source excluded zero. These are repeated evaluations of common seeds, not 384 independent final seeds.

Compared with unchanged-effect training, decreases were clear in runs 2 and 3; run 1's interval included zero. For the same model's matched first-choice conditional probabilities, disabling the effect decreased preference in runs 1 and 2 but increased it in run 3. Thus saturation was reduced, but consistent effect-sensitive behavior across repeats was not demonstrated. Conditional family probabilities and actual offered-choice rates are distinct measurements.

### Cost and inference

| Target | Time to development goal, including preparation/evaluation | Scratch budget exhausted without goal |
| --- | --- | --- |
| Numeric | 6.0–7.5 s | 36.9–61.3 s |
| Effect removal | 81.6–99.0 s | 100.4–132.5 s |

Numeric transfer reached the goal at initial evaluation; effect transfer reached it after preparation, before PPO. Scratch was censored at the budget limit, so no exact time-to-goal multiplier is claimed. The first two numeric jobs shared CPU with compilation/tests. The JSON accounts for the 64 PPO games/run, preparation, prototypes and pilots separately.

Forward time was 0.381–0.385 ms for numeric models and 0.429–0.433 ms for effect models. Separate one-thread, four-game probes measured process peak RSS of 60–85 MiB and 82–99 MiB. These include game/model/SQLite memory, not isolated adapter allocation.

## Reproduction

Run from `tower-defense/simulator/`. Original configurations and recipe are checked in under `configs/retraining-study-20261011/`. Checkpoints and raw artifacts remain local. Use a new study directory for a new run; existing final output is immutable and used final seeds must not become another study's final set.

```sh
cargo build --release --bin td-simulator
python3 scripts/run_adaptation_study.py plan \
  --study artifacts/retraining/effect-adaptation-isolated-20261011 \
  --pilot configs/retraining-study-20261011/recipe.json
python3 scripts/run_adaptation_study.py train \
  --study artifacts/retraining/effect-adaptation-isolated-20261011 --workers 2 --threads 5
cargo build --release --example replay_content_teacher_cost
python3 scripts/run_adaptation_study.py account \
  --study artifacts/retraining/effect-adaptation-isolated-20261011
python3 scripts/run_adaptation_study.py benchmark \
  --study artifacts/retraining/effect-adaptation-isolated-20261011 --threads 1
python3 scripts/run_adaptation_study.py evaluate \
  --study artifacts/retraining/effect-adaptation-isolated-20261011 --threads 6
python3 scripts/summarize_adaptation_study.py \
  --study artifacts/retraining/effect-adaptation-isolated-20261011 \
  --output ../docs/game-ai/effect-adaptation-isolated-20261011-results.json
```

The runner records the resolved release executable, command line and hash, resumes completed checkpoints with the same spec, and refuses to overwrite final evaluation. Stop after `train` if only reproducing training. Later representation changes require their explicit transfer path; this command sequence describes the recorded study and is not a claim that old final artifacts satisfy every later strict contract.

## Validation at the time of the study

Core: 200 passed. Simulator release library: 426 passed, 20 ignored, 0 failed. Repairs covered stale enemy-HP fixtures, actual critic sample count, a legal single-candidate entropy fixture and the previous teacher 32-position regression limit.

Four completed checkpoint-driven games also produced 4 simulation rows, 853 events and 100 stage results in SQLite. Progress on seeds 4,000,000–4,000,003 matched the terminal evaluator within floating-point tolerance. Current submission validation is recorded separately in [26](26-construction-and-content-retraining-plan.md).
