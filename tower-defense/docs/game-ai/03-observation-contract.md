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
- base damage(`damage_raw`), range, cooldown
- card polish와 template에서 확정 가능한 upgrade bonus를 적용한 `effective_damage_raw`
- engraving에서 파생된 공격 효과
- 현재 유물이 적용된 뒤 확정할 수 있는 modifier
- reroll count처럼 tower 결과에 영향을 주는 run state

이 값은 policy가 독자적으로 포커 규칙을 추측해서 만들지 않는다. authoritative rule이 candidate context를 계산한다.

`effective_damage_raw`는 `damage_raw`에 card polish와 `UpgradeCollection::tower_upgrade_bonus_raw_for_template`이 돌려주는, 해당 template만으로 확정 가능한 upgrade bonus를 적용한 값이다. `TowerState::attack_damage_raw`와 같은 authoritative 계산을 공유하며 simulator에서 재구현하지 않는다. 단, `NameTag`처럼 실제 `PlaceTower` 실행 중 배정되는 tower ID에 의존하는 placement-trigger 효과는 template 시점에 알 수 없으므로 포함하지 않는다 - 그런 upgrade가 있으면 `effective_damage_raw`는 실제 배치 이후의 `attack_damage_raw`보다 작을 수 있다. `damage_raw`와 `effective_damage_raw`가 이 두 값을 구분해서 노출하므로, policy는 `owned_upgrades`의 runtime parameter(예: `NameTag`의 미배정 상태)로 그 gap을 추론할 수 있다. placement를 실제 실행해서 얻은 값이 아니다.

현재 구현에서는 `Observation.build_tower_candidates`가 이 context를 제공한다. 각 항목은 canonical card slot subset과 authoritative tower template을 함께 가진다. 전체 hand을 사용하는 subset은 빈 slot 목록으로 표현하며, 관측은 `Shopping`과 `SelectingTower` 상태에서 생성된다. 따라서 `BuildTower` candidate encoder는 card subset을 다시 계산하지 않고 해당 resulting tower를 직접 참조한다.

semantic macro-action이 shop에서 바로 선택될 수 있으므로 정책은 `StartSelectingTower`를 먼저 고른 뒤 resulting tower를 추론할 필요가 없다. historically 이 변경으로 observation schema는 4, feature schema는 8로 올랐다.

`TowerTemplateObservation`에 `effective_damage_raw`를 추가하면서(card polish/upgrade damage bonus double-count correctness fix 포함) observation schema를 다시 올렸다.

현재 값은 `simulator/src/ml/contract.rs`가 유일한 출처다: `OBSERVATION_SCHEMA_VERSION` 7, `FEATURE_SCHEMA_VERSION` 11, `DATASET_SCHEMA_VERSION` 5, `ML_CONTRACT_SCHEMA_VERSION` 2. 이전 checkpoint와 dataset은 자동으로 혼용하지 않는다.

이 목록은 placement 결과를 미리 실행한 값이 아니다. 위치별 route, occupancy, coverage feature는 candidate template과 별도로 현재 map에서 계산한다. 미래 RNG나 search 전용 값도 포함하지 않는다.

## 유물과 upgrade

단순 `relic_id`만 제공하지 않는다. 현재 observation은 upgrade key와 key ID에 더해 실행 중인 수치 상태를 제한된 semantic representation으로 제공한다.

```text
behavior_id
relevant_tags
numeric_parameters
stack_count
```

구체적으로 `OwnedUpgradeObservation`은 `scalar_values`, `ratio_values`, `bool_values`를 제공한다. encoder는 이를 고정 폭 numeric row로 정규화한다. 이 값들은 게임 실행 semantics 전체를 복제하지 않고 현재 판단에 필요한 upgrade runtime parameter만 노출한다.

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

