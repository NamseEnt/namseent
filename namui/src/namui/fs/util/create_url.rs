use crate::fs::types::PathLike;

pub fn create_url(path_like: impl PathLike) -> String {
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
    components.insert(0, "/bundle".to_string());
    components.join("/")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn create_url() {
        let test_cases: Vec<(PathBuf, String)> = vec![
            (PathBuf::from(""), "/bundle".to_string()),
            (PathBuf::from("/"), "/bundle".to_string()),
            (PathBuf::from("path"), "/bundle/path".to_string()),
            (
                PathBuf::from("path/to/some/where"),
                "/bundle/path/to/some/where".to_string(),
            ),
            (
                PathBuf::from("path/to/some/where/"),
                "/bundle/path/to/some/where".to_string(),
            ),
            (
                PathBuf::from("/path/to/some/where"),
                "/bundle/path/to/some/where".to_string(),
            ),
            (
                PathBuf::from("path/to/./some/where"),
                "/bundle/path/to/some/where".to_string(),
            ),
            (
                PathBuf::from("path/to/../some/where"),
                "/bundle/path/some/where".to_string(),
            ),
            (
                PathBuf::from("path/to/../../../some/where"),
                "/bundle/some/where".to_string(),
            ),
        ];
        for (input, expected) in test_cases {
            let output = super::create_url(input);
            assert_eq!(output, expected);
        }
    }
}
