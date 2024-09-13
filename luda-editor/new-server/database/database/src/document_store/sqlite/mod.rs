//! # Important Note for Developer
//! 1. SK is optional. If it's None, it should be a empty byte blob, not NULL.

mod backup;

use super::*;
use crate::Result;
use document::TransactItems;
use rusqlite::{Connection, OpenFlags, OptionalExtension, Transaction};
use std::{
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, SystemTime},
};
use tokio::sync::{mpsc, oneshot};

type BackupNotify = oneshot::Sender<()>;

#[derive(Clone)]
pub struct SqliteKvStore {
    write: Arc<Mutex<Connection>>,
    backup_req_tx: mpsc::UnboundedSender<BackupNotify>,
}

const DB_FILENAME: &str = "db.sqlite";
const DB_WAL_FILENAME: &str = "db.sqlite-wal";
const DB_S3_KEY: &str = "db.sqlite";

impl SqliteKvStore {
    pub async fn new(s3_client: aws_sdk_s3::Client, bucket_name: String) -> anyhow::Result<Self> {
        backup::try_fetch_db_file_from_s3(&s3_client, &bucket_name).await?;

        let conn = Connection::open(DB_FILENAME).unwrap();

        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "NORMAL").unwrap();
        conn.execute(
            // - sk: If it's None, it will be a empty byte blob.
            // - expired_at: 0 means no expiration,
            //               otherwise it's the Unix Time, the number of seconds since 1970-01-01 00:00:00 UTC.
            "CREATE TABLE IF NOT EXISTS
                    documents (
                        name TEXT NOT NULL,
                        pk BLOB NOT NULL,
                        sk BLOB NOT NULL,
                        value BLOB,
                        version INTEGER,
                        expired_at INTEGER,
                        PRIMARY KEY (name, pk, sk)
                    )",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE INDEX IF NOT EXISTS
                documents_name_index
            ON
                documents (name)
                ",
            [],
        )
        .unwrap();

        migrate().await?;

        let write = Arc::new(Mutex::new(conn));

        let (backup_req_tx, mut backup_req_rx) = mpsc::unbounded_channel();

        tokio::spawn({
            let write = write.clone();
            async move {
                let mut backup_waits: Vec<BackupNotify> = vec![];
                loop {
                    tokio::select! {
                        result = backup_req_rx.recv() => {
                            let Some(notify) = result else {
                                println!("Backup stopped, backup_req_rx is dropped");
                                return;
                            };
                            backup_waits.push(notify);
                        }
                        _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {}
                    }

                    loop {
                        match backup_req_rx.try_recv() {
                            Ok(notify) => {
                                backup_waits.push(notify);
                            }
                            Err(err) => match err {
                                mpsc::error::TryRecvError::Empty => {
                                    break;
                                }
                                mpsc::error::TryRecvError::Disconnected => {
                                    println!("Backup stopped, backup_req_rx is dropped");
                                    return;
                                }
                            },
                        }
                    }

                    match backup::backup(write.clone(), &s3_client, &bucket_name).await {
                        Ok(_) => {
                            for notify in backup_waits.drain(..) {
                                let _ = notify.send(());
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to backup db: {:?}", err);
                        }
                    }
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
                            "DELETE FROM
                                documents
                            WHERE
                                expired_at != 0 AND expired_at < unixepoch()",
                            (),
                        )
                        .unwrap();

                    println!(
                        "Deleted expired keys in {:?}, deleted count: {deleted_count}",
                        time.elapsed()
                    );
                }
            }
        });

        Ok(Self {
            write,
            backup_req_tx,
        })
    }

    fn read_conn<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        thread_local! {
            static READ: Connection = Connection::open_with_flags(DB_FILENAME, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
        }
        READ.with(f)
    }
    fn write_conn(&self) -> MutexGuard<Connection> {
        self.write.lock().unwrap()
    }
}

