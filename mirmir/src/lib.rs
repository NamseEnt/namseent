#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_apfloat;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_type_ir;

use regex::Regex;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Instant;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::*;

struct JsTranspileCallback {}

static START_TIME: OnceLock<Instant> = OnceLock::new();

impl Callbacks for JsTranspileCallback {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, _tcx: TyCtxt<'tcx>) -> Compilation {
        let elapsed = START_TIME.get().unwrap().elapsed();
        println!("transpiled in {} ms", elapsed.as_millis());

        Compilation::Stop
    }
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
}

fn find_project_root(start_path: &Path) -> Option<PathBuf> {
    let mut current = start_path;

    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            return Some(current.to_path_buf());
        }

        current = current.parent()?;
    }
}

fn extract_rustc_command(project_root: &Path, crate_name: &str) -> Result<String, String> {
    println!("🔨 cargo build 실행 중 (의존성 빌드)...");
    let build_status = Command::new("cargo")
        .arg("build")
        .current_dir(project_root)
        .status()
        .map_err(|e| format!("cargo build 실행 실패: {}", e))?;

    if !build_status.success() {
        return Err("cargo build 실패".to_string());
    }

    println!("✅ 의존성 빌드 완료");

    // 해당 크레이트의 빌드 결과 삭제 (재컴파일을 위해)
    println!("🗑️  기존 빌드 결과 삭제...");
    let crate_name_underscore = crate_name.replace("-", "_");
    let target_dir = project_root.join("target/debug/deps");

    // rlib, rmeta, .d 파일 삭제
    if let Ok(entries) = std::fs::read_dir(&target_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if (file_name_str.starts_with(&format!("lib{}", crate_name_underscore))
                || file_name_str.starts_with(&crate_name_underscore))
                && (file_name_str.ends_with(".rlib")
                    || file_name_str.ends_with(".rmeta")
                    || file_name_str.ends_with(".d"))
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }

    println!("📝 rustc 명령어 추출 중...");

    let output = Command::new("cargo")
        .arg("build")
        .arg("-v")
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("cargo build -v 실행 실패: {}", e))?;

    // stdout과 stderr 모두 확인
    let stderr_str = String::from_utf8_lossy(&output.stderr);
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let combined_output = format!("{}\n{}", stdout_str, stderr_str);

    // tower_defense 같은 하이픈이 있는 크레이트는 언더스코어로 변환됨
    let crate_name_underscore = crate_name.replace("-", "_");

    let pattern = format!(
        r"Running.*rustc.*--crate-name {}",
        regex::escape(&crate_name_underscore)
    );
    let re = Regex::new(&pattern).map_err(|e| format!("정규식 생성 실패: {}", e))?;

    for line in combined_output.lines() {
        if re.is_match(line) {
            // "Running `" 부분 제거
            let cmd = line
                .trim()
                .strip_prefix("Running `")
                .and_then(|s| s.strip_suffix("`"))
                .ok_or("명령어 추출 실패")?;

            println!("✅ rustc 명령어 추출 완료");
            println!();
            println!("📋 추출된 rustc 명령어:");
            println!("{}", cmd);
            println!();
            return Ok(cmd.to_string());
        }
    }

    Err(format!(
        "크레이트 '{}'의 rustc 명령어를 찾을 수 없습니다",
        crate_name
    ))
}

pub fn run(path: &str) {
    println!("=========================================");
    println!("  mirmir: Rust to JS Transpiler");
    println!("=========================================");
    println!();

    let file_path = Path::new(path);
    let abs_path = file_path
        .canonicalize()
        .expect("파일 경로를 절대 경로로 변환할 수 없습니다");

    let project_root = abs_path
        .parent()
        .and_then(find_project_root)
        .expect("Cargo.toml을 찾을 수 없습니다");

    println!("📂 프로젝트 루트: {}", project_root.display());
    println!();

    // cargo metadata 실행하여 크레이트 이름 가져오기
    println!("📝 패키지 메타데이터 수집 중...");
    let metadata_output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .current_dir(&project_root)
        .output()
        .expect("cargo metadata 실행 실패");

    let metadata: CargoMetadata =
        serde_json::from_slice(&metadata_output.stdout).expect("cargo metadata JSON 파싱 실패");

    let package = &metadata.packages[0];
    let crate_name = &package.name;
    let pkg_version = &package.version;
    println!("   Package: {} v{}", crate_name, pkg_version);
    println!();

    // rustc 명령어 추출
    let rustc_cmd =
        extract_rustc_command(&project_root, crate_name).expect("rustc 명령어 추출 실패");

    // 명령어를 인자로 파싱
    let mut args: Vec<String> = shlex::split(&rustc_cmd).expect("명령어 파싱 실패");

    // 첫 번째 인자는 rustc 경로이므로 제거하고 "ignored"로 대체
    if !args.is_empty() {
        args[0] = "ignored".to_string();
    }

    println!();
    println!("🚀 rustc_driver 실행 중...");
    println!();

    // rustc_driver를 실행하기 전에 프로젝트 루트로 디렉토리 변경
    std::env::set_current_dir(&project_root).expect("프로젝트 루트로 디렉토리 변경 실패");

    // cargo 환경 변수 설정
    println!("🔧 cargo 환경 변수 설정 중...");
    set_cargo_env_vars(&project_root, crate_name, pkg_version);
    println!();

    let mut callback = JsTranspileCallback {};
    START_TIME.get_or_init(Instant::now);
    rustc_driver::run_compiler(&args, &mut callback);
}

