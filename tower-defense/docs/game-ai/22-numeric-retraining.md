# 수치 변경 후 PPO 재학습

## 구현 범위

[`21-retraining-implementation-plan.md`](21-retraining-implementation-plan.md)의 첫 구현 단위다. 현재 `GameConfig`로 표현되는 공격력, HP, 사거리, cooldown, 시작 자원 같은 수치를 바꾸고 기존 Top8 PPO actor/critic에서 새 run을 시작한다. 기존 학습 run과 원본 checkpoint는 보존한다.

- 변경한 configuration을 Phase 4 데이터 생성·학습·평가에 전달한다.
- 새 PPO checkpoint에 configuration snapshot/digest와 관측·행동·feature·encoder·규칙 계약을 저장한다.
- 원본 가중치를 가져오고 Adam optimizer를 새로 시작한다. KL 기준 정책은 재학습 시작 시점의 원본 PPO다.
- 변경 후 규칙에서 rollout을 수집하고, 원본 정책과 재학습 정책을 같은 변경 후 configuration에서 평가한다.
- 중단 후 같은 spec으로 실행하면 최신 완료 checkpoint와 optimizer 상태를 복원한다.
- 새 configuration을 기존 run의 resume에 섞거나 원본 디렉터리에 재학습 결과를 쓰는 시도를 거부한다.

현재는 동일한 입력·행동 계약과 모델 구조의 full-actor Top8 / AllBuildOptions PPO를 지원한다. 안정적인 기존 ID를 유지한 보물·아이템·카드 서비스 catalog 추가와, 입력 구조가 같은 규칙 변경은 새 run에서 가중치를 재사용한다. 관측 필드·행동 의미·모델 구조가 바뀌면 명시적인 migration adapter가 필요하다. 신규 효과의 의미를 자동으로 이해하거나 새로운 입력 구조를 자동 생성하는 기능은 아니다. 자세한 계약은 [24](24-construction-and-content-adaptation.md)를 따른다.

## 실행

`tower-defense/simulator/`에서 release binary를 빌드한다.

```sh
cargo build --release --bin td-simulator
target/release/td-simulator ml phase4 retrain --spec retrain.json --threads 8
```

`retrain.json` 예시:

```json
{
  "schema_version": 1,
  "source_checkpoint": "artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200",
  "source_config": "configs/original.jsonc",
  "target_config": "configs/changed.jsonc",
  "run_dir": "artifacts/retraining/balance-001",
  "iterations": 25,
  "train_seed_start": 5000000,
  "seed": 3,
  "evaluate_every": 5,
  "development_seeds": 128
}
```

경로는 spec 파일이 있는 디렉터리를 기준으로 해석한다. `iterations`는 새 run의 총 iteration 수다. 25회까지 완료한 뒤 50회까지 이어서 실행하려면 같은 spec의 `iterations`만 50으로 늘린다. 완료된 iteration보다 작은 값으로 되돌리지 않는다.

`ppo`를 생략하면 원본 run의 PPO recipe를 사용한다. training seed block과 PPO seed는 spec의 값으로 교체한다. 다른 recipe를 사용할 때는 `PpoConfig` 전체를 `ppo`에 지정한다. resume에서는 recipe, seed, 설정, 평가 주기와 평가 seed 수를 유지한다.

학습 seed 범위는 dataset/development/final 예약 범위 및 원본 run에 기록된 학습 범위와 겹칠 수 없다. 이전 독립 실험이나 원본의 선행 phase에서 이미 사용한 범위도 피해서 지정한다. 이 명령은 새로운 final evaluation을 수행하지 않는다.

## 과거 checkpoint

설정 snapshot이 없는 과거 checkpoint는 `source_config`가 필요하다. 원본 학습에서 사용한 파일을 제공해야 하며, 원본 dataset provenance의 digest와 version이 일치하는지 검사한다. 새 checkpoint에 snapshot이 있으면 이 필드는 생략할 수 있다.

과거 checkpoint의 입력 의미는 기록된 Git commit의 encoder/model/normalization 소스와 현재 binary에 포함된 소스를 비교해 확인한다. 원본 Git object가 없거나 해당 소스가 달라졌으면 자동 이관하지 않는다. 예외는 source/target SHA-256을 고정한 Top8 → AllBuildOptions의 추가적 후보 adapter다. 관측·행동·candidate version은 일치해야 한다. 규칙 epoch 변경은 명시적인 새 재학습 run에서만 허용하며, 일반 로딩·기존 run resume에서는 거부한다. core source hash는 별도로 기록하고, 동일 run을 재개할 때는 일치를 요구한다. 새 규칙이나 관측 의미가 바뀌면 계약 version을 올려야 한다.

원본 iteration 디렉터리와 부모 run의 `ppo.json`이 필요하다. 원본 BC 디렉터리와 teacher dataset은 재학습 시작에 필요하지 않다. 원본 가중치·actor metadata의 SHA-256을 저장하며, 재개 시 파일 변경을 확인한다.

## 결과물

