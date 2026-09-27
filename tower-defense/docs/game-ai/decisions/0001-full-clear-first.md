# 0001: Fixed balance full-clear를 먼저 최적화한다

- 상태: Accepted
- 결정일: 2026-09-17

## 결정

새 AI의 첫 목표는 현재 고정 밸런스에서 held-out seed full-clear 확률을 최대화하는 것이다.

HP, 진행도, 누수 피해는 학습 신호와 진단에는 사용할 수 있지만 최종 모델 선택에서 승률을 대체하지 않는다. 인간형 플레이와 balance-conditioned generalization은 강한 fixed-balance AI 이후로 미룬다.

## 이유

여러 문제를 동시에 풀면 실패 원인이 action contract, representation, training, generalization 중 어디에 있는지 분리하기 어렵다. 먼저 명확한 환경과 목표에서 강한 정책을 만든다.

## 결과

- 최종 평가는 full-clear rate를 primary metric으로 사용한다.
- balance 변경 후에는 fixed policy의 유효성을 자동으로 가정하지 않는다.
- 인간 실력 profile은 별도 후속 단계로 관리한다.
