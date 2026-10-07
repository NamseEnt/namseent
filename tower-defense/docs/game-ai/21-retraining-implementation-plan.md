# 기획 변경 후 재학습 구현 계획

## 목표와 현재 상태

[`00-goals-and-acceptance.md`](00-goals-and-acceptance.md)의 재학습 목표를 구현 단위로 나눈다. 성공 판정은 [`09-balance-experiments.md`](09-balance-experiments.md)의 비교 계약을 따른다. 이 문서는 구현 계획이며 아래 기능의 구현 완료나 재학습 속도 개선을 보고하는 문서가 아니다.

현재 재사용할 수 있는 기반:

- authoritative core 기반 simulator와 legal action 생성
- `GameConfig` JSONC 로딩과 configuration digest
- BC/PPO checkpoint 저장, 동일 규칙의 resume, 기존 PPO actor/critic으로 새 run을 시작하는 경로
- 관측·행동·dataset version 검사, deterministic replay, paired terminal evaluation
- tower status/splash 및 build template splash의 구조화된 feature 추출

현재 보완할 부분:

- `ml/phase4_cli.rs`의 기본 설정 생성 경로를 사용자 지정 configuration으로 통일해야 한다.
- 기존 PPO loader는 다른 game rules epoch나 candidate/encoder version을 거부한다. 변경 후 재학습을 위한 명시적인 이관 경로가 필요하다.
- `PpoActorFile`만으로는 전체 configuration, 입력 필드 의미, vocabulary 대응과 이관 내역을 확인할 수 없다. run/BC metadata와 분리된 배포 checkpoint에도 필요한 계약을 기록해야 한다.
- `encoding/combat.rs`와 `encoding/dense_build.rs`의 status/splash 표현은 현재 PPO/BC 모델에 연결되지 않았다.
- 아이템·upgrade는 현재 typed 입력에서 ID와 제한된 수치로 표현된다. 새로운 효과의 발동 조건과 대상을 모델에 전달하는 경로를 확장해야 한다.

## 1. 변경한 게임 설정을 학습·평가 전체에 적용

주요 위치: `simulator/src/ml/phase4_cli.rs`, `config.rs`, `phase4_dataset.rs`, `semantic_ppo.rs`, `phase4_eval.rs`.

- dataset 생성, BC/critic 학습, PPO rollout, development/terminal 평가가 동일한 명시적 `GameConfig`를 사용하도록 연결한다. 기본 설정도 기록 가능한 입력으로 취급한다.
- checkpoint에 실제 사용한 config snapshot/digest, core 규칙 식별자, 관측·행동·feature/candidate 계약을 저장한다. 재학습의 원본과 대상 configuration을 함께 식별한다.
- resume 시 원래 run의 config/규칙 계약이 일치하는지 검사한다. 변경된 configuration은 원본을 덮어쓰지 않고 새 run으로 시작한다.
- config snapshot만으로 식별할 수 없는 새 효과 구현은 core revision/규칙 version으로 구분한다.

완료 조건: 변경한 공격력·가격 등이 rollout과 평가에 동일하게 반영되고, 다른 configuration을 같은 run에 이어 붙이려는 시도가 거부된다. 기존 default-config 실행과 checkpoint의 지원 범위도 유지한다.

## 2. 호환성 검사와 checkpoint 이관

주요 위치: `semantic_ppo.rs`, `semantic_bc.rs`, `policy_v2.rs`, `model.rs`, `feature_contract.rs`, `vocabulary.rs`. 호환성 판정과 이관 report는 별도 모듈로 분리한다.

- source/target 계약을 비교해 동일 계약 재개, 수치 변경 이관, 지원되는 구조 변경 이관, 미지원 변경을 판정한다.
- 필드 이름·의미·정규화, entity/action 종류, vocabulary의 stable key와 tensor 대응을 기록한다. 크기가 같은 tensor라는 이유만으로 복사하지 않는다.
- 의미가 동일한 가중치는 복사하고, 새 ID/입력/출력은 명시적으로 초기화한다. 삭제·재배열된 항목은 stable key로 대응한다.
- 원본 checkpoint hash, source/target 계약, 복사·변환·초기화된 부분과 난수 seed를 이관 report에 저장한다. 기존 artifact에 계약 정보가 부족하면 필요한 원본 metadata를 명시적으로 제공받거나 이관을 거부한다.
- critic은 새 규칙의 가치 분포를 고려해 재사용·재학습·초기화를 선택하고 기록한다. optimizer 상태는 이관 정책에 따라 처리하며 기본 경로는 새 optimizer로 시작한다. 동일 규칙의 정확한 resume은 optimizer 상태를 그대로 복원한다.
- 이관된 checkpoint는 dataset/BC 경로의 이전 규칙 검사에 우회 의존하지 않고 학습 시작점으로 사용할 수 있어야 한다. 이후 checkpoint에는 target 계약과 원본 연결을 저장한다.

첫 구현 범위는 입력·행동 의미와 모델 구조가 같은 수치 변경이다. 구조 변경은 명시적인 adapter를 구현한 경우에 지원한다.

완료 조건: 수치 변경 이관 직후 동일한 입력에 대한 actor 출력이 원본과 일치하고, 새 규칙에서 정상적인 추가 학습과 저장·재개가 가능하다. source checkpoint는 보존되며 미지원 구조 변경은 설명과 함께 거부된다.

## 3. 효과 정보를 실제 actor·critic 입력에 연결

