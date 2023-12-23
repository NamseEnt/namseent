fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let target = args
        .get(1)
        .expect("target is required. pass it as the first argument");
    let dest = args
        .get(2)
        .expect("dest is required. pass it as the second argument");

    println!("target: {target}");
    println!("dest: {dest}");

    let cargo_toml_dirents = walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().file_name() == Some("Cargo.toml".as_ref()));

    let mut target_cargo_toml_paths = vec![];

    for cargo_toml_dirent in cargo_toml_dirents {
        let cargo_toml_path = cargo_toml_dirent.path();
        let cargo_toml = std::fs::read_to_string(cargo_toml_path)
            .unwrap()
            .parse::<toml::Table>()
            .unwrap();

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

    println!("result: {}", output_json_array);

    std::fs::write(dest, output_json_array).unwrap();
}