impl DocumentStore for SqliteKvStore {
    async fn get(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<ValueBuffer>> {
        self.read_conn(|read_conn| {
            let mut stmt = read_conn.prepare(
                "
                SELECT
                    value
                FROM
                    documents 
                WHERE
                    name = ?
                    AND pk = ?
                    AND sk = ?
                    AND (expired_at = 0 OR expired_at >= unixepoch())
            ",
            )?;
            let vec: Option<Vec<_>> = stmt
                .query_row((name, pk, sk.unwrap_or_default()), |row| row.get(0))
                .optional()?;

            Ok(vec.map(ValueBuffer::Vec))
        })
    }

    async fn get_with_expiration(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<(ValueBuffer, Option<SystemTime>)>> {
        self.read_conn(|read_conn| {
            let mut stmt = read_conn.prepare(
                "
                SELECT
                    value,
                    expired_at
                FROM
                    documents 
                WHERE
                    name = ?
                    AND pk = ?
                    AND sk = ?
                    AND (expired_at = 0 OR expired_at >= unixepoch())
            ",
            )?;
            let output: Option<(Vec<_>, Option<u64>)> = stmt
                .query_row((name, pk, sk.unwrap_or_default()), |row| {
                    Ok((row.get(0)?, row.get(1)?))
                })
                .optional()?;

            Ok(output.map(|(vec, expired_at)| {
                (
                    ValueBuffer::Vec(vec),
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

    async fn query(&self, name: &'static str, pk: &[u8]) -> Result<Vec<ValueBuffer>> {
        self.read_conn(|read_conn| {
            let mut stmt = read_conn.prepare(
                "
                SELECT
                    value
                FROM
                    documents 
                WHERE
                    name = ?
                    AND pk = ?
                    AND (expired_at = 0 OR expired_at >= unixepoch())
            ",
            )?;
            let mut rows = stmt.query((name, pk))?;

            let mut vec = Vec::new();
            while let Some(row) = rows.next()? {
                vec.push(ValueBuffer::Vec(row.get(0)?));
            }

            Ok(vec)
        })
    }

    async fn put(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let mut write_conn = self.write_conn();
        let trx = write_conn.transaction()?;
        put(&trx, name, pk, sk, value, ttl)?;
        trx.commit()?;
        Ok(())
    }

    async fn delete(&self, name: &'static str, pk: &[u8], sk: Option<&[u8]>) -> Result<()> {
        let mut write_conn = self.write_conn();
        let trx = write_conn.transaction()?;
        delete(&trx, name, pk, sk)?;
        trx.commit()?;
        Ok(())
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let mut write_conn: MutexGuard<Connection> = self.write_conn();
        let trx = write_conn.transaction()?;
        create(&trx, name, pk, sk, value_fn, ttl)?;
        trx.commit()?;
        Ok(())
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        let mut write_conn: MutexGuard<Connection> = self.write_conn();
        let trx = write_conn.transaction()?;

        for item in transact_items {
            match item {
                TransactItem::Put {
                    name,
                    pk,
                    sk,
                    value,
                    ttl,
                } => {
                    put(&trx, name, pk, sk.as_deref(), &value, *ttl)?;
                }
                TransactItem::Create {
                    name,
                    pk,
                    sk,
                    value_fn,
                    ttl,
                } => {
                    create(
                        &trx,
                        name,
                        pk,
                        sk.as_deref(),
                        || Ok(value_fn.take().unwrap()()?),
                        *ttl,
                    )?;
                }
                TransactItem::Update {
                    name,
                    pk,
                    sk,
                    update_fn,
                } => {
                    let maybe_aborted = update(&trx, name, pk, sk.as_deref(), |vec| {
                        Ok(update_fn.take().unwrap()(vec)?)
                    })?;
                    if maybe_aborted.is_aborted() {
                        return Ok(maybe_aborted);
                    }
                }
                TransactItem::Delete { name, pk, sk } => delete(&trx, name, pk, sk.as_deref())?,
            }
        }
        trx.commit()?;
        Ok(MaybeAborted::No)
    }

    async fn wait_backup(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.backup_req_tx
            .send(tx)
            .map_err(|e| Error::BackupAborted(e.to_string()))?;
        rx.await.map_err(|e| Error::BackupAborted(e.to_string()))?;
        Ok(())
    }
}

async fn migrate() -> anyhow::Result<()> {
    let pramga_conn = Connection::open(DB_FILENAME)?;
    let current_version = {
        let result =
            pramga_conn.query_row("PRAGMA main.user_version", [], |row| row.get::<_, usize>(0));
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
            ) -> anyhow::Result<()> {
                let read_conn =
                    Connection::open_with_flags(DB_FILENAME, OpenFlags::SQLITE_OPEN_READ_ONLY)
                        .unwrap();
                let mut write_conn = Connection::open(DB_FILENAME)?;

                let mut read_stmt = read_conn.prepare(
                    "
                        SELECT
                            value, pk, sk
                        FROM
                            documents
                        WHERE
                            name = ?
                            AND (expired_at = 0 OR expired_at >= unixepoch())
                        ",
                )?;
                let mut rows = read_stmt.query([From::name()])?;

                let trx = write_conn.transaction()?;
                {
                    let mut write_stmt = trx.prepare(
                        "
                        UPDATE
                            documents
                        SET
                            name = ?,
                            value = ?,
                            version = 0
                        WHERE
                            pk = ?
                            AND sk = ?
                        ",
                    )?;

                    while let Some(row) = rows.next()? {
                        let from_bytes = row.get::<_, Vec<u8>>(0)?;
                        let pk = row.get::<_, String>(1)?;
                        let sk = row.get::<_, Option<String>>(2)?;

                        let from = From::from_bytes(from_bytes)?;
                        let to = f(from);
                        let to_bytes = to.to_bytes()?;

                        write_stmt.execute((to_bytes.as_slice(), pk, sk.unwrap_or_default()))?;
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
        &format!("PRAGMA main.user_version = {}", migration::LATEST_VERSION),
        [],
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

fn put(
    trx: &Transaction<'_>,
    name: &'static str,
    pk: &[u8],
    sk: Option<&[u8]>,
    value: &impl AsRef<[u8]>,
    ttl: Option<Duration>,
) -> Result<()> {
    let mut stmt = trx.prepare(
        "
            INSERT OR REPLACE INTO documents 
            (name, pk, sk, value, version, expired_at)
            VALUES (?, ?, ?, ?, 0, ?)",
    )?;

    assert_eq!(
        stmt.execute((
            name,
            pk,
            sk.unwrap_or_default(),
            value.as_ref(),
            ttl_to_expired_at(ttl)
        ))?,
        1
    );

    Ok(())
}

fn delete(trx: &Transaction<'_>, name: &'static str, pk: &[u8], sk: Option<&[u8]>) -> Result<()> {
    let mut stmt = trx.prepare(
        "
        DELETE FROM
            documents
        WHERE
            name = ?
            AND pk = ?
            AND sk = ?
    ",
    )?;
    stmt.execute((name, pk, sk.unwrap_or_default()))?;

    Ok(())
}

fn create<Bytes: AsRef<[u8]>>(
    trx: &Transaction<'_>,
    name: &'static str,
    pk: &[u8],
    sk: Option<&[u8]>,
    value_fn: impl FnOnce() -> Result<Bytes>,
    ttl: Option<Duration>,
) -> Result<()> {
    let mut stmt = trx.prepare(
        "
        SELECT count(*)
        FROM documents
        WHERE
            name = ?
            AND pk = ?
            AND sk = ?
            AND (expired_at = 0 OR expired_at >= unixepoch())
        ",
    )?;
    let count: i8 = stmt.query_row((name, &pk, &sk.unwrap_or_default()), |row| row.get(0))?;
    if count != 0 {
        return Err(Error::AlreadyExistsOnCreate);
    }

    let value = value_fn()?;
    let mut stmt = trx.prepare(
        // 'replace' to ignore expired documents. So should check expiration before this query.
        "
        INSERT OR REPLACE INTO documents
        (name, pk, sk, value, version, expired_at)
        VALUES (?, ?, ?, ?, 0, ?)",
    )?;
    assert_eq!(
        stmt.execute((
            name,
            &pk,
            &sk.unwrap_or_default(),
            value.as_ref(),
            ttl_to_expired_at(ttl)
        ))?,
        1
    );

    Ok(())
}

fn update<AbortReason>(
    trx: &Transaction<'_>,
    name: &'static str,
    pk: &[u8],
    sk: Option<&[u8]>,
    update_fn: impl FnOnce(&mut Vec<u8>) -> Result<WantUpdate<AbortReason>>,
) -> Result<MaybeAborted<AbortReason>> {
    let mut stmt = trx.prepare(
        "
        SELECT value
        FROM documents
        WHERE
            name = ?
            AND pk = ?
            AND sk = ?
            AND (expired_at = 0 OR expired_at >= unixepoch())
        ",
    )?;
    let Some(mut value): Option<Vec<u8>> = stmt
        .query_row((name, &pk, &sk.unwrap_or_default()), |row| row.get(0))
        .optional()?
    else {
        return Err(Error::NotExistsOnUpdate);
    };

    match update_fn(&mut value)? {
        WantUpdate::No => Ok(MaybeAborted::No),
        WantUpdate::Abort { reason } => Ok(MaybeAborted::Aborted { reason }),
        WantUpdate::Yes => {
            let mut stmt = trx.prepare(
                "
                UPDATE documents
                SET value = ?
                WHERE
                    name = ?
                    AND pk = ?
                    AND sk = ?
                    AND (expired_at = 0 OR expired_at >= unixepoch())
                ",
            )?;
            assert_eq!(
                stmt.execute((name, &pk, &sk.unwrap_or_default(), value))?,
                1
            );

            Ok(MaybeAborted::No)
        }
    }
}
