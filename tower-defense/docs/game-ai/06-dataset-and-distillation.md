# Dataset과 Distillation

## 목적

느린 rollout teacher의 판단을 대량 시뮬레이션에 사용할 수 있는 빠른 policy로 압축한다. 단일 정답 action만 저장하는 behavior cloning dataset보다 후보별 가치와 불확실성을 보존한다.

## Dataset record

각 decision record는 최소한 다음 정보를 포함한다.

- environment, action, observation, feature, RNG schema version
- Git revision과 configuration digest
- episode seed와 decision sequence
- observation 또는 재현 가능한 state reference
- deterministic ordering의 전체 legal candidate ID
- pruning 전 후보 수와 pruning 후 후보 수
- 후보별 teacher sample count
- 후보별 mean outcome, variance, standard error
- teacher가 선택한 action
- 기존 heuristic이 선택한 action
- expert regret
- horizon과 continuation policy version
- future scenario seed schedule digest

build-placement는 카드 label과 위치 label을 따로만 저장하지 않는다. 최종 pair identity와 joint teacher value를 보존한다.

현재 최소 teacher report는 각 decision에 observation, 후보별 통계, teacher 선택 action과 value, scripted baseline action과 value, `expert_regret`을 함께 저장한다. 이 JSON은 아직 train/validation dataset loader가 아니며, 다음 단계에서 seed split과 schema validation을 추가해야 한다.

## Split 규칙

decision row를 무작위로 나누지 않는다. 같은 episode에서 나온 인접 state가 train과 validation에 동시에 들어가면 누수가 발생한다.

분리는 가장 바깥의 gameplay seed 기준으로 수행한다.

- train seeds
- model-selection validation seeds
- 최종 test seeds

teacher scenario seed는 gameplay seed와 별도 domain으로 관리한다. 최종 test gameplay seed는 dataset 생성과 hyperparameter 선택에 사용하지 않는다.

## Target

다음 target을 비교한다.

- teacher best action에 대한 hard classification
- 후보별 expected return regression
- 후보 pair ranking
- teacher uncertainty를 반영한 soft target
- state value auxiliary target

teacher 상위 후보의 차이가 표준 오차보다 작으면 하나의 확정 정답으로 과도하게 학습시키지 않는다. tie 또는 soft target으로 처리한다.

## Sampling

빈번한 `Continue`와 쉬운 행동이 dataset을 지배하지 않게 한다. 다음 축으로 분포를 기록하고 필요한 경우 stratified sampling을 사용한다.

- decision point
- stage 구간
- 성공/실패 trajectory
- teacher regret 구간
- candidate count
- build hand type
- relic 및 upgrade 조합

oversampling을 사용해도 validation과 최종 평가의 자연 분포는 바꾸지 않는다.

## Distillation 승인 기준

- 전체 accuracy뿐 아니라 action type별 top-1/top-k accuracy를 보고한다.
- teacher value 기준 student regret을 보고한다.
- build-placement candidate recall과 pair ranking 품질을 보고한다.
- teacher trajectory와 student trajectory의 held-out full-game 승률을 비교한다.
- student inference latency와 memory를 측정한다.
- dataset 생성 코드와 loader가 schema mismatch를 거부한다.

BC accuracy가 높아도 teacher 자체의 승률이 낮으면 성공으로 인정하지 않는다. 반대로 teacher action과 일부 다르더라도 full-clear 승률이 같거나 높고 regret이 낮다면 정책 후보로 유지할 수 있다.
