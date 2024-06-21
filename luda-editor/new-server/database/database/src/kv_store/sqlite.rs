use super::KvStore;
use anyhow::Result;
use aws_sdk_s3::primitives::ByteStream;
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::{
    sync::{atomic::AtomicPtr, Arc, Mutex, MutexGuard},
    time::{Duration, Instant, SystemTime},
};

#[derive(Clone)]
pub struct SqliteKvStore {
    write: Arc<Mutex<Connection>>,
}

const DB_PATH: &str = "db.sqlite";
const BACKUP_PATH: &str = "db.sqlite.backup";

impl SqliteKvStore {
    pub async fn new(s3_client: aws_sdk_s3::Client, bucket_name: String) -> Result<Self> {
        try_fetch_db_file_from_s3(&s3_client, &bucket_name).await?;

        let conn = Connection::open(DB_PATH).unwrap();

        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "NORMAL").unwrap();
        conn.execute(
            // - expired_at: 0 means no expiration,
            //               otherwise it's the Unix Time, the number of seconds since 1970-01-01 00:00:00 UTC.
            "CREATE TABLE IF NOT EXISTS
                    kv_store (
                        key TEXT PRIMARY KEY,
                        value BLOB,
                        version INTEGER,
                        expired_at INTEGER
                    )",
            [],
        )
        .unwrap();

        migrate().await?;

