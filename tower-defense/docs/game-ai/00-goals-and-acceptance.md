# 목표와 승인 기준

## 현재 구현 목표 (2026-10-09)

사용자 지시에 따라 현재 작업 범위는 **AI가 simulator에서 게임을 여러 번 실행하고 기존 `all-stats` 수준의 지표를 저장·조회하는 기능**이다. 밸런스 조정과 방법론은 콘텐츠 추가·레벨 디자인 단계에서 진행한다. 아래 학습·재학습 연구 목표는 후속 요구사항으로 보존한다.

현재 완료 기준:

1. 현재 PPO/BC 모델을 지정해 여러 seed에서 게임을 실제 종료까지 반복 실행한다.
2. 기존 진행도 분포, 피해량, 아이템·유물·카드 서비스 등의 선택 및 성과 통계를 수집한다.
3. 기존 SQLite 기록과 `stats` 조회 경로를 사용하고 모델·설정·seed 출처를 기록한다.
4. 불법 행동·정책 오류·제한 도달을 정상 게임 결과에 섞지 않는다.
5. 완료에 사람 대표성, 최적성 증명, 특정 완주율 또는 새 밸런스 방법론을 요구하지 않는다.
6. 건설은 scripted 상위 후보 밖의 합법적인 카드 조합을 탐색·학습할 수 있다. 배치는 scripted여도 된다.
7. 콘텐츠 변경 후 기존 checkpoint에서 별도 run으로 추가 학습하고, 가장 많이 선택한 보물의 효과를 제거하는 실험으로 선택 변화 여부를 확인한다. 지원되는 변경과 구조 migration의 경계는 [24](24-construction-and-content-adaptation.md)에 기록한다.

상세 범위는 [`23-batch-simulation-statistics.md`](23-batch-simulation-statistics.md), 결정은 [`0010`](decisions/0010-collect-existing-simulator-statistics.md)을 따른다.

## 장기 문제 정의

이 프로젝트의 1차 산출물은 현재 게임 규칙에서 강한 플레이를 하는 AI와, 기획 변경 후 기존 학습 결과를 활용해 빠르고 쉽게 재학습하는 경로다. 2026-10-09 사용자 설명에 따라 현재 밸런스는 완주가 가능한 밸런스가 아니라는 전제로 개발한다. 완주율을 현재 AI의 필수 합격 기준으로 두지 않는다. 이 설명을 수학적인 완주 불가능성 증명으로 취급하지는 않는다. 사람처럼 보이는 행동, 수학적인 최적성 증명, 모든 밸런스 버전에 즉시 적응하는 범용 정책은 1차 목표가 아니다.

게임의 의사결정에는 다음 요소가 함께 작용한다.

- 개별 카드의 영구 강화와 engraving
- 카드 조합으로 만들어지는 족보와 타워
- 보유 유물과 아이템
- 타워의 설치 위치와 기존 타워 조합
- 설치 및 철거에 따른 경로 변화
- 현재와 이후 웨이브
- 자원 소비와 장기 run 가치

따라서 높은 포커 족보나 즉시 damage만 최대화하는 정책은 목표를 충족하지 않는다.

## 플레이 성능 연구 목표

고정된 balance configuration과 held-out seed 분포에서, 사전에 정한 게임 성과를 개선한다. 현재 밸런스의 기본 primary metric은 실제 terminal까지 실행한 게임의 평균 진행도다. 기존 artifact의 `terminal_clear_rate`는 진행도(0–100)이며 완주율이 아니다.

- 같은 configuration과 seed에서 heuristic 및 기존 강한 checkpoint 대비 진행도 차이를 비교한다.
- stage별 도달률·탈락 분포, 진행도 중앙값과 하위 분위수를 함께 보고한다. 일부 seed의 큰 개선이 다수 seed의 악화를 가리지 않는지 확인한다.
- 완주가 가능한 실험에서는 사전에 full-clear rate를 primary metric으로 선택할 수 있다. 결과를 본 뒤 유리한 지표로 바꾸지 않는다.
- 평가 지표의 정의·version, 최소 개선 폭과 허용 회귀 기준은 실험 전에 고정한다. 구체적인 수치는 비교 실험 설계 시 정하며 아직 달성 기준이 확정되었다고 주장하지 않는다.

강한 기준 정책 대비 재현 가능한 개선은 AI 성능의 증거지만 최적 플레이 달성의 증명은 아니다. AI의 실패만으로 밸런스가 불가능하다고 판정하지 않으며, 밸런스를 바꾸기 전후의 원시 진행도 차이를 AI 실력 향상으로 해석하지 않는다. 평가 계약은 [`08-evaluation.md`](08-evaluation.md), 변경 결정은 [`decisions/0009-balance-appropriate-evaluation.md`](decisions/0009-balance-appropriate-evaluation.md)를 따른다.

## 기획 변경 후 재학습 목표

수치 조정, 아이템 효과 추가, 광역공격 같은 전투 규칙 변경 후에도 기존 checkpoint의 유효한 학습 결과를 재사용해 새 규칙에 적응할 수 있어야 한다. 이 목표는 현재 개발의 설계 요구사항에 포함한다. 재학습 효과는 비교에 사용할 기준 checkpoint를 확보한 뒤 변경 유형별로 검증한다.

