# AI 반복 실행과 기존 통계 수집

## 현재 작업 범위

2026-10-09 사용자 지시에 따라 현재 산출물은 simulator에서 AI로 게임을 여러 번 실행하고 기존 `all-stats` 수준의 통계를 수집하는 기능이다. 밸런스 조정과 방법론은 콘텐츠 추가·레벨 디자인 단계에서 진행한다.

사람 대표성, 공략의 허용 폭, 공략 학습 난이도, 밸런스 자동 최적화는 현재 통계 기능의 선행 조건이나 승인 기준이 아니다. 기존 재학습 기능과 연구 기록은 보존하고 후속 작업으로 관리한다.

## 실행

`tower-defense/simulator/`에서:

```sh
cargo build --release --bin td-simulator
target/release/td-simulator simulate \
  --checkpoint artifacts/phase4b/kl-epoch-transaction-r2-p2/iter-0200 \
  --samples 1000 --seed-start 6000000 --threads 8 \
  --db ai-stats.db --all-stats

target/release/td-simulator stats --db ai-stats.db
```

`--checkpoint`는 PPO iteration 디렉터리(`ppo-actor.json`), BC run 디렉터리(`bc.json`), 과거 neural checkpoint 파일을 받는다. 모델은 실행 전에 한 번 로딩하며, 현재 PPO/BC는 기존 semantic 정책의 greedy 선택을 사용한다. 실행 중 학습하지 않는다. 미래 RNG를 참조하거나 scripted로 실패를 대신 처리하지 않는다.

- `--samples`: 실행할 게임 수. 각 게임은 승리·패배로 실제 종료될 때까지 실행한다.
- `--seed-start`: 첫 게임 seed. 명시한 seed부터 연속 실행한다. 개발 반복에 이전 잠금 final-test seed를 재사용하지 않는다.
- `--threads`: 병렬 실행 수. 0이면 Rayon 기본값이다.
- `--config`: 모든 게임에 적용할 GameConfig JSONC. 기본은 현재 default config다.
- `--allow-checkpoint-config-change`: 설정 digest 차이를 명시적으로 허용하며 DB에 기록한다. 관측·행동·encoder·규칙 version 검사는 유지한다. 설정 snapshot이 있는 PPO는 수치 변경 여부와 core source도 검사한다. 설정 변경 후의 성능 유효성을 보증하는 옵션은 아니다.
- `--all-stats`: 진행도 분포, 피해량 요약, 기존 전략·아이템·유물·카드 서비스 통계를 출력한다.
- `--trace-steps`: 한 게임의 decision trace. `--samples 1 --threads 1`이 필요하다.
- `--fresh-db`: 지정 DB를 명시적으로 초기화한다. 생략하면 이번 실행의 결과를 추가한다.

동일 seed를 다시 실행해도 실행별 simulation ID를 사용하므로 기존 행과 충돌하지 않는다. `all-stats` 출력은 이번 실행의 게임만 집계하고, `stats` 조회 화면은 지정 DB에 누적된 완료 게임을 집계한다. 모델이나 설정별로 누적 결과를 분리하려면 DB 파일을 구분한다.

## 통계와 저장

기존 `SimEvent`, SQLite recorder와 `Database` 통계 함수를 사용한다.

- 게임 결과, final stage, 진행도, HP·gold, 설치 타워·사용 아이템 수, 받은 피해·획득 gold.
- core metrics의 타워 피해량 합계를 가한 피해로 저장하고 출력한다. 과거 행은 새 열의 기본값 0을 가지며 실제 측정값과 구분해야 한다.
- 구매 아이템·유물, 보물 선택, 아이템 사용, 카드 서비스 완료, 설치·철거, 리롤 이벤트.
- stage별 성공 여부, HP·gold 전후, 설치 수, 선택 타워와 리롤 횟수.
- 아이템·유물·카드 서비스별 표본 수, 선택 횟수, 완주율, 평균 진행도와 분산. 상세 분포와 선택 횟수별 결과는 기존 `stats` 화면에서 조회한다.
- checkpoint 경로·iteration, policy 종류, 설정 digest, 환경·행동 version과 seed 범위.

`clear_rate`는 기존 통계와 동일한 게임 진행도(0–100)다. `win_rate`는 여러 게임 중 완주한 비율이다. Items 통계는 기존 구매 기준을 유지한다. 사용 이벤트도 저장하지만 구매 기준 결과와 혼합하지 않는다. 성과의 인과 효과나 밸런스 적합 여부를 자동 판정하지 않는다.

병렬 작업은 thread 수에 비례하는 작은 묶음으로 처리하고 완료 결과를 DB에 저장한다. 전체 게임의 상세 trace를 메모리에 누적하지 않는다. 진행도 값은 분포 출력용으로 게임당 하나 보관한다. 중단 전 저장된 묶음은 남지만 자동 재개 기능은 제공하지 않는다.

정책 오류, 불법 행동, 시간·decision 제한 도달은 오류로 보고한다. 불완전한 게임을 정상 패배로 집계하지 않는다. 상세 저장 후 마지막에 `completed_at`을 기록하므로 저장이 완료되지 않은 행은 기존 통계 쿼리에서 제외된다.

## 구현 상태

현재 semantic PPO/BC를 `simulate`에 연결하고 이벤트·stage 기록 및 기존 통계 출력을 보완했다. release binary로 개발 seed 4000000–4000003의 실제 종료 게임 4판과 `all-stats`·SQLite 저장을 확인했다. 각 판의 진행도는 기존 terminal evaluator와 float 표현 오차 이내에서 일치했다. trace와 streaming 기록의 동등성, 실행별 집계 분리와 미완료 게임 제외 테스트도 통과했다. 전체 테스트의 기존 실패와 추가 실험 결과는 [24](24-construction-and-content-adaptation.md)에 기록한다.