이 구분은 checkpoint metadata에도 명시된다. `simulator/src/ml/contract.rs`의 `MlContract::balance_scope`(타입 `PolicyBalanceScope`)는 `Fixed` 또는 `Conditioned { parameters: Vec<ConditionedBalanceParameter> }` 중 하나다. `Fixed`는 해당 policy가 `config_digest` 하나로 식별되는 정확히 하나의 balance configuration에서만 학습·검증되었음을 뜻하며, 어떤 balance parameter도 policy 입력으로 randomize되거나 노출되지 않는다. `Conditioned`는 아직 구현하지 않은 미래 balance-conditioned policy를 위한 자리로, `parameters`는 policy가 학습 시 실제로 관측한 각 parameter의 이름과 raw range(`min_raw`/`max_raw`)를 기록한다. Phase 2에서 생성하는 모든 contract/checkpoint의 `balance_scope`는 반드시 `Fixed`이며, 실제로 지원하지 않는 conditioned range를 가짜로 채우지 않는다. 필드가 없는 legacy contract는 `#[serde(default)]`로 `Fixed`로 역직렬화된다. 이 필드 추가로 `ML_CONTRACT_SCHEMA_VERSION`을 2로 올렸다.

## 정규화와 누락값

- 모든 numeric feature는 단위, scale, clipping 범위를 계약에 기록한다.
- categorical vocabulary는 안정적인 key와 schema version을 사용한다.
- 누락과 값 0을 같은 표현으로 합치지 않는다.
- variable-cardinality entity set의 multiplicity를 보존한다.
- typed entity numeric row는 upgrade runtime parameter를 포함할 수 있도록 6폭으로 zero-padding한다. 모델의 entity input도 같은 폭을 사용한다.
- feature를 제거하거나 의미를 변경하면 `FEATURE_SCHEMA_VERSION`을 증가시킨다. historically upgrade parameter row 폭 변경으로 feature schema를 7로 올린 적이 있으며, 이후 `effective_damage_raw` 추가로 다시 올라 현재 `FEATURE_SCHEMA_VERSION`은 11이다(`simulator/src/ml/contract.rs`가 유일한 출처). 이전 checkpoint와 섞지 않는다.
- damage/effective damage/tower range/cooldown·shoot interval/card polish/route progress/rerolled count/upgrade scalar·ratio·bool처럼 고정 scale을 쓰는 feature family는 `simulator/src/ml/encoding/normalize.rs`의 공유 `normalize_*`/`denormalize_*` 헬퍼로 나눗셈 상수를 한 곳에 고정하고, 각 family의 raw -> normalized -> raw round-trip을 `normalize.rs`의 단위 테스트로 검증한다. position x/y, route coord index처럼 가변 extent(map 크기, route 길이)로 나누는 family는 같은 파일의 `normalize_axis_ratio`/`denormalize_axis_ratio`로 검증한다. categorical vocabulary(수트/랭크/engraving/upgrade key 등)와 hp 비율처럼 두 raw 값의 비율인 feature는 decoder가 의미 없는 lossy/비-scale 값이므로 round-trip 대상에서 제외한다.

## 승인 기준

- teacher decision에 사용된 현재 정보가 observation에서 표현 가능하다 (`Observation`/`TypedObservation`/`DenseBuildFeatureBundle`가 card identity, resulting tower context, 유물 runtime parameter, map/route/wave/자원 상태를 모두 노출한다).
- 미래 RNG 또는 hidden order가 포함되지 않는다 (`build_tower_candidates`/`extra_tower_card_templates`는 authoritative core가 계산한 template이며 실제 배치를 실행하지 않는다; `docs/game-ai/03-observation-contract.md` 정보 경계 참고).
- 동일 의미의 entity가 hand reorder 후에도 stable identity를 유지한다 (`ml::encoding::dense_build::tests::subset_identity_is_stable_across_hand_reorder`).
- resulting tower context가 card subset마다 정확히 계산된다 (`ml::encoding::dense_build::tests::template_row_matches_observation_for_every_subset_and_slot`).
- 유물 parameter 변경이 observation 값 변화로 나타난다 (`ml::encoding::observation::tests::upgrade_scalar_runtime_change_is_reflected_in_encoding`, `..._ratio_...`, `..._bool_...`; scalar/ratio/bool 표현 방식을 각각 한 번씩 검증).
- feature 단위와 normalization의 round-trip test가 있다 (`ml::encoding::normalize::tests::*`).
- fixed configuration 범위와 conditioned configuration 범위가 checkpoint metadata에 구분된다 (`MlContract::balance_scope: PolicyBalanceScope`; `ml::contract::tests::contract_from_config_is_fixed_balance_scope`, `legacy_contract_without_balance_scope_field_defaults_to_fixed`, `conditioned_balance_scope_round_trips_through_json`).
