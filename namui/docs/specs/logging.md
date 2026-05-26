# namui 로깅 시스템 스펙

상태: Implemented (Phase 1+2+3)
작성일: 2026-05-26
대상 브랜치: crash-reporter

## 1. 배경

현재 namui 프레임워크는 `println!` / `eprintln!` / `dbg!` 를 로그 시스템으로 직접 사용하고 있다. 워크스페이스 전체에 약 92곳에서 사용 중이며, public API에는 로깅 매크로가 노출되어 있지 않다 (`namui::log!` 매크로가 lib.rs에 존재하나 본문이 깨진 dead code 이며 어디서도 호출되지 않음).

`println!` 직접 사용은 다음 문제를 가진다.

- 로그 레벨이 없어 빌드/환경별 필터링 불가
- 타임스탬프 / 모듈 경로 / 스레드 ID 메타데이터 부재
- stdout 단일 sink만 사용 가능 (파일, 원격 텔레메트리, 인게임 콘솔 등 불가)
- 멀티스레드 환경에서 인터리빙으로 가독성 저하
- 릴리스 빌드에서 verbose 로그를 컴파일 타임에 제거 불가
- 구조화 로깅 불가 (JSON 파싱, 옵저버빌리티 도구 연동 불가)

Rust 생태계의 de facto 표준은 `log` 파사드 또는 `tracing` 이며, Rust 게임 엔진 중 가장 대표적인 Bevy 는 `bevy_log` 가 내부적으로 `tracing` 을 기반으로 동작한다. namui 도 동일한 노선을 따른다.

## 2. 목표 / 비목표

### 목표

1. namui 사용자는 `use namui::*; info!(...)` 형태로 즉시 로깅 가능 (제로 학습 비용).
2. 빌드 타깃(wasm32-wasi-web / native / android 등)에 맞춰 자동으로 적절한 sink 로 라우팅.
3. 기본 설정(zero-config)으로도 합리적인 동작, 환경변수 `RUST_LOG` 호환, 필요시 builder 로 커스터마이즈.
4. 게임 루프의 매-프레임 스팸을 막는 `info_once!` / 스로틀 매크로 제공.
5. 인게임 콘솔에 최근 로그를 표시할 수 있는 링 버퍼 layer 제공.
6. crash-reporter 와 통합하여 panic / signal 발생 시 최근 N개 로그를 크래시 리포트에 첨부.
7. native-runner 의 dylib 동적 로드 환경에서 단일 dispatcher 로 통일.

### 비목표

- 사용자 코드의 `println!` 을 강제로 금지하지 않는다 (계속 동작, 다만 가이드에서 권장하지 않음).
- 외부 로그 수집 인프라(Datadog, Sentry 등) 연동은 본 스펙 범위 외 — Layer 추가만으로 가능하도록 확장점만 보장.
- 사용자 게임 로직의 마이그레이션은 본 스펙 범위 외 — namui 프레임워크 내부 코드만 마이그레이션 대상.

## 3. 채택 기술

| 항목 | 선택 |
|------|------|
| 로깅 파사드 | `tracing` |
| Subscriber | `tracing-subscriber` (env-filter, fmt) |
| log 크레이트 호환 | `tracing-log` (외부 의존 라이브러리가 log 사용 시 자동 흡수) |
| Wasm 브라우저 출력 | `tracing-wasm` 또는 `web_sys::console` 직접 호출 layer |

`log` 크레이트를 직접 채택하지 않는 이유: namui 는 async tokio runtime 기반 게임 루프 + 멀티스레드 환경이며, span 기반 컨텍스트 추적이 평면 로그보다 가치가 크다. `tracing-log` 어댑터로 `log` 매크로 호출도 자동 흡수되므로 호환성 손실 없음.

## 4. Public API

### 4-1. 매크로 re-export

`namui/src/lib.rs` 에서 다음을 re-export.

```rust
pub use tracing::{trace, debug, info, warn, error, instrument, span, Level};
```

사용자는 `use namui::*;` 이후 다음과 같이 호출.

```rust
info!("game started");
info!(player_id = id, "player spawned");
warn!("low fps: {:.1}", fps);
error!(?err, "asset load failed");

#[instrument(skip(world))]
fn update(world: &mut World) { ... }
```

기존 `namui::log!` 매크로는 dead code 이므로 삭제.

### 4-2. 게임 특화 매크로

namui 자체적으로 다음 매크로를 추가 제공 (`namui/src/system/log/macros.rs`).

