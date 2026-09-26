# 평가 계약

## 최종 지표

최종 모델 선택의 primary metric은 held-out gameplay seed에서의 full-clear rate다.

```text
full_clear_rate = full_clear_count / evaluated_episode_count
```

다음 값은 secondary diagnostic이다.

- 평균 및 분위수 stage progress
- 종료 시 HP와 shield
- 누수 피해
- tower 및 relic 선택 분포
- truncation rate와 원인
- action type별 regret
- episode당 decision과 tick
- simulation 및 inference throughput

secondary metric이 좋아도 full-clear rate가 낮은 모델을 우승 모델로 선택하지 않는다.

## Seed 분리

- training gameplay seed
- validation gameplay seed
- final test gameplay seed
- teacher future scenario seed

네 집합을 명시적으로 구분한다. seed 목록과 digest를 artifact에 저장한다. final test seed는 architecture, reward, hyperparameter 선택에 사용하지 않는다.

## 비교 방식

두 정책은 가능한 한 같은 gameplay seed에서 paired evaluation한다. 각 seed별 결과 차이를 보존하고 다음을 보고한다.

- full-clear count와 rate
- binomial confidence interval
- paired clear/loss 전환표
- paired bootstrap 또는 사전에 정한 paired test
- 최소 효과 크기

표본 수는 예상 승률과 판별하려는 최소 차이를 기준으로 정한다. 초기 개발 validation은 작은 고정 집합으로 빠르게 반복하고, 최종 승인은 더 큰 잠금 test 집합에서 수행한다.

## 단계별 gate

### Simulator gate

- legal action set equivalence
- replay hash equivalence
- normalized throughput 개선

### Teacher gate

- heuristic 대비 낮은 regret
- seed/horizon 증가 시 label 안정성
- held-out full-game 승률 개선

### Distillation gate

- teacher 대비 허용 가능한 regret
- production latency budget 충족
- teacher 없이 실행한 full-game 승률 유지

### RL gate

- BC checkpoint 대비 held-out full-clear rate 개선
- catastrophic regression 부재
- 최소 3회 독립 training run의 결과 분포 보고

### Balance gate

- configuration별 동일 seed 비교
- 정책의 해당 configuration 유효 범위 확인
- 통계와 artifact provenance 보존

## Candidate proposal 평가

후보를 top-K로 줄이는 단계가 생기면 oracle legal set을 기준으로 다음을 측정한다.

- teacher best candidate recall
- top-N valuable candidate recall
- action type별 recall
- pruning으로 발생한 expected regret
- 후보 수와 latency trade-off

card proposal에서 한 번 누락된 조합은 position scorer가 복구할 수 없으므로 build decision의 recall을 별도로 관리한다.

## 실패 분석

평균값 외에 다음 사례를 자동 수집한다.

- 이전 정책은 clear했지만 새 정책은 실패한 seed
- 큰 expert/student regret state
- build-placement pair 순위가 뒤집힌 state
- 잘못된 reroll 후 회복하지 못한 run
- 철거 또는 경로 변경으로 급격히 악화된 run
- policy entropy가 비정상적으로 붕괴한 decision point

각 사례는 replay, config digest, checkpoint hash로 재현 가능해야 한다.

## 최종 보고서

최종 report는 다음을 포함한다.

- Git revision과 dirty state
- 모든 contract version
- config digest와 RNG version
- checkpoint hash
- training run ID
- train/validation/test seed digest
- full-clear 통계와 confidence interval
- secondary diagnostic
- throughput
- baseline 대비 paired comparison
- 알려진 제한 사항
