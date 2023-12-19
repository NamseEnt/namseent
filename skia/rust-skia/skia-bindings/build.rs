use std::{fs, io};

use build_support::{
    binaries_config,
    cargo::{self, Target},
    features, skia, skia_bindgen,
};

mod build_support;

fn main() -> Result<(), io::Error> {
    // since 0.25.0
    if cfg!(feature = "shaper") {
        cargo::warning("The feature 'shaper' has been removed. To use the SkShaper bindings, enable the feature 'textlayout'.");
    }

    if env::is_docs_rs_build() {
        println!("DETECTED DOCS_RS BUILD");
        return fake_bindings();
    }

    let skia_debug = env::is_skia_debug();
    let features = features::Features::default();
    let binaries_config =
        binaries_config::BinariesConfiguration::from_features(&features, skia_debug);

    #[allow(unused_variables)]
    let build_skia = true;

    #[cfg(feature = "binary-cache")]
    let build_skia = build_support::binary_cache::try_prepare_download(&binaries_config);

    assert!(!build_skia);

    binaries_config.commit_to_cargo();

    #[cfg(feature = "binary-cache")]
    if let Some(staging_directory) = build_support::binary_cache::should_export() {
        build_support::binary_cache::publish(&binaries_config, &staging_directory);
    }

    Ok(())
}

fn build_from_source(
    features: features::Features,
    binaries_config: &binaries_config::BinariesConfiguration,
    skia_source_dir: &std::path::Path,
    skia_debug: bool,
    offline: bool,
) -> skia::FinalBuildConfiguration {
    let build_config = skia::BuildConfiguration::from_features(features, skia_debug);
    let final_configuration = skia::FinalBuildConfiguration::from_build_configuration(
        &build_config,
        skia::env::use_system_libraries(),
        skia_source_dir,
    );

    skia::build(
        &final_configuration,
        binaries_config,
        skia::env::ninja_command(),
        skia::env::gn_command(),
        offline,
    );

    final_configuration
}

fn generate_bindings(
    features: &features::Features,
    definitions: Vec<skia_bindgen::Definition>,
    binaries_config: &binaries_config::BinariesConfiguration,
    skia_source_dir: &std::path::Path,
    target: Target,
    sysroot: Option<&str>,
) {
    // Emit the ninja definitions, to help debug build consistency.
    skia_bindgen::definitions::save_definitions(&definitions, &binaries_config.output_directory)
        .expect("failed to write Skia defines");

    let bindings_config = skia_bindgen::Configuration::new(features, definitions, skia_source_dir);
    skia_bindgen::generate_bindings(
        &bindings_config,
        &binaries_config.output_directory,
        target,
        sysroot,
    );
}

/// On docs.rs, rustdoc runs inside a container with no networking, so copy a pre-generated
/// `bindings.rs` file.
fn fake_bindings() -> Result<(), io::Error> {
    println!("COPYING bindings_docs.rs to OUT_DIR/skia/bindings.rs");
    let bindings_target = cargo::output_directory()
        .join(binaries_config::SKIA_OUTPUT_DIR)
        .join("bindings.rs");
    fs::copy("bindings_docs.rs", bindings_target).map(|_| ())
}

/// Environment variables used by this build script.
mod env {
    use crate::build_support::cargo;
    use std::path::PathBuf;

    /// The path to the Skia source directory.
    pub fn source_dir() -> Option<PathBuf> {
        cargo::env_var("SKIA_SOURCE_DIR").map(PathBuf::from)
    }

    /// The path to where a pre-built Skia library can be found.
    pub fn skia_lib_search_path() -> Option<PathBuf> {
        cargo::env_var("SKIA_LIBRARY_SEARCH_PATH").map(PathBuf::from)
    }

    pub fn is_skia_debug() -> bool {
        matches!(cargo::env_var("SKIA_DEBUG"), Some(v) if v != "0")
    }

    pub fn is_docs_rs_build() -> bool {
        matches!(cargo::env_var("DOCS_RS"), Some(v) if v != "0")
    }
}
