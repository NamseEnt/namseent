use super::{DB_FILENAME, DB_S3_KEY, DB_WAL_FILENAME};
use aws_sdk_s3::primitives::ByteStream;
use rusqlite::Connection;
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};

const BACKUP_DIR: &str = "backup";
fn backup_dir_path() -> PathBuf {
    PathBuf::from(BACKUP_DIR)
}

pub async fn backup<'a>(
    write: Arc<Mutex<Connection>>,
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> anyhow::Result<()> {
    let now = std::time::SystemTime::now();

    {
        let mut write = write.lock().unwrap();
        let _trx = write.transaction()?;
        copy_db_and_wal()?;
    }

    save_db_backup_to_s3(s3_client, bucket_name).await?;

    println!("Sqlite Backup {}ms", now.elapsed().unwrap().as_millis());

    Ok(())
}

pub async fn try_fetch_db_file_from_s3(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> anyhow::Result<()> {
    if std::fs::metadata(DB_FILENAME).is_ok() {
        return Ok(());
    }

    let result = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(DB_S3_KEY)
        .send()
        .await;
    let object = match result {
        Ok(object) => object,
        Err(err) => match err.as_service_error() {
            Some(aws_sdk_s3::operation::get_object::GetObjectError::NoSuchKey(_)) => {
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    let vec = object.body.collect().await?.to_vec();
    decompress_backup(&vec)?;

    Ok(())
}

fn copy_db_and_wal() -> anyhow::Result<()> {
    let backup_dir_path = backup_dir_path();
    if backup_dir_path.exists() {
        std::fs::remove_dir_all(BACKUP_DIR)?;
    }

    std::fs::create_dir_all(BACKUP_DIR)?;

    let backup_db_path = backup_dir_path.join(DB_FILENAME);
    let backup_wal_path = backup_dir_path.join(DB_WAL_FILENAME);

    let wal_path = Path::new(DB_WAL_FILENAME);

    if is_support_reflink(DB_FILENAME) {
        cp_reflink_always(DB_FILENAME, backup_db_path)?;
        if wal_path.exists() {
            cp_reflink_always(wal_path, backup_wal_path)?;
        }
    } else {
        std::fs::copy(DB_FILENAME, backup_db_path)?;
        if wal_path.exists() {
            std::fs::copy(wal_path, backup_wal_path)?;
        }
    }

    Ok(())
}

fn compress_backup() -> anyhow::Result<Vec<u8>> {
    Ok(lz4_flex::compress_prepend_size(&tar_backup()?))
}

fn tar_backup() -> anyhow::Result<Vec<u8>> {
    let mut tar_builder = tar::Builder::new(vec![]);

    let backup_dir_path = backup_dir_path();
    let backup_db_path = backup_dir_path.join(DB_FILENAME);
    let backup_wal_path = backup_dir_path.join(DB_WAL_FILENAME);

    tar_builder.append_file(DB_FILENAME, &mut File::open(backup_db_path)?)?;
    tar_builder.append_file(DB_WAL_FILENAME, &mut File::open(backup_wal_path)?)?;

    Ok(tar_builder.into_inner()?)
}

fn decompress_backup(bytes: &[u8]) -> anyhow::Result<()> {
    let tar = lz4_flex::decompress_size_prepended(bytes)?;

    let mut archive = tar::Archive::new(std::io::Cursor::new(tar));
    archive.unpack(".")?;

    Ok(())
}

async fn save_db_backup_to_s3(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> anyhow::Result<()> {
    let compressed = compress_backup()?;

    // TODO: multipart
    s3_client
        .put_object()
        .bucket(bucket_name)
        .key(DB_S3_KEY)
        .body(ByteStream::from(compressed))
        .send()
        .await?;

    Ok(())
}

fn get_filesystem_type(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "df -T {} | awk 'NR==2 {{print $2}}'",
            path.as_ref().display()
        ))
        .output()?;

    if output.status.success() {
        let filesystem = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(filesystem)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get filesystem type",
        ))
    }
}

fn cp_reflink_always(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> anyhow::Result<()> {
    let output = Command::new("cp")
        .arg("--reflink=always")
        .arg(src.as_ref())
        .arg(dst.as_ref())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to copy file with reflink: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn is_support_reflink(path: impl AsRef<Path>) -> bool {
    static IS_SUPPORT_REFLINK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *IS_SUPPORT_REFLINK
        .get_or_init(|| matches!(get_filesystem_type(path).unwrap().as_str(), "btrfs" | "xfs"))
}
