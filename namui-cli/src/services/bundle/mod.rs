mod lexicon;
mod token;

use super::resource_collect_service::CollectOperation;
use crate::util::get_cli_root_path;
use lexicon::*;
use std::{collections::HashMap, path::PathBuf};
use token::*;

#[derive(Debug)]
pub struct NamuiBundleManifest {
    project_bundle: Bundle,
    system_bundle: Bundle,
    project_root_path: PathBuf,
    metadata_json: String,
    url_src_path_map: HashMap<PathBuf, PathBuf>,
}

#[derive(Debug)]
struct Bundle {
    include: Vec<IncludeOperation>,
    exclude: Vec<ExcludeOperation>,
}

impl NamuiBundleManifest {
    pub fn parse(project_root_path: PathBuf) -> Result<Self, crate::Error> {
        let project_bundle = parse_bundle(&project_root_path)?;
        let system_bundle = parse_bundle(&get_cli_root_path())?;

        let url_src_path_map = {
            let project_bundle_query = project_bundle.query(&project_root_path, &PathBuf::new())?;
            let system_bundle_query = system_bundle.query(&get_cli_root_path(), &PathBuf::new())?;
            let merged_query = {
                let mut merged_query = HashMap::new();
                merged_query.extend(project_bundle_query);
                merged_query.extend(system_bundle_query);
                merged_query
            };
            convert_src_dest_path_map_to_url_src_path_map(merged_query)
        };

        let metadata_json = {
            let url_list: Vec<&PathBuf> = url_src_path_map.keys().collect();
            let bundle_metadata_string = serde_json::to_string(&url_list).map_err(|error| {
                format!("serde_json error while creating bundle_metadata: {}", error)
            })?;

            bundle_metadata_string
        };
        Ok(Self {
            project_bundle,
            system_bundle,
            project_root_path,
            url_src_path_map,
            metadata_json,
        })
    }

    fn query(&self, dest_root_path: &PathBuf) -> Result<HashMap<PathBuf, PathBuf>, crate::Error> {
        let project_bundle_query = self
            .project_bundle
            .query(&self.project_root_path, dest_root_path)?;
        let system_bundle_query = self
            .system_bundle
            .query(&get_cli_root_path(), dest_root_path)?;

        let mut merged_query = HashMap::new();
        merged_query.extend(project_bundle_query);
        merged_query.extend(system_bundle_query);
        Ok(merged_query)
    }

    pub fn get_collect_operations(
        &self,
        dest_root_path: &PathBuf,
    ) -> Result<Vec<CollectOperation>, crate::Error> {
        let ops: Vec<CollectOperation> = self
            .query(&dest_root_path)?
            .iter()
            .map(|(src_path, dest_path)| CollectOperation::new(src_path, dest_path))
            .collect();

        Ok(ops)
    }
    pub fn metadata_json(&self) -> &str {
        &self.metadata_json
    }
    pub fn get_src_path(&self, url: &PathBuf) -> Result<Option<PathBuf>, crate::Error> {
        Ok(self
            .url_src_path_map
            .get(url)
            .and_then(|src_path| Some(src_path.clone())))
    }

    pub fn create_bundle_metadata_file(&self, dest: &PathBuf) -> Result<(), crate::Error> {
        std::fs::create_dir_all(dest)?;
        std::fs::write(dest.join("bundle_metadata.json"), self.metadata_json())
            .map_err(|error| format!("could not create bundle_metadata.json: {}", error).into())
    }
}

fn parse_bundle(root_path: &PathBuf) -> Result<Bundle, crate::Error> {
    let bundle_manifest_path = root_path.join(".namuibundle");
    let bundle_manifest_string = match bundle_manifest_path.exists() {
        true => std::fs::read(bundle_manifest_path)
            .map_err(|error| format!("namui_bundle read error: {}", error))
            .and_then(|file| {
                String::from_utf8(file)
                    .map_err(|error| format!("parse namui_bundle fail: {}", error))
            })?,
        false => String::new(),
    };

    let parse_result = Lexer::new(Tokenizer::new(bundle_manifest_string)).parse()?;
    Ok(Bundle {
        include: parse_result.include,
        exclude: parse_result.exclude,
    })
}

fn convert_src_dest_path_map_to_url_src_path_map(
    src_dest_path_map: HashMap<PathBuf, PathBuf>,
) -> HashMap<PathBuf, PathBuf> {
    let url_src_path_map: HashMap<PathBuf, PathBuf> = src_dest_path_map
        .into_iter()
        .filter_map(|(src_path, dest_dir_path)| {
            let src_file_name = src_path.file_name();
            match src_file_name {
                Some(src_file_name) => {
                    let url_path = dest_dir_path.join(src_file_name);
                    Some((url_path, src_path))
                }
                None => None,
            }
        })
        .collect();
    url_src_path_map
}

impl Bundle {
    fn query(
        &self,
        src_root_path: &PathBuf,
        dest_root_path: &PathBuf,
    ) -> Result<HashMap<PathBuf, PathBuf>, crate::Error> {
        let mut src_dest_path_map = HashMap::new();

        for include_operation in self.include.iter() {
            let target_dest_path =
                include_operation.join_dest_path_under_dest_root_path(&dest_root_path)?;
            include_operation.visit(
                &src_root_path,
                &target_dest_path,
                0,
                false,
                &mut |src_path, dest_path| {
                    src_dest_path_map.insert(src_path, dest_path);
                },
            )?;
        }

        for exclude_operation in self.exclude.iter() {
            exclude_operation.visit(&src_root_path, 0, &mut |src_path| {
                src_dest_path_map.remove(&src_path);
            })?;
        }

        Ok(src_dest_path_map)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::HashMap, path::PathBuf};

    #[test]
    fn test_convert_src_dest_path_map_to_url_src_path_map() {
        let mut src_dest_path_map = HashMap::new();
        src_dest_path_map.insert(
            PathBuf::from("/path/to/one.ext"),
            PathBuf::from("first_dir"),
        );
        src_dest_path_map.insert(
            PathBuf::from("/path/to/two.ext"),
            PathBuf::from("seconde_dir"),
        );
        src_dest_path_map.insert(
            PathBuf::from("/path/to/three.ext"),
            PathBuf::from("seconde_dir"),
        );
        let url_src_path_map = convert_src_dest_path_map_to_url_src_path_map(src_dest_path_map);

        assert_eq!(
            url_src_path_map
                .get(&PathBuf::from("first_dir/one.ext"))
                .unwrap(),
            &PathBuf::from("/path/to/one.ext")
        );
        assert_eq!(
            url_src_path_map
                .get(&PathBuf::from("seconde_dir/two.ext"))
                .unwrap(),
            &PathBuf::from("/path/to/two.ext")
        );
        assert_eq!(
            url_src_path_map
                .get(&PathBuf::from("seconde_dir/three.ext"))
                .unwrap(),
            &PathBuf::from("/path/to/three.ext")
        );
    }
}
