# AI 행동 계약

## 목표

정책이 UI 조작 절차가 아니라 게임의 전략적 결정을 한 번에 선택하게 한다. authoritative core가 legality와 실제 state transition을 소유하는 원칙은 유지한다.

## 정책에 노출할 결정

정책 수준의 행동은 다음 의미 단위를 갖는다.

- 상점 항목 구매
- 카드 subset reroll
- 카드 subset으로 tower를 만들고 위치에 배치
- 설치된 tower 철거
- defense 시작
- 유물 선택 또는 폐기
- 카드 서비스 대상 선택
- 아이템 사용
- 자동 진행이 필요한 구간의 continue

`Begin*`, 카드 한 장 선택, 선택 취소, 확인과 같은 UI용 단계는 정책 action space에서 제거한다.

## 안정적인 entity identity

정책 action과 trajectory에는 가능한 한 card ID, tower ID, upgrade ID처럼 상태 변화에도 의미가 유지되는 identity를 사용한다. mutable vector index와 hand slot index는 authoritative command로 변환하는 순간에만 해석한다.

bounded subset의 내부 최적화로 bitmask를 사용할 수 있지만 다음 조건을 지켜야 한다.

- bit 위치와 stable entity ID의 mapping이 해당 decision record에 저장된다.
- hand reorder 이후 같은 mask를 재사용하지 않는다.
- dataset과 replay의 외부 계약은 stable identity를 기준으로 한다.

## Reroll

정책 행동은 reroll할 카드 집합 전체다.

```text
Reroll(card_ids)
```

기본 hand처럼 후보 수가 작은 경우 legal subset을 전부 생성한다. hand slot 확장으로 후보 수가 커질 때만 사전에 정한 candidate proposal 또는 beam limit을 적용한다.

정책은 reroll을 선택한 뒤 카드별 `SelectHandCard`와 `ConfirmCardSelection`을 별도 decision으로 수행하지 않는다.

## Build와 placement

카드 조합과 위치는 전략적으로 결합된 하나의 결정이다.

```text
P(cards, position | state)
  = P(cards | state)
  * P(position | state, cards, resulting_tower)
```

이 factorization은 계산 구조이지 greedy 결정 순서가 아니다. 다음 구현은 허용하지 않는다.

```text
best_cards = argmax P(cards | state)
best_position = argmax P(position | state, best_cards)
```

최종 선택은 여러 카드 조합과 각 조합의 placement를 결합한 `(card_ids, position)` 후보 사이에서 이루어진다.

### 후보 생성

1. authoritative rule로 legal card subset을 생성한다.
2. 각 subset의 resulting tower를 계산한다.
3. resulting tower별 legal placement를 생성한다.
4. 전체 pair가 작으면 모두 평가한다.
5. 전체 pair가 계산 예산을 넘을 때만 card subset top-K와 position top-K를 사용한다.

기본 hand가 5장인 구간에서는 card subset을 우선 전수 평가한다. pruning은 실제 후보 수와 처리량 측정으로 필요성이 확인된 뒤 도입한다.

현재 simulator에는 `AgentAction::BuildTower`와 semantic 실행 경로가 추가되었다. authoritative oracle은 legal position 전체와 card subset 전체의 pair를 생성한다. 실제 정책 benchmark는 route 근접도 deterministic proposal을 사용해 position을 최대 `DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT`(현재 64)개로 제한하며, 이 제한은 후보 recall을 별도로 검증해야 하는 provisional 단계다.

`cargo test --release -- --ignored phase1_candidate_recall_report`(`simulator/src/teacher.rs`)로 24 seed × 6 decision point = 144 sample을 측정했다. rollout이 37,000~185,000개에 달하는 전체 oracle `BuildTower` 후보에 대해 불가능하므로, `rank_build_tower_actions_by_heuristic`(coverage, route distance, damage 순)의 rollout-free ranking을 oracle proxy로 사용했다.

| position_limit | mean legal recall | exact-best retention | top-5 retention | top-10 retention | mean coverage regret | max coverage regret |
| --- | --- | --- | --- | --- | --- | --- |
| 8 | 0.68% | 0% | 0% | 0% | 7.14 | 8 |
| 16 | 1.35% | 0% | 0% | 0% | 6.73 | 7 |
| 32 (이전 기본값) | 2.71% | 0% | 0% | 0% | 6.73 | 7 |
| 48 | 4.06% | 0% | 25.8% | 12.9% | 2.13 | 6 |
| 64 (현재 기본값) | 5.41% | 100% | 80% | 80% | **0.00** | **0** |

