# Custom Rust Sysroot with MIR Encoding for wasm32-wasip1-threads

이 프로젝트는 wasm32-wasip1-threads 타겟을 위한 `-Zalways-encode-mir` 플래그가 적용된 커스텀 Rust sysroot를 빌드하고, std 라이브러리의 MIR에 접근 가능한지 검증합니다.

## 프로젝트 개요

Rust 컴파일러는 일반적으로 표준 라이브러리(std, core)의 MIR(Mid-level Intermediate Representation)을 메타데이터에 포함하지 않습니다. 이 프로젝트는 `-Zalways-encode-mir` 플래그를 사용하여 std 라이브러리를 다시 빌드함으로써, 런타임에 std 함수들의 MIR에 접근할 수 있도록 합니다.

## 디렉토리 구조

```
/Users/namse/custom-sysroot/
├── README.md                          # 이 파일
├── sysroot-builder/                   # Sysroot 빌드 프로젝트
│   ├── Cargo.toml
│   ├── .cargo/config.toml            # build-std 설정
│   └── src/lib.rs
├── custom-sysroot/                    # 생성된 커스텀 sysroot
│   └── lib/rustlib/wasm32-wasip1-threads/lib/
│       ├── libstd-*.rlib
│       ├── libcore-*.rlib
│       ├── liballoc-*.rlib
│       └── ... (기타 rlib 파일들)
└── mir-verifier/                      # MIR 접근 검증 프로그램
    ├── Cargo.toml
    └── src/main.rs
```

## 빌드 과정

### 1. 환경 준비

필요한 Rust 툴체인 및 컴포넌트 설치:

```bash
# Nightly 툴체인 설치
rustup toolchain install nightly

# 필수 컴포넌트 설치
rustup component add rust-src --toolchain nightly
rustup component add rustc-dev --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly
rustup target add wasm32-wasip1-threads --toolchain nightly
```

### 2. Sysroot 빌드 프로젝트 생성

```bash
# 프로젝트 루트 디렉토리 생성
mkdir -p custom-sysroot
cd custom-sysroot

# Sysroot 빌드용 라이브러리 프로젝트 생성
cargo new --lib sysroot-builder
cd sysroot-builder
```

### 3. Build-std 설정

`.cargo/config.toml` 파일 생성:

```toml
[build]
target = "wasm32-wasip1-threads"

[unstable]
build-std = ["std", "core", "alloc", "panic_abort"]
```

### 4. MIR 인코딩과 함께 빌드

```bash
cd sysroot-builder

# -Zalways-encode-mir 플래그를 사용하여 std, core 빌드
RUSTFLAGS="-Zalways-encode-mir" cargo +nightly build \
  --target wasm32-wasip1-threads \
  -Z build-std=std,core
```

이 명령은 다음을 수행합니다:

- `-Zalways-encode-mir`: 모든 함수의 MIR을 메타데이터에 인코딩
- `-Z build-std`: 표준 라이브러리를 소스부터 빌드
- `wasm32-wasip1-threads`: 타겟 플랫폼 지정

### 5. Sysroot 디렉토리 구조 생성

```bash
cd ..

# Sysroot 디렉토리 구조 생성
mkdir -p custom-sysroot/lib/rustlib/wasm32-wasip1-threads/lib

# 빌드된 라이브러리 파일 복사
cp sysroot-builder/target/wasm32-wasip1-threads/debug/deps/*.rlib \
   custom-sysroot/lib/rustlib/wasm32-wasip1-threads/lib/
```

생성된 sysroot는 다음 파일들을 포함합니다:

- `libcore-*.rlib` (~40MB) - Core 라이브러리
- `libstd-*.rlib` (~12MB) - 표준 라이브러리
- `liballoc-*.rlib` (~5.8MB) - 할당자 라이브러리
- 기타 의존성 라이브러리들

## MIR 검증

### 검증 프로그램 생성

```bash
# 검증 프로그램 생성
cargo +nightly new mir-verifier
cd mir-verifier
```

### 검증 프로그램 코드 (`src/main.rs`)

검증 프로그램은 다음을 수행합니다:

