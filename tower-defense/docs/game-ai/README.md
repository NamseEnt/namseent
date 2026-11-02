# Game AI documentation

This index separates the supported simulator workflow from implementation
contracts and historical evidence. Historical reports describe the code,
configuration, and seeds used at the time; they are not current performance
claims.

## Routine workflow

From `tower-defense/simulator/`:

```sh
cargo build --release
target/release/td-simulator ml train        # resume, transfer or bootstrap
target/release/td-simulator ml status
target/release/td-simulator simulate --samples 100 --all-stats
target/release/td-simulator balance
```

See [the managed policy guide](../ml-policy.md) for behavior and limits. The
historical reports below mention the removed `ml phase4` commands; the code
paths they describe are now the managed `ml train` pipeline. Development-only
teacher commands require `--features diagnostics`.

## Maintained contracts

| Document | Purpose |
| --- | --- |
| [Action contract](02-action-contract.md) | Legal policy actions and construction candidates |
| [Observation contract](03-observation-contract.md) | Public state and model input boundaries |
| [Evaluation contract](08-evaluation.md) | Metrics, seed splits, and approval criteria |
| [Candidate architecture review](11-candidate-architecture-review.md) | Rationale for the current candidate/joint-action representation |
| [Rollout teacher](05-rollout-teacher.md) | Teacher behavior still referenced by simulator code |
| [Batch simulation statistics](23-batch-simulation-statistics.md) | SQLite recording and statistics interface |

## Historical implementation evidence

These reports support specific retained code paths or reproduce past results.
Their metrics are scoped to their recorded revisions and configurations.

- [Phase 4A behavior cloning and distillation](13-phase4a-bc-distillation.md)
- [Phase 4B semantic PPO](14-phase4b-ppo.md)
- [Policy v2](15-policy-v2.md)
- [Target-KL epoch transaction experiment](20-target-kl-epoch-transaction.md)
- [Effect-input retraining study](25-effect-inputs-and-retraining-study.md)
- [Construction and content adaptation evidence](26-construction-and-content-retraining-plan.md)

Architecture decisions are recorded in [decisions/](decisions/).
