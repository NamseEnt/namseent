# Rollout Teacher

## 역할

teacher는 느리지만 현재 heuristic보다 강한 행동 label과 후보별 가치를 만든다. 대량 밸런스 시뮬레이션의 기본 agent가 아니다. 최종 실행 agent는 teacher 결과를 학습한 빠른 policy다.

처음부터 MCTS나 AlphaZero 형태의 반복 시스템을 만들지 않는다. 가장 단순한 실제 simulator rollout이 heuristic보다 좋은지 먼저 검증한다.

## 최소 teacher

각 decision state에서 다음 순서로 동작한다.

1. authoritative legal candidate 또는 build-placement pair를 생성한다.
2. 후보별로 같은 future scenario seed 집합을 배정한다.
3. 후보를 적용한 뒤 고정 horizon까지 실제 simulator를 실행한다.
4. 후보별 outcome의 평균과 분산을 계산한다.
5. 같은 scoring contract로 후보를 비교한다.

초기 실험값의 예시는 다음과 같다.

- build-placement pair 최대 32개
- future scenario seed 16개
- horizon 1~3 wave

이 값은 확정된 기본값이 아니다. simulator 처리량 측정과 label 안정성 실험으로 정한다.

현재 최소 구현은 `td-simulator teacher` 명령과 `run_semantic_teacher_episode` API다. candidate마다 `GameEnvironment::fork_for_rollout_seed`를 사용하고, `horizon_decisions` 동안 canonical continuation을 실행한다(아래 "Continuation policy" 참고). report에는 candidate action, sample count, mean score, variance, standard error, wins, 평균 clear rate와 stage가 포함된다. `build_tower_rollout_limit`을 지정하면 `dense_semantic_candidates`가 `DenseBuildTowerScoreTable`의 전체 map 랭킹에서 상위 `build_tower_rollout_limit`개의 `BuildTower` action만 teacher에 공급해 smoke 또는 비용 제한 실험을 할 수 있다(`Reroll`/shop/inventory/treasure 같은 non-`BuildTower` action은 이 한도와 무관하게 항상 전부 포함된다). 기본 CLI 값은 검증 가능한 작은 smoke workload이며 production dataset의 최종값이 아니다.

### Legacy candidate confound (해소됨)

과거 production teacher candidate 경로는 `dense_semantic_candidates`(전체 map 기준 dense `BuildTower` 랭킹)를 사용하면서도, `evaluate_semantic_candidate_set`의 baseline action과 rollout continuation은 여전히 `semantic_legal_actions_with_position_limit(position_candidate_limit)`(위치 제한이 걸린 legacy proposal)에서 나왔다. 즉 teacher candidate는 전체 map 기준인데 baseline/continuation은 제한된 proposal 기준이어서, 측정된 "teacher가 baseline보다 낫다"는 개선폭이 실제 rollout 품질이 아니라 candidate 표현 방식 차이(artifact)일 수 있었다.

이 confound은 `policy_runner::canonical_scripted_semantic_action`을 도입해 해소했다. 이 함수는 baseline과 continuation 모두에 쓰이는 유일한 canonical heuristic이며, `semantic_legal_actions_with_position_limit`을 전혀 사용하지 않는다: semantic card decision이 가능하면 `GameEnvironment::semantic_non_build_actions`(모든 non-`BuildTower` action, pruning 없음)에 `DenseBuildTowerScoreTable::best_action()`(전체 map 기준 global-best `BuildTower` action 단 하나)을 더해 `scripted_expert_action`에 넘긴다. card decision이 불가능한 상태(예: 잔여 `TowerPlacement`)에서는 `semantic_non_build_actions`가 이미 전체 legal action set으로 fallback하므로 그대로 사용한다.

legacy `position_candidate_limit`은 프로덕션 teacher 계약에서 완전히 제거되었다 - `RolloutTeacherConfig`, teacher CLI, teacher dataset collector 어디에도 남아 있지 않다. 남은 곳은 `simulator/src/teacher.rs`의 `#[cfg(test)]` 전용 legacy diagnostic 벤치마크(`phase1_candidate_recall_report`, `phase2_candidate_limit_bias_report`, `dense_candidate_migration_benchmark`)뿐이며, 이들은 historical evidence로만 유지한다.

