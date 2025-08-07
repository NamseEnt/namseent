fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let target = args
        .get(1)
        .expect("target is required. pass it as the first argument");

    println!("target: {target}");

    let cargo_toml_dirents = walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().file_name() == Some("Cargo.toml".as_ref()));

    let mut target_project_paths = vec![];

    for cargo_toml_dirent in cargo_toml_dirents {
        let cargo_toml_path = cargo_toml_dirent.path();
        let content = std::fs::read_to_string(cargo_toml_path).unwrap();

        let cargo_toml = content.parse::<toml::Value>().unwrap_or_else(|e| {
            panic!(
                "failed to parse Cargo.toml. path: {}, error: {}",
                cargo_toml_path.to_str().unwrap(),
                e
            )
        });
        if cargo_toml.is_str() {
            continue;
        }

        let cargo_toml = cargo_toml.as_table().unwrap_or_else(|| {
            panic!(
                "cargo_toml should be a table. path: {}",
                cargo_toml_path.to_str().unwrap()
            )
        });

        if Some(true)
            == || -> Option<bool> {
                Some(
                    cargo_toml
                        .get("package")?
                        .get("metadata")?
                        .get("namui")?
                        .get("targets")?
                        .as_array()?
                        .iter()
                        .any(|value| value.is_str() && value.as_str().unwrap() == target),
                )
            }()
        {
            target_project_paths.push(cargo_toml_path.parent().unwrap().to_owned());
        }
    }

    let cargo_version = {
        let output = std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .unwrap();
        let version = String::from_utf8_lossy(&output.stdout);
        version.split_whitespace().nth(1).unwrap().to_string()
    };

    let output_cargo_toml = format!(
        r#"[workspace]
resolver = "3"
package.rust-version = "{cargo_version}"
exclude = [
    "namui/third-party-forks/rusqlite",
    "namui/third-party-forks/rust-skia/skia-safe",
    "namui/third-party-forks/tokio/tokio",
    "namui/third-party-forks/tokio/tokio-stream",
    "namui/namui-cli",
    "github-actions-tools/workspace-maker",
]
members = [{}]
"#,
        target_project_paths
            .iter()
            .map(|path| format!(
                "\"{}\"",
                &path.to_str().unwrap()[2..] // remove "./"
                    .replace("\\", "/") // cargo support only "/"
            ))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    println!("{output_cargo_toml}");

    std::fs::write("Cargo.toml", output_cargo_toml).unwrap();
}
