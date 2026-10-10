# Goals and Acceptance Criteria

## Current implementation goals (2026-10-09)

Per the user's direction, the current scope is to run the AI through multiple simulator games and save and query metrics at the existing `all-stats` level. Balance changes and methodology work belong to the content and level-design phase. The learning and retraining goals below remain follow-up requirements.

Current completion criteria:

1. Run the current PPO/BC model to actual game termination across multiple seeds.
2. Collect existing progress distributions, damage, and item, treasure, and card-service selection and outcome statistics.
3. Use the existing SQLite records and `stats` query path, and record model, configuration, and seed provenance.
4. Keep illegal actions, policy errors, and limit hits out of normal game results.
5. Do not require human-like behavior, proof of optimality, a particular clear rate, or a new balance methodology for completion.
6. Allow construction to explore and learn legal card combinations outside the scripted top candidates. Placement may remain scripted.
7. Document checkpoint retraining after content changes and its supported scope. Changing the representation so a policy can understand a new effect, and verifying adaptation behavior, are follow-up work. See [24](24-construction-and-content-adaptation.md) for supported changes and the boundary for structural migrations.

See [`23-batch-simulation-statistics.md`](23-batch-simulation-statistics.md) for detailed scope and [`0010`](decisions/0010-collect-existing-simulator-statistics.md) for the decision.

## Long-term problem definition

The long-term deliverables are strong play under current game rules and a fast, convenient retraining path that reuses valid learning after design changes. Following the user's explanation on 2026-10-09, development assumes the current balance is not designed to permit a full clear. A full-clear rate is not a required acceptance criterion for the current AI. This is not a mathematical proof that a full clear is impossible. Human-like behavior, mathematical proof of optimality, and immediate adaptation to every balance version are not first-stage goals.

Game decisions depend on several interacting factors:

- Permanent card upgrades and engraving
- Tower combinations formed by cards
- Owned treasures and items
- Tower placement and existing tower combinations
- Path changes caused by placement and demolition
- Current and upcoming waves
- Resource use and long-term run value

Therefore, a policy that only maximizes poker-hand strength or immediate damage does not meet the goal.

## Play-performance research goals

Improve a preselected game outcome under a fixed balance configuration and held-out seed distribution. Under the current balance, mean progress over games that reach a real terminal state is the default primary metric. The existing artifact field `terminal_clear_rate` means progress from 0 to 100, not full-clear rate.

- Compare progress differences against heuristics and existing strong checkpoints using the same configuration and seeds.
- Also report stage reach rates, elimination distribution, median progress, and lower quantiles. Check that a few large gains do not hide regressions on many seeds.
- When full clears are possible, full-clear rate may be selected as the primary metric in advance. Do not switch to a more favorable metric after seeing results.
- Fix the metric definition and version, minimum improvement, and allowed regression before the experiment. Exact values will be set when a comparison is designed; no numeric acceptance threshold has been approved yet.

A reproducible improvement over a strong baseline is evidence of better AI performance, not proof of optimal play. AI failure alone does not show that a balance is impossible, and raw progress changes across balance versions must not be interpreted as AI improvement. See [`08-evaluation.md`](08-evaluation.md) for the evaluation contract and [`decisions/0009-balance-appropriate-evaluation.md`](decisions/0009-balance-appropriate-evaluation.md) for the decision.

## Retraining after design changes

After numeric tuning, adding item effects, or changing combat rules such as area attacks, the system should reuse valid learning from an existing checkpoint and adapt to the new rules. This remains a design requirement. Verify retraining effectiveness by change type after establishing a baseline checkpoint for comparison.

- Provide continued training from an existing model for numeric changes that preserve input and action meaning.
- For new items expressible as combinations of existing effects, expose effect type, target, and value to the policy, then verify whether the existing model can be reused.
- For changes that need new effects or actions, first extend the authoritative core, simulator, observations, and legal actions. Transfer compatible model components and train components needed for new inputs or outputs.
- Generate new rollouts under the changed rules. Do not treat trajectories, rewards, or teacher labels from the old rules as ground truth under the new rules.
- Preserve the source checkpoint and record its provenance and transfer details in a new training run. Distinguish resuming under the same rules from transferring a model after a rule change.
- Provide a repeatable procedure that selects the change and source checkpoint, checks compatibility, transfers the model, trains, and evaluates it. Reduce the manual work needed for each change.

