use super::{
    rust_build_service::{BuildOption, BuildResult, RustBuildService},
    rust_project_watch_service::RustProjectWatchService,
    wasm_bundle_web_server::WasmBundleWebServer,
};
use crate::{cli::Target, util::print_build_result};
use crate::{util::get_cli_root_path, *};
use std::{path::PathBuf, sync::Arc};
use tokio::try_join;

pub struct DrawerBuildService {}

type AfterBuild = Option<Arc<dyn Fn() + Send + Sync + 'static>>;

pub struct WatchArgs {
    pub target: Target,
    pub after_build: AfterBuild,
    pub wasm_bundle_web_server: Arc<WasmBundleWebServer>,
}
impl DrawerBuildService {
    pub async fn spawn_watch(args: WatchArgs) -> Result<()> {
        let project_root_path = get_cli_root_path().join("../namui-drawer");
        let build_dist_path = project_root_path.join("pkg/drawer");

        args.wasm_bundle_web_server
            .add_static_dir("drawer/", &build_dist_path);

        let rust_project_watch_service = Arc::new(RustProjectWatchService::new());
        let rust_build_service = Arc::new(RustBuildService::new());

        try_join!(
            cancel_and_start_build(
                rust_build_service.clone(),
                build_dist_path.clone(),
                project_root_path.clone(),
                args.target,
                args.after_build.clone(),
            ),
            rust_project_watch_service.watch(project_root_path.join("Cargo.toml"), {
                let rust_build_service = rust_build_service.clone();
                let build_dist_path = build_dist_path.clone();
                let project_root_path = project_root_path.clone();
                let after_build = args.after_build.clone();
                move || {
                    tokio::spawn(cancel_and_start_build(
                        rust_build_service.clone(),
                        build_dist_path.clone(),
                        project_root_path.clone(),
                        args.target,
                        after_build.clone(),
                    ));
                }
            }),
        )?;

        return Ok(());

        pub async fn cancel_and_start_build(
            rust_build_service: Arc<RustBuildService>,
            build_dist_path: PathBuf,
            project_root_path: PathBuf,
            target: Target,
            after_build: AfterBuild,
        ) -> Result<()> {
            debug_println!("build fn run");
            match rust_build_service.cancel_and_start_build(&BuildOption {
                target,
                dist_path: build_dist_path,
                project_root_path: project_root_path.clone(),
                watch: true,
            }) {
                BuildResult::Canceled => {
                    debug_println!("build canceled");
                }
                BuildResult::Successful(cargo_build_result) => {
                    print_build_result(&cargo_build_result.error_messages, &vec![]);
                    after_build.as_ref().map(|f| f());
                }
                BuildResult::Failed(err) => {
                    eprintln!("failed to build: {}", err);
                    bail!("failed to build: {}", err);
                }
            }

            Ok(())
        }
    }

    // pub fn just_build(project_root_path: PathBuf, target: Target) -> Result<()> {
    //     let build_dist_path = project_root_path.join("pkg");
    //     let runtime_target_dir = project_root_path.join("target/namui");
    //     let rust_build_service = RustBuildService::new();

    //     runtime_project::wasm::generate_runtime_project(GenerateRuntimeProjectArgs {
    //         target_dir: runtime_target_dir.clone(),
    //         project_path: project_root_path.clone(),
    //     })?;

    //     match rust_build_service.cancel_and_start_build(&BuildOption {
    //         target,
    //         dist_path: build_dist_path,
    //         project_root_path: runtime_target_dir,
    //         watch: false,
    //     }) {
    //         BuildResult::Successful(cargo_build_result) => {
    //             print_build_result(&cargo_build_result.error_messages, &vec![]);
    //             Ok(())
    //         }
    //         BuildResult::Canceled => unreachable!(),
    //         BuildResult::Failed(error) => Err(error.into()),
    //     }
    // }
}
