use super::KvStore;
use crate::*;
use anyhow::Result;
use aws_sdk_s3::primitives::ByteStream;
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::sync::{atomic::AtomicPtr, Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct SqliteKvStore {
    write: Arc<Mutex<Connection>>,
}

const DB_PATH: &str = "db.sqlite";
const BACKUP_PATH: &str = "db.sqlite.backup";

impl SqliteKvStore {
    pub async fn new() -> Result<Self> {
        try_fetch_db_file_from_s3(DB_PATH).await?;

        let conn = Connection::open(DB_PATH).unwrap();

        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "NORMAL").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS
                    kv_store (
                        key TEXT PRIMARY KEY,
                        value BLOB,
                        version INTEGER\
                    )",
            [],
        )
        .unwrap();

        let sqlite3 = AtomicPtr::new(conn.db.borrow_mut().db);
        let write = Arc::new(Mutex::new(conn));

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                if let Err(err) = backup(&sqlite3).await {
                    eprintln!("Failed to backup db: {}", err);
                }
            }
        });

        Ok(Self { write })
    }
    fn read_conn<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        thread_local! {
            static READ: Connection = Connection::open_with_flags(DB_PATH, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
        }
        READ.with(f)
    }
    fn write_conn(&self) -> MutexGuard<Connection> {
        self.write.lock().unwrap()
    }
}

impl KvStore for SqliteKvStore {
    async fn get(&self, key: impl AsRef<str>) -> Result<Option<super::ValueBuffer>> {
        self.read_conn(|read_conn| {
            let mut stmt = read_conn.prepare("SELECT value FROM kv_store WHERE key = ?")?;
            let vec: Option<Vec<_>> = stmt
                .query_row([key.as_ref()], |row| row.get(0))
                .optional()?;

            Ok(vec.map(super::ValueBuffer::Vec))
        })
    }

    async fn put(&self, key: impl AsRef<str>, value: &impl AsRef<[u8]>) -> Result<()> {
        let write_conn = self.write_conn();
        let mut stmt = write_conn
            .prepare("INSERT OR REPLACE INTO kv_store (key, value, version) VALUES (?, ?, 0)")?;
        assert_eq!(stmt.execute((key.as_ref(), value.as_ref()))?, 1);

        Ok(())
    }

    async fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        let write_conn = self.write_conn();
        let mut stmt = write_conn.prepare("DELETE FROM kv_store WHERE key = ?")?;
        stmt.execute([key.as_ref()])?;

        Ok(())
    }

    // async fn update<T, Fut>(
    //     &self,
    //     key: impl AsRef<str>,
    //     update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    // ) -> Result<bool>
    // where
    //     T: rkyv::Archive + rkyv::Serialize<AllocSerializer<64>>,
    //     Fut: Future<Output = Option<Option<T>>>,
    // {
    //     let output: Option<(Vec<_>, usize)> = self.read_conn(|read_conn| {
    //         let mut stmt =
    //             read_conn.prepare("SELECT (value, version) FROM kv_store WHERE key = ?")?;
    //         stmt.query_row([key.as_ref()], |row| Ok((row.get(0)?, row.get(1)?)))
    //             .optional()
    //     })?;

    //     let (old_value, version) = match output {
    //         Some((vec, version)) => (Some(HeapArchived::new(vec)), Some(version)),
    //         None => (None, None),
    //     };

    //     let Some(new_value) = update(old_value).await else {
    //         return Ok(true);
    //     };

    //     let write_conn = self.write_conn();
    //     let Some(new_value) = new_value else {
    //         let mut stmt = write_conn.prepare("DELETE FROM kv_store WHERE key = ?")?;
    //         stmt.execute([key.as_ref()])?;
    //         return Ok(true);
    //     };

    //     let buffer = rkyv::to_bytes(&new_value)?;

    //     if let Some(version) = version {
    //         let mut stmt = write_conn.prepare(
    //             "UPDATE kv_store SET value = ?, version = version + 1 WHERE key = ? AND version = ?",
    //         )?;
    //         let changed = stmt.execute((buffer.as_slice(), key.as_ref(), version))?;
    //         return Ok(changed == 0);
    //     }

    //     let mut stmt =
    //         write_conn.prepare("INSERT INTO kv_store (key, value, version) VALUES (?, ?, 0)")?;

    //     match stmt.execute((key.as_ref(), buffer.as_slice())) {
    //         Ok(count) => {
    //             assert_eq!(count, 1);
    //             Ok(true)
    //         }
    //         Err(err) => {
    //             if let rusqlite::Error::SqliteFailure(err, _) = err {
    //                 if let rusqlite::ErrorCode::ConstraintViolation = err.code {
    //                     return Ok(false);
    //                 }
    //             }
    //             Err(err.into())
    //         }
    //     }
    // }
}

async fn try_fetch_db_file_from_s3(db_path: &str) -> Result<()> {
    if std::fs::metadata(db_path).is_ok() {
        return Ok(());
    }

    let object = crate::s3()
        .get_object()
        .bucket(bucket_name())
        .key("db.sqlite")
        .send()
        .await?;

    let mut file = tokio::fs::File::create(db_path).await?;
    tokio::io::copy(&mut object.body.into_async_read(), &mut file).await?;

    Ok(())
}

async fn backup(sqlite3: &AtomicPtr<rusqlite::ffi::sqlite3>) -> Result<()> {
    println!("Start Backup db.sqlite");
    let now = std::time::SystemTime::now();

    let _ = std::fs::remove_file(BACKUP_PATH);

    rusqlite::backup::Backup::custom_backup(sqlite3, BACKUP_PATH, 256, || async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
    })
    .await?;

    save_db_backup_to_s3(BACKUP_PATH).await?;

    println!(
        "Successfully backed up db.sqlite in {:?}",
        now.elapsed().unwrap()
    );

    Ok(())
}

async fn save_db_backup_to_s3(backup_path: &str) -> Result<()> {
    // TODO: multipart
    crate::s3()
        .put_object()
        .bucket(bucket_name())
        .key("db.sqlite")
        .body(ByteStream::from_path(backup_path).await?)
        .send()
        .await?;

    Ok(())
}
