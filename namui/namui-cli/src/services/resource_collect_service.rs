use super::bundle::NamuiBundleManifest;
use crate::*;
use crate::{cli::NamuiTarget, debug_println, util::get_cli_root_path};
use std::{
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
};

pub fn collect_all(
    project_path: impl AsRef<std::path::Path>,
    dest_path: impl AsRef<std::path::Path>,
    target: NamuiTarget,
    bundle_manifest: NamuiBundleManifest,
    additional_runtime_path: Option<&PathBuf>,
    release: bool,
) -> Result<()> {
    let mut ops: Vec<CollectOperation> = vec![];
    collect_runtime(&mut ops, additional_runtime_path, target)?;
    collect_rust_build(&mut ops, &project_path, target, release)?;
    collect_deep_link_manifest(&mut ops, &project_path, target)?;

    collect_resources(&project_path, &dest_path, ops)?;
    collect_bundle(&bundle_manifest, &dest_path)?;

    bundle_manifest.create_bundle_metadata_file(&dest_path)?;
    Ok(())
}

fn collect_resources(
    project_root_path: impl AsRef<std::path::Path>,
    dest_path: impl AsRef<std::path::Path>,
    ops: Vec<CollectOperation>,
) -> Result<()> {
    let project_root_path = project_root_path.as_ref();
    let dest_path = dest_path.as_ref();
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
    target: NamuiTarget,
) -> Result<()> {
    match target {
        NamuiTarget::Wasm32WasiWeb => {
            let namui_browser_runtime_path = get_cli_root_path().join("www");
            ops.push(CollectOperation::new(
                namui_browser_runtime_path,
                PathBuf::from(""),
            ));
        }
        NamuiTarget::X86_64PcWindowsMsvc => {}
        NamuiTarget::X86_64UnknownLinuxGnu => {}
        NamuiTarget::Aarch64AppleDarwin => {}
    }
    if let Some(additional_runtime_path) = additional_runtime_path {
        ops.push(CollectOperation::new(
            additional_runtime_path,
            PathBuf::from(""),
        ));
    }
    Ok(())
}

fn collect_rust_build(
    ops: &mut Vec<CollectOperation>,
    project_path: impl AsRef<std::path::Path>,
    target: NamuiTarget,
    release: bool,
) -> Result<()> {
    let project_path = project_path.as_ref();
    match target {
        NamuiTarget::Wasm32WasiWeb => {
            let build_dist_path = project_path.join("pkg");
            ops.push(CollectOperation::new(
                build_dist_path.join("bundle.js"),
                PathBuf::from(""),
            ));
            ops.push(CollectOperation::new(
                build_dist_path.join("bundle_bg.wasm"),
                PathBuf::from(""),
            ));
        }
        NamuiTarget::X86_64PcWindowsMsvc => {
            let build_dist_path = project_path.join(format!(
                "target/namui/target/x86_64-pc-windows-msvc/{}",
                if release { "release" } else { "debug" }
            ));
            ops.push(CollectOperation::new(
                build_dist_path.join("namui-runtime-x86_64-pc-windows-msvc.exe"),
                PathBuf::from(""),
            ));
            ops.push(CollectOperation::new(
                build_dist_path.join("namui_runtime_x86_64_pc_windows_msvc.pdb"),
                PathBuf::from(""),
            ));
        }
        NamuiTarget::X86_64UnknownLinuxGnu => {
            let build_dist_path = project_path.join(format!(
                "target/namui/target/x86_64-unknown-linux-gnu/{}",
                if release { "release" } else { "debug" }
            ));
            ops.push(CollectOperation::new(
                build_dist_path.join("namui-runtime-x86_64-unknown-linux-gnu"),
                PathBuf::from(""),
            ));
        }
        NamuiTarget::Aarch64AppleDarwin => {
            let build_dist_path = project_path.join(format!(
                "target/namui/target/aarch64-apple-darwin/{}",
                if release { "release" } else { "debug" }
            ));
            ops.push(CollectOperation::new(
                build_dist_path.join("namui-runtime-aarch64-apple-darwin"),
                PathBuf::from(""),
            ));
        }
    }
    Ok(())
}

fn collect_bundle(
    bundle_manifest: &NamuiBundleManifest,
    dest_path: impl AsRef<std::path::Path>,
) -> Result<()> {
    bundle_manifest.bundle_to_sqlite(dest_path.as_ref().join("bundle.sqlite"))?;

    Ok(())
}

fn collect_deep_link_manifest(
    ops: &mut Vec<CollectOperation>,
    project_path: impl AsRef<std::path::Path>,
    target: NamuiTarget,
) -> Result<()> {
    let _ = ops;
    match target {
        NamuiTarget::Wasm32WasiWeb => {}
        NamuiTarget::X86_64PcWindowsMsvc
        | NamuiTarget::X86_64UnknownLinuxGnu
        | NamuiTarget::Aarch64AppleDarwin => {
            // TODO, but not priority
        }
    }
    Ok(())
}

pub struct CollectOperation {
    pub src_path: PathBuf,
    pub dest_dir_path: PathBuf,
}

impl CollectOperation {
    pub fn new(
        src_path: impl AsRef<std::path::Path>,
        dest_dir_path: impl AsRef<std::path::Path>,
    ) -> Self {
        Self {
            src_path: src_path.as_ref().to_path_buf(),
            dest_dir_path: dest_dir_path.as_ref().to_path_buf(),
        }
    }

    pub fn execute(
        &self,
        project_root_path: impl AsRef<std::path::Path>,
        release_path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let src_path = project_root_path.as_ref().join(&self.src_path);
        let dest_dir_path = release_path.as_ref().join(&self.dest_dir_path);
        copy_resource(src_path, dest_dir_path)
    }

    pub fn dest_path(&self) -> PathBuf {
        self.dest_dir_path.join(self.src_path.file_name().unwrap())
    }
}

fn copy_resource(
    from: impl AsRef<std::path::Path>,
    dest_dir_path: impl AsRef<std::path::Path>,
) -> Result<()> {
    let from = from.as_ref();
    let dest_dir_path = dest_dir_path.as_ref();
    debug_println!(
        "resource_collect_service: copy {:?} -> {:?}/{:?}",
        &from,
        &dest_dir_path,
        &from.file_name().unwrap()
    );
    ensure_dir(dest_dir_path)?;
    assert!(from.is_file());
    copy_file(from, dest_dir_path)
}

fn copy_file(
    from: impl AsRef<std::path::Path>,
    dest_dir_path: impl AsRef<std::path::Path>,
) -> Result<()> {
    let from = from.as_ref();
    let dest_dir_path = dest_dir_path.as_ref();

    const COPY_OPTION: fs_extra::file::CopyOptions = fs_extra::file::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
    };
    let src_file_name = &from.file_name().unwrap();
    let dest_path_with_file_name = &dest_dir_path.join(src_file_name);
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

fn remove_dir(path: impl AsRef<std::path::Path>) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(());
    }
    remove_dir_all(path)
        .map_err(|error| anyhow!("resource_collect_service: remove dir failed\n\t{}", error))
}

fn ensure_dir(path: impl AsRef<std::path::Path>) -> Result<()> {
    create_dir_all(path)
        .map_err(|error| anyhow!("resource_collect_service: ensure dir failed\n\t{}", error))
}