`candidate_limit`은 단순 앞부분 truncate가 아니라 `select_candidates_fairly`로 선택했다(legacy). `phase2_candidate_limit_bias_report`(`simulator/src/teacher.rs`, `cargo test --release -- --ignored phase2_candidate_limit_bias_report`)로 측정한 결과, 단순 truncate는 card subset의 97%(limit 64 기준)를 완전히 배제했고 이 배제가 hand slot index 기반 생성 순서와 체계적으로 상관되어 있었다(뒤쪽 subset은 limit 1024에서도 여전히 대부분 배제). `PurchaseShopItem`/`UseInventoryItem`도 항상 card action 뒤에 생성되어 limit 2048 미만에서는 한 번도 살아남지 못했다. 이 truncate 방식에서 oracle 순위 1위 후보의 exact-best 보존율은 limit 1024까지 0%였다. `select_candidates_fairly`는 각 card subset(및 그 외 action)을 하나의 block으로 유지한 채, block의 순서만 hand slot 생성 순서와 무관한 키(card id 합)로 재배열한다. 같은 벤치마크에서 exact-best 보존율이 limit 128에서 60%, limit 512 이상에서 100%로 개선되었고 coverage regret은 limit 128 이상에서 0으로 측정되었다. 후보 수가 늘어난 만큼 rollout 비용도 대략 선형으로 늘어난다(release 측정: limit 64→256에서 벽시계 시간 약 3.3배).

`candidate_limit=64`에서도 exact-best 보존율이 34%에 그치고 100%를 얻으려면 ≈512가 필요하다는 점, 그리고 rollout 비용이 후보 수에 선형으로 붙는다는 점은 quality와 throughput이 구조적으로 trade-off 관계에 있음을 보여준다. 이 flattened candidate list 표현 자체를 [`11-candidate-architecture-review.md`](11-candidate-architecture-review.md)에서 재검토했으며, vectorized joint scorer로 교체할 것을 권고했다([`decisions/0008-vectorized-joint-action-scoring.md`](decisions/0008-vectorized-joint-action-scoring.md)). 이 절의 flattened-candidate-bias 수치는 legacy 경로에 대한 historical evidence로 유지한다 - 현재 production candidate 표현은 `dense_semantic_candidates`(dense 전체 map `BuildTower` top-K + 모든 non-`BuildTower` action)다.

현재 score는 `stage_progress_v1` 계약으로 stage 진행도와 현재 stage completion을 합산하고, full clear에는 1,000의 terminal victory bonus를 준다. 이 score는 candidate ranking용 fixed-horizon signal이며 최종 승률 평가를 대체하지 않는다.

## Common random numbers

후보 A와 B는 가능한 한 같은 외생 random scenario를 경험해야 한다.

```text
A: scenario seeds 1, 2, 3, ... N
B: scenario seeds 1, 2, 3, ... N
```

단순히 하나의 global RNG stream을 후보마다 같은 상태로 복제하는 것만으로 충분하지 않을 수 있다. 행동에 따라 RNG 소비 횟수가 달라지면 이후 사건이 서로 다른 의미로 대응하기 때문이다.

가능한 경우 RNG domain을 카드 draw, wave spawn, shop, treasure 등으로 분리하고 scenario seed에서 domain별 stream을 파생한다. domain separation이 구현되기 전에는 action-dependent RNG divergence를 teacher report에 제한 사항으로 기록한다.

현재 simulator는 `GameEnvironment::fork_for_rollout_seed`를 통해 현재 authoritative snapshot을 복제하고 `ML_TOWER_TEACHER_SCENARIO` domain에서 scenario seed를 파생한다. 따라서 후보 평가 시작 시점의 공개 observation과 legal action은 유지하면서 미래 RNG stream만 분리할 수 있다. domain별 RNG 소비가 후보 행동에 따라 달라지는 한계는 여전히 report에 남긴다.

## Non-cheating 규칙

- 실제 다음 카드나 미래 treasure 결과를 보고 후보를 선택하지 않는다.
- 후보 생성은 현재 observation과 공개된 규칙만 사용한다.
- future scenario seed는 모든 후보를 평가하기 위해 simulator 내부에서만 사용한다.
- teacher dataset에 실제 미래 결과를 observation feature처럼 저장하지 않는다.
- candidate pruning policy도 hidden future를 입력으로 받지 않는다.

