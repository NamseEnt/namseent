# 밸런스 실험

## 시작 조건

이 단계는 fixed current balance에서 강한 policy가 검증된 뒤 시작한다. 기존 balance에만 학습된 policy를 큰 규칙 변경에 그대로 적용한 결과는 새로운 balance에서의 최선 플레이를 대표하지 않는다.

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

게임 플레이 AI의 목적은 full clear지만 밸런스 설계의 목적은 단일 승률 숫자만으로 충분하지 않다. balance experiment는 설계자가 지정한 목표와 제약을 별도로 가진다.

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
