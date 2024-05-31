use super::bundle::NamuiBundleManifest;
use super::drawer_watch_build_service;
use crate::*;
use crate::{cli::Target, debug_println, util::get_cli_root_path};
use std::path::Path;
use std::{
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
};

pub fn collect_all(
    project_path: &Path,
    dest_path: &PathBuf,
    target: Target,
    bundle_manifest: NamuiBundleManifest,
    additional_runtime_path: Option<&PathBuf>,
    release: bool,
) -> Result<()> {
    let mut ops: Vec<CollectOperation> = vec![];
    collect_runtime(&mut ops, additional_runtime_path, target)?;
    collect_rust_build(&mut ops, project_path, target, release)?;
    collect_bundle(&mut ops, &bundle_manifest)?;
    collect_deep_link_manifest(&mut ops, project_path, target)?;

    collect_resources(project_path, dest_path, ops)?;

    bundle_manifest.create_bundle_metadata_file(dest_path)?;
    Ok(())
}

fn collect_resources(
    project_root_path: &Path,
    dest_path: &PathBuf,
    ops: Vec<CollectOperation>,
) -> Result<()> {
    println!("start collecting resources");
    remove_dir(dest_path)?;
    ensure_dir(dest_path)?;
    for op in ops {
        op.execute(project_root_path, dest_path)?
    }
    Ok(())
}

fn collect_runtime(
    ops: &mut Vec<CollectOperation>,
    additional_runtime_path: Option<&PathBuf>,
    target: Target,
) -> Result<()> {
    match target {
        Target::Wasm32WasiWeb => {
            let namui_browser_runtime_path = get_cli_root_path().join("www");
            ops.push(CollectOperation::new(
                &namui_browser_runtime_path,
                &PathBuf::from(""),
            ));
        }
        Target::X86_64PcWindowsMsvc => {}
    }
    if let Some(additional_runtime_path) = additional_runtime_path {
        ops.push(CollectOperation::new(
            additional_runtime_path,
            &PathBuf::from(""),
        ));
    }
    Ok(())
}

fn collect_rust_build(
    ops: &mut Vec<CollectOperation>,
    project_path: &Path,
    target: Target,
    release: bool,
) -> Result<()> {
    match target {
        Target::Wasm32WasiWeb => {
            let build_dist_path = project_path.join("pkg");
            ops.push(CollectOperation::new(
                &build_dist_path.join("bundle.js"),
                &PathBuf::from(""),
            ));
            ops.push(CollectOperation::new(
                &build_dist_path.join("bundle_bg.wasm"),
                &PathBuf::from(""),
            ));

            let drawer_dist_path = drawer_watch_build_service::project_root_path().join("pkg");
            ops.push(CollectOperation::new(
                &drawer_dist_path.join("drawer/bundle.js"),
                &PathBuf::from("drawer"),
            ));
            ops.push(CollectOperation::new(
                &drawer_dist_path.join("drawer/bundle_bg.wasm"),
                &PathBuf::from("drawer"),
            ));
        }
        Target::X86_64PcWindowsMsvc => {
            let build_dist_path = project_path.join(format!(
                "target/namui/target/x86_64-pc-windows-msvc/{}",
                if release { "release" } else { "debug" }
            ));
            ops.push(CollectOperation::new(
                &build_dist_path.join("namui-runtime-x86_64-pc-windows-msvc.exe"),
                &PathBuf::from(""),
            ));
            ops.push(CollectOperation::new(
                &build_dist_path.join("namui_runtime_x86_64_pc_windows_msvc.pdb"),
                &PathBuf::from(""),
            ));
        }
    }
    Ok(())
}

fn collect_bundle(
    ops: &mut Vec<CollectOperation>,
    bundle_manifest: &NamuiBundleManifest,
) -> Result<()> {
    let mut bundle_ops = bundle_manifest.get_collect_operations(&PathBuf::from("bundle"))?;
    ops.append(&mut bundle_ops);
    Ok(())
}

fn collect_deep_link_manifest(
    ops: &mut Vec<CollectOperation>,
    project_path: &Path,
    target: Target,
) -> Result<()> {
    let _ = ops;
    match target {
        Target::Wasm32WasiWeb => {}
        Target::X86_64PcWindowsMsvc => {
            // TODO, but not priority
        }
    }
    Ok(())
}

pub struct CollectOperation {
    src_path: PathBuf,
    dest_path: PathBuf,
}

impl CollectOperation {
    pub fn new(src_path: &Path, dest_path: &Path) -> Self {
        Self {
            src_path: src_path.to_path_buf(),
            dest_path: dest_path.to_path_buf(),
        }
    }

    fn execute(&self, project_root_path: &Path, release_path: &Path) -> Result<()> {
        let src_path = project_root_path.join(&self.src_path);
        let dest_path = release_path.join(&self.dest_path);
        copy_resource(&src_path, &dest_path)
    }
}

fn copy_resource(from: &PathBuf, to: &PathBuf) -> Result<()> {
    debug_println!("resource_collect_service: copy {:?} -> {:?}", &from, &to);
    ensure_dir(to)?;
    match from.is_dir() {
        true => copy_dir(from, to),
        false => copy_file(from, to),
    }
}

fn copy_file(from: &PathBuf, to: &Path) -> Result<()> {
    const COPY_OPTION: fs_extra::file::CopyOptions = fs_extra::file::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
    };
    let src_file_name = &from.file_name().unwrap();
    let dest_path_with_file_name = &to.join(src_file_name);
    fs_extra::file::copy(from, dest_path_with_file_name, &COPY_OPTION).map_err(|error| {
        anyhow!(
            "resource_collect_service: copy file {:?} -> {:?}\n\t{}",
            from,
            dest_path_with_file_name,
            error
        )
    })?;
    Ok(())
}

fn copy_dir(from: &PathBuf, to: &PathBuf) -> Result<()> {
    const COPY_OPTION: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        copy_inside: true,
        content_only: true,
        buffer_size: 64000,
        depth: 0,
    };
    fs_extra::dir::copy(from, to, &COPY_OPTION).map_err(|error| {
        anyhow!(
            "resource_collect_service: copy dir {:?} -> {:?}\n\t{}",
            &from,
            &to,
            error
        )
    })?;
    Ok(())
}

fn remove_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    remove_dir_all(path)
        .map_err(|error| anyhow!("resource_collect_service: remove dir failed\n\t{}", error))
}

fn ensure_dir(path: &PathBuf) -> Result<()> {
    create_dir_all(path)
        .map_err(|error| anyhow!("resource_collect_service: ensure dir failed\n\t{}", error))
}