주요 위치: `encoding/combat.rs`, `encoding/dense_build.rs`, `encoding/observation.rs`, `semantic_candidates.rs`, `model.rs`, `policy_v2.rs`, `feature_contract.rs`.

- 배치된 tower의 광역공격 발동 종류·범위·피해 비율, status 종류·값·남은 시간을 actor와 critic이 읽도록 연결한다.
- 아직 건설하지 않은 후보도 resulting tower의 효과를 읽도록 연결한다. 현재 배치된 tower 정보만 추가해서 새 후보의 가치가 보이지 않는 상태를 피한다.
- 여러 효과를 소유 tower/template과 연결된 가변 길이 집합으로 인코딩한다. 효과 개수를 조용히 자르거나 ID 하나로 대체하지 않는다.
- 아이템 효과의 발동 조건·대상·수치 중 판단에 필요한 정보를 core의 authoritative 표현에서 제공한다. 새 효과가 기존 표현에 들어가지 않으면 관측과 encoder를 확장한다.
- 기존 encoder/scorer를 재사용할 수 있도록 효과 입력을 모듈로 추가하고, 이관 시 새 경로의 초기 기여를 제어한다. 기존 입력에서의 출력 보존과 새 경로의 학습 가능성을 함께 확인한다.
- feature, normalization, model/checkpoint version을 함께 관리한다. 새 효과를 추출하는 것부터 batching, actor/critic forward, rollout 저장과 학습까지 연결한다.

완료 조건: 효과 정보만 다른 상태·후보가 서로 다른 모델 입력이 되고, 새 효과 경로가 실제 학습 gradient를 받는다. 변경 전 입력에 대한 이관 결과와 inference 시간·memory도 확인한다.

새 게임 규칙의 구현과 관측 의미 추가는 해당 기획 변경에 필요한 개발 작업이다. 재학습 workflow는 등록된 규칙과 입력을 사용한다.

## 4. 변경 후 추가 학습 workflow

주요 위치: `phase4_cli.rs`, `semantic_ppo.rs`, `training_progress.rs`와 재학습 orchestration 모듈.

- source checkpoint, target config/규칙, migration 정책, training/evaluation seed, 계산 예산을 받는 재학습 spec을 정의한다.
- 호환성 확인 → 이관 → 초기 평가 → 새 규칙 rollout과 PPO 학습 → checkpoint 선택 → 평가 report를 반복 실행 가능한 workflow로 제공한다. CLI 이름과 옵션은 이 구현 단계에서 확정한다.
- 변경 후 새 rollout을 사용하고 이전 규칙의 PPO log-probability·reward를 새 on-policy 표본으로 섞지 않는다.
- actor/critic 초기화, 업데이트할 모델 부분, optimizer 처리와 학습 recipe를 spec에 명시한다. 전체 추가 학습을 먼저 지원하고 부분 동결 등은 비교 실험에서 필요성이 확인되면 확장한다.
- 중단 후 최신 완료 checkpoint에서 재개하며 계산 예산을 누적한다. 새 규칙에서 illegal action, action mismatch, nonfinite와 runaway 상태를 확인한다.

완료 조건: 수치 변경을 지정한 한 spec으로 이관부터 학습·평가까지 실행할 수 있고, 재개 시 변경된 규칙과 source/target 연결을 유지한다.

## 5. 재학습 효율 비교와 결과 보고

주요 위치: `phase4_eval.rs`, `phase4_analysis.rs`, `training_progress.rs`와 재학습 평가 모듈.

- 변경 후 처음부터 학습하는 대조군과 기존 checkpoint에서 시작하는 실험군을 동일 target 모델 구조와 seed 계약으로 실행한다.
- source 모델의 과거 학습 비용과 이번 변경 후 발생한 준비·학습 비용을 구분해서 기록한다. 대조군도 BC/critic 준비를 포함한 전체 비용을 기록한다.
- 목표 full-clear 성능까지의 게임 수·semantic decisions·wall time, 동일 예산의 성능, 수동 준비 단계, inference 시간·memory를 report로 출력한다.
- 수치 변경과 새 효과/전투 규칙 변경을 최소 하나씩 검증한다. 최소 3회 독립 학습의 분포와 held-out confidence interval을 보고한다.
- development에서 선택한 뒤 잠근 final seed를 사용한다. 이전 final seed 범위는 재사용하지 않는다.
- 목표 완주 성능에 미달하면 진행도 개선과 재학습 시간 측정은 개발 진단으로 보고한다. 완주 목표 달성으로 승인하지 않는다.

완료 조건: 처음부터 학습하는 것보다 재학습이 빠른지, 실제로 같은 목표 성능에 도달했는지 결과로 판정할 수 있다. 개선이 없는 변경 유형도 기록한다.

## 구현 순서

1. **수치 변경 경로:** 1 → 2의 동일 구조 이관 → 4의 최소 workflow → 5의 수치 변경 비교. 기존 checkpoint 재사용을 가장 작은 변경으로 검증한다.
2. **새 효과 경로:** 3 → 2의 구조 변경 adapter → 4/5의 효과 변경 비교. 이미 추출 가능한 광역공격·status 정보부터 연결한다.
3. **사용 절차 정리:** 검증된 spec과 workflow를 문서화하고 변경 유형별 지원 범위·필요 작업·측정된 비용을 안내한다.

전투 성능 개선 연구와 이 구현은 각각 추적한다. 재학습 workflow가 완성되어도 높은 확률의 full-clear 목표가 충족되는 것은 아니며, 각 목표는 자신의 승인 기준으로 확인한다.