1. **rustc_driver API 사용**: 컴파일러를 프로그래밍 방식으로 실행
2. **Callbacks 구현**: 컴파일 과정에 후킹하여 분석 후 MIR 접근
3. **tcx.optimized_mir() 호출**: 로컬 및 외부 크레이트 함수의 MIR 조회
4. **std 함수 MIR 검증**: println! 매크로가 호출하는 std 함수들의 MIR 확인

핵심 기능:

```rust
#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_middle;
extern crate rustc_interface;

impl rustc_driver::Callbacks for MirVerifier {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        // 로컬 코드의 MIR 추출
        for local_def_id in tcx.mir_keys(()) {
            let mir = tcx.optimized_mir(local_def_id.to_def_id());
            // MIR 분석...
        }

        // std 함수 호출 분석
        // tcx.optimized_mir()로 std 라이브러리 함수의 MIR 접근
        // ...
    }
}
```

### 검증 실행

```bash
cd mir-verifier
cargo +nightly build --release
cargo +nightly run --release
```

## 검증 결과

테스트 코드:

```rust
fn main() {
    let a = 5;
    println!("{}", a);
}
```

### ✅ 성공적으로 접근한 std 함수 MIR:

1. **`core::fmt::rt::Argument::<'_>::new_display`**

   - DefId: `DefId(2:12034 ~ core[c456]::fmt::rt::{impl#0}::new_display)`
   - Basic blocks: 3개
   - MIR 완전히 접근 가능

2. **`core::fmt::rt::<impl std::fmt::Arguments<'a>>::new_v1`**

   - DefId: `DefId(2:12063 ~ core[c456]::fmt::rt::{impl#1}::new_v1)`
   - Basic blocks: 1개
   - MIR 완전히 접근 가능

3. **`std::io::_print`**
   - DefId: `DefId(1:3771 ~ std[8ef9]::io::stdio::_print)`
   - Basic blocks: 2개
   - MIR 완전히 접근 가능

### 결론

✅ **`-Zalways-encode-mir` 플래그가 정상적으로 작동함**

커스텀 sysroot를 사용하면 std 라이브러리의 함수들의 MIR에 `tcx.optimized_mir(def_id)`를 통해 접근할 수 있습니다. 이는 다음과 같은 용도로 활용될 수 있습니다:

- 정적 분석 도구 개발
- 컴파일러 플러그인 작성
- 런타임 코드 검증
- 프로그램 분석 및 최적화

## 커스텀 Sysroot 사용법

다른 프로젝트에서 이 커스텀 sysroot를 사용하려면:

```bash
rustc +nightly your_code.rs \
  --target wasm32-wasip1-threads \
  --sysroot=/Users/namse/custom-sysroot/custom-sysroot \
  -Zalways-encode-mir
```

또는 Cargo 프로젝트에서:

```bash
RUSTFLAGS="--sysroot=/Users/namse/custom-sysroot/custom-sysroot -Zalways-encode-mir" \
cargo +nightly build --target wasm32-wasip1-threads
```

## 주요 파일

- `sysroot-builder/.cargo/config.toml` - build-std 설정
- `custom-sysroot/lib/rustlib/wasm32-wasip1-threads/lib/` - 빌드된 라이브러리들
- `mir-verifier/src/main.rs` - MIR 접근 검증 코드

## 기술 스택

- **Rust Nightly**: 1.92.0-nightly (2025-10-22)
- **Target**: wasm32-wasip1-threads
- **APIs**: rustc_driver, rustc_middle, rustc_interface
- **Flags**: `-Zalways-encode-mir`, `-Z build-std`

## 주의사항

1. **Nightly 전용**: 이 프로젝트는 Rust nightly 툴체인이 필요합니다.
2. **빌드 시간**: 전체 std 라이브러리를 다시 빌드하므로 시간이 소요됩니다 (~15초).
3. **파일 크기**: MIR을 포함하면 라이브러리 파일 크기가 증가합니다.
4. **API 안정성**: rustc 내부 API는 불안정하므로 nightly 버전에 따라 코드 수정이 필요할 수 있습니다.

## 라이센스

이 프로젝트는 교육 및 연구 목적으로 작성되었습니다.
