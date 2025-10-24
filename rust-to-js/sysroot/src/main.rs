use rustc_build_sysroot::{BuildMode, SysrootBuilder, SysrootConfig};
use std::path::PathBuf;

fn build_custom_sysroot() -> PathBuf {
    let sysroot = SysrootBuilder::new(
        &std::env::current_dir().unwrap().join("custom_sysroot"),
        "wasm32-wasip1-threads",
    )
    .rustflags(&["-Zalways-encode-mir"])
    .build_mode(BuildMode::Build) // 또는 BuildMode::Check
    .sysroot_config(SysrootConfig::WithStd {
        std_features: vec![],
    })
    .build_from_source(std::path::Path::new("rustc"))
    .expect("Failed to build sysroot");

    sysroot.sysroot
}

fn main() {
    let sysroot_path = build_custom_sysroot();

    // rustc_driver에서 사용
    let mut args = vec![
        "rustc".to_string(),
        "--sysroot".to_string(),
        sysroot_path.to_string_lossy().to_string(),
        // ... 나머지 args
    ];

    rustc_driver::RunCompiler::new(&args, &mut MyCallbacks).run();
}
