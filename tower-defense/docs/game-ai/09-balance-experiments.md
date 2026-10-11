# 밸런스 실험과 기획 변경 후 재학습

> 현재 범위: 2026-10-09 사용자 지시에 따라 밸런스 조정·방법론과 사람 모델, 추가 재학습 연구는 후속 작업으로 관리한다. 현재 구현 목표는 [AI 반복 실행과 기존 통계 수집](23-batch-simulation-statistics.md)이며, 이 문서의 연구 gate를 통계 기능 완료의 선행 조건으로 두지 않는다.


## 시작 조건

밸런스 자동 최적화는 fixed current balance에서 강한 policy가 검증된 뒤 시작한다. 기존 balance에만 학습된 policy를 큰 규칙 변경에 그대로 적용한 결과는 새로운 balance에서의 최선 플레이를 대표하지 않는다.

기획 변경 후 재학습 지원은 [`00-goals-and-acceptance.md`](00-goals-and-acceptance.md)의 1차 목표에도 포함한다. 호환성과 모델 이관 절차는 현재 개발부터 설계하며, 아래 재학습 비교는 원본으로 사용할 기준 checkpoint가 확보되면 수행할 수 있다. 밸런스 자동 최적화의 완료를 기다릴 필요는 없다.

## 기획 변경 후 재학습 검증 계약

구현 항목과 순서는 [`21-retraining-implementation-plan.md`](21-retraining-implementation-plan.md)에서 관리한다.

### 변경 유형과 이관 범위

| 변경 유형 | 재학습 전에 필요한 작업 | 재사용 대상 |
| --- | --- | --- |
| damage, cooldown, 가격 등 수치 변경 | 새 configuration과 입력 수치 확인 | 의미와 구조가 호환되는 기존 모델 가중치 |
| 기존 효과 조합으로 표현되는 새 아이템 | 새 항목의 효과·대상·수치와 legal action 노출 확인 | 기존 encoder와 scorer의 호환 부분 |
| 새 효과, 광역공격 방식, 새 의사결정 추가 | core/simulator 규칙, 관측, candidate encoding 및 action contract 확장 | 의미가 유지되는 가중치; 새 입력·출력 부분은 별도 초기화 |

공격 효과가 관측 데이터에 존재하는 것과 실제 actor/critic 입력에 연결된 것은 별도로 확인한다. 새 아이템 ID만 추가했다고 AI가 새로운 효과를 판단할 수 있다고 가정하지 않는다. 공유 가중치의 이관이 적응을 방해하는 경우도 비교 결과에 기록한다.

### 실행 절차

1. 변경 전후 code revision, config digest, observation/action/feature version과 원본 checkpoint hash를 기록한다.
2. 입력과 출력의 의미 및 tensor 대응을 검사하고 재사용, 변환, 초기화할 부분을 명시한다. schema/provenance 검사를 우회하지 않고 명시적인 migration artifact를 만든다. critic과 optimizer 상태의 재사용 또는 초기화 여부도 기록한다.
3. 원본 run을 보존하고 변경 후 규칙용 새 run을 만든다. 동일 규칙에서의 정확한 resume과 이관 후 추가 학습은 별도 경로로 제공한다.
4. 변경된 규칙에서 rollout을 새로 수집한다. 과거 trajectory, value target, teacher label이 필요하면 새 규칙에서 다시 생성하거나 유효성을 검증한다. 과거 PPO rollout을 새 규칙의 on-policy 표본으로 사용하지 않는다.
5. 호환성 확인, 이관, 학습, 평가를 하나의 반복 가능한 명령 또는 workflow로 제공한다. 새 규칙 자체의 구현과 관측 추가에 필요한 개발 작업은 별도로 기록한다.

### 비교와 승인

- 대표적인 수치 변경과 새로운 효과 또는 전투 규칙 추가를 최소 하나씩 포함한다. 기획 변경의 구현이 정확한지는 재학습 성능 평가 전에 확인한다.
- 실험 전에 변경 내용, 새 규칙에 맞는 primary metric과 목표 성능, 최대 학습 예산, model-selection 규칙, gameplay seed split을 고정한다. 현재처럼 완주를 기대할 수 없는 밸런스에서는 진행도를 사용할 수 있다. 지표 정의·version과 허용 회귀 기준을 기록하고, 이전 규칙의 목표를 그대로 복사하지 않는다.
- 기존 모델 이관과 처음부터 학습하는 대조군을 변경 후 동일한 규칙, 목표 모델 구조, 대응하는 training seed와 평가 seed에서 비교한다. 대조군의 초기화·BC·critic 준비 비용을 포함한 시간과 데이터 예산을 명시한다.
- 최소 3회 독립 학습에서 목표 성능까지의 게임 수와 wall time, 같은 예산의 primary metric 성능 및 paired confidence interval을 보고한다. 변경 전 모델의 추가 학습 전 성능도 가능한 경우 측정한다.
- 목표 성능 도달에 필요한 게임 수와 wall time을 모두 기록하고, 시간 또는 데이터 중 무엇을 줄일지 사전에 정한다. 목표를 충족하면서 해당 예산의 절감이 재현되어야 빠른 재학습으로 승인한다. 목표 성능에 도달하지 못한 실험은 미달성으로 보고한다.
- 변경마다 필요한 수동 단계, 데이터 재생성, 모델 변환 및 준비 비용을 기록해 재학습을 쉽게 실행할 수 있는지도 평가한다.

