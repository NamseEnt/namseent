use std::{fs, path::PathBuf};

const SEPARATOR: &str = ":";
const COMMENT_FLAG: &str = "#";

type NamuiBundleIndex = Vec<(String, String)>;

pub fn get_namui_bundle_index(project_root_path: &PathBuf) -> Result<NamuiBundleIndex, String> {
    let namui_bundle_index_path = project_root_path.join(".namuibundle");
    if !namui_bundle_index_path.exists() {
        return Ok(vec![]);
    }
    fs::read(namui_bundle_index_path)
        .map_err(|error| format!("namui config read error: {}", error))
        .and_then(|file| {
            let namui_bundle_index_string = String::from_utf8(file)
                .map_err(|error| format!("parse namui_bundle fail: {}", error))?;
            parse_namui_bundle_index(namui_bundle_index_string)
        })
}

fn parse_namui_bundle_index(namui_bundle_index_string: String) -> Result<NamuiBundleIndex, String> {
    Ok(namui_bundle_index_string
        .lines()
        .filter_map(|line| split_by_separator(line))
        .collect())
}

fn split_by_separator(line: &str) -> Option<(String, String)> {
    if line.starts_with(COMMENT_FLAG) {
        return None;
    }
    if line.trim() == "" {
        return None;
    }
    match line.find(SEPARATOR) {
        Some(index) => Some((
            line[0..index].to_string(),
            line[index + 1..line.len()].to_string(),
        )),
        None => Some((line.to_string(), "".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::split_by_separator;
    use crate::util::get_namui_bundle_index::{COMMENT_FLAG, SEPARATOR};

    #[test]
    fn parse_namui_bundle_index() {
        let namui_bundle_index_string = vec![
            format!("{}should be ignore", COMMENT_FLAG),
            format!("    "),
            format!("path/of/src{}path/of/dest", SEPARATOR),
            format!("path/of/src"),
            format!("path/of/src{}", SEPARATOR),
            format!("{}path/of/dest", SEPARATOR),
        ]
        .join("\n");
        assert_eq!(
            super::parse_namui_bundle_index(namui_bundle_index_string).unwrap(),
            vec![
                ("path/of/src".to_string(), "path/of/dest".to_string()),
                ("path/of/src".to_string(), "".to_string()),
                ("path/of/src".to_string(), "".to_string()),
                ("".to_string(), "path/of/dest".to_string())
            ]
        )
    }

    #[test]
    fn split_ignore_comment() {
        let line = format!("{}should be ignore", COMMENT_FLAG);
        assert_eq!(split_by_separator(line.as_str()), None);
    }

    #[test]
    fn split_ignore_empty() {
        let line = "    ";
        assert_eq!(split_by_separator(line), None);
    }

    #[test]
    fn split_with_separator() {
        let line = format!("path/of/src{}path/of/dest", SEPARATOR);
        assert_eq!(
            split_by_separator(line.as_str()),
            Some(("path/of/src".to_string(), "path/of/dest".to_string()))
        );
    }

    #[test]
    fn split_without_separator() {
        let line = "path/of/src";
        assert_eq!(
            split_by_separator(line),
            Some(("path/of/src".to_string(), "".to_string()))
        );
    }

    #[test]
    fn split_without_src_but_with_separator() {
        let line = format!("path/of/src{}", SEPARATOR);
        assert_eq!(
            split_by_separator(line.as_str()),
            Some(("path/of/src".to_string(), "".to_string()))
        );
    }

    #[test]
    fn split_without_dest_but_with_separator() {
        let line = format!("{}path/of/dest", SEPARATOR);
        assert_eq!(
            split_by_separator(line.as_str()),
            Some(("".to_string(), "path/of/dest".to_string()))
        );
    }
}
