# Construction exploration and content retraining: implementation and evidence

Updated 2026-10-14. This separates the original functional requirements from optional strategy-quality studies. [27](27-retraining-quickstart.md) is the operational guide; [25](25-effect-inputs-and-retraining-study.md) records the controlled treasure experiment.

## Original acceptance criteria

1. Construction chooses cards and a resulting tower. Every legal construction choice must be available to learning; scripted representative placement is allowed.
2. Existing checkpoints must support continued training after compatible balance/content changes, with correct save/resume and explicit representation migration where needed.
3. Remove or weaken the most-selected treasure and measure whether retraining reduces its offered-choice selection rate.

Beating Top8, proving a context-dependent ranking reversal, generalizing to unseen states, adapting with PPO alone, and automatically understanding arbitrary new mechanics are separate questions. They are not additional functional acceptance criteria.

## Current support

| Requirement | Implementation and evidence | Limits |
| --- | --- | --- |
| Construction outside scripted proposals | `all_build_options` enumerates every legal card-subset/tower choice and uses each choice's scripted best legal position. Full actor PPO remains trainable. Production transfer and checkpoint statistics executed successfully. | Opt-in. Candidate widening changes normalization immediately; long-term strength is unverified. |
| Numeric/existing-effect changes | Explicit transfer of source actor/critic into a new configuration/run; fresh Adam. Same-run resume restores both optimizers. | Same representation or a specifically supported migration. |
| Appended treasure | Experimental TrainingMelon Core effect, stable IDs, transfer/training/resume/inference. | Reuses existing effect semantics; not a general new-mechanic guarantee. |
| Appended item | Experimental ShieldBerry Core use/consumption, actual PPO exposure/use, transfer/resume/inference. | Short smoke, not demonstrated item strategy. |
| Appended card service | Experimental CardPolisher purchase/target/confirmation, actual PPO exposure and completion, transfer/resume/inference. | Effect semantics are explicit for polish/suit services only. |
| Treasure ablation | Black/White effect removal: source 69/69 versus transferred models 109/202 combined. | Preparation plus PPO; controls and same-model effect sensitivity were inconsistent across repeats. |

Historical snapshots below describe the code/contracts used by those runs. Weights, databases, binaries and raw logs are local ignored artifacts, not GitHub attachments. Stable study results are checked in as [JSON](effect-adaptation-isolated-20261011-results.json). Historical artifacts may need explicit transfer after later encoder changes.

## 1. Complete construction candidates

Core-generated legal card subsets and resulting tower slots were compared with the actor's construction choices, ignoring position. Fixtures checked exact set equality, no duplicate construction choices, legal representative positions, outside-Top8 coverage and unchanged nonconstruction actions.

| Fixture | Core / actor construction choices | Outside Top8 |
| --- | --- | --- |
| Ordinary hand, no extra tower | 31 / 31 | 29 |
| Ordinary hand, two extra towers | 93 / 93 | 91 |
| Mixed Spades/Clubs straight, Black/White, one extra tower | 62 / 62 | 61 |

The ordinary fixtures allowed only 1,201 of 1,536 starting map coordinates. The treasure fixture also verified the authoritative change in resulting tower kind.

Selected-card attributes and extra-tower templates/effects were connected through separate migration inputs. Entity schema 6/candidate schema 1 migrate explicitly to 7/2, preserving the base pooling path. Existing effect/migration adapters are preserved; only absent paths initialize at zero. Actual source-converted and learned-effect artifacts had actor/critic maximum output error 0 against a 1e-6 tolerance with the same Top8 candidate set. Tests also cover gradient flow and learned response to new inputs.

### Actual release resume smoke, 2026-10-13

Local evidence: `simulator/artifacts/retraining/release-smoke-20261013/release-smoke-summary.json`.

- Source: `effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004`.
- Target: effect inputs, all-build-options, same disabled-effect configuration; no teacher.
- Two iterations × four games, PPO seed 880013, game seeds 8,800,000–8,800,007, one thread.
- Actor LR 1e-5, critic LR 3e-4. An earlier 3e-4 actor diagnostic rejected every actor update under the unchanged KL guard and was retained separately.
- Rollout/update/development seconds: 2.37/6.45/1.59 and 1.92/5.02/1.60.
- Outside-Top8 sampled construction: 84.932% and 90.625%. Illegal/fallback/mismatch/mutation counts were zero.
- Continuous and 1+1 resume runs had identical final actor, critic and actor metadata. All 33 actor and 25 critic Adam parameter states matched after stable-ParamId ordering; raw optimizer bytes differed in serialization order.
- Existing adapters were preserved and new migration adapters changed from zero. Final checkpoint inference succeeded, including a separate development seed 8,900,000. Observed process RSS was about 1.4 GiB.

