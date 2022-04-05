use super::{ExcludeOperation, IncludeOperation};
use std::{collections::HashMap, path::PathBuf};

type SrcDestPathMap = HashMap<PathBuf, PathBuf>;

#[derive(PartialEq, Eq, Debug)]
pub struct NamuiBundleList {
    include: Vec<IncludeOperation>,
    exclude: Vec<ExcludeOperation>,
}
impl NamuiBundleList {
    pub fn new(include: Vec<IncludeOperation>, exclude: Vec<ExcludeOperation>) -> Self {
        Self { include, exclude }
    }

    pub fn flatten(
        self: &Self,
        src_root_path: &PathBuf,
        dest_root_path: &PathBuf,
    ) -> Result<SrcDestPathMap, String> {
        let mut src_dest_path_map = HashMap::new();

        for include_operation in &self.include {
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

        for exclude_operation in &self.exclude {
            exclude_operation.visit(src_root_path, 0, &mut |src_path| {
                src_dest_path_map.remove(&src_path);
            })?;
        }

        Ok(src_dest_path_map)
    }
}
