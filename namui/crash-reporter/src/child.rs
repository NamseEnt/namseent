use crate::{Config, Error, context, dump_summary, log_capture, namsh, queue};
use minidumper::{LoopAction, MinidumpBinary, Server, ServerHandler, SocketName};
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::atomic::AtomicBool,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn server_main(socket_name: &str, config: &Config) -> Result<(), Error> {
    let queue_dir = queue::queue_dir(&config.app_name)?;
    std::fs::create_dir_all(&queue_dir)?;

    let install_id = std::env::var("NAMUI_CRASH_INSTALL_ID")
        .ok()
        .or_else(|| crate::install_id::get_or_create(&config.app_name).ok())
        .unwrap_or_default();

    let parent_start_unix = std::env::var("NAMUI_CRASH_PARENT_START_UNIX")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let handler = Handler {
        queue_dir,
        config: config.clone(),
        install_id,
        parent_start_unix,
    };

    let socket_path: PathBuf = socket_name.into();
    let mut server = Server::with_name(SocketName::Path(&socket_path))?;
    let shutdown = AtomicBool::new(false);
    server.run(
        Box::new(handler),
        &shutdown,
        Some(Duration::from_secs(60 * 60 * 24)),
    )?;
    Ok(())
}

struct Handler {
    queue_dir: PathBuf,
    config: Config,
    install_id: String,
    parent_start_unix: u64,
}

impl ServerHandler for Handler {
    fn create_minidump_file(&self) -> Result<(File, PathBuf), std::io::Error> {
        let dump_id = uuid::Uuid::new_v4().to_string();
        let path = self.queue_dir.join(format!("{dump_id}.dmp"));
        let file = File::create(&path)?;
        Ok((file, path))
    }

    fn on_minidump_created(
        &self,
        result: Result<MinidumpBinary, minidumper::Error>,
    ) -> LoopAction {
        match result {
            Ok(binary) => {
                if let Err(e) = self.process_dump(&binary.path) {
                    eprintln!("[crash-reporter:child] process_dump error: {e}");
                }
            }
            Err(e) => eprintln!("[crash-reporter:child] minidump capture failed: {e:?}"),
        }
        LoopAction::Exit
    }

    fn on_message(&self, _kind: u32, _buffer: Vec<u8>) {}

    fn on_client_disconnected(&self, num_clients: usize) -> LoopAction {
        if num_clients == 0 {
            LoopAction::Exit
        } else {
            LoopAction::Continue
        }
    }
}

impl Handler {
    fn process_dump(&self, dump_path: &Path) -> Result<(), Error> {
        let summary = dump_summary::parse(dump_path)?;
        let session_uptime_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs().saturating_sub(self.parent_start_unix))
            .unwrap_or(0);
        let gpu_adapter = crate::gpu_info::read_from_env();
        let log_tail = log_capture::read_tail();
        let ctx = context::collect(context::CollectArgs {
            config: &self.config,
            install_id: &self.install_id,
            session_uptime_sec,
            error_message: summary.error_message,
            gpu_adapter,
            gpu_driver: None,
            log_tail,
        });
        queue::write_sidecar(
            dump_path,
            &queue::PendingEntry {
                stack_hash: summary.stack_hash,
                context: ctx,
            },
        )?;
        if let Err(e) = namsh::upload_single(&self.config, dump_path) {
            eprintln!("[crash-reporter:child] immediate upload failed (queued): {e}");
        }
        Ok(())
    }
}