`mean legal recall`(전체 legal action 대비 proposal 크기)은 핵심 지표가 아니다 — 목적은 legal action을 많이 보존하는 게 아니라 좋은 행동을 잃지 않는 것이다. 32 이하에서는 144 sample 전부(share_with_nonzero_regret=100%)에서 coverage regret이 발생했고, 64에서는 144 sample 전부 regret이 0이었다. release benchmark로 측정한 처리량 영향은 잡음 범위 내(동일 seed 2회 실행 기준 decisions/sec 9.19 vs 9.51, candidate_evaluations는 1.7배 증가했지만 벽시계 시간은 거의 동일)로, position 후보 생성 자체가 decision당 지배적 비용이 아님을 시사한다.

따라서 `DEFAULT_SEMANTIC_POSITION_CANDIDATE_LIMIT`을 64로 올렸다. 이것을 최종 설계로 확정하지는 않는다: 원인은 route-distance 기준 position 정렬이 "route에 가장 가까운 칸"과 "route를 가장 많이 커버하는 칸"을 동일시하지 않는 것으로 보이며(48에서도 35.4%는 regret 0이지만 64.6%는 여전히 손실), 이는 모든 card subset이 하나의 공유 순서를 재사용하는 현재 구조와 얽힌 설계 문제다. 64에서 regret이 0으로 확인되었으므로 지금 이 순서 자체를 재설계하지는 않지만, 추후 근거가 쌓이면 route-distance / coverage / tower-range별 순위의 top-K union 같은 저비용 hybrid 방식을 검토한다. 결과는 `artifacts/benchmarks/phase1-candidate-recall.json`에 저장했다.

### 최종 pair 점수

서로 다른 의미의 값을 임의로 곱하지 않는다. 예를 들어 `P(cards)`와 별도 heuristic coverage score를 곱해 joint value라고 부르지 않는다.

최종 비교에는 다음 중 하나의 일관된 기준을 사용한다.

- end-to-end로 학습된 joint log probability
- `(state, cards, position)`에 대한 joint Q estimate
- 같은 rollout 계약으로 측정한 expected return

proposal head의 점수는 후보 축소에 사용할 수 있지만 최종 가치와 구분해 기록한다.

## 환경 전이

정책은 build-placement를 한 번 결정하고 환경은 추가 policy inference 없이 authoritative transition을 완료한다. core 내부에서 tower 생성과 placement가 두 command로 남더라도 외부 policy horizon에는 한 decision으로 기록한다.

현재 semantic 실행은 이 macro action을 내부의 `SelectTower`와 `PlaceTower` command로 확장해 최종 상태를 만든다. policy trace에는 외부 macro identity를 한 단계로 기록하고, authoritative replay에는 내부 command sequence를 보존한다.

macro transition은 부분 적용 상태를 남겨서는 안 된다. placement legality가 바뀌거나 command가 실패하면 카드만 소비된 상태가 남지 않아야 한다. 구현 전에 다음 중 비용과 core ownership에 맞는 방식을 선택한다.

- core에 atomic build-and-place command 추가
- mutation 전 authoritative prevalidation과 실패 불가능한 commit 경로 추가
- transaction 형태의 state delta 계산 후 commit

hot path에서 전체 state clone으로 atomicity를 달성하는 방식은 성능 측정 없이 채택하지 않는다.

## Legal action generator

- legal action은 authoritative game rule에서 생성한다.
- 정책은 불법 후보를 출력한 뒤 보정받는 방식으로 학습하지 않는다.
- candidate ordering은 deterministic해야 한다.
- stable ID, coordinate, action kind 순서 등 ordering rule을 schema에 명시한다.
- candidate proposal을 도입할 경우 oracle legal set 대비 candidate recall을 측정한다.

## Replay와 schema

이 변경은 최소한 environment, action, trajectory, dataset, feature, policy checkpoint contract를 변경한다. 기존 버전을 재사용하지 않는다.

replay에는 policy-level macro action과 최종 authoritative state hash를 기록한다. 내부적으로 여러 core transition을 사용하더라도 동일 macro action replay가 같은 최종 hash를 만들어야 한다.

## 승인 기준

- 카드 reroll과 build에 UI micro-action decision이 없다.
- build-placement 사이에 추가 inference가 없다.
- 테스트가 greedy card-first 구현을 탐지한다.
- 작은 synthetic state에서 모든 legal pair를 열거한 결과와 factorized evaluator의 최종 선택이 일치한다.
- pair score의 단위와 학습 target이 문서화되어 있다.
- illegal action과 partial mutation이 발생하지 않는다.
- deterministic candidate ordering과 replay가 검증된다.
