# 시뮬레이터 성능 계획

## 목적

학습 알고리즘과 teacher가 충분한 경험을 생성할 수 있도록 authoritative headless simulation의 처리량을 높인다. 모델 구조를 고도화하기 전에 환경 transition 비용을 줄인다.

macro-action 변경과 성능 최적화는 함께 진행한다. UI micro-action 제거 자체가 observation 생성, legal action 생성, inference 횟수를 줄이므로 기존 action contract에서만 측정한 최적화 결과는 최종 처리량을 대표하지 않는다.

GPU는 simulation 자체를 대체하지 않는다. pathfinding, legal action 생성, mutable state transition처럼 분기와 작은 메모리 접근이 많은 작업은 CPU에서 최적화한다. GPU는 큰 tensor batch를 처리하는 학습과 batch inference에 집중한다.

## 기준 지표

판당 시간만 사용하지 않는다. 강한 정책은 더 오래 생존하므로 episode/sec가 오히려 감소할 수 있다.

필수 지표는 다음과 같다.

- simulation ticks/sec
- policy decisions/sec
- completed episodes/sec
- stage 또는 wave당 wall time
- path query count/sec
- path query당 latency
- legal candidate 생성 latency
- candidate validation latency
- observation encoding latency
- policy inference latency
- state clone 횟수와 clone bytes 추정치
- peak resident memory

모든 report에는 다음 metadata를 포함한다.

- Git revision
- configuration digest
- environment/action/RNG schema version
- build profile
- 머신과 thread 수
- policy 종류
- seed 집합
- max tick과 max decision

## 확인된 초기 병목

초기 placement legal action 생성은 coordinate discovery와 tower별 action 생성에서 `can_place_tower`를 반복 호출했다. 현재 구현은 공통 `TowerPlacementContext`를 만들고 coordinate당 topology 검사를 한 번만 수행한다. `can_place_tower`의 전체 state clone도 제거했으며, 후보 적법성에서는 경로 벡터를 만들지 않고 연결 가능성만 확인한다. 실제 설치 command는 여전히 완전한 route를 계산해 authoritative 결과를 만든다.

우선순위는 다음과 같다.

1. 같은 `(tower, coordinate, state revision)` validation 중복 제거 — 반영
2. coordinate-level topology legality와 tower-specific legality 분리 — 반영
3. 전체 state clone 없이 placement delta 검증 — 반영
4. blocker 변화가 없는 후보 사이의 route 결과 재사용
5. board revision 기반 cache invalidation
6. 필요할 경우 incremental path update 또는 더 적합한 path algorithm 검토

cache는 state mutation 이후 stale route를 반환해서는 안 된다. cache hit보다 correctness가 우선이며 state hash/replay test로 검증한다.

## 작업 단계

### P0: benchmark harness

- fixed seed와 fixed decision budget benchmark
- no-policy simulation과 policy 포함 simulation 분리
- path, candidate generation, observation, inference timing 분해
- 결과를 machine-readable report로 저장

### P1: macro-action overhead 제거

- 카드 선택당 반복 decision 제거
- build와 placement 사이의 추가 observation/inference 제거
- macro action 하나당 authoritative state transition 횟수 측정

### P2: placement validation 중복 제거

- legal coordinate 계산을 한 번 수행
- 같은 후보에 대한 중복 `can_place_tower` 호출 제거
- tower 특성이 topology legality에 영향을 주지 않는 부분을 공유

현재 브랜치에서 coordinate-level route existence 결과를 tower hand 전체가 공유하도록 구현했다.

### P3: clone과 path recalculation 축소

- placement가 변경하는 blocker delta만 계산
- path query input을 compact representation으로 분리
- 동일 blocker set 결과를 state revision 범위에서 재사용
- 실제 commit과 dry-run validation 결과가 동일한지 property test

현재 브랜치에서 state clone 제거와 existence-only path query를 구현했다. blocker-set cache와 board revision invalidation은 아직 남아 있다.

### P4: 병렬 rollout

