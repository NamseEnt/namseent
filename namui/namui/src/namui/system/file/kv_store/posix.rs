use crate::system::InitResult;
use anyhow::{anyhow, Result};
use rusqlite::{Connection, OptionalExtension};
use std::path::PathBuf;

pub async fn init() -> InitResult {
    sqlite(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS local_storage (key TEXT PRIMARY KEY, value BLOB)",
            [],
        )?;
        anyhow::Ok(())
    })?;
    Ok(())
}

pub fn get(key: impl AsRef<str>) -> Result<Option<Vec<u8>>> {
    let key = key.as_ref();

    sqlite(|conn| {
        Ok(conn
            .query_row(
                "SELECT value FROM local_storage WHERE key = ?",
                [key],
                |row| row.get::<usize, Vec<u8>>(0),
            )
            .optional()?)
    })
}

pub fn set(key: impl AsRef<str>, content: &[u8]) -> Result<()> {
    let key = key.as_ref();
    sqlite(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO local_storage (key, value) VALUES (?, ?)",
            (key, content),
        )?;
        Ok(())
    })
}

pub fn delete(key: impl AsRef<str>) -> Result<()> {
    let key = key.as_ref();
    sqlite(|conn| {
        conn.execute("DELETE FROM local_storage WHERE key = ?", [key])?;
        Ok(())
    })
}

fn sqlite<T>(func: impl FnOnce(&Connection) -> T) -> T {
    thread_local! {
        static SQLITE: Connection = Connection::open(local_storage_sqlite_path().unwrap()).unwrap();
    }

    SQLITE.with(|sqlite| func(sqlite))
}

fn local_storage_sqlite_path() -> Result<PathBuf> {
    Ok(std::env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow!("No parent of current_exe"))?
        .join("kv_store.sqlite"))
}
