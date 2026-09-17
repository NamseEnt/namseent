# 현재 시스템 기준선

## 기록 범위

이 문서는 새 구현의 출발점을 기록한다. 수치는 코드가 바뀌면 자동으로 현재 사실이 되지 않는다.

- 관찰한 Git revision: `8d24cbfe`
- 관찰일: 2026-09-17
- 로컬 기준 머신: Apple M1, 16GB memory, 8 CPU core
- 원격 머신: 접속 및 사양 확인 필요

## 현재 구조

현재 학습 흐름은 다음과 같다.

```text
scripted/heuristic expert
    -> behavior dataset
    -> behavior cloning
    -> Deep Sets actor-critic
    -> PPO rollout and update
    -> held-out validation checkpoint
```

주요 구현 위치는 다음과 같다.

- 환경과 legal action: `simulator/src/environment.rs`
- 기존 expert: `simulator/src/policy_runner.rs`
- dataset과 BC: `simulator/src/ml/dataset.rs`, `simulator/src/ml/bc.rs`
- 모델: `simulator/src/ml/model.rs`
- PPO: `simulator/src/ml/ppo.rs`
- 검증: `simulator/src/ml/validation.rs`
- authoritative command와 observation: `core/src/game_state/command.rs`, `core/src/game_state/observation.rs`
- 게임 설정: `core/src/game_state/config.rs`, `gameconfig.jsonc`

## 현재 행동 계약

현재 `AgentAction`에는 다음과 같은 UI형 micro-action이 포함된다.

- `BeginRerollSelection`
- `BeginTowerSelection`
- `SelectHandCard`
- `DeselectHandCard`
- `ConfirmCardSelection`
- `CancelCardSelection`

실제 `Reroll`과 `SelectTower` macro action도 존재하지만 정책 실행 경로에서는 카드 선택 FSM이 별도 decision step으로 노출된다. 이 구조는 전략적 결정 하나를 여러 inference와 reward credit assignment 단계로 늘린다.

현재 tower build와 placement는 `SelectTower` 이후 `PlaceTower`로 나뉜다. 새 계약에서는 core의 내부 전이는 유지할 수 있지만, 정책은 중간 UI 상태에서 다시 추론하지 않아야 한다.

## 현재 observation

현재 observation에는 카드, deck, tower template, 설치 타워, monster, route, shop, inventory, owned upgrade 등이 포함된다. 그러나 다음 정보는 새 계약 관점에서 부족하거나 불안정하다.

- 행동이 mutable hand slot index에 의존한다.
- owned upgrade는 주로 `key`와 `key_id`로 표현되며 조정 가능한 effect parameter가 직접 드러나지 않는다.
- build 후보와 placement 후보가 하나의 resulting tower context로 결합되어 있지 않다.
- 현재 balance configuration 자체는 정책 입력 계약이 아니다.

## 현재 모델

현재 `DeepSetsActorCritic`은 typed entity set을 entity별로 인코딩한 뒤 mean, max, count pooling을 사용한다. 카드, 유물, 타워 간 복잡한 관계를 직접 표현하기보다 집합 요약을 통해 전달한다.

이 구조가 실제 병목이라는 결론은 아직 없다. 새 action/observation contract와 같은 teacher dataset에서 작은 attention 모델과 비교하기 전에는 교체를 확정하지 않는다.

## 현재 expert의 한계

`monte_carlo_expert`라는 이름의 현재 구현은 full simulator rollout이나 MCTS가 아니다.

- placement에서는 legal placement를 최대 32개 무작위 추출한다.
- 각 위치를 `coverage * 1000 + damage proxy`로 비교한다.
- reroll에서는 최대 12장에 대한 subset을 열거하고 mask별 8회 draw sample을 수행한다.
- reroll 결과는 rank, suit count, polish, 고정 engraving 가중치를 사용한 proxy로 평가한다.

이 expert는 카드 강화, 유물, 실제 resulting tower, 경로 변화와 장기 run 결과를 충분히 평가하지 않는다. 따라서 기존 BC accuracy가 높아도 강한 게임 플레이를 의미하지 않는다.

## 확인된 성능 병목

현재 tower placement 후보 생성은 다음 비용을 반복한다.

1. `placement_coordinates`가 각 tile과 hand tower에 `can_place_tower`를 호출한다.
2. `tower_placement_actions`가 같은 coordinate를 tower별로 다시 `can_place_tower`로 검사한다.
3. `can_place_tower`는 전체 game state를 clone한 뒤 실제 placement를 시도한다.
4. placement 시도는 route를 다시 계산한다.

2026-09-17의 임시 release 측정에서는 random-legal seed 0, 최대 512 decision 실행이 stage 10, 372 decision에서 약 59.82초가 걸렸다. 8초 sampling profile에서는 상위 stack sample의 약 76.8%가 `find_shortest_route` 경로에 있었다.

이 수치는 재현 가능한 benchmark harness가 생기기 전의 임시 기준이다. Phase 0에서 정확한 명령, build profile, tick 수, path query 수와 함께 다시 측정한다.

## 체크포인트 상태

관찰 시점의 체크인 checkpoint는 현재 코드 계약과 호환되지 않았다.

- 저장소 루트 checkpoint: ML contract mismatch
- `simulator` checkpoint: observation schema expected 2, got 1

새 행동 및 observation 계약은 기존 checkpoint를 마이그레이션하지 않는다. schema version을 증가시키고 새 dataset과 checkpoint를 만든다.

## 유지할 요소

- authoritative core command와 legality
- deterministic seed와 replay 검증
- variable-cardinality legal candidate scoring이라는 문제 구조
- typed entity observation이라는 기본 개념
- held-out seed validation
- checkpoint provenance와 configuration digest
- headless와 rendered game의 동일한 simulation-step 규칙

## 교체하거나 재검증할 요소

- UI micro-action 기반 policy horizon
- hand slot index 기반 macro action identity
- 중복 placement legality 계산
- clone 기반 `can_place_tower` hot path
- heuristic expert label 품질
- Deep Sets가 관계 표현에 충분하다는 가정
- PPO가 최종 학습 방식이라는 가정

## 교체 전략

기존 AI 학습·정책 코드는 새 계약에 맞춰 점진적으로 고치는 대상이 아니라 별도 경로에서 다시 작성하는 대상이다. 기존 checkpoint와 dataset도 새 schema로 마이그레이션하지 않는다.

다만 기존 구현을 먼저 삭제하지는 않는다. 다음 용도로 읽기 전용에 가깝게 동결한다.

- random/scripted/기존 policy 기준선 실행
- 동일 seed의 회귀 비교
- 새 simulator transition의 결과 검증
- 새 teacher와 heuristic의 regret 비교

새 구현이 평가 gate를 통과하면 다음 legacy 요소를 제거한다.

- policy에 노출되는 카드 선택 micro-action FSM
- 기존 heuristic expert와 그 전용 dataset 생성 경로
- 기존 action/feature schema에 묶인 Deep Sets model과 checkpoint loader
- 기존 schema에 묶인 BC/PPO training path

authoritative core, deterministic replay, configuration digest, 통계 및 검증 기반은 제거하지 않는다. legacy와 new 경로가 core 규칙을 복제해 서로 다른 게임을 실행하게 만들지 않는다.