## 2. Optional construction-context probe

Local evidence: `simulator/artifacts/retraining/construction-context-development-20261013/artifact-manifest.json`; reproduction code is `construction_context_study.rs` and `scripts/summarize_construction_context.py`.

The probe compared actual Core states with different treasure ownership or public draw/discard membership. It used common resampled futures and scripted continuation, not the actual hidden future RNG.

- Treasure changed the full-hand result from Straight to StraightFlush. Eight-scenario mean terminal progress changed 45.75→49.00 for the full hand and 22.875→31.75 for outside-Top8 candidate `[13]`.
- Four paired scenarios had broad 95% t intervals: [−6.55, 12.55] and [−28.98, 43.98]. No stable candidate rank reversal was established.
- Moving card 46 from draw to discard changed public deck inputs but all 31 candidates had identical measured outcomes in the paired probe.
- Actual actor probability L1 changes were 0.05720 for treasure and 0.00000141 for deck. These are forward sensitivity, not newly learned strategy.
- Total continuations: 1,792, below the 2,000 cap; teacher time 86.08 seconds. No labels, fitting or checkpoint update followed because value evidence was insufficient.

This optional experiment does not block the original functional requirements. Its expensive probes and retained-checkpoint actor probe are explicitly ignored in ordinary test runs and remain available for manual reproduction with `--ignored`.

## 3. Appended-content integration

All three examples use `experimental-content`, append stable IDs and reuse existing effect/action semantics. Default catalogs remain 37 treasures, 11 items and 16 card services. Experimental catalogs contain 38/12/17. Experimental checkpoints cannot be used by a default build that lacks those kinds.

### TrainingMelon treasure

Local evidence: `simulator/artifacts/retraining/training-melon-20261013/experiment-summary.json` and `content-migration-report.json`.

- Core raw ID 37, observation ID 38, candidate categorical ID 39; existing IDs retained.
- Acquisition increases maximum HP by 8,000 raw and heals 12,000 raw, using existing Watermelon semantics.
- Actual acquisition fixture: HP 30,000→42,000; maximum HP 60,000→68,000. Observation carries the new ID and MaxHP effect.
- Source: `release-smoke-20261013/active-continuous/iter-0002`; target: `run-final-freshdev/iter-0002`.
- Two iterations × four games, train seeds 9,100,000–9,100,007, PPO seed 910013. Development used 4,000,002–4,000,004, not novel held-out seeds.
- Iterations generated 574 and 497 transitions; rollout/update time 1.82/4.75 and 1.35/3.75 seconds. No treasure offers occurred in those PPO rollouts.
- A deterministic actual-input fixture demonstrated embedding gradient and update under entropy loss. This is connectivity evidence, not treasure-value learning.
- Iteration 1 resume, iteration 2 strict inference and existing-weight preservation were checked. The new kind uses a reserved embedding row rather than resizing the table.

### ShieldBerry item

Local evidence: `simulator/artifacts/retraining/shield-berry-20261013/`.

- Core raw ID 11; observation/ML item ID 12; use-candidate categorical ID 13 includes the existing padding offset.
- Existing Shield effect: +12,000 raw shield. Actual `CoreSession` use kept HP 50,000, changed shield 0→12,000 and inventory size 4→3, consuming the item.
- Source: TrainingMelon `run-final-freshdev/iter-0002`; target: ShieldBerry `run/iter-0002`. Iteration-0 actor/critic bytes matched source; learned adapters and existing embedding rows were preserved.
- Two iterations × four games, PPO seed 920013, train 9,200,000–9,200,007, development 4,000,005–4,000,007. Rollout/update time 1.60/4.88 and 1.54/4.19 seconds.
- New-item input/candidate/use counts: 0/0/0 in iteration 1 and 35/35/1 in iteration 2. These inputs reached PPO batches.
- Saved Adam steps for all 33 actor and 25 critic states resumed 6→12. Final checkpoint inference and TrainingMelon regression passed.
- Separate entropy-gradient evidence is connectivity only; the observed use does not establish a learned strategy.

### CardPolisher service

Local evidence: `simulator/artifacts/retraining/card-polisher-20261014/experiment-summary.json`.

