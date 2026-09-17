# 인간 플레이어 모델

## 상태

이 문서는 후속 목표를 보존하기 위한 설계 경계다. 강한 기본 AI가 완성되기 전에는 구현하지 않는다.

## 목표

최종적으로 다음과 같은 최소 두 실력 수준을 만들 수 있어야 한다.

- 게임을 매우 잘하는 사용자
- 적당한 수준으로 플레이하는 사용자

두 모델의 차이는 단순한 random action noise가 아니다. 실제 사람처럼 다음과 같은 제한에서 발생해야 한다.

- 중요한 요소를 고려 대상에서 빠뜨림
- 카드, 유물, 배치 간 관계를 잘못 평가함
- 후보를 충분히 탐색하지 않음
- 장기 결과보다 짧은 결과를 과대평가함
- 시간 제한 때문에 조기에 결정을 확정함
- 드문 규칙이나 상호작용을 잘못 기억함

## 강한 AI와의 관계

강한 AI는 인간 모델의 기준 evaluator 역할을 한다. 인간 모델은 강한 policy action에 확률적으로 오류를 섞는 방식만으로 만들지 않는다. 대신 판단 과정의 일부를 제한한다.

가능한 모델 축은 다음과 같다.

- observation omission: 일부 feature 또는 관계를 보지 못함
- limited candidate generation: 유망해 보이는 소수 후보만 고려함
- shallow horizon: 먼 웨이브 가치를 덜 반영함
- imperfect value model: 특정 유물이나 족보를 체계적으로 과대/과소평가함
- bounded compute: decision당 제한된 평가 횟수 사용
- stale belief: 최근 balance 변경을 완전히 반영하지 못함

## 정의에 필요한 증거

현재는 실제 사용자 실수 분포가 없으므로 잘하는 사용자와 적당한 사용자의 정확한 parameter를 확정할 수 없다. 다음 자료가 생기면 calibration한다.

- 사용자 replay
- action별 생각 시간
- 고려한 후보와 실제 선택
- stage별 실패 원인
- 카드/유물/배치 유형별 반복 실수
- 숙련도별 clear rate와 행동 분포

자료가 없을 때는 모델을 `human-like`라고 단정하지 않고 `bounded-compute synthetic profile`로 표시한다.

## 평가

- 목표 clear-rate 구간
- 강한 AI 대비 regret 분포
- 실수 유형별 빈도
- action type별 선택 분포
- 동일 state에서의 선택 일관성
- 실제 사용자 데이터와의 calibration error

적당한 모델은 단순히 모든 행동을 더 나쁘게 선택하는 모델이 아니다. 쉬운 상황에서는 강한 모델과 같고, 관계가 복잡하거나 탐색이 필요한 상황에서 특정한 방식으로 실패해야 한다.

## 구현 시작 조건

- 강한 fixed-balance policy가 검증됨
- action regret evaluator가 존재함
- candidate와 observation을 제한할 수 있는 ablation path가 존재함
- 최소한의 사용자 replay 또는 명시적으로 승인된 synthetic behavior 가설이 있음
