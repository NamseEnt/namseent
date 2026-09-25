# Game AI 설계 문서

이 디렉터리는 Tower Defense를 높은 확률로 클리어하는 빠른 AI와, 그 AI를 이용한 밸런스 실험 시스템의 설계 및 구현 기준을 관리한다.

기존 [`../ml-policy.md`](../ml-policy.md)는 현재 구현된 BC/PPO 시스템의 설명이다. 이 디렉터리는 그 구현을 무조건 유지하거나 폐기하는 문서가 아니라, 새 계약으로 교체하기 위한 기준이다. 새 계약이 구현되고 검증되기 전까지 기존 문서를 현재 시스템의 참고 자료로 유지한다.

## 최종 합의

- 1차 목표는 사람처럼 보이는 고수의 재현이 아니라 현재 고정 밸런스에서 full-clear 확률이 높은 AI다.
- 최종 평가 목적은 held-out seed에서의 full-clear 확률이다.
- HP, 웨이브 진행도, 누수 피해 등의 값은 학습 보조 신호와 진단 지표로 사용할 수 있지만 최종 평가 목적을 대체하지 않는다.
- AI에는 UI 조작 순서가 아니라 의미 있는 macro action을 제공한다.
- 카드 조합과 배치 위치는 서로 결합된 판단으로 취급하되 factorized joint policy로 표현한다.
- 카드 조합 하나를 먼저 greedy하게 확정한 뒤 위치를 고르지 않는다.
- macro action 도입과 시뮬레이터 고속화를 하나의 구현 단계로 진행한다.
- search/lookahead는 대량 시뮬레이션의 기본 실행 경로가 아니라 더 강한 학습 데이터를 만드는 teacher로 먼저 사용한다.
- 첫 teacher는 fixed-horizon, common-random-seed rollout으로 작게 시작한다.
- 동일한 teacher dataset으로 표현 구조를 비교하며, 모델 구조를 먼저 확정하지 않는다.
- PPO는 첫 RL baseline이지만 최종 알고리즘으로 고정하지 않는다.
- 기존 AI의 학습·정책 계층은 새 계약으로 다시 작성하되 authoritative core와 검증 기반은 재사용한다.
- 기존 AI는 새 구현이 승인 기준을 통과할 때까지 동결된 baseline으로 유지한 뒤 제거한다.
- simulation과 legal action 생성은 CPU에 두고, 학습과 충분히 큰 batch inference는 GPU를 사용한다.
- 고정 밸런스 AI를 먼저 검증한 뒤 balance-conditioned policy와 인간형 실수 모델로 확장한다.

## 문서 지도

| 문서 | 역할 | 상태 |
| --- | --- | --- |
| [`00-goals-and-acceptance.md`](00-goals-and-acceptance.md) | 목표, 비목표, 전체 승인 기준 | Accepted |
| [`01-current-system-baseline.md`](01-current-system-baseline.md) | 현재 구현과 측정 기준 | Draft |
| [`02-action-contract.md`](02-action-contract.md) | semantic action과 joint build-placement 계약 | Accepted design |
| [`03-observation-contract.md`](03-observation-contract.md) | 정책과 teacher가 사용하는 상태 정보 | Verified |
| [`04-simulator-performance.md`](04-simulator-performance.md) | 시뮬레이터 프로파일링과 최적화 계획 | Accepted plan |
| [`05-rollout-teacher.md`](05-rollout-teacher.md) | non-cheating rollout teacher | Implemented (minimum) |
| [`06-dataset-and-distillation.md`](06-dataset-and-distillation.md) | teacher dataset과 빠른 정책 압축 | Proposed |
| [`07-policy-and-rl.md`](07-policy-and-rl.md) | 표현 구조와 RL fine-tuning | Proposed |
| [`08-evaluation.md`](08-evaluation.md) | 모델 비교와 최종 평가 | Accepted design |
| [`09-balance-experiments.md`](09-balance-experiments.md) | 밸런스 파라미터 실험 | Deferred |
| [`10-human-player-models.md`](10-human-player-models.md) | 실제 인간형 실수를 포함한 실력 모델 | Deferred |
| [`11-candidate-architecture-review.md`](11-candidate-architecture-review.md) | Phase 1/2 결과에 따른 candidate architecture 재검토 | Proposed |
| [`12-adaptive-teacher-budget.md`](12-adaptive-teacher-budget.md) | Adaptive teacher rollout budget design | Proposed |

