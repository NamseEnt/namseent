# 0002: UI micro-action을 제거하고 joint build-placement를 사용한다

- 상태: Accepted
- 결정일: 2026-09-17

## 결정

정책 action space에서 카드별 선택, 취소, 확인과 같은 UI FSM 단계를 제거한다.

Reroll은 카드 subset 하나로 표현한다. Build와 placement는 factorized representation을 사용하지만 최종적으로 여러 `(cards, position)` pair를 함께 비교한다. 카드 조합 하나를 먼저 greedy하게 확정하지 않는다.

## 이유

UI 단계는 전략적 의미 없이 horizon과 credit assignment를 늘린다. 반면 카드 조합과 위치는 resulting tower의 성질 때문에 강하게 결합되어 있으므로 완전히 독립된 전략 판단으로 분리할 수 없다.

## 결과

- environment 내부 전이는 여러 단계일 수 있지만 policy inference는 macro decision당 한 번이다.
- 후보가 작을 때는 모든 legal pair를 비교한다.
- pruning이 필요하면 top-K/beam을 사용하고 candidate recall을 측정한다.
- 최종 pair는 joint log probability, joint Q 또는 동일 rollout return처럼 일관된 단위로 비교한다.
