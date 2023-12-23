fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let target = args
        .get(1)
        .expect("target is required. pass it as the first argument");
    let dest = args
        .get(2)
        .expect("dest is required. pass it as the second argument");
    let output_key = args
        .get(3)
        .expect("output_key is required. pass it as the third argument");

    println!("target: {target}");
    println!("dest: {dest}");

    let cargo_toml_dirents = walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().file_name() == Some("Cargo.toml".as_ref()));

    let mut target_cargo_toml_paths = vec![];

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
            target_cargo_toml_paths.push(cargo_toml_path.parent().unwrap().to_owned());
        }
    }

    let output_json_array = format!(
        "[{}]",
        target_cargo_toml_paths
            .iter()
            .map(|path| format!("\"{}\"", path.to_str().unwrap()))
            .collect::<Vec<_>>()
            .join(",")
    );

    println!(
        "result: {output_key}={output_json_array}"
    );

    std::fs::write(dest, format!("{output_key}={output_json_array}")).unwrap();
}
