mod db_thread;
mod document_store_impl;
mod flush_wals;
mod wal;

use bytes::Bytes;
use db_thread::*;
use flush_wals::*;
use std::{
    any::Any,
    io::ErrorKind,
    sync::{atomic::AtomicU64, Arc},
    thread,
};
use tokio::sync::oneshot;
use wal::*;

type Result<T> = std::result::Result<T, NfsStoreError>;

fn crc() -> crc::Crc<u64> {
    crc::Crc::<u64>::new(&crc::CRC_64_REDIS)
}

#[derive(Clone)]
pub struct NfsV4DocStore<'a> {
    db_request_tx: std::sync::mpsc::Sender<DbThreadRequest<'a>>,
}

impl<'a> NfsV4DocStore<'a> {
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
    async fn read(&self, key: String) -> Result<Option<Bytes>> {
        let (tx, rx) = oneshot::channel();
        self.db_request_tx
            .send(DbThreadRequest::Read { key, tx })
            .map_err(|_| NfsStoreError::DbThreadDown)?;
        rx.await.map_err(|_| NfsStoreError::DbThreadDown)?
    }

    async fn write(&'a self, tuples: Vec<TransactionWrite<'a>>) -> Result<()> {
        if tuples.is_empty() {
            return Ok(());
        }
        let (tx, rx) = oneshot::channel();
        self.db_request_tx
            .send(DbThreadRequest::Write { writes: tuples, tx })
            .map_err(|_| NfsStoreError::DbThreadDown)?;
        rx.await.map_err(|_| NfsStoreError::DbThreadDown)?
    }
}

pub enum TransactionWrite<'a> {
    Put {
        key: String,
        value: Bytes,
    },
    Delete {
        key: String,
    },
    Update {
        key: String,
        data_fn: Box<
            dyn FnOnce(Option<Bytes>) -> std::result::Result<Option<Bytes>, Box<dyn Any>>
                + Send
                + 'a,
        >,
    },
}

impl TransactionWrite<'_> {
    fn key(&self) -> &str {
        match self {
            Self::Put { key, .. } | Self::Delete { key, .. } | Self::Update { key, .. } => key,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NfsStoreError {
    DbThreadDown,
    LockFailed,
    IoError(std::io::ErrorKind),
    TxSendError,
    RxRecvError,
    UpdateAbort(Arc<dyn Any + Send + Sync>),
}
impl std::fmt::Display for NfsStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for NfsStoreError {}

type DataResult = Result<Option<Bytes>>;
type DataTx = oneshot::Sender<DataResult>;
