use crate::fs::types::{Dirent, DirentKind};
use dashmap::DashMap;
use std::path::PathBuf;

pub fn make_path_dirent_list_map(bundle_metadata: &Vec<PathBuf>) -> DashMap<PathBuf, Vec<Dirent>> {
    let path_dirent_list_map: DashMap<PathBuf, DashMap<PathBuf, Dirent>> = DashMap::new();
    for path in bundle_metadata {
        create_all_dirent_in_path(&path_dirent_list_map, path);
    }
    path_dirent_list_map
        .into_iter()
        .map(|(path, dirent_map)| {
            (
                path,
                dirent_map.into_iter().map(|(_, dirent)| dirent).collect(),
            )
        })
        .collect()
}

fn create_all_dirent_in_path(
    path_dirent_list_map: &DashMap<PathBuf, DashMap<PathBuf, Dirent>>,
    path: &PathBuf,
) {
    let component_names =
        path.components()
            .fold(Vec::new(), |mut acc, component| match component {
                std::path::Component::ParentDir => {
                    acc.pop();
                    acc
                }
                std::path::Component::Normal(name) => {
                    acc.push(name.to_string_lossy().to_string());
                    acc
                }
                _ => acc,
            });
    let last_component_index = component_names.len() - 1;
    let mut last_path = PathBuf::new();

    for (index, name) in component_names.into_iter().enumerate() {
        let kind = match index == last_component_index {
            true => DirentKind::File,
            false => DirentKind::Directory,
        };
        push_dirent(path_dirent_list_map, &last_path, name.as_str(), kind);
        last_path.push(name);
    }
}

fn push_dirent(
    path_dirent_list_map: &DashMap<PathBuf, DashMap<PathBuf, Dirent>>,
    dir_path: &PathBuf,
    name: &str,
    kind: DirentKind,
) {
    ensure_dir(path_dirent_list_map, dir_path);
    let pre_dirent_set = path_dirent_list_map.get_mut(dir_path).unwrap();
    let path = dir_path.join(name);
    pre_dirent_set.insert(path.clone(), Dirent::new(path, kind));
}

fn ensure_dir(path_dirent_list_map: &DashMap<PathBuf, DashMap<PathBuf, Dirent>>, path: &PathBuf) {
    if path_dirent_list_map.get(path).is_none() {
        path_dirent_list_map.insert(path.clone(), DashMap::new());
    }
}

#[cfg(test)]
mod tests {
    use super::make_path_dirent_list_map;
    use std::path::PathBuf;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn create_structure() {
        let bundle_metadata = vec![
            PathBuf::from("three/two/one/file1"),
            PathBuf::from("three/two/file2"),
            PathBuf::from("three/file3"),
            PathBuf::from("three/file4"),
        ];
        let path_dirent_list_map = make_path_dirent_list_map(&bundle_metadata);
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("three"))
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("three/two"))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("three/two/one"))
                .unwrap()
                .len(),
            1
        );

        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("three/two/one"))
                .unwrap()
                .first()
                .unwrap()
                .is_file(),
            true
        );
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("three/two/one"))
                .unwrap()
                .first()
                .unwrap()
                .path(),
            &PathBuf::from("three/two/one/file1")
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn ignore_root_dir() {
        let bundle_metadata = vec![
            PathBuf::from("/with/root/dir"),
            PathBuf::from("with/out/root/dir"),
        ];
        let path_dirent_list_map = make_path_dirent_list_map(&bundle_metadata);
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("with"))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn overwrite_duplicated_dirent() {
        let bundle_metadata = vec![PathBuf::from("path/to"), PathBuf::from("path/to/some")];
        let path_dirent_list_map = make_path_dirent_list_map(&bundle_metadata);
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("path"))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            path_dirent_list_map
                .get(&PathBuf::from("path"))
                .unwrap()
                .first()
                .unwrap()
                .is_dir(),
            true
        );
    }
}
