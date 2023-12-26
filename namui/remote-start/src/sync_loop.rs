use crate::State;
use anyhow::Result;
use flate2::read::GzDecoder;
use rayon::prelude::*;
use std::{
    io::Read,
    path::PathBuf,
    process::{Child, Command},
    sync::{Arc, RwLock},
    time::Duration,
};
use walkdir::WalkDir;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct HeadFile {
    path: PathBuf,
    crc32_hash: u32,
}

const RUNNING_DIR: &str = "./running";

pub(crate) fn sync_loop(state: Arc<RwLock<State>>) -> Result<()> {
    let mut child_process = None;
    loop {
        let client = reqwest::blocking::Client::new();

        let uri = { state.read().unwrap().uri.clone() };
        let head_files = client
            .execute(
                client
                    .post(format!("{uri}/get_head_files"))
                    .timeout(Duration::from_secs(3))
                    .build()?,
            )?
            .json::<Vec<HeadFile>>()?;

        let removed_files = get_removed_files(&head_files)?;
        let updated_files = get_updated_files(&head_files)?;

        let no_update = removed_files.is_empty() && updated_files.is_empty();

        if no_update {
            std::thread::sleep(Duration::from_secs(1));
            continue;
        }

        if let Some(child_process) = child_process.take() {
            stop_running_process(child_process)?;
        }

        remove_files(removed_files)?;

        let response = client.execute(
            client
                .post(format!("{uri}/get_files_archive"))
                .body(serde_json::to_string(&updated_files)?)
                .timeout(Duration::from_secs(10))
                .build()?,
        )?;

        let gz_decoder = GzDecoder::new(response);
        let mut tar_archive = tar::Archive::new(gz_decoder);
        tar_archive.unpack(RUNNING_DIR)?;

        child_process = Some(start_running_process()?);
    }
}

const PROCESS_NAME: &str = "namui-runtime-x86_64-pc-windows-msvc.exe";

fn start_running_process() -> Result<Child> {
    let process = Command::new(PROCESS_NAME)
        .current_dir(RUNNING_DIR)
        .spawn()?;
    Ok(process)
}

fn stop_running_process(mut process: Child) -> Result<()> {
    process.kill()?;
    Ok(())
}

fn remove_files(removed_files: Vec<PathBuf>) -> Result<()> {
    for path in removed_files {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn get_updated_files(head_files: &[HeadFile]) -> Result<Vec<HeadFile>> {
    let running_dir = PathBuf::from(RUNNING_DIR);

    let output = head_files
        .par_iter()
        .map(|head_file| {
            let path = running_dir.join(&head_file.path);
            let crc32_hash = calculate_crc_32(path)?;
            if crc32_hash != head_file.crc32_hash {
                return Ok(Some(head_file.clone()));
            }
            Ok(None)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(output)
}

fn calculate_crc_32(path: PathBuf) -> Result<u32> {
    let crc = crc::Crc::<u32>::new(&crc::algorithm::CRC_32_CKSUM);
    let mut digest = crc.digest();

    let mut file = std::fs::File::open(path)?;
    let mut buffer = [0u8; 64 * 1024];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }

    Ok(digest.finalize())
}

fn get_removed_files(head_files: &[HeadFile]) -> Result<Vec<PathBuf>> {
    Ok(WalkDir::new(RUNNING_DIR)
        .into_iter()
        .map(|entry| {
            let entry = entry?;
            if entry.file_type().is_dir() {
                return Ok(None);
            }
            let path = entry.path().strip_prefix(RUNNING_DIR)?.to_owned();
            if !head_files.iter().any(|f| f.path == path) {
                return Ok(Some(path));
            }
            Ok(None)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}
