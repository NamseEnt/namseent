use crate::*;
use std::{fs, path::PathBuf};

pub struct DeepLinkManifest {
    path: PathBuf,
    deep_link_schemes: Vec<String>,
}
impl DeepLinkManifest {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn deep_link_schemes(&self) -> &Vec<String> {
        &self.deep_link_schemes
    }

    pub fn try_load(project_root_path: &PathBuf) -> Result<Option<Self>> {
        let namui_deep_link_manifest_path = project_root_path.join(".namuideeplink");
        match namui_deep_link_manifest_path.exists() {
            true => {
                let namui_deep_link_manifest_file = fs::read(&namui_deep_link_manifest_path)?;
                let namui_deep_link_manifest_string =
                    String::from_utf8(namui_deep_link_manifest_file)?;
                let deep_link_schemes =
                    parse_namui_deep_link_manifest(namui_deep_link_manifest_string.as_str());

                Ok(Some(DeepLinkManifest {
                    path: namui_deep_link_manifest_path.clone(),
                    deep_link_schemes,
                }))
            }
            false => Ok(None),
        }
    }
}

fn parse_namui_deep_link_manifest(namui_deep_link_manifest_string: &str) -> Vec<String> {
    let mut deep_link_schemes = Vec::new();
    for line in namui_deep_link_manifest_string.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 || trimmed_line.starts_with("#") {
            continue;
        }
        deep_link_schemes.push(trimmed_line.to_string());
    }
    deep_link_schemes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let input = "
        # comment
        one

        two
        "
        .to_string();
        let expected_schemes = vec!["one", "two"];
        let mut actual_schemes = parse_namui_deep_link_manifest(input.as_str()).into_iter();
        for expected_scheme in expected_schemes {
            assert_eq!(actual_schemes.next().unwrap(), expected_scheme);
        }
    }
}