```rust
namui::info_once!(...)             // 프로세스 생애 동안 호출 위치별 1회만 출력
namui::warn_once!(...)
namui::error_once!(...)
namui::info_every_n!(n, ...)       // n번 호출당 1회 출력
namui::warn_every_n!(n, ...)
namui::info_throttled!(duration, ...)  // duration 간격 스로틀
namui::warn_throttled!(duration, ...)
```

구현은 호출 위치별 `AtomicU64` (마지막 출력 카운터 또는 timestamp) 를 lazy static 으로 두는 패턴.

### 4-3. 초기화 API

`namui::start` 가 내부에서 LogPlugin 을 자동 초기화. 명시적으로 설정하려면 `namui::start_with` 사용.

```rust
namui::start(root_component);  // 자동 LogPlugin

namui::start_with(
    namui::StartConfig::default()
        .log(|l| l
            .level(namui::Level::INFO)
            .filter("wgpu=warn,my_game::ai=trace")
            .file_output("./logs")
            .in_game_console(true)
        ),
    root_component,
);
```

`StartConfig::log` 의 closure 는 `LogConfigBuilder` 를 받아 다음을 설정 가능.

- `level(Level)` — 기본 레벨
- `filter(&str)` — `RUST_LOG` 문법의 필터 문자열 (지정 시 `level` 보다 우선)
- `file_output(path)` — 로그 파일 출력 디렉터리 (native 전용, 일일 회전)
- `in_game_console(bool)` — 링 버퍼 layer 활성화
- `ring_buffer_capacity(usize)` — 링 버퍼 용량 (기본 1024)
- `additional_layer(Box<dyn Layer>)` — 사용자 정의 layer 추가

`RUST_LOG` 환경변수가 설정되어 있으면 코드의 `filter` 보다 우선 적용.

## 5. 플랫폼별 라우팅

`namui::system::log::init_log_plugin(config)` 가 빌드 타깃에 따라 sink 조합을 결정.

| 타깃 | 기본 sink | 포맷 |
|------|-----------|------|
| `target_os = "wasi"` (wasm32-wasi-web 브라우저) | `web_sys::console::log/warn/error` | 레벨 색상 활용 |
| native dev (`debug_assertions`) | stderr | ANSI 컬러, 타임스탬프, target, 레벨 |
| native release | stderr + 회전 파일 | stderr: 평문, 파일: JSON |
| android (장기) | logcat (`tracing-android`) | 기본 |
| 인게임 콘솔 활성화 시 | 위 sink + 링 버퍼 layer | — |

### 5-1. wasm32-wasi-web 의 특수성

namui 는 wasm32-wasi-web 을 주 타깃으로 한다. `cfg(target_os = "wasi")` 일 때:

- 기본적으로 stderr 도 호스트 (브라우저 wasi-shim) 에 캡처되므로 fallback 가능.
- 그러나 브라우저 DevTools 에서 레벨별 색상을 활용하려면 `web_sys::console::*` 호출이 필요.
- namui 가 wasm-bindgen 을 사용 중인지에 따라 결정 — `web_sys` 호출이 가능하면 그것을 우선, 아니면 stderr.

구현 시 `namui-cfg` 의 cfg 매크로로 분기.

### 5-2. native 파일 출력 경로

`file_output` 미지정 시 기본 경로:

- macOS: `$HOME/Library/Logs/<app-name>/`
- Linux: `$XDG_STATE_HOME/<app-name>/logs/` (없으면 `$HOME/.local/state/<app-name>/logs/`)
- Windows: `%LOCALAPPDATA%/<app-name>/logs/`

`<app-name>` 은 `Cargo.toml` 의 패키지명 또는 `StartConfig::app_name` 으로 지정.

릴리스 빌드에서만 파일 출력 활성화 (debug 빌드는 stderr 만).

## 6. 인게임 콘솔 Layer

`namui::system::log::ring_buffer::RingBufferLayer` 를 제공.

- 고정 크기 락-프리 링 버퍼 (capacity 는 config 로 조정).
- 각 엔트리: `{ timestamp, level, target, message }`.
- `namui::system::log::recent_logs(n: usize) -> Vec<LogEntry>` 로 최근 로그 조회.
- 별도 namui-prebuilt 위젯(`DebugConsole`) 에서 구독하여 화면에 렌더 (별도 PR 에서 추가).

스레드 안전: `crossbeam::queue::ArrayQueue` 또는 `parking_lot::Mutex<VecDeque>` 사용.

## 7. crash-reporter 통합

현재 `native-runner/src/main.rs` 가 시그널 핸들러 + panic hook 으로 fatal 이벤트를 잡아 stderr 에 출력한다. tracing 도입 후:

