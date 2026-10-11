# 평가 계약

## 최종 지표

configuration별로 평가 지표를 실험 전에 고정한다. 현재 밸런스는 사용자 설명상 완주를 기대할 수 없으므로 primary metric은 실제 terminal까지 실행한 게임의 평균 진행도다.

```text
mean_terminal_progress = mean(terminal_clear_rate)
```

`terminal_clear_rate`는 기존 artifact의 진행도 필드(0–100)다. 이름에 clear rate가 있어도 여러 게임 중 완주한 비율을 뜻하지 않는다. 진행도 계산 계약과 version을 기록하며, 서로 다른 계산 계약의 결과를 직접 비교하지 않는다.

다음 값을 함께 보고한다.

- 진행도 중앙값과 하위 분위수, stage별 도달률·탈락 분포
- 동일 seed에서의 진행도 차이 및 개선/악화/동률 수
- 종료 시 HP와 shield, 누수 피해
- full-clear count와 rate (현재는 보조 통계)
- tower 및 relic 선택 분포
- truncation rate와 원인
- action type별 regret
- episode당 decision과 tick
- simulation 및 inference throughput

완주가 가능한 실험에서는 다음 값을 primary metric으로 사전에 선택할 수 있다.

```text
full_clear_rate = full_clear_count / evaluated_episode_count
```

실험 spec에는 primary metric, model-selection 순서, 최소 효과 크기, 하위 분위수 등의 허용 회귀 기준을 기록한다. 수치 기준은 실험 설계 시 고정한다. full-clear가 모두 0인 결과를 동률이라는 이유로 평가 불능이나 AI 실패로 처리하지 않는다. 모든 지표가 포화되면 최종 결과를 재해석하지 않고 다음 실험의 지표나 평가 configuration을 다시 설계한다.

실제 terminal에 도달하지 못한 safety-cap/truncation을 정상 패배나 승리로 섞지 않는다. 이를 별도 보고하고 승인 전에 원인과 평가 완전성을 확인한다. 오래 버티기나 decision 수 증가만으로 진행도 개선을 인정하지 않는다.

AI 평가와 밸런스 판단은 구분한다. 같은 configuration에서 기준 정책보다 강하다는 결과는 비교 대상에 대한 성능 증거다. 완주 0회는 완주 불가능성이나 최적 플레이의 증명이 아니다. 필요하면 별도 난이도의 진단용 configuration에서 규칙 이해와 학습 작동을 확인할 수 있지만, 그 결과가 현재 configuration의 성능을 대신하지 않는다.

## Seed 분리

- training gameplay seed
- validation gameplay seed
- final test gameplay seed
- teacher future scenario seed

네 집합을 명시적으로 구분한다. seed 목록과 digest를 artifact에 저장한다. final test seed는 architecture, reward, hyperparameter 선택에 사용하지 않는다.

## 비교 방식

두 정책은 가능한 한 같은 gameplay seed에서 paired evaluation한다. 각 seed별 결과 차이를 보존하고 다음을 보고한다.

- primary metric의 seed별 차이와 평균 차이
- paired bootstrap confidence interval 또는 사전에 정한 paired test
- 진행도 중앙값·하위 분위수와 stage별 생존 분포
- full-clear count/rate와 binomial confidence interval; 승패 비교 시 paired clear/loss 전환표
- 최소 효과 크기와 허용 회귀 기준

표본 수는 primary metric의 분산과 판별하려는 최소 차이를 기준으로 정한다. 초기 개발 validation은 작은 고정 집합으로 빠르게 반복하고, 최종 승인은 더 큰 잠금 test 집합에서 수행한다.

## 단계별 gate

### Simulator gate

- legal action set equivalence
- replay hash equivalence
- normalized throughput 개선

### Teacher gate

- heuristic 대비 낮은 regret
- seed/horizon 증가 시 label 안정성
- 사전에 정한 held-out full-game primary metric 개선

### Distillation gate

- teacher 대비 허용 가능한 regret
- production latency budget 충족
- teacher 없이 실행한 full-game primary metric의 허용 회귀 기준 만족

### RL gate

- BC checkpoint 대비 held-out primary metric 개선
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

- 이전 정책보다 진행도가 크게 하락한 seed; 완주 가능한 경우 승리에서 패배로 바뀐 seed
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
- primary metric의 정의/version, 목표와 허용 회귀 기준
- primary metric의 통계와 paired confidence interval
- full-clear 통계와 confidence interval
- secondary diagnostic
- throughput
- baseline 대비 paired comparison
- 알려진 제한 사항
