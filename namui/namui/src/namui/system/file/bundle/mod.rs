use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::system::InitResult;
use anyhow::anyhow;
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::io::{self, Error};

pub async fn init() -> InitResult {
    Ok(())
}

fn sqlite<T>(func: impl FnOnce(&Connection) -> T) -> T {
    thread_local! {
        static SQLITE: Connection = Connection::open_with_flags(
            bundle_sqlite_path().unwrap(), OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    }

    SQLITE.with(|sqlite| func(sqlite))
}

fn bundle_sqlite_path() -> io::Result<PathBuf> {
    if cfg!(target_os = "wasi") {
        Ok(PathBuf::from("file:./bundle.sqlite?immutable=1"))
    } else {
        Ok(std::env::current_exe()?
            .parent()
            .ok_or_else(|| io::Error::new(ErrorKind::Other, anyhow!("No parent")))?
            .join("bundle.sqlite?immutable=1"))
    }
}

pub async fn read(path_like: impl PathLike) -> io::Result<Vec<u8>> {
    let path = path_like.path();

    tokio::task::spawn_blocking(move || {
        sqlite(|conn| {
            conn.query_row(
                "SELECT data FROM bundle WHERE path = ?",
                [path.to_str().unwrap()],
                |row| row.get::<usize, Vec<u8>>(0),
            )
            .optional()
            .map_err(|error| io::Error::new(ErrorKind::Other, error))?
            .ok_or_else(|| io::Error::new(ErrorKind::NotFound, anyhow!("Not found")))
        })
    })
    .await
    .map_err(|error| io::Error::new(ErrorKind::Other, error))?
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(|error| Error::new(ErrorKind::Other, error))
}

pub fn read_dir(_path: impl PathLike) -> io::Result<Vec<Dirent>> {
    todo!()
}