- episode별 독립 RNG와 deterministic result 유지
- worker당 mutable environment 소유
- shared immutable configuration 사용
- memory pressure와 scheduling overhead를 thread 수별로 측정

### P5: CPU-GPU pipeline

- 여러 CPU worker가 observation과 legal candidate를 생성
- inference 요청을 크기 또는 짧은 latency window로 batching
- GPU가 batched policy/value inference 수행
- 결과를 원래 environment와 decision sequence에 정확히 반환
- CPU simulation과 GPU inference를 겹쳐 실행
- queue 대기 시간, batch 크기, GPU utilization, end-to-end decisions/sec 측정

단건 GPU dispatch는 작은 모델에서 CPU보다 느릴 수 있다. GPU 경로의 채택 기준은 kernel 시간만이 아니라 queue와 tensor transfer를 포함한 전체 처리량이다. M1의 unified memory도 논리적 tensor 변환과 dispatch 비용을 제거하지는 않는다.

### P6: teacher batch evaluation

- candidate별 continuation을 독립 environment 전체 clone으로 시작하지 않도록 snapshot 비용 측정
- copy-on-write, compact snapshot, state delta 중 가장 단순하고 빠른 방식을 benchmark로 선택
- 같은 scenario seed를 candidate batch에 효율적으로 배포

## 장치별 책임

| 작업 | 기본 장치 | 이유 |
| --- | --- | --- |
| Game state transition | CPU | 분기와 mutable state가 많음 |
| Legal action 생성 | CPU | authoritative rule과 작은 불규칙 작업 |
| Pathfinding과 placement validation | CPU | graph 탐색과 cache 중심 |
| Episode 병렬 실행 | CPU | environment 간 독립성이 높음 |
| BC/distillation 학습 | GPU | 큰 tensor batch 연산 |
| PPO 또는 후속 RL update | GPU | forward/backward batch 연산 |
| 단일 environment inference | CPU baseline | GPU dispatch 비용과 비교 필요 |
| 다수 environment/candidate inference | Batched GPU 후보 | batch가 충분할 때 높은 처리량 가능 |

Apple M1에서는 WGPU Metal backend를 사용 후보로 둔다. 원격 머신은 GPU vendor, driver, memory를 확인하기 전까지 backend를 확정하지 않는다.

## 초기 성능 목표

2026-09-17 임시 smoke benchmark의 약 60초 결과를 기준으로 다음을 초기 engineering target으로 둔다.

- 1차: 같은 benchmark 조건에서 5초 미만
- 확장: 가능한 경우 1초 미만

이 시간 목표는 normalized throughput을 대체하지 않는다. action contract가 바뀌면 decision 수가 달라지므로 ticks/sec, path query count, candidate latency 개선을 함께 통과해야 한다.

## correctness 불변 조건

- 한 simulation step은 정확히 한 `SimTick`을 전진한다.
- fast-forward는 step 수만 바꾸며 tick duration을 바꾸지 않는다.
- rendered와 headless 실행은 같은 authoritative simulation-step 함수를 사용한다.
- 동일 seed, config, macro action sequence는 동일 state hash를 만든다.
- cache 유무가 legal action과 전투 결과를 바꾸지 않는다.
- 최적화 때문에 불법 placement가 허용되거나 합법 placement가 누락되지 않는다.

## 승인 기준

- benchmark report가 재현 가능하다.
- placement candidate 생성에서 같은 validation이 중복 실행되지 않는다.
- path query 수와 latency가 단계별로 보고된다.
- 새 macro-action 기준 normalized throughput이 baseline보다 개선된다.
- 목표 시간 또는 그에 준하는 병목 제거 근거가 있다.
- deterministic replay와 legal-set equivalence test가 통과한다.
- CPU-only와 CPU-GPU pipeline을 같은 workload로 비교한다.
- GPU 경로는 end-to-end decisions/sec 또는 training wall time을 실제로 개선할 때만 기본값으로 채택한다.
- M1 16GB에서 queue, rollout state, tensor와 optimizer를 포함한 peak memory가 한도를 넘지 않는다.