- Core raw ID 16, observation/ML vocabulary ID 17, candidate categorical row 18.
- Common rarity, base price 45 gold with existing price variation. Existing enhancement behavior adds 3,000,000 raw polish without changing suit.
- Actual purchase fixture: gold 100→55, card 0 polish 0→3,000,000, purchased-slot state set, pending service cleared on confirmation. A seeded shop probe also purchased at the authoritative varied price 67.
- Actor controls purchase, target-card selection and confirmation. Scripted code enumerates candidates, not target choices.
- Source: ShieldBerry `run/iter-0002`; target: CardPolisher `run/iter-0002`. Existing weights/adapters and catalog prefixes were preserved, with fresh optimizers.
- Two iterations × four games, PPO seed 930013, train 9,300,000–9,300,007, development 4,000,008–4,000,009.
- 1,366 transitions; 23 shop exposures, four purchase-candidate decisions, two purchases, two target selections/completions and four PPO input examples.
- Rollout/update time 2.59/6.91 and 1.77/4.73 seconds. Both optimizer step sets resumed 6→12. Reserved row 18 changed by L2 0.0001572 after PPO.
- Final shop/target inference, prior treasure/item inference, and a 128-decision release load/forward smoke passed. No strategy-quality claim follows.

## 4. Card-service context and effect inputs

Local evidence: `simulator/artifacts/retraining/card-service-context-20261014/experiment-summary.json`, `source-target-contract.json` and `attempt-005/`.

The original target-selection inputs were not completely aliased: typed shop rows retained the purchased service ID. However, pending service context, effect meaning and predicted card result were not explicit inputs.

Separate zero-initialized residual paths now carry:

- State: stable service ID, selection stage/counts, legal-candidate count, progress and filter.
- Effect: supported kind, authoritative polish delta and optional suit replacement.
- Candidate result: current card suit/rank/engraving/polish and predicted polish/suit.

Actor reads state and candidate paths; critic reads state/effect context. These paths do not alter existing pooling or adapter inputs. Core behavior and descriptors share the same authoritative enhancement definition.

| Service | Explicit effect input |
| --- | --- |
| LongSword / Staff / Mace / ClubSword | +2,000,000 raw polish; Spades / Clubs / Hearts / Diamonds |
| Brush | +3,000,000 raw; Hearts; face-card legality remains authoritative |
| FountainPen | +3,000,000 raw; suit unchanged; number-card legality |
| Tricycle | +2,000,000 raw; suit unchanged; Ace/2/3 legality |
| CardPolisher (experimental) | +3,000,000 raw; suit unchanged |
| Eraser, MagicWand, Pliers, Screwdriver, Copier, Magnet, Cactus, SpinningTop, Battery | ID/selection context; effect meaning unsupported and explicitly flagged |

This is seven ordinary polish services plus experimental CardPolisher, not all services. Effect kinds are 0 unsupported, 1 polish delta, 2 polish delta plus suit replacement.

Source was CardPolisher `run/iter-0002`, observation/entity/candidate 10/7/2. Target is 11/8/2. Explicit transfer verifies the eight recorded source encoder hashes. Existing parameters/adapters are preserved; absent service residuals initialize to zero with stable ParamIds. Same-candidate actor/critic maximum error was 0, within the predeclared 1e-6 tolerance, for shop, target selection and construction states.

Controlled actor cross-entropy to candidate 0 and critic MSE to value 1 confirmed gradients, parameter changes and exact save/load. These arbitrary targets test connectivity, not service value.

### Attempts and final short run

| Attempt | Outcome |
| --- | --- |
| Root preflight | Default binary rejected the experimental source catalog before training. |
| 002 | Migration preflight remains; no completed PPO checkpoint. Separate exit cause cannot be reconstructed from retained logs. |
| 003 | Projection-fixture correction; short PPO, seed 940113, train block 9,500,000. Candidate-projection validation needed correction. |
| 004 | Before explicit effect-kind row, seed 940114, block 9,510,000. Input-contract development run. |
| 005 | Final context contract; seed 940115, train 9,520,000–9,520,007, development starting 4,000,014. |

Seeds/blocks changed alongside input/projection corrections. These attempts are development history, not independent performance repeats.

Attempt 005 used full actor/critic, effects/all-options, four games × two iterations, actor LR 1e-5, critic LR 3e-4 and no teacher. Iteration 1 was resumed normally.

| Iteration | Transitions | Service-context decisions | CardPolisher target/completion | Actor/critic context batch examples | Rollout/update seconds |
| --- | --- | --- | --- | --- | --- |
| 1 | 711 | 6 | 1 / 1 | 12 / 12 | 2.46 / 6.75 |
| 2 | 475 | 2 | 1 / 1 | 4 / 4 | 1.53 / 4.02 |