상태의 의미는 다음과 같다.

- `Draft`: 현재 사실을 기록했지만 다시 측정하거나 보완해야 한다.
- `Proposed`: 구현 전에 세부 계약을 검토해야 한다.
- `Accepted design`: 설계 방향이 합의되었고 구현 세부를 확정할 수 있다.
- `Accepted plan`: 작업 순서와 검증 방식이 합의되었다.
- `Implemented`: 코드에 반영되었지만 최종 성능 승인은 남아 있다.
- `Verified`: 문서의 승인 기준을 실제 결과가 통과했다.
- `Deferred`: 선행 단계가 끝난 뒤 진행한다.

## 구현 단계

### Phase 0: 기준선 고정

1. 현재 Git revision, 설정 digest, seed 집합을 기록한다.
2. 현재 random, scripted expert, 기존 BC/PPO가 실행 가능한 범위를 측정한다.
3. 환경 실행 시간과 path validation 비용을 분리해 측정한다.
4. 기존 AI 경로를 기능 추가 없이 동결하고 제거 대상과 재사용 대상을 확정한다.

완료 조건은 재현 가능한 baseline report가 생성되는 것이다.

기준선 report는 다음 CLI로 생성한다.

```text
cargo run --release --manifest-path simulator/Cargo.toml -- benchmark \
  --policy random-legal \
  --seed-start 0 --seed-end 3 \
  --max-decisions 512 \
  --threads 8 \
  --output artifacts/benchmarks/phase0-random-0-3.json
```

같은 seed와 제한으로 `scripted`와 `checkpoint`를 각각 실행한다. checkpoint가 현재 contract와 호환되지 않으면 실패를 숨기지 않고 baseline 상태에 기록한다.

semantic action oracle과 proposal 경로는 다음처럼 별도로 측정한다.

```text
cargo run --release --manifest-path simulator/Cargo.toml -- benchmark \
  --action-mode semantic \
  --policy scripted \
  --seed-start 0 --seed-end 3 \
  --max-decisions 512 \
  --threads 8 \
  --output artifacts/benchmarks/phase1-semantic-scripted-0-3.json
```

### Phase 1: 행동 계약과 시뮬레이터 처리량

1. UI micro-action을 simulator-local semantic action으로 교체한다.
2. build와 placement를 factorized joint decision으로 생성하고 평가한다.
3. 전체 pair oracle과 deterministic proposal의 candidate recall을 비교한다.
4. 행동당 반복 observation, legal action 생성, 상태 복제, 중복 경로 계산을 제거한다.
5. authoritative core 규칙과 deterministic replay를 유지한다.

행동 계약 변경과 성능 최적화는 따로 배포하지 않는다. 새 계약 기준으로 다시 프로파일링해야 하기 때문이다.

### Phase 2: 관측 계약

1. 카드의 영구 강화와 engraving을 안정적인 entity identity와 함께 노출한다.
2. 선택한 카드로 생성될 tower representation을 placement 평가에 제공한다.
3. 유물의 behavior, 관련 태그, 수치 파라미터를 필요한 범위에서 노출한다.
4. 지도, 경로, 웨이브, 상점, 자원, 설치 타워 상태를 정책 입력으로 확정한다.

완료 조건은 teacher가 사용한 합법적인 현재 정보가 student observation에도 표현되는 것이다.

### Phase 3: 최소 rollout teacher

1. 후보별로 동일한 future scenario seed 집합을 사용한다.
2. 제한된 horizon을 실제 simulator로 실행한다.
3. 현재 heuristic의 선택과 대안들의 추정 가치를 비교한다.
4. held-out full-game 평가에서 기존 heuristic보다 강한지 검증한다.

현재 최소 구현은 다음 명령으로 한 episode의 decision별 후보 평가 report를 생성한다.

```text
cargo run --release --manifest-path simulator/Cargo.toml -- teacher \
  --seed 0 \
  --max-decisions 8 \
  --scenario-count 4 \
  --horizon-decisions 4 \
  --build-tower-rollout-limit 32 \
  --output artifacts/teacher/seed-0.json
```