        let sqlite3 = AtomicPtr::new(conn.db.borrow_mut().db);
        let write = Arc::new(Mutex::new(conn));

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                if let Err(err) = backup(&sqlite3, &s3_client, &bucket_name).await {
                    eprintln!("Failed to backup db: {}", err);
                }
            }
        });

        // ttl removal
        tokio::spawn({
            let write = write.clone();
            async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;

                    let time = std::time::Instant::now();

                    let write_conn = write.lock().unwrap();
                    let deleted_count = write_conn
                    .execute(
                        "DELETE FROM kv_store WHERE expired_at != 0 AND expired_at < unixepoch()",
                        [],
                    )
                    .unwrap();

                    println!(
                        "Deleted expired keys in {:?}, deleted count: {deleted_count}",
                        time.elapsed()
                    );
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
            let mut stmt = read_conn.prepare(
                "
                SELECT value
                FROM kv_store 
                WHERE key = ? 
                    AND (expired_at = 0 OR expired_at >= unixepoch())
            ",
            )?;
            let vec: Option<Vec<_>> = stmt
                .query_row([key.as_ref()], |row| row.get(0))
                .optional()?;

            Ok(vec.map(super::ValueBuffer::Vec))
        })
    }

    async fn get_with_expiration(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<(super::ValueBuffer, Option<SystemTime>)>> {
        self.read_conn(|read_conn| {
            let mut stmt = read_conn.prepare(
                "
                SELECT value, expired_at
                FROM kv_store 
                WHERE key = ? 
                    AND (expired_at = 0 OR expired_at >= unixepoch())
            ",
            )?;
            let output: Option<(Vec<_>, Option<u64>)> = stmt
                .query_row([key.as_ref()], |row| Ok((row.get(0)?, row.get(1)?)))
                .optional()?;

            Ok(output.map(|(vec, expired_at)| {
                (
                    super::ValueBuffer::Vec(vec),
                    expired_at.and_then(|expired_at| {
                        if expired_at == 0 {
                            None
                        } else {
                            Some(std::time::UNIX_EPOCH + std::time::Duration::from_secs(expired_at))
                        }
                    }),
                )
            }))
        })
    }

    async fn put(
        &self,
        key: impl AsRef<str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let write_conn = self.write_conn();
        let mut stmt = write_conn.prepare(
            "
                INSERT OR REPLACE INTO kv_store 
                (key, value, version, expired_at)
                VALUES (?, ?, 0, ?)",
        )?;

        assert_eq!(
            stmt.execute((key.as_ref(), value.as_ref(), ttl_to_expired_at(ttl)))?,
            1
        );

        Ok(())
    }

    async fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        let write_conn = self.write_conn();
        let mut stmt = write_conn.prepare("DELETE FROM kv_store WHERE key = ?")?;
        stmt.execute([key.as_ref()])?;

        Ok(())
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        key: impl AsRef<str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let mut write_conn = self.write_conn();
        let tx = write_conn.transaction()?;

        let mut stmt = tx.prepare(
            "
            SELECT expired_at
            FROM kv_store
            WHERE key = ?",
        )?;
        let expired_at: Option<u64> = stmt
            .query_row([key.as_ref()], |row| row.get(0))
            .optional()?;
        if let Some(expired_at) = expired_at {
            if expired_at == 0 || now_epoch_time_secs() < expired_at {
                return Ok(());
            }
        }

        let value = value_fn()?;
        let mut stmt = tx.prepare(
            "
            INSERT OR REPLACE INTO kv_store
            (key, value, version, expired_at)
            VALUES (?, ?, 0, ?)",
        )?;
        assert_eq!(
            stmt.execute((key.as_ref(), value.as_ref(), ttl_to_expired_at(ttl)))?,
            1
        );

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

async fn try_fetch_db_file_from_s3(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> Result<()> {
    if std::fs::metadata(DB_PATH).is_ok() {
        return Ok(());
    }

    let object = s3_client
        .get_object()
        .bucket(bucket_name)
        .key("db.sqlite")
        .send()
        .await?;

    let mut file = tokio::fs::File::create(DB_PATH).await?;
    tokio::io::copy(&mut object.body.into_async_read(), &mut file).await?;

    Ok(())
}

async fn backup(
    sqlite3: &AtomicPtr<rusqlite::ffi::sqlite3>,
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> Result<()> {
    println!("Start Backup db.sqlite");
    let now = std::time::SystemTime::now();

    let _ = std::fs::remove_file(BACKUP_PATH);

    rusqlite::backup::Backup::custom_backup(sqlite3, BACKUP_PATH, 256, || async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
    })
    .await?;

    save_db_backup_to_s3(s3_client, bucket_name).await?;

    println!(
        "Successfully backed up db.sqlite in {:?}",
        now.elapsed().unwrap()
    );

    Ok(())
}

async fn save_db_backup_to_s3(s3_client: &aws_sdk_s3::Client, bucket_name: &str) -> Result<()> {
    // TODO: multipart
    s3_client
        .put_object()
        .bucket(bucket_name)
        .key("db.sqlite")
        .body(ByteStream::from_path(BACKUP_PATH).await?)
        .send()
        .await?;

    Ok(())
}

async fn migrate() -> Result<()> {
    let pramga_conn = Connection::open(DB_PATH)?;
    let current_version = {
        let result = pramga_conn.query_row("PRAGMA kv_store.user_version", [], |row| {
            row.get::<_, usize>(0)
        });
        if let Err(rusqlite::Error::QueryReturnedNoRows) = result {
            None
        } else {
            Some(result?)
        }
    };

    if let Some(current_version) = current_version {
        struct MigrationConnection {}

        impl migration::DatabaseConnection for MigrationConnection {
            fn map<From: document::Document, To: document::Document>(
                &self,
                mut f: impl FnMut(From) -> To,
            ) -> Result<()> {
                let read_conn =
                    Connection::open_with_flags(DB_PATH, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
                let mut write_conn = Connection::open(DB_PATH)?;

                let mut read_stmt =
                    read_conn.prepare("SELECT value, key FROM kv_store WHERE key LIKE ?")?;
                let mut rows = read_stmt.query([format!("{}-%", From::name())])?;

                let trx = write_conn.transaction()?;
                {
                    let mut write_stmt =
                        trx.prepare("UPDATE kv_store SET value = ?, version = 0 WHERE key = ?")?;

                    while let Some(row) = rows.next()? {
                        let from_bytes = row.get::<_, Vec<u8>>(0)?;
                        let key = row.get::<_, String>(1)?;

                        let from = From::from_bytes(from_bytes)?;
                        let to = f(from);
                        let to_bytes = to.to_bytes()?;

                        write_stmt.execute((to_bytes.as_slice(), key))?;
                    }
                }
                trx.commit()?;

                Ok(())
            }
        }
        let migration_connection = MigrationConnection {};

        migration::migrate(current_version, migration_connection).await?;
    }

    pramga_conn.execute(
        "PRAGMA kv_store.user_version = ?",
        [migration::LATEST_VERSION],
    )?;

    Ok(())
}

fn ttl_to_expired_at(ttl: Option<Duration>) -> u64 {
    ttl.map(|ttl| ttl.as_secs() + now_epoch_time_secs())
        .unwrap_or(0)
}

fn now_epoch_time_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
