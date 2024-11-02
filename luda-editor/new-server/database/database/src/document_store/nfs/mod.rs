mod bp_id_tree;
mod db_thread;
mod document_store_impl;
mod flush_wals;
mod wal;

use bytes::Bytes;
use db_thread::*;
use flush_wals::*;
use rayon::prelude::*;
use std::{io::ErrorKind, sync::atomic::AtomicU64, thread};
use tokio::sync::oneshot;
use wal::*;

type Result<T> = std::result::Result<T, TransactionError>;

fn crc() -> crc::Crc<u64> {
    crc::Crc::<u64>::new(&crc::CRC_64_REDIS)
}

#[derive(Clone)]
pub struct NfsV4DocStore {
    db_request_tx: std::sync::mpsc::Sender<DbThreadRequest>,
}

impl NfsV4DocStore {
    pub fn new(mount_point: impl AsRef<std::path::Path>) -> Self {
        let mount_point = mount_point.as_ref().to_path_buf();

        let (db_request_tx, db_request_rx) = std::sync::mpsc::channel();

        flush_wals(&mount_point.join("wal"), &mount_point.join("doc"));

        thread::spawn({
            let db_request_tx = db_request_tx.clone();
            move || db_thread::db_thread(mount_point, db_request_tx, db_request_rx)
        });

        Self { db_request_tx }
    }
    /// Consistency: Read-After-Write
    async fn read(&self, pk: &str, sk: Option<&str>) -> Result<Option<Bytes>> {
        let (tx, rx) = oneshot::channel();
        self.db_request_tx
            .send(DbThreadRequest::Read {
                pk: pk.to_string(),
                sk: sk.map(|s| s.to_string()),
                tx,
            })
            .map_err(|_| TransactionError::DbThreadDown)?;
        rx.await.map_err(|_| TransactionError::DbThreadDown)?
    }

    async fn write(&self, tuples: Vec<TransactionWrite>) -> Result<()> {
        if tuples.is_empty() {
            return Ok(());
        }
        let (tx, rx) = oneshot::channel();
        self.db_request_tx
            .send(DbThreadRequest::Write { writes: tuples, tx })
            .map_err(|_| TransactionError::DbThreadDown)?;
        rx.await.map_err(|_| TransactionError::DbThreadDown)?
    }
}

pub enum TransactionWrite {
    Put {
        key: String,
        value: Bytes,
    },
    Delete {
        key: String,
    },
    Update {
        key: String,
        tx: DataTx,
        rx: oneshot::Receiver<Result<Option<Bytes>>>,
    },
}

#[derive(Debug, Clone)]
pub struct TransactionAbort {}

impl TransactionWrite {
    fn key(&self) -> &str {
        match self {
            Self::Put { key, .. } | Self::Delete { key, .. } | Self::Update { key, .. } => key,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransactionError {
    DbThreadDown,
    LockFailed,
    IoError(std::io::ErrorKind),
    TxSendError,
    RxRecvError,
    Abort(TransactionAbort),
}
impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for TransactionError {}

type DataResult = Result<Option<Bytes>>;
type DataTx = oneshot::Sender<DataResult>;
