use crate::file::types::PathLike;
use namui_cfg::namui_cfg;

pub fn create_bundle_url(path_like: impl PathLike) -> String {
    let mut components: Vec<String> = Vec::new();
    for component in path_like.path().components() {
        match component {
            std::path::Component::CurDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => (),
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::Normal(os_str) => {
                components.push(String::from(os_str.to_str().unwrap_or("")));
            }
        }
    }
    components.insert(0, bundle_url_prefix());
    components.join("/")
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub fn bundle_url_prefix() -> String {
    "/bundle".to_string()
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub fn bundle_url_prefix() -> String {
    "bundle".to_string()
}

#[cfg(test)]
mod tests {
    use namui_cfg::namui_cfg;
    use std::path::PathBuf;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[namui_cfg(all(target_env = "electron", not(watch_reload)))]
    #[test]
    #[wasm_bindgen_test]
    fn create_bundle_url() {
        let test_cases: Vec<(PathBuf, String)> = vec![
            ("", "bundle"),
            ("/", "bundle"),
            ("path", "bundle/path"),
            ("path/to/some/where", "bundle/path/to/some/where"),
            ("path/to/some/where/", "bundle/path/to/some/where"),
            ("/path/to/some/where", "bundle/path/to/some/where"),
            ("path/to/./some/where", "bundle/path/to/some/where"),
            ("path/to/../some/where", "bundle/path/some/where"),
            ("path/to/../../../some/where", "bundle/some/where"),
        ]
        .into_iter()
        .map(|(path, url)| (PathBuf::from(path), url.to_string()))
        .collect();
        for (input, expected) in test_cases {
            let output = super::create_bundle_url(input);
            assert_eq!(output, expected);
        }
    }

    #[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
    #[test]
    #[wasm_bindgen_test]
    fn create_bundle_url() {
        let test_cases: Vec<(PathBuf, String)> = vec![
            ("", "/bundle"),
            ("/", "/bundle"),
            ("path", "/bundle/path"),
            ("path/to/some/where", "/bundle/path/to/some/where"),
            ("path/to/some/where/", "/bundle/path/to/some/where"),
            ("/path/to/some/where", "/bundle/path/to/some/where"),
            ("path/to/./some/where", "/bundle/path/to/some/where"),
            ("path/to/../some/where", "/bundle/path/some/where"),
            ("path/to/../../../some/where", "/bundle/some/where"),
        ]
        .into_iter()
        .map(|(path, url)| (PathBuf::from(path), url.to_string()))
        .collect();
        for (input, expected) in test_cases {
            let output = super::create_bundle_url(input);
            assert_eq!(output, expected);
        }
    }
}