## Horizon과 점수

full-game rollout이 충분히 싸지기 전에는 fixed horizon을 사용한다. 짧은 horizon은 장기 build를 과소평가할 수 있으므로 다음을 함께 기록한다.

- horizon 도중 terminal win/loss 여부
- 완료한 stage/wave
- player survival과 HP
- 누수 피해
- endpoint state
- 후보별 sample mean, variance, standard error

초기 teacher score는 실험 전에 고정하고 dataset metadata에 기록한다. 진행도와 HP를 사용하더라도 최종 teacher 채택은 held-out full-game 승률로 결정한다.

서로 다른 horizon 또는 seed 수에서 선택 action이 자주 바뀌면 label이 안정적이지 않은 것으로 본다. seed 수를 늘렸을 때 상위 후보 순위와 expected return이 수렴하는지 측정한다.

## Continuation policy

후보 적용 후 horizon까지 사용할 continuation policy도 teacher 계약의 일부다.

현재 production continuation policy는 `policy_runner::canonical_scripted_semantic_action` 하나로 고정되어 있다. 모든 후보가 첫 action 이후 `horizon_decisions`가 끝날 때까지 매 decision마다 이 동일한 canonical heuristic으로 continuation을 진행하며, 후보마다 다른 continuation policy를 사용하지 않는다. legacy position-limited proposal, 후보별로 다른 policy, teacher 자신의 재귀적 선택, MCTS, learned value bootstrap, learned policy continuation은 사용하지 않는다. policy가 개선되면 teacher dataset version도 변경한다.

## Baseline과 Expert regret

기존 expert(baseline)의 품질은 imitation accuracy가 아니라 regret으로 측정한다.

```text
regret(state)
  = estimated_value(best_candidate)
  - estimated_value(baseline_action)
```

baseline action은 매 decision마다 `canonical_scripted_semantic_action(environment)`으로 결정되며, teacher candidate set에 포함되어 있지 않으면 명시적으로 추가한 뒤 action identity로 dedup한다. baseline과 모든 teacher candidate는 정확히 동일한 `config.scenario_seeds` 스케줄과 동일한 continuation policy로 평가된다 - baseline만 다른(더 적은 또는 다른) scenario 집합으로 평가되는 경우는 없다. 이 계약 덕분에 `RolloutTeacherDecision`의 `baseline_action_id`/`baseline_mean_score`/`expert_regret`는 항상(옵션이 아니라) 채워진다.

action type별 regret, 큰 regret state의 비율, catastrophic choice 예시를 기록한다. 기존 heuristic이 자주 틀리는 decision point부터 teacher dataset을 집중 생성할 수 있다.

## Teacher behavior dataset

teacher가 선택한 macro-action을 기존 UI micro-action trajectory와 섞지 않도록 별도 수집 명령을 사용한다.

```text
cargo run --release --manifest-path simulator/Cargo.toml --features simulator-wgpu -- ml collect-teacher \
  --seed-start 0 --seed-end 3 \
  --max-decisions 64 --scenario-count 16 --horizon-decisions 8 \
  --build-tower-rollout-limit 64 \
  --output artifacts/datasets/semantic-teacher.jsonl
```

dataset observation에는 현재 state와 legal macro candidates만 저장한다. candidate rollout의 future result는 label 생성에만 사용하고 observation feature로 저장하지 않는다. `--build-tower-rollout-limit`은 teacher strength를 제한하므로 production dataset에서는 recall과 held-out full-clear 승률을 함께 검증한다.

## Stability evaluation harness (`teacher-eval`)

`td-simulator teacher-eval` (`simulator/src/teacher_eval.rs`)은 서로 다른 scenario count/horizon/`build_tower_rollout_limit` 설정에서 label이 얼마나 안정적인지, 그리고 비용이 얼마나 늘어나는지 비교하는 Phase 3 harness다. 이 harness는 teacher를 "충분히 강하다"고 승인하지 않는다 - 실제 held-out full-game strength gate는 별도 후속 작업이다.

