# 목표와 승인 기준

## 문제 정의

이 프로젝트의 1차 산출물은 현재 게임 규칙에서 높은 확률로 끝까지 클리어하는 AI다. 사람처럼 보이는 행동, 수학적인 최적성 증명, 모든 밸런스 버전에 즉시 적응하는 범용 정책은 1차 목표가 아니다.

게임의 의사결정에는 다음 요소가 함께 작용한다.

- 개별 카드의 영구 강화와 engraving
- 카드 조합으로 만들어지는 족보와 타워
- 보유 유물과 아이템
- 타워의 설치 위치와 기존 타워 조합
- 설치 및 철거에 따른 경로 변화
- 현재와 이후 웨이브
- 자원 소비와 장기 run 가치

따라서 높은 포커 족보나 즉시 damage만 최대화하는 정책은 목표를 충족하지 않는다.

## 1차 목표

고정된 현재 balance configuration에서 다음 값을 최대화한다.

```text
P(full clear | fixed balance configuration, held-out seed distribution)
```

최종 모델 비교에서는 full-clear 여부를 가장 먼저 사용한다. 승률이 낮은 정책을 잔여 HP나 평균 진행도가 높다는 이유로 선택하지 않는다.

## 학습 신호와 평가 목적의 구분

학습 안정화를 위해 다음 값을 reward shaping이나 auxiliary target으로 사용할 수 있다.

- 웨이브 진행도
- 잔여 HP와 shield
- 적 누수 피해
- boss clear
- 자원 효율
- value prediction target

이 값들은 학습을 돕기 위한 수단이다. 최종 평가 목적은 full-clear 확률이다. shaping을 변경할 때마다 정책이 shaping 지표만 최적화하고 승률을 잃지 않았는지 held-out seed로 확인한다.

## 비목표

1차 구현에서 다음을 목표로 삼지 않는다.

- 수학적 최적성 증명
- 매 행동마다 full MCTS 실행
- UI 버튼 조작을 사람처럼 흉내 내기
- 임의의 미래 balance configuration에 대한 무제한 일반화
- 무작위 action noise로 인간 실력을 흉내 내기
- 전체 게임 규칙을 재구현하는 범용 Effect DSL
- 보드 이미지를 입력으로 받는 범용 vision agent

## 운영 제약

- 학습과 시뮬레이션은 Apple M1 16GB에서 실행 가능해야 한다.
- 원격 머신에서도 같은 dataset, checkpoint, seed 계약으로 실행할 수 있어야 한다.
- 원격 머신의 실제 사양은 연결 가능한 시점에 별도로 측정하며 문서에 추측으로 기록하지 않는다.
- M1에서는 WGPU의 Metal backend를 GPU 학습과 batch inference 후보로 사용한다.
- 원격 머신은 실제 GPU 종류를 확인한 뒤 지원되는 CUDA 또는 WGPU backend를 선택한다.
- branch가 많은 게임 simulation, legal action, pathfinding은 CPU 최적화를 기본으로 한다.
- 작은 단건 inference를 무조건 GPU로 보내지 않고 CPU와 batched GPU의 end-to-end 처리량을 비교한다.
- 대량 밸런스 통계는 search 없이 빠른 distilled policy로 실행하는 것을 기본으로 한다.
- 실행에 필요한 인증 정보는 설정 파일, dataset, checkpoint, 문서에 저장하지 않는다.

## 전체 승인 기준

새 AI가 완료되었다고 판단하려면 다음 조건을 모두 만족해야 한다.

1. UI micro-action 없이 semantic decision 단위로 동작한다.
2. 불법 행동을 정책이 직접 교정하지 않고 authoritative legal action generator가 차단한다.
3. 카드 조합과 위치를 결합해 비교하며 카드 조합 하나를 먼저 greedy하게 확정하지 않는다.
4. 같은 seed와 configuration에서 deterministic replay가 유지된다.
5. 새 simulator contract에서 normalized throughput이 baseline보다 유의미하게 개선된다.
6. rollout teacher가 숨겨진 미래 RNG를 보지 않는다.
7. teacher가 기존 heuristic보다 held-out full-game 승률을 개선한다.
8. distilled policy가 search 없이 정해진 inference latency 예산을 만족한다.
9. 최종 정책이 사전에 고정한 held-out seed에서 기존 정책보다 높은 full-clear 승률을 보인다.
10. 결과가 서로 다른 최소 3회 학습 run에서도 재현되는지 보고한다.
11. M1 16GB에서 CPU simulator와 GPU learner가 memory limit 안에서 장시간 실행된다.
12. 기존 AI는 새 경로의 승인 완료 후에만 제거된다.

승률 차이가 표본 오차 범위에 있을 경우 개선으로 승인하지 않는다. 정확한 seed 수와 통계 검정은 [`08-evaluation.md`](08-evaluation.md)에서 관리한다.

## 후속 목표

1차 목표가 검증된 뒤 다음 순서로 확장한다.

1. 밸런스 파라미터 민감도 측정
2. 좁은 범위의 configuration randomization
3. balance-conditioned policy
4. 실제 인간 행동 자료에 근거한 잘하는 사용자와 적당한 사용자 모델
