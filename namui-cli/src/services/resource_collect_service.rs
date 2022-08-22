use crate::debug_println;
use std::{
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
};

pub struct ResourceCollectService {
    project_root_path: PathBuf,
    release_path: PathBuf,
}

impl ResourceCollectService {
    pub fn new(project_root_path: &PathBuf, release_path: &PathBuf) -> Self {
        Self {
            project_root_path: project_root_path.clone(),
            release_path: release_path.clone(),
        }
    }

    pub fn collect_resources(self: &Self, ops: Vec<CollectOperation>) -> Result<(), String> {
        println!("start collecting resources");
        remove_dir(&self.release_path)?;
        ensure_dir(&self.release_path)?;
        for op in ops {
            op.execute(&self.project_root_path, &self.release_path)?
        }
        Ok(())
    }
}

pub struct CollectOperation {
    src_path: PathBuf,
    dest_path: PathBuf,
}

impl CollectOperation {
    pub fn new(src_path: &PathBuf, dest_path: &PathBuf) -> Self {
        Self {
            src_path: src_path.clone(),
            dest_path: dest_path.clone(),
        }
    }

    fn execute(
        self: &Self,
        project_root_path: &PathBuf,
        release_path: &PathBuf,
    ) -> Result<(), String> {
        let src_path = project_root_path.join(&self.src_path);
        let dest_path = release_path.join(&self.dest_path);
        copy_resource(&src_path, &dest_path)
    }
}

fn copy_resource(from: &PathBuf, to: &PathBuf) -> Result<(), String> {
    debug_println!("resource_collect_service: copy {:?} -> {:?}", &from, &to);
    ensure_dir(&to)?;
    match from.is_dir() {
        true => copy_dir(from, to),
        false => copy_file(from, to),
    }
}

fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), String> {
    const COPY_OPTION: fs_extra::file::CopyOptions = fs_extra::file::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
    };
    let src_file_name = &from.file_name().unwrap();
    let dest_path_with_file_name = &to.join(src_file_name);
    fs_extra::file::copy(from, dest_path_with_file_name, &COPY_OPTION).map_err(|error| {
        format!(
            "resource_collect_service: copy file {:?} -> {:?}\n\t{}",
            from, dest_path_with_file_name, error
        )
    })?;
    Ok(())
}

fn copy_dir(from: &PathBuf, to: &PathBuf) -> Result<(), String> {
    const COPY_OPTION: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        copy_inside: true,
        content_only: true,
        buffer_size: 64000,
        depth: 0,
    };
    fs_extra::dir::copy(from, to, &COPY_OPTION).map_err(|error| {
        format!(
            "resource_collect_service: copy dir {:?} -> {:?}\n\t{}",
            &from, &to, error
        )
    })?;
    Ok(())
}

fn remove_dir(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    remove_dir_all(path)
        .map_err(|error| format!("resource_collect_service: remove dir failed\n\t{}", error))
}

fn ensure_dir(path: &PathBuf) -> Result<(), String> {
    create_dir_all(path)
        .map_err(|error| format!("resource_collect_service: ensure dir failed\n\t{}", error))
}