development state corpus는 **teacher가 선택한 action이 아니라 canonical baseline trajectory**(`canonical_scripted_semantic_action`)를 따라 생성한다. teacher의 선택으로 corpus를 만들면 방문하는 state 자체가 평가 대상 config에 의존하게 되어 안정성 비교가 오염되기 때문이다. 각 baseline state에서 환경을 변형하지 않고 grid의 모든 config(`scenario_counts x horizon_decisions x build_tower_rollout_limits`)를 평가한 뒤에만, 실제 environment를 canonical baseline action으로 한 스텝 전진시킨다.

scenario count N에 대한 seed schedule은 항상 `scenario_seed_start .. scenario_seed_start + N` prefix다(nested common-random-number schedule, 예: 2 -> `[0,1]`, 4 -> `[0,1,2,3]`). "reference setting"은 grid에서 가장 큰 `scenario_count`, 가장 큰 `horizon_decisions`, 가장 큰 `build_tower_rollout_limit`(`None`/unlimited가 가장 큼) 조합이며, "ground truth"가 아니라 안정성 비교의 기준점일 뿐이다.

state/config 레벌 record에는 최소한 seed, decision_index, state_hash, decision_point, scenario_count, horizon_decisions, build_tower_rollout_limit, candidate_count, selected_action_id, baseline_action_id, selected/baseline mean score, expert_regret, selected/baseline standard error, elapsed_seconds, score_margin(가능한 경우)이 포함된다. aggregate report는 reference 대비 agreement rate, decision_point별 agreement, mean/median regret, positive-regret 비율, large-regret 비율, 평균 candidate 수, decision당 평균 wall time, 총 scenario rollout 수를 포함한다. "large regret" 임계값은 실험 전에 `LARGE_REGRET_THRESHOLD = 0.05`로 고정하고 report metadata에 기록하며, 결과를 보고 사후에 조정하지 않는다.

## Paired full-game evaluation (foundation)

같은 `teacher-eval` 모듈은 canonical baseline과 rollout teacher를 동일한 gameplay seed 목록에서 비교하는 paired full-game API(`run_paired_full_game_evaluation`)도 제공한다. seed별로 baseline/teacher의 victory, clear rate, final stage, decision count를 기록하고, aggregate로 각각의 full-clear rate, teacher-win/baseline-loss와 baseline-win/teacher-loss 수, 평균 clear-rate/최종 stage 차이를 계산한다. 이 API는 현재 smoke 규모로만 실행하며, 그 결과로 teacher 강도를 승인하지 않는다 - 실제 held-out validation은 더 큰 규모로 후속 진행한다.

CLI 예시:

```text
cargo run --release --manifest-path simulator/Cargo.toml -- teacher-eval \
  --seed-start 0 --seed-end 3 \
  --max-decisions 64 --state-limit-per-seed 8 \
  --scenario-seed-start 1000 \
  --scenario-counts 2,4 --horizons 2,4 --build-tower-rollout-limits 8,16 \
  --run-paired-full-game --paired-seed-start 0 --paired-seed-end 1 --paired-max-decisions 128 \
  --output artifacts/teacher-eval/smoke.json
```

report는 provenance로 teacher score/observation/action schema version, config digest, RNG algorithm version, seed range digest, scenario schedule, grid 값, decision/state 제한을 함께 기록한다.

## 확장 조건

최소 teacher가 기존 heuristic보다 강하다는 것이 확인된 후에만 다음을 추가한다.

- policy-guided candidate pruning
- learned value bootstrap
- adaptive horizon
- uncertainty 기반 seed 추가
- 반복 policy improvement
- 새 policy로 continuation을 교체한 dataset 재생성

## 승인 기준

- hidden future RNG를 사용하지 않는다.
- 후보별 동일 scenario seed 계약이 재현된다.
- seed 수와 horizon 증가에 따른 label 안정성이 보고된다.
- teacher 계산 비용이 state/action type별로 보고된다.
- 기존 heuristic의 regret이 측정된다.
- teacher 행동으로 실행한 held-out full game 승률이 기존 heuristic보다 개선된다.
- 성공하기 전에는 복잡한 search infrastructure를 추가하지 않는다.
