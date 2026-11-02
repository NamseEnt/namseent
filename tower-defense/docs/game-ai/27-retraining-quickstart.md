# Retraining Quickstart

Run from `tower-defense/simulator/` with the release binary.

1. Change content or numbers.
2. `td-simulator ml train` (continues from the best checkpoint; bootstraps if
   none exists or the change is structural).
3. `td-simulator simulate --samples 1000 --all-stats` and inspect the results.
4. Adjust values and repeat; use `td-simulator balance` to tune stage enemy HP
   toward a target clear distribution.

Use `--config FILE` with `ml train` and `simulate` to work on a config other
than the default. Policy strength is only meaningful after a long run; train
several invocations (20 iterations each) before judging results.

Reset everything with `rm -rf artifacts/ml`. Details: [ml-policy](../ml-policy.md).