| 파일 | 내용 |
| --- | --- |
| `migration.json` | 원본 checkpoint/hash, source 계약, 변경 수치, optimizer 정책, 기존 학습 비용 |
| `retraining-spec.json` | 성공적으로 적용된 실행 spec과 해석된 경로 |
| `ppo.json` | target 계약, iteration별 학습·평가, 변경 후 학습 비용 |
| `iter-NNNN/` | actor/critic와 각각의 optimizer, standalone actor 계약 |
| `retraining-report.json` | 개발 평가로 선택한 checkpoint, 변경 후 원본/재학습/canonical 비교, 비용 범위 |

checkpoint는 개발 seed의 평균 terminal progress로 선택하고, 동률이면 최신 iteration을 선택한다. 완주 횟수는 선택의 선행 조건이 아니다. iteration 0도 포함하므로 추가 학습이 나빠지면 초기 정책을 유지할 수 있다. 후보 모드를 넓혔다면 가중치가 같아도 확률 정규화가 달라지므로 초기 정책의 행동은 원본과 다를 수 있다. 마지막 학습 iteration에도 개발 평가를 수행한다.

`post_change_budget`은 rollout, optimizer, 학습 중 개발 평가의 비용이다. 준비 시간과 report 생성용 평가 시간은 별도로 기록한다. 원본 모델을 만들 때의 학습 비용은 `source_budget`으로 분리한다. `workflow_invocation_seconds`는 이번 실행의 시간이며 이전 실행이나 중단 중 발생한 모든 비용의 합계가 아니다.

이 report는 개발 진단이다. 처음부터 학습하는 대조군, 독립 반복, 새 final seed를 사용한 검증은 별도 연구로 남아 있다. 따라서 `speedup_verified`는 `false`이며, 실행 성공이나 작은 개발 표본의 진행도 상승만으로 빠른 재학습 또는 최종 성능 승인을 주장하지 않는다. 진행도는 현재 밸런스에서 유효한 primary metric이며, 승인을 위해서는 사전에 정한 목표와 대조군·독립 반복·held-out 검증이 필요하다.

## 개별 Phase 4 명령의 설정 지정

```sh
target/release/td-simulator ml phase4 --config configs/changed.jsonc collect \
  --split phase4b-canonical-train --source canonical --count 2 \
  --output artifacts/datasets/changed-canonical

target/release/td-simulator ml phase4 --config configs/changed.jsonc terminal-eval \
  --split ppo-development --count 4 \
  --policy retrained=artifacts/retraining/balance-001/iter-0025 \
  --output artifacts/retraining/balance-001/manual-dev.json
```

`retrain`은 target configuration을 spec에서 읽으므로 동시에 `--config`를 지정하지 않는다. 개별 terminal 평가에서는 policy의 설정과 지정한 설정의 대응을 확인한다. 변경 후 configuration에서 원본 policy도 비교하는 용도는 `retrain`의 report에 포함된다.

## 병합 문제 수정

구현 시작 시 base는 `3e5b41d8`이다. master 병합에서 card service의 단조로운 선택 계약, 공개된 후보 제한, rollout fork 시 후보 유지와 관련 test가 되돌아가 있었다. 통합 test에서 `confirm_card_service_selection`의 `InvalidSelection`을 재현하여 Phase 4B에서 검증된 처리와 test를 복구했다. 다른 PR의 표시·보상 연출 변경은 유지한다.

## 기술 검증 결과

- release library test 25개 통과: 새 재학습 workflow와 계약 검사 4개, CLI 설정 전달 1개, card service 8개, rollout fork 5개, 기존 PPO resume 1개, epoch transaction 4개, terminal evaluation 2개.
- workflow test에서 원본 actor/critic 가중치 보존, 새 Adam 초기화, 원본 BC 디렉터리 없이 실행, configuration 기록, 연속 실행과 중단·재개 실행의 actor/critic 및 Adam 상태 일치를 확인했다.
- 실제 `kl-epoch-transaction-r2-p2/iter-0200`에서 시작 gold를 100 → 105로 바꿔 실행했다. 원본의 학습 configuration을 당시 Git revision에서 복원하고 기존 계약과 대조했다.
- release binary의 실제 executable과 command line을 확인했다. 2 episodes/iteration, 1 update epoch, 개발 평가 4 seeds로 1 iteration 실행 후 같은 spec을 2 iterations까지 재개했다. rollout/update의 첫 iteration 시간은 약 0.60/1.10초였다.
- 실제 run의 iteration 0 actor/critic 파일은 원본과 동일했다. 총 4회 학습 episode에서 illegal action, action mismatch, nonfinite rollout, nonfinite update skip, truncation이 모두 0이었다. 시작 gold를 다시 106으로 바꿔 같은 run에 재개하는 시도는 거부됐다.
- 개발 seed 4개의 평균 진행도는 원본 46.77, 선택된 재학습 checkpoint 47.19였으며 두 정책의 완주는 0이었다. 작은 기능 확인용 표본이므로 성능 개선이나 재학습 효율의 증거로 사용하지 않는다.

로컬 결과: `simulator/artifacts/retraining/numeric-smoke-rbmh_9s1/run/retraining-report.json`. 원본과 변경 설정, spec, 실행·재개 log는 같은 `numeric-smoke-rbmh_9s1/` 디렉터리에 있다. 다른 실험 artifact와 마찬가지로 Git 추적 대상이 아니다.
