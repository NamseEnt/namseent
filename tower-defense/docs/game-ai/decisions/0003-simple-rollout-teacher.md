# 0003: 단순한 rollout teacher부터 검증한다

- 상태: Accepted
- 결정일: 2026-09-17

## 결정

첫 teacher는 candidate별 common random numbers와 fixed horizon simulator rollout을 사용한다. 처음부터 MCTS, learned bootstrap, iterative search system을 만들지 않는다.

## 이유

현재 heuristic보다 실제로 강한 label을 만들 수 있는지 확인하기 전에 복잡한 search infrastructure를 구축하면 실패 원인과 비용이 커진다.

## 결과

- teacher는 실제 미래 RNG를 미리 보지 않는다.
- 후보별 같은 future scenario seed 집합을 사용한다.
- seed 수와 horizon에 대한 label 안정성을 측정한다.
- held-out full-game 승률이 기존 heuristic보다 높을 때만 teacher를 확장한다.
- 성공 후 policy pruning, value bootstrap, 반복 distillation을 단계적으로 추가할 수 있다.
