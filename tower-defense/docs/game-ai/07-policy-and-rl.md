# Policy와 RL

## 기본 구조

정책은 variable-cardinality legal candidate를 점수화한다.

```text
structured observation
    -> state/entity encoder
    -> candidate proposal and context
    -> joint candidate scorer
    -> legal candidate distribution
```

build-placement에서는 card subset representation으로 resulting tower context를 만들고, 그 context를 조건으로 position을 평가한다. 최종 선택은 여러 pair의 일관된 joint score를 비교한다.

## 첫 번째 모델 비교

다음 두 구조를 동일한 dataset, feature, candidate set, optimizer budget으로 비교한다.

1. 현재 방식에 가까운 Deep Sets encoder
2. 카드, 유물, tower context 사이의 관계를 처리하는 작은 attention encoder

attention은 전체 게임을 Transformer로 바꾸는 것이 아니다. 관계가 중요한 entity에만 제한한다. ID만 입력한 채 attention을 추가하는 실험은 semantic observation 개선을 대신하지 않는다.

CNN은 우선순위가 낮다. topology 실패가 확인되면 handcrafted route feature와 path graph/GNN을 비교한다.

## 계산 예산

기본 inference는 Apple M1 16GB에서 대량 병렬 simulation을 방해하지 않아야 한다.

- 모델 크기보다 decisions/sec와 full-clear 승률의 trade-off를 측정한다.
- CPU inference를 이식 가능한 baseline으로 유지한다.
- WGPU 사용은 실제 end-to-end throughput이 개선될 때 채택한다.
- 원격 머신은 동일 checkpoint와 contract를 읽을 수 있어야 한다.
- architecture별 parameter 수, inference latency, peak memory를 기록한다.

정확한 latency budget은 Phase 1 simulator throughput 결과를 기준으로 정한다. 환경보다 inference가 지배적이지 않도록 목표를 설정한다.

## PPO baseline

distillation checkpoint에서 PPO fine-tuning을 수행한다. 새 macro-action과 observation contract만 바꾸어도 기존보다 credit assignment가 쉬워질 수 있으므로 PPO를 먼저 공정하게 재평가한다.

확인할 항목은 다음과 같다.

- held-out full-clear 승률
- policy entropy와 KL
- value loss와 explained variance
- advantage distribution
- action type별 선택 비율과 regret
- BC checkpoint 대비 성능 변화
- rollout, optimization, validation 시간

BC가 괜찮고 PPO 후 나빠진다면 representation을 먼저 교체하지 않는다. reward scale, GAE, clip range, entropy, learning rate, KL, advantage normalization, value target을 우선 진단한다.

## Off-policy 비교 조건

PPO는 on-policy data를 반복 재사용하기 어렵다. simulator sample이 계속 비싸고 PPO가 teacher-pretrained policy를 개선하지 못할 때 discrete replay/off-policy 방식을 비교한다.

비교 시 다음을 고정한다.

- observation과 action contract
- teacher pretraining checkpoint
- environment transition budget
- wall-clock budget
- held-out seed
- 모델 크기 범위

연속 action용 알고리즘을 이름만 보고 적용하지 않는다. variable candidate set과 복합 discrete action을 직접 지원하거나 올바르게 변환할 수 있어야 한다.

## Production policy

대량 밸런스 simulation에서 기본 policy는 search를 호출하지 않는다. candidate generation, encoder, scorer만으로 행동을 선택한다.

search policy는 다음 용도로 별도 유지할 수 있다.

- teacher dataset 생성
- 어려운 state 분석
- distilled policy의 regret 진단
- 소수의 최고 품질 reference run

## 승인 기준

- 같은 teacher dataset에서 architecture ablation이 수행된다.
- 최종 pair score가 명확한 학습 target과 연결된다.
- M1 16GB에서 학습 또는 최소한 inference와 rollout이 memory limit 안에 동작한다.
- PPO가 BC 대비 승률을 개선하는지 분리해 보고한다.
- PPO가 실패할 때만 같은 예산의 off-policy 비교를 시작한다.
- production policy의 평가 경로에는 rollout search가 없다.
