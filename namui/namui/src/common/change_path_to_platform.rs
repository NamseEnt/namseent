use crate::file::types::PathLike;
use std::path::PathBuf;

pub(crate) fn change_path_to_platform(
    platform_prefix: impl AsRef<std::path::Path>,
    path_like: impl PathLike,
) -> PathBuf {
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

    platform_prefix.as_ref().join(output_path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn change_path_to_platform() {
        let bundle_path = "abc";
        let test_cases_platform_prefix_expected: Vec<(PathBuf, PathBuf)> = vec![
            ("", "abc"),
            ("/", "/abc"),
            ("path", "path/abc"),
            ("path/to/some/where", "path/to/some/where/abc"),
            ("path/to/some/where/", "path/to/some/where//abc"),
            ("/path/to/some/where", "/path/to/some/where/abc"),
            ("path/to/./some/where", "path/to/./some/where/abc"),
            ("path/to/../some/where", "path/to/../some/where/abc"),
            (
                "path/to/../../../some/where",
                "path/to/../../../some/where/abc",
            ),
        ]
        .into_iter()
        .map(|(from, to)| (PathBuf::from(from), PathBuf::from(to)))
        .collect();
        for (input, expected) in test_cases_platform_prefix_expected {
            let output = super::change_path_to_platform(input, bundle_path);
            assert_eq!(output, expected);
        }
    }
}