fn set_cargo_env_vars(project_root: &Path, crate_name: &str, version: &str) {
    unsafe {
        let manifest_path = project_root.join("Cargo.toml");

        // CARGO_* 기본 변수들
        if let Ok(cargo_path) = which::which("cargo") {
            std::env::set_var("CARGO", cargo_path);
        }
        std::env::set_var("CARGO_MANIFEST_DIR", project_root);
        std::env::set_var("CARGO_MANIFEST_PATH", &manifest_path);
        std::env::set_var("CARGO_PKG_NAME", crate_name);
        std::env::set_var("CARGO_PKG_VERSION", version);

        // 버전 파싱
        let version_parts: Vec<&str> = version.split('.').collect();
        if version_parts.len() >= 3 {
            std::env::set_var("CARGO_PKG_VERSION_MAJOR", version_parts[0]);
            std::env::set_var("CARGO_PKG_VERSION_MINOR", version_parts[1]);

            let patch_and_pre: Vec<&str> = version_parts[2].split('-').collect();
            std::env::set_var("CARGO_PKG_VERSION_PATCH", patch_and_pre[0]);
            if patch_and_pre.len() > 1 {
                std::env::set_var("CARGO_PKG_VERSION_PRE", patch_and_pre[1]);
            } else {
                std::env::set_var("CARGO_PKG_VERSION_PRE", "");
            }
        }

        std::env::set_var("CARGO_PKG_AUTHORS", "");
        std::env::set_var("CARGO_PKG_DESCRIPTION", "");
        std::env::set_var("CARGO_PKG_HOMEPAGE", "");
        std::env::set_var("CARGO_PKG_REPOSITORY", "");
        std::env::set_var("CARGO_PKG_LICENSE", "");
        std::env::set_var("CARGO_PKG_LICENSE_FILE", "");
        std::env::set_var("CARGO_PKG_README", "");
        std::env::set_var("CARGO_PKG_RUST_VERSION", "");

        let crate_name_underscore = crate_name.replace("-", "_");
        std::env::set_var("CARGO_CRATE_NAME", &crate_name_underscore);
        std::env::set_var("CARGO_PRIMARY_PACKAGE", "1");

        // Target/Host 정보
        let host_triple = get_host_triple();
        std::env::set_var("TARGET", &host_triple);
        std::env::set_var("HOST", &host_triple);

        // CARGO_CFG_* 변수들 (aarch64-apple-darwin 기준으로 하드코딩)
        // TODO: 실제 타겟에 따라 동적으로 설정해야 함
        std::env::set_var("CARGO_CFG_TARGET_ARCH", "aarch64");
        std::env::set_var("CARGO_CFG_TARGET_OS", "macos");
        std::env::set_var("CARGO_CFG_TARGET_FAMILY", "unix");
        std::env::set_var("CARGO_CFG_TARGET_ENV", "");
        std::env::set_var("CARGO_CFG_TARGET_POINTER_WIDTH", "64");
        std::env::set_var("CARGO_CFG_TARGET_ENDIAN", "little");
        std::env::set_var("CARGO_CFG_TARGET_VENDOR", "apple");
        std::env::set_var("CARGO_CFG_TARGET_HAS_ATOMIC", "128,16,32,64,8,ptr");
        std::env::set_var("CARGO_CFG_UNIX", "1");

        // Profile 정보
        std::env::set_var("PROFILE", "debug");
        std::env::set_var("DEBUG", "true");
        std::env::set_var("OPT_LEVEL", "0");

        // 컴파일러 정보
        if let Ok(rustc_path) = which::which("rustc") {
            std::env::set_var("RUSTC", rustc_path);
        }
        if let Ok(rustdoc_path) = which::which("rustdoc") {
            std::env::set_var("RUSTDOC", rustdoc_path);
        }

        // Dynamic library path (macOS)
        if cfg!(target_os = "macos") {
            let home = std::env::var("HOME").unwrap_or_default();
            std::env::set_var(
                "DYLD_FALLBACK_LIBRARY_PATH",
                format!("{}/lib:/usr/local/lib:/usr/lib", home),
            );
        }
    }
}

fn get_host_triple() -> String {
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .arg("--verbose")
        .output()
        .expect("rustc --version --verbose 실행 실패");

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.starts_with("host:") {
            return line.split(':').nth(1).unwrap().trim().to_string();
        }
    }

    panic!("host triple을 찾을 수 없습니다");
}