- 입력과 행동의 의미가 유지되는 수치 변경은 기존 모델에서 추가 학습하는 경로를 제공한다.
- 기존 효과의 조합으로 표현되는 새 아이템은 효과의 종류, 대상, 수치를 정책 입력으로 전달하고 기존 모델의 재사용 가능성을 확인한다.
- 새로운 효과나 행동이 필요한 변경은 authoritative core, simulator, 관측, legal action을 먼저 확장한다. 호환되는 모델 부분을 이관하고 새 입력이나 출력에 필요한 부분을 학습한다.
- 변경된 규칙에서 새 rollout을 생성한다. 이전 규칙의 trajectory, reward, teacher label을 새 규칙의 정답으로 그대로 사용하지 않는다.
- 원본 checkpoint를 보존하고 새 학습 run에 출처와 이관 내용을 기록한다. 같은 규칙의 실행 재개와 규칙 변경 후 모델 이관을 구분한다.
- 변경 내용과 원본 checkpoint를 지정해 호환성 확인, 모델 이관, 학습, 평가를 반복 실행할 수 있는 절차를 제공한다. 변경마다 모델을 수동으로 다시 구성하는 작업을 줄인다.

빠른 재학습의 성공 여부는 변경 후 처음부터 학습하는 대조군과 비교해 판단한다. 새 규칙에서 사전에 정한 성능에 도달하기까지의 게임 수, wall time, 준비 작업을 기록하며, 동일 예산에서 사전에 정한 평가 지표의 성능도 비교한다. 구체적인 변경 시나리오와 평가 절차는 [`09-balance-experiments.md`](09-balance-experiments.md)의 재학습 검증 계약에서 관리한다. 단순히 checkpoint를 읽는 데 성공한 것만으로 이 목표를 달성했다고 판단하지 않는다.

## 학습 신호와 평가 목적의 구분

학습 안정화를 위해 다음 값을 reward shaping이나 auxiliary target으로 사용할 수 있다.

- 웨이브 진행도
- 잔여 HP와 shield
- 적 누수 피해
- boss clear
- 자원 효율
- value prediction target

학습 reward와 최종 평가 지표는 별도로 정의한다. 현재 밸런스에서는 웨이브 진행도를 평가에도 사용할 수 있지만, shaping reward 증가 자체를 성능 개선으로 간주하지 않는다. shaping을 변경할 때마다 실제 terminal 진행도와 생존 분포가 개선되는지 held-out seed로 확인한다. HP, 생존 시간 또는 decision 수만 늘리고 게임 진행은 개선하지 못하는 정책을 승인하지 않는다.

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

## 장기 AI 연구 승인 기준

아래는 기존 AI 연구 전체의 승인 기준이며 현재 통계 수집 기능 완료의 선행 조건이 아니다.

1. UI micro-action 없이 semantic decision 단위로 동작한다.
2. 불법 행동을 정책이 직접 교정하지 않고 authoritative legal action generator가 차단한다.
3. 카드 조합과 위치를 결합해 비교하며 카드 조합 하나를 먼저 greedy하게 확정하지 않는다.
4. 같은 seed와 configuration에서 deterministic replay가 유지된다.
5. 새 simulator contract에서 normalized throughput이 baseline보다 유의미하게 개선된다.
6. rollout teacher가 숨겨진 미래 RNG를 보지 않는다.
7. teacher가 기존 heuristic보다 사전에 정한 held-out full-game 성과를 개선한다.
8. distilled policy가 search 없이 정해진 inference latency 예산을 만족한다.
9. 최종 정책이 사전에 고정한 held-out seed에서 기존 정책보다 primary metric을 개선하고 허용 회귀 기준을 만족한다. 현재 밸런스에서 완주는 필수 조건이 아니다.
10. 결과가 서로 다른 최소 3회 학습 run에서도 재현되는지 보고한다.
11. M1 16GB에서 CPU simulator와 GPU learner가 memory limit 안에서 장시간 실행된다.
12. 기존 AI는 새 경로의 승인 완료 후에만 제거된다.
13. 대표적인 수치 변경과 새로운 효과 또는 전투 규칙 추가에서 기존 checkpoint를 활용한 재학습을 검증한다. 변경 후 처음부터 학습하는 경우보다 사전 목표 성능에 적은 학습 예산으로 도달하는지 독립 run으로 확인하고, 호환성 판정부터 평가까지의 반복 가능한 절차를 제공한다.

primary metric의 차이가 표본 오차 범위에 있을 경우 개선으로 승인하지 않는다. 정확한 seed 수와 통계 검정은 [`08-evaluation.md`](08-evaluation.md)에서 관리한다.

## 후속 목표

1차 목표가 검증된 뒤 다음 순서로 확장한다.

1. 밸런스 파라미터 민감도 측정
2. 좁은 범위의 configuration randomization
3. balance-conditioned policy
4. 실제 인간 행동 자료에 근거한 잘하는 사용자와 적당한 사용자 모델