1. namui 측 (dylib) 의 panic hook 을 `error!("panic at {}: {}", location, payload)` 로 흘려서 모든 sink 에 동시 기록.
2. RingBufferLayer 의 내용을 crash report payload 에 첨부 (스냅샷 함수 `dump_ring_buffer() -> Vec<LogEntry>` 제공).
3. native-runner 측은 본 스펙의 범위 밖. signal 핸들러는 async-signal-safe 제약으로 tracing 사용 불가하므로 현행 stderr write 유지.

crash-reporter 브랜치의 별도 작업과 본 작업의 인터페이스는 `dump_ring_buffer()` 한 함수로 정의.

## 8. dylib / 다중 dispatcher 문제

native-runner 는 게임 dylib 을 `libloading` 으로 동적 로드한다. tracing 의 global default dispatcher 는 정적 변수이므로, 같은 프로세스 내에서도 정적 링크 단위(host vs dylib) 별로 별개의 인스턴스가 생길 수 있다.

### 8-1. 현황 분석

- `native-runner` crate 의 dependencies: `namui-skia`, `namui-drawer`, `namui-rendering-tree`, `namui-type`, `winit`, `libloading`, `notify`, `bincode`, `mimalloc`, `anyhow` — 즉 `namui` 본체 의존 없음.
- 게임 dylib 은 `namui` 본체를 의존하여 컴파일됨.
- 따라서 native-runner 가 `tracing` 을 직접 의존하지 않는 한 dispatcher 충돌은 발생하지 않음. dylib 측의 `tracing` 정적 인스턴스 하나만 존재.

### 8-2. 결정

- native-runner 는 tracing 을 의존하지 않는다. 기존 `eprintln!("[runner] ...")` 유지.
- 사용자 게임 코드와 namui 프레임워크 코드는 모두 dylib 내에서 동일한 `tracing` dispatcher 를 공유한다.
- 추후 native-runner 도 tracing 도입이 필요해지면, dylib 진입점에 `namui::system::log::install_external_dispatcher(dispatch: tracing::Dispatch)` 를 추가하여 host 의 dispatcher 를 dylib 으로 전달한다 (본 스펙 범위 외).

## 9. 디렉터리 구조

namui crate 내 신규 모듈.

```
namui/src/system/log/
  mod.rs              # public API, init_log_plugin, LogConfigBuilder
  config.rs           # LogConfigBuilder, LogConfig
  init.rs             # platform 분기 init
  ring_buffer.rs      # RingBufferLayer
  macros.rs           # info_once!, info_every_n!, throttled! 등
  paths.rs            # 플랫폼별 로그 디렉터리
```

기존 `namui/src/lib.rs` 의 깨진 `log!` 매크로(라인 131-136)는 삭제.

## 10. 의존성 추가

`namui/Cargo.toml` 에 추가.

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "registry"] }
tracing-log = "0.2"

[target.'cfg(target_os = "wasi")'.dependencies]
# wasm32-wasi-web 의 web_sys 가용성에 따라 결정. 우선은 stderr fallback.
# tracing-wasm = "0.2"  # 추후 활성화

