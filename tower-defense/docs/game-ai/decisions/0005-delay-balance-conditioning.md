# 0005: Balance conditioning은 fixed policy 이후로 미룬다

- 상태: Accepted
- 결정일: 2026-09-17

## 결정

처음부터 전체 balance configuration을 입력받는 범용 policy를 만들지 않는다.

다음 순서를 사용한다.

1. 현재 fixed configuration에서 강한 policy
2. 좁은 parameter range randomization
3. 필요한 parameter만 입력받는 balance-conditioned policy

## 이유

현재 게임을 잘 깨는 정책도 완성되지 않은 상태에서 다양한 규칙에 동시에 일반화하면 학습 문제가 커지고 실패 원인을 분리하기 어렵다.

## 결과

- fixed policy의 검증 configuration을 checkpoint metadata에 기록한다.
- balance 변경이 검증 범위를 벗어나면 재학습하거나 conditioned policy를 사용한다.
- 유물과 카드 효과의 현재 수치는 fixed policy에서도 observation에 의미 있게 표현한다.
