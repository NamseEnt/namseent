use serde::Deserialize;
use std::{env::current_dir, fs::read_to_string, path::PathBuf, str::FromStr};

#[derive(Deserialize, Debug)]
struct NamuiConfigFromJson {
    resources: Option<String>,
}

#[derive(Debug)]
pub struct NamuiConfig {
    // extends by engine
    pub resources: Option<String>,
    // set by engine
    pub root_directory_path: String,
}

pub fn get_namui_config<'a>(manifest_path: &'a str) -> NamuiConfig {
    let root_directory_path = get_project_root_directory_path(manifest_path);
    let namui_config_path = root_directory_path.join("namui-config.json");
    let namui_config_string = read_to_string(&namui_config_path)
        .expect(format!("no such file or directory {:?}", &namui_config_path).as_str());
    let namui_config: NamuiConfigFromJson = serde_json::from_str(&namui_config_string).unwrap();

    NamuiConfig {
        resources: if let Some(resources) = namui_config.resources {
            Some(
                root_directory_path
                    .join(resources)
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            )
        } else {
            None
        },
        root_directory_path: root_directory_path
            .into_os_string()
            .into_string()
            .unwrap(),
    }
}

fn get_project_root_directory_path<'a>(manifest_path: &'a str) -> PathBuf {
    let mut current_dir_path = current_dir().unwrap();
    let mut manifest_dir_path = PathBuf::from_str(manifest_path).unwrap();
    manifest_dir_path.pop();
    current_dir_path.push(manifest_dir_path);
    current_dir_path
}