[target.'cfg(not(target_os = "wasi"))'.dependencies]
tracing-appender = "0.2"  # 파일 회전
```

`tracing-wasm` 활성화 여부는 namui 의 wasm-bindgen 사용 현황 확인 후 결정.

## 11. 마이그레이션 단계

### Phase 1 — 인프라 도입 (본 PR 범위)

1. 의존성 추가 (`namui/Cargo.toml`).
2. `namui/src/system/log/` 모듈 생성.
3. `namui::start` 에서 `init_log_plugin(LogConfig::default())` 호출.
4. `namui/src/lib.rs` 에서 `tracing` 매크로 re-export, 깨진 `log!` 매크로 삭제.
5. 게임 특화 매크로(`info_once!`, `info_every_n!`, `info_throttled!` 등) 구현.
6. `RingBufferLayer` 구현, `dump_ring_buffer()` 공개 함수 추가.

### Phase 2 — 런타임 핵심 경로 마이그레이션 (본 PR 범위)

`namui` 본체 및 `namui` crate 의존하는 런타임 핵심 crate 의 production code 만 본 PR 에서 마이그레이션. 빌드 도구 / 네이티브 백엔드 등 별도 의존성 트리에 있는 crate 는 Phase 3 로 이관.

본 PR 마이그레이션 대상 (production code):
- `namui` — `ffi.rs`, `hooks/looper.rs` 의 `println!` / `eprintln!` 교체 완료.
- `namui-prebuilt` — `rich_text/mod.rs` 의 `eprintln!` 교체 완료 (namui 의존 통해).
- `namui-rendering-tree` — `mouse_cursor.rs` 의 `println!` 교체 완료 (`tracing` 직접 의존).

레벨 기준:
- 빌드 진행/단계 표시 → `info!`
- 셰이더 컴파일 실패, 에셋 로드 실패, FFI panic → `error!`
- 디버그 보조 출력 → `debug!` 또는 `trace!`
- 사용자 입력 검증 실패, 설정 오류 → `warn!`

제외 대상 (본 PR 에서 제외, Phase 3 로 이관):
- `native-runner` (앞 8-2 결정 — 시그널 핸들러는 async-signal-safe 제약, eprintln 유지).
- `namui-cli` (빌드 도구, 별도 의존성 트리).
- `namui-drawer` (네이티브 백엔드, 별도 의존성 트리).
- `namui-skia` 래퍼 (low-level).
- `namui-audio-native` (네이티브 백엔드).
- `namui-hooks` (production code 내 `println!` 없음).
- `sample/` 내 예제 코드.
- `asset-macro/examples/test-project` (테스트 픽스처).
- `third-party-forks/` (외부 코드).
- 모든 `*/tests/` `*/benches/` `*/examples/`.

### Phase 3 — 잔여 마이그레이션 및 통합 (본 PR 범위, 완료)

본 PR 마이그레이션 대상 (production code):
- `namui-skia` — `canvas.rs` 의 `println!` 교체 완료 (`tracing` 자체 의존).
- `namui-audio-native` — `lib.rs` 의 `eprintln!` 6개소 교체 완료 (`tracing` 자체 의존).

본 PR 인프라:
- `std::panic::set_hook` 통합 — 모든 panic 이 `error!(target: "namui::panic")` 으로 흐른 뒤 기존 hook 으로 전파. ring buffer 와 자동 통합.
- `tracing-appender` 도입 — `LogConfigBuilder::file_output(path)` 또는 `app_name` 지정 시 일일 회전 파일 출력 활성화 (non-wasi 전용, 비동기 non-blocking).
- ring buffer 구독 API — `dump_ring_buffer()`, `dump_recent_logs(n)`, `is_ring_buffer_installed()`.
- crash-reporter 인터페이스 — `dump_ring_buffer()` / `dump_recent_logs(n)` 가 panic / signal 핸들러에서 직접 호출 가능.

`namui-cli` 는 마이그레이션 영구 제외:
- CLI 도구이므로 stdout 출력 자체가 사용자 인터페이스. `tracing` 으로 바꾸면 `RUST_LOG` 필터 영향을 받아 UX 가 망가짐.
- `println!` 유지가 표준 (cargo, rustc 등 다른 Rust CLI 도구와 동일).

### Phase 4 — 미래 작업

- 인게임 콘솔 위젯 (`namui-prebuilt::DebugConsole`) — `dump_recent_logs(n)` 구독해서 화면에 렌더하는 위젯 추가.
- crash-reporter 브랜치의 실제 리포트 payload 에 ring buffer 덤프 첨부.
- `tracing-wasm` 도입 검토 (namui 가 wasm-bindgen 채택 시).
- dylib dispatcher 동기화 API (`install_external_dispatcher`) — 현재는 dylib 단일 dispatcher 가정으로 충분하지만 다중 dylib 로드 시나리오가 발생하면 추가.

## 12. 검증

- `cargo check` / `cargo build` 가 모든 타깃에서 통과해야 한다.
  - `aarch64-apple-darwin`
  - `x86_64-pc-windows-msvc` (가능하면)
  - `wasm32-wasi-web`
- 기본 `namui::start` 호출만으로 stderr 에 INFO 이상이 출력되는지 수동 확인.
- `RUST_LOG=trace cargo run` 으로 trace 까지 출력되는지 확인.

## 13. Out of scope (명시적 제외)

- 사용자 게임 코드의 마이그레이션 (사용자가 자기 코드에서 `info!` / `warn!` 등을 직접 채택).
- 외부 로그 수집 서비스 연동 (Datadog, Sentry 등).
- `native-runner` 의 tracing 통합 (signal-safety 제약, `eprintln!` 유지).
- `namui-cli` 의 println 제거 (CLI 인터페이스로 보존).
- 인게임 콘솔의 UI 구현 (Phase 4).
- crash-reporter 의 실제 업로드 경로 (Phase 4 / 별도 브랜치).