Measure retraining success against a model trained from scratch under the changed rules. Record games, wall time, and setup work needed to reach a preselected target, and compare performance under a fixed budget using a preselected metric. The retraining validation contract is maintained in [`09-balance-experiments.md`](09-balance-experiments.md). Loading a checkpoint successfully does not by itself demonstrate retraining effectiveness.

## Separate learning signals from evaluation goals

The following values may be used for reward shaping or auxiliary targets to stabilize learning:

- Wave progress
- Remaining HP and shield
- Damage from enemy leaks
- Boss clears
- Resource efficiency
- Value prediction targets

Training rewards and final evaluation metrics must be defined separately. Under the current balance, wave progress may also be used for evaluation, but an increase in shaping reward alone does not count as an improvement. Whenever shaping changes, use held-out seeds to check whether terminal progress and survival distribution improve. Do not accept a policy that only increases HP, survival time, or decision count without improving game progress.

## Non-goals

The first implementation does not target:

- Mathematical proof of optimality
- Running full MCTS on every action
- Imitating human UI button presses
- Unbounded generalization to arbitrary future balance configurations
- Imitating human skill with random action noise
- A general-purpose Effect DSL that reimplements the entire game
- A general-purpose vision agent that takes board images as input

## Operational constraints

- Training and simulation must run on an Apple M1 with 16 GB of memory.
- The same dataset, checkpoint, and seed contracts must work on remote machines.
- Measure remote machine specifications when access is available; do not guess them in documentation.
- Use WGPU's Metal backend as a candidate for GPU training and batch inference on M1.
- On remote machines, select CUDA or WGPU after identifying the actual GPU.
- Optimize branching game simulation, legal actions, and pathfinding for CPU by default.
- Compare end-to-end throughput on CPU and batched GPU before sending small single inferences to the GPU.
- Use a fast distilled policy without search by default for large-scale balance statistics.
- Do not store credentials needed for execution in configuration files, datasets, checkpoints, or documentation.

## Long-term AI research acceptance criteria

These are acceptance criteria for the broader AI research effort, not prerequisites for completing the current statistics collection feature.

1. Operate at semantic decision granularity without UI micro-actions.
2. Have the authoritative legal-action generator block illegal actions rather than relying on policy self-correction.
3. Compare card combinations and positions jointly instead of greedily fixing a card combination first.
4. Preserve deterministic replay for the same seed and configuration.
5. Improve normalized throughput over the baseline under the new simulator contract.
6. Ensure rollout teachers cannot see hidden future RNG state.
7. Improve preselected held-out full-game outcomes over the existing heuristic.
8. Meet the specified inference-latency budget without search.
9. Improve the primary metric over the existing policy on preselected held-out seeds and meet allowed regression limits. A full clear is not required under the current balance.
10. Report whether results reproduce across at least three independent training runs.
11. Run the simulator and GPU learner for long periods within the memory limit on an M1 with 16 GB of memory.
12. Remove the existing AI only after approval of the replacement path.
13. Validate checkpoint reuse and retraining for representative numeric changes and changes that add effects or combat rules. Use independent runs to determine whether the changed policy reaches a preselected target with less training budget than training from scratch, and provide a repeatable process from compatibility checks through evaluation.

Do not approve an improvement when the primary metric difference is within sampling error. See [`08-evaluation.md`](08-evaluation.md) for exact seed counts and statistical tests.

## Follow-up goals

After validating the first-stage goals, expand in this order:

1. Measure balance-parameter sensitivity.
2. Add narrow configuration randomization.
3. Build a balance-conditioned policy.
4. Model strong and average players using representative human behavior data.