이 계약은 향후 검증 요구사항이다. 기존 동일 규칙 PPO 재개 실험이나 개발 시드의 안정화 결과를 기획 변경 후 재학습 성공으로 간주하지 않는다.

## 최적화 대상

최종적으로 다음 항목을 실험할 수 있어야 한다.

- 유물 능력 수치
- 유물 등장 확률
- 웨이브별 monster 구성
- monster HP, 이동 속도, 피해량, 보상
- tower damage, range, cooldown
- 카드 강화 수치와 비용
- reroll, gold, dice 등 경제 파라미터
- 게임 난이도에 영향을 주는 기타 설정

현재 `GameConfig`는 player, tower, monster, stage wave의 주요 수치를 포함하지만 유물 능력과 등장 확률 등은 완전히 configuration-driven하지 않다. 먼저 최적화 대상 inventory를 만들고 authoritative parameter로 이동해야 한다.

## 실험 단위

각 balance candidate는 다음으로 식별한다.

- canonical parameter vector
- config schema version
- config digest
- game code revision
- policy checkpoint와 학습 configuration 범위
- gameplay seed digest

같은 parameter candidate를 중복 실행하지 않도록 결과 cache를 사용할 수 있다. code revision이나 contract가 다르면 같은 config digest만으로 cache를 재사용하지 않는다.

## 초기 접근

시작부터 모든 파라미터를 동시에 자동 최적화하지 않는다.

1. 한 변수 또는 작은 변수군의 sensitivity sweep
2. interaction이 예상되는 소규모 조합 실험
3. Latin hypercube, random search 또는 grid로 response surface 확인
4. 차원이 적고 실행 비용이 클 때 Bayesian optimization 검토
5. 충분한 training distribution이 확보된 뒤 balance-conditioned policy 검토

최적화 알고리즘은 simulator 처리량, 변수 차원, noise 수준을 측정한 뒤 선택한다.

## 목적 함수

게임 플레이 AI는 해당 configuration에서 사전에 정한 성과를 개선한다. 밸런스 설계에는 별도의 목표가 필요하며 단일 진행도나 승률 숫자만으로 충분하지 않다. balance experiment는 설계자가 지정한 목표와 제약을 별도로 가진다.

예시는 다음과 같다.

- 목표 사용자 모델의 clear rate 구간
- stage별 탈락 분포
- 특정 유물이나 tower의 과도한 지배 방지
- 선택지별 사용률과 성과 차이
- run 간 분산과 극단적인 불가능 seed 비율
- 플레이 시간 또는 decision 수 범위

목표 가중치는 자동으로 정하지 않는다. 설계자가 승인한 target profile을 versioned experiment spec으로 저장한다.

## 정책 적응 단계

### Fixed policy sensitivity

아주 좁은 수치 변화에서 기존 강한 policy의 민감도를 빠르게 본다. 이 결과는 정책 적응 전의 local sensitivity이며 최종 balance 판정이 아니다.

### Retrained policy comparison

중요한 candidate는 같은 training budget으로 재학습하거나 fine-tuning해 비교한다. 정책 학습 차이와 balance 차이를 분리한다.

### Balance-conditioned policy

충분히 좁고 명시적인 parameter 범위에서 다음 정책을 학습한다.

```text
policy(action | state, selected_balance_parameters)
```

training 범위 밖 configuration은 OOD로 표시하고 결과를 신뢰하지 않는다.

## 통계 출력

- full-clear rate와 confidence interval
- stage별 생존/탈락 분포
- HP, 누수, gold, reroll 사용량
- tower kind 및 위치 분포
- relic 선택률, 보유율, 조건부 승률
- 카드 강화와 족보 분포
- action type과 run 길이
- parameter별 sensitivity와 interaction
- policy/checkpoint/config provenance

조건부 승률은 선택 편향을 포함할 수 있으므로 유물 자체의 인과 효과로 단정하지 않는다. 가능한 경우 matched seed와 controlled intervention을 사용한다.

## 승인 기준

- 모든 조정 대상이 authoritative config 또는 versioned experiment override로 표현된다.
- candidate 비교가 같은 gameplay seed를 사용한다.
- policy가 해당 configuration을 학습하거나 검증한 범위가 명시된다.
- 한 숫자만 맞추면서 선택 다양성을 붕괴시키는 candidate를 자동 채택하지 않는다.
- 결과가 재현 가능한 artifact로 저장된다.
