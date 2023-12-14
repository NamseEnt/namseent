use crate::file::types::PathLike;
use anyhow::Result;
use std::path::PathBuf;

pub(crate) fn change_path_to_platform(
    platform_prefix: impl AsRef<std::path::Path>,
    path_like: impl PathLike,
) -> Result<PathBuf> {
    let path = path_like.path();
    let mut components = path.components();
    let mut output_path = std::path::PathBuf::new();

    let first = components.next().unwrap();
    if let Some(colon_index) = first.as_os_str().to_str().unwrap().find(':') {
        output_path.push(&first.as_os_str().to_str().unwrap()[colon_index + 1..]);
    } else {
        output_path.push(first.as_os_str());
    };

    for component in components {
        match component {
            std::path::Component::CurDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => (),
            std::path::Component::ParentDir => {
                output_path.pop();
            }
            std::path::Component::Normal(os_str) => {
                output_path.push(os_str);
            }
        }
    }
    crate::log!("let path: {}", output_path.display());

    let path = platform_prefix.as_ref().join(output_path);

    crate::log!("change_path_to_platform: {}", path.display());
    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn change_path_to_platform() {
        let test_cases: Vec<(PathBuf, PathBuf)> = vec![
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
        .map(|(from, to)| (PathBuf::from(from), PathBuf::from(to)))
        .collect();
        for (input, expected) in test_cases {
            let output = super::change_path_to_platform(input, "bundle").unwrap();
            assert_eq!(output, expected);
        }
    }
}
