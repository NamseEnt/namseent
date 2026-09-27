# 0006: AI 계층은 다시 작성하고 core는 유지한다

- 상태: Accepted
- 결정일: 2026-09-18

## 결정

기존 AI의 action adapter, expert, dataset, model, BC/PPO 학습 경로는 새 계약을 중심으로 별도 구현한다. 기존 checkpoint와 dataset을 새 schema로 마이그레이션하지 않는다.

authoritative game core, legal rule, deterministic RNG/replay, configuration digest, 통계와 평가 기반은 재사용한다.

## 제거 순서

기존 AI를 먼저 삭제하지 않는다. 기능 추가 없이 baseline으로 동결하고 새 구현이 승인 기준을 통과한 뒤 제거한다.

## 이유

기존 AI를 먼저 제거하면 동일 seed 비교, regression 확인, heuristic regret 측정 근거를 잃는다. 반대로 기존 schema를 계속 고치는 방식은 새 macro-action과 observation 계약을 불필요하게 제약한다.

## 결과

- 새 구현은 legacy checkpoint 호환성을 목표로 하지 않는다.
- legacy와 new 경로는 같은 authoritative core를 사용한다.
- 새 production policy가 검증되기 전까지 기존 경로는 비교 실행만 지원한다.
- 검증 후 legacy 전용 코드와 artifact를 제거한다.
