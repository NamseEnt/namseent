# AI 관측 계약

## 목표

AI가 현재 상태에서 합법적으로 알 수 있는 전략 정보를 명시적으로 표현한다. teacher가 어떤 현재 정보를 사용해 행동을 선택했다면 distilled policy도 그 의미를 입력으로 받을 수 있어야 한다.

관측은 게임 실행 semantics를 복제하는 범용 DSL이 아니다. 정책이 결정을 구분하는 데 필요한 의미만 제공한다.

## 정보 경계

포함한다.

- 현재까지 공개된 game state
- 플레이어가 알 수 있는 deck 구성 정보
- 현재 카드의 강화 및 engraving
- 보유 유물과 발동에 필요한 현재 파라미터
- 선택한 카드로 생성되는 tower의 확정 가능한 속성
- 현재 map, route, tower, monster, wave, 자원 상태
- 현재 configuration에서 이미 플레이어에게 알려진 규칙

포함하지 않는다.

- 아직 뽑히지 않은 다음 카드의 실제 순서
- 미래 random event 결과
- 다른 후보를 선택했을 때만 소비될 실제 RNG 값
- simulator 내부 cache와 검색용 private state
- 사람 플레이어에게 공개되지 않는 미래 정보

## Card entity

각 카드는 최소한 다음 의미를 가진다.

- stable card ID
- suit와 rank
- polish 또는 영구 강화 수치
- engraving 종류와 관련 수치
- 현재 위치 구분: hand, draw, discard

정책 action은 card ID를 사용하며 observation encoder가 필요에 따라 categorical ID와 numeric parameter로 분리한다.

## Resulting tower context

각 build subset 후보에 대해 placement scorer가 다음 context를 받을 수 있어야 한다.

- 사용한 card ID 집합
- 결과 족보/tower kind
- suit/rank 관련 속성
- 실제 damage, range, cooldown
- engraving에서 파생된 공격 효과
- 현재 유물이 적용된 뒤 확정할 수 있는 modifier
- reroll count처럼 tower 결과에 영향을 주는 run state

이 값은 policy가 독자적으로 포커 규칙을 추측해서 만들지 않는다. authoritative rule이 candidate context를 계산한다.

## 유물과 upgrade

단순 `relic_id`만 제공하지 않는다. 조정 가능한 효과에는 다음 형태의 제한된 semantic representation을 사용한다.

```text
behavior_id
relevant_tags
numeric_parameters
stack_count
```

예를 들어 특정 suit damage bonus라면 behavior, suit tag, multiplier가 구분되어야 한다. 복잡하고 고유한 유물은 behavior ID와 정책 판단에 필요한 핵심 파라미터만 제공할 수 있다.

게임 실행 로직 전체를 trigger/condition/operation 언어로 다시 표현하지 않는다. effect의 논리 구조가 바뀌어 기존 representation이 거짓이 되면 observation schema를 증가시키고 재학습한다.

## 보드와 topology

초기 구현은 엔진이 정확히 계산 가능한 구조화 정보를 유지한다.

- map 크기와 blocked tile
- 현재 route 좌표와 progress
- tower 위치, 범위, cooldown, damage, 종류
- monster 위치, route progress, HP, 속도, 피해량
- 후보 위치의 coverage와 route 관련 engineered feature

전체 grid image용 CNN은 우선 도입하지 않는다. handcrafted feature로 부족한 topology 관계가 계측된 경우 path/navigation graph encoder 또는 GNN을 후속 비교한다.

## 웨이브와 장기 상태

- 현재 stage와 wave 상태
- active/queued monster 수와 종류
- 공개된 wave composition
- player HP, shield, gold, dice/reroll 자원
- shop, inventory, treasure capacity
- 이미 설치된 tower와 철거 가능 대상

장기 판단에 필요한 공개 정보가 configuration에만 있고 observation에 없다면 명시적으로 추가한다.

## Balance configuration

Phase 1 정책은 현재 balance configuration에 고정한다. 모든 balance parameter를 처음부터 입력으로 넣지 않는다.

확장 순서는 다음과 같다.

1. fixed configuration에서 강한 policy
2. 좁은 parameter range randomization
3. 선택한 parameter만 포함한 balance-conditioned policy

configuration을 바꾼 뒤 fixed policy 결과를 그대로 새 밸런스의 강한 플레이 결과로 간주하지 않는다. 변경 범위가 policy의 검증 범위를 벗어나면 재학습하거나 conditioned policy를 사용한다.

## 정규화와 누락값

- 모든 numeric feature는 단위, scale, clipping 범위를 계약에 기록한다.
- categorical vocabulary는 안정적인 key와 schema version을 사용한다.
- 누락과 값 0을 같은 표현으로 합치지 않는다.
- variable-cardinality entity set의 multiplicity를 보존한다.
- feature를 제거하거나 의미를 변경하면 `FEATURE_SCHEMA_VERSION`을 증가시킨다.

## 승인 기준

- teacher decision에 사용된 현재 정보가 observation에서 표현 가능하다.
- 미래 RNG 또는 hidden order가 포함되지 않는다.
- 동일 의미의 entity가 hand reorder 후에도 stable identity를 유지한다.
- resulting tower context가 card subset마다 정확히 계산된다.
- 유물 parameter 변경이 observation 값 변화로 나타난다.
- feature 단위와 normalization의 round-trip test가 있다.
- fixed configuration 범위와 conditioned configuration 범위가 checkpoint metadata에 구분된다.
