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

## Common random numbers

후보 A와 B는 가능한 한 같은 외생 random scenario를 경험해야 한다.

```text
A: scenario seeds 1, 2, 3, ... N
B: scenario seeds 1, 2, 3, ... N
```

단순히 하나의 global RNG stream을 후보마다 같은 상태로 복제하는 것만으로 충분하지 않을 수 있다. 행동에 따라 RNG 소비 횟수가 달라지면 이후 사건이 서로 다른 의미로 대응하기 때문이다.

가능한 경우 RNG domain을 카드 draw, wave spawn, shop, treasure 등으로 분리하고 scenario seed에서 domain별 stream을 파생한다. domain separation이 구현되기 전에는 action-dependent RNG divergence를 teacher report에 제한 사항으로 기록한다.

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

초기에는 다음 중 재현 가능하고 가장 강한 하나를 baseline으로 고정한다.

- 개선된 deterministic heuristic
- 현재 distilled policy
- 제한된 추가 rollout decision

후보마다 다른 continuation policy를 사용하지 않는다. policy가 개선되면 teacher dataset version도 변경한다.

## Expert regret

기존 expert의 품질은 imitation accuracy가 아니라 regret으로 측정한다.

```text
regret(state)
  = estimated_value(best_candidate)
  - estimated_value(expert_candidate)
```

action type별 regret, 큰 regret state의 비율, catastrophic choice 예시를 기록한다. 기존 heuristic이 자주 틀리는 decision point부터 teacher dataset을 집중 생성할 수 있다.

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