Stable service-parameter optimizer steps resumed 6→10. Final shop/target/build inference and TrainingMelon/ShieldBerry regressions passed. Binary/proc records are retained with the attempt.

## 5. Ordinary production checkpoint: complete execution

Local evidence: `simulator/artifacts/retraining/production-all-options-20261014/execution-record.md`, `retraining-spec.json` and `run/migration.json`.

A default-feature release build transferred production `phase4b/kl-epoch-transaction-r2-p2/iter-0200` directly, without an intermediate checkpoint. The source's parent PPO/BC provenance, known historical encoder hashes and supplied configuration digest were checked.

- Source contract: Top8, observation 9/entity 6/candidate 1, normalized, hidden 64.
- Target: all-build-options, observation 11/entity 8/candidate 2, normalized-effects, default catalogs 37/11/16.
- Actor SHA-256: `b1a7d7bb655fe49ef4ab3916fb3ea23cb5e0234b43c17a4e991cb1a085088cd4`.
- Critic SHA-256: `86c4decf135e278f36c1ee108af8fd101e36cdcdc7f2331233fc7deeffc48641`.
- Source configuration digest: `3541ca79abe7ad98cc849242a884b613ddd7fee4f4a3e8ccfab17e6c287f4ed4`, using the retained `original.jsonc` for both source and target.
- PPO seed 960013, train seeds 9,600,000–9,600,003, four games, one iteration, four epochs, no teacher, fresh Adam.
- 881 decisions, 16 accepted actor and critic updates. Rollout/update/development time 2.397/13.701/0.765 seconds. Illegal/mismatch/fallback/truncation counts were zero.
- Outside-Top8 construction was 74.359% sampled and 83.333% greedy.
- Completed `run/iter-0001` ran with the same ordinary binary and seed 5,000,000, generating one completed SQLite simulation, 155 events and 17 stage results.

Built-in development seed 4,000,020 had been used before and is not called fresh. The separate statistics smoke used previously unused, nonreserved seed 5,000,000. No final split was used. This demonstrates the operational path, not policy strength.

## Submission validation

Earlier full simulator validation was interrupted by debug checkpoint-decoder stack overflow and expensive development probes. PR preparation reproduced the overflow in the generated MessagePack/Burn record decoder and moved policy checkpoint loading to an 8 MiB dedicated stack. The original debug resume regression now passes without increasing the test runner's global stack.

Terminal teacher probes and tests requiring retained local experiment checkpoints are explicitly ignored for ordinary clean-checkout test runs. They remain runnable by exact name with `--ignored`; required environment variables and local files still apply. Synthetic Core, encoder, gradient and transfer/resume regressions remain ordinary tests.

Final PR validation completed on 2026-10-14:

| Suite | Passed | Failed | Ignored |
| --- | ---: | ---: | ---: |
| Core default | 200 | 0 | 0 |
| Core experimental | 203 | 0 | 0 |
| Simulator release default | 439 | 0 | 24 |
| Simulator release experimental | 444 | 0 | 31 |
| Native game app | 324 | 0 | 0 |

The original debug checkpoint-resume regression also passed. Core all-target/all-feature Clippy and simulator all-target Clippy (default and experimental) passed with `-D warnings`. Formatting, diff checks, app compilation and Python script syntax checks passed.

Submission repairs also made catalog-boundary fixtures feature-aware, checked the critic workflow's selected validation epoch rather than assuming the final epoch is best, stopped rollout-prefix fixtures at terminal states, and pinned decimal parsing inputs instead of stale live-balance HP constants. The service gradient/save-load test now uses a seeded synthetic model and runs without local checkpoint files. The pre-cleanup schema-11 encoder manifest has a narrowly audited explicit transfer path; unknown manifests remain rejected.

The checked-in production spec was rerun after the policy/effect lint cleanup: four games, 701 decisions, sampled outside-Top8 93.651%, greedy outside-Top8 83.333%, and one completed checkpoint-driven statistics game. SQLite contained 1 simulation, 245 events and 27 stage rows. Development selection chose initialization (iteration 0); iteration 1 was separately loaded for this functionality smoke. This is not evidence of strength improvement.

[Submission commands, log hashes and smoke provenance](results/2026-10-14-submission-validation.json) record these results. App tests used local linker aliases to the installed Fontconfig/FreeType runtime libraries because development linker symlinks were absent; no system package was changed. Earlier counts in historical sections are not final submission counts.
