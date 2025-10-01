use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::system::InitResult;
use anyhow::anyhow;
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::env;
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
        // https://www.sqlite.org/uri.html
        let mut file_path = std::env::current_exe()?
            .parent()
            .ok_or_else(|| io::Error::other(anyhow!("No parent")))?
            .join("bundle.sqlite")
            .to_str()
            .unwrap()
            .to_string();

        file_path = file_path.replace('?', "%3f");
        file_path = file_path.replace('#', "%23");
        if env::consts::OS == "windows" {
            file_path = file_path.replace('\\', "/");
        }
        file_path = prune_slashes(&file_path);
        if env::consts::OS == "windows" {
            append_slash_if_starts_with_drive_letter(&mut file_path);
        }
        file_path.insert_str(0, "file:");
        file_path.push_str("?immutable=1");

        Ok(PathBuf::from(file_path))
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
            .map_err(io::Error::other)?
            .ok_or_else(|| io::Error::new(ErrorKind::NotFound, anyhow!("Not found")))
        })
    })
    .await
    .map_err(io::Error::other)?
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(Error::other)
}

pub fn read_dir(_path: impl PathLike) -> io::Result<Vec<Dirent>> {
    todo!()
}

fn prune_slashes(path: &str) -> String {
    let mut result = String::new();
    let mut iter = path.chars().peekable();
    while let Some(c) = iter.next() {
        if c == '/' {
            result.push(c);
            while let Some('/') = iter.peek() {
                iter.next();
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn append_slash_if_starts_with_drive_letter(path: &mut String) {
    if path.len() >= 2 && path.chars().nth(1) == Some(':') {
        path.insert(0, '/');
    }
}
