# 0007: CPU simulation과 GPU 학습·배치 추론을 결합한다

- 상태: Accepted
- 결정일: 2026-09-18

## 결정

game simulation, legal action, pathfinding은 CPU에서 최적화한다. neural network 학습은 GPU를 기본 후보로 사용하고, inference는 CPU 즉시 실행과 GPU batching을 실제 end-to-end 처리량으로 비교한다.

Apple M1에서는 WGPU Metal backend를 사용한다. 원격 머신은 실제 GPU 사양을 확인한 뒤 지원 backend를 결정한다.

## 이유

게임 simulation은 분기와 불규칙한 mutable state 접근이 많아 GPU에 적합하지 않다. 반면 policy/value network의 batch forward와 backward는 GPU가 유리하다. 작은 단건 inference는 GPU dispatch와 batching 대기 때문에 CPU보다 느릴 수 있다.

## 결과

- 여러 CPU environment worker가 inference request를 batch queue에 전달한다.
- GPU learner와 rollout worker를 겹쳐 실행한다.
- device 선택은 kernel benchmark가 아니라 전체 decisions/sec, training wall time, queue latency와 memory로 결정한다.
- M1 16GB shared memory 한도 안에서 batch, rollout queue, optimizer state를 함께 관리한다.
