use crate::{change_path_to_platform::change_path_to_platform, file::types::PathLike};

pub fn create_bundle_url(path_like: impl PathLike) -> String {
    change_path_to_platform("bundle", path_like)
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use wasm_bindgen_test::wasm_bindgen_test;

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
}