이 명령은 semantic candidate를 동일 scenario seed로 평가하고, 결과의 평균 score, variance, standard error, 승리 수와 선택 action을 JSON으로 저장한다. baseline action은 `canonical_scripted_semantic_action`으로 결정되며 항상 후보 집합에 포함되어 `baseline_action_id`/`baseline_mean_score`/`expert_regret`가 항상 채워진다. 현재는 canonical scripted continuation과 고정 decision horizon만 제공하며, teacher가 기존 heuristic보다 강한지 확인하기 전까지 learned value나 반복 policy improvement는 추가하지 않는다.

teacher-selected macro-action을 distillation dataset으로 저장하려면 다음 경로를 사용한다.

```text
cargo run --release --manifest-path simulator/Cargo.toml --features simulator-wgpu -- ml collect-teacher \
  --seed-start 0 --seed-end 3 \
  --max-decisions 64 --scenario-count 16 --horizon-decisions 8 \
  --build-tower-rollout-limit 64 \
  --output artifacts/datasets/semantic-teacher.jsonl
```

이 dataset은 semantic macro-action trajectory만 포함하며, teacher rollout의 future sample은 관측에 저장하지 않는다.

label 안정성과 비용을 scenario/horizon/build-tower rollout budget 축으로 비교하려면 `teacher-eval` 명령을 사용한다([`05-rollout-teacher.md`](05-rollout-teacher.md)의 "Stability evaluation harness" 참고). 이 harness와 그 paired full-game API는 진단 도구이며, 실제 held-out full-game strength gate는 별도 후속 작업이다.

성공하기 전에는 tree search, learned value bootstrap, 반복 policy improvement를 추가하지 않는다.

### Phase 4: dataset과 빠른 정책

1. teacher의 후보별 평가와 불확실성을 저장한다.
2. 빠른 candidate scorer를 distillation/BC로 학습한다.
3. 같은 dataset과 계산 예산으로 Deep Sets와 작은 attention 구조를 비교한다.
4. CPU inference와 batched GPU inference의 end-to-end 처리량을 비교한다.

### Phase 5: RL fine-tuning

1. distillation checkpoint에서 PPO baseline을 학습한다.
2. PPO가 실제로 개선되는지 같은 평가 계약으로 확인한다.
3. 환경 샘플 비용이 여전히 지배적일 때만 적합한 discrete replay/off-policy 방식을 비교한다.
4. CPU rollout worker와 GPU learner를 겹쳐 실행하는 pipeline의 utilization과 memory를 측정한다.

### Phase 5 종료: Legacy AI 제거

새 production policy가 simulator, teacher, distillation, RL 평가 gate를 통과하면 기존 AI 전용 micro-action adapter, heuristic dataset 경로, 기존 모델과 학습 코드를 제거한다. 그 전에는 legacy 경로를 새 기능 개발 없이 baseline과 회귀 비교 용도로만 유지한다.

### Phase 6: 밸런스 실험

1. 강한 fixed-balance policy를 기준으로 민감도와 통계 수집기를 만든다.
2. 좁은 파라미터 범위에서 재학습 또는 randomization을 검증한다.
3. 필요성이 입증된 뒤 balance-conditioned policy로 확장한다.

### Phase 7: 인간 플레이어 모델

강한 기본 AI가 완성된 뒤 실제 인간의 누락, 제한된 탐색, 잘못된 가치 판단을 별도 모델로 정의한다. 무작위 action noise만으로 실력 차이를 만들지 않는다.

## 변경 규칙

- 설계 문서의 상태를 `Implemented`로 바꿀 때 관련 코드와 검증 결과를 함께 연결한다.
- schema 의미가 바뀌면 기존 버전을 재사용하지 않고 관련 contract version을 증가시킨다.
- 성능 수치에는 Git revision, build profile, 머신, seed, decision/tick 제한을 함께 기록한다.
- 실험에서 채택되지 않은 가설은 구현 사실처럼 쓰지 않는다.
- 중요한 방향 변경은 [`decisions`](decisions/)에 별도 기록하고 기존 결정을 `Superseded`로 표시한다.
