# 0004: 행동 계약과 시뮬레이터를 함께 최적화한다

- 상태: Accepted
- 결정일: 2026-09-17

## 결정

semantic macro-action 구현과 simulator throughput 최적화를 같은 단계에서 수행한다.

## 이유

macro-action은 반복 observation, legal action 생성, inference를 제거한다. 이전 micro-action 기준으로만 환경을 최적화하면 최종 병목을 잘못 측정할 수 있다.

현재 placement 후보 생성은 중복 legality 검사, 전체 state clone, 반복 path recalculation을 일으키므로 모델 architecture보다 먼저 해결할 가치가 크다.

## 결과

- 새 action contract 기준으로 benchmark를 다시 측정한다.
- episode/sec 외에 ticks/sec, decisions/sec, path query 수와 latency를 사용한다.
- deterministic replay와 legal action equivalence를 성능 최적화의 불변 조건으로 둔다.
