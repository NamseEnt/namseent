use crate::util::get_namui_bundle_manifest;
use std::{collections::HashMap, fs, path::PathBuf, sync::RwLock};

pub struct BundleMetadataService {
    bundle_metadata_string: RwLock<String>,
    url_src_path_map: RwLock<HashMap<PathBuf, PathBuf>>,
}

impl BundleMetadataService {
    pub fn new() -> Self {
        BundleMetadataService {
            bundle_metadata_string: RwLock::new(String::new()),
            url_src_path_map: RwLock::new(HashMap::new()),
        }
    }

    pub fn load_bundle_manifest(&self, project_root_path: &PathBuf) -> Result<(), String> {
        let namui_bundle_manifest = get_namui_bundle_manifest(project_root_path)?;
        let src_dest_path_map = namui_bundle_manifest.query(project_root_path, &PathBuf::new())?;
        let url_src_path_map: HashMap<PathBuf, PathBuf> =
            convert_src_dest_path_map_to_url_src_path_map(src_dest_path_map);

        let url_list: Vec<&PathBuf> = url_src_path_map.keys().collect();
        let bundle_metadata_string = serde_json::to_string(&url_list).map_err(|error| {
            format!("serde_json error while creating bundle_metadata: {}", error)
        })?;

        *self
            .url_src_path_map
            .write()
            .map_err(|error| format!("could not update url_src_path_map: {}", error))? =
            url_src_path_map;
        *self
            .bundle_metadata_string
            .write()
            .map_err(|error| format!("could not update url_src_path_map: {}", error))? =
            bundle_metadata_string;
        Ok(())
    }

    pub fn get_src_path(&self, url: &PathBuf) -> Result<Option<PathBuf>, String> {
        Ok(self
            .url_src_path_map
            .read()
            .map_err(|error| format!("could not update url_src_path_map: {}", error))?
            .get(url)
            .and_then(|src_path| Some(src_path.clone())))
    }

    pub fn bundle_metadata(&self) -> Result<String, String> {
        Ok(self
            .bundle_metadata_string
            .read()
            .map_err(|error| format!("could not update url_src_path_map: {}", error))?
            .clone())
    }

    pub fn create_bundle_metadata_file(&self, dest_dir_path: &PathBuf) -> Result<(), String> {
        fs::write(
            dest_dir_path.join("bundle_metadata.json"),
            self.bundle_metadata_string
                .read()
                .map_err(|error| format!("could not update url_src_path_map: {}", error))?
                .clone(),
        )
        .map_err(|error| format!("could not create bundle_metadata.json{}", error))
    }
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

#[cfg(test)]
mod test {
    use crate::services::bundle_metadata_service::convert_src_dest_path_map_to_url_src_path_map;
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
