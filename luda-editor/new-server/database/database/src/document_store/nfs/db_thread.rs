use super::*;
use bytes::Bytes;
use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

pub(crate) fn db_thread(
    mount_point: PathBuf,
    db_request_tx: std::sync::mpsc::Sender<DbThreadRequest>,
    db_request_rx: std::sync::mpsc::Receiver<DbThreadRequest>,
) {
    let mut cache = HashMap::<String, CachedItem>::new();
    let mut file_accesses = HashMap::<String, FileAccess>::new();

    let wal_dir = mount_point.join("wal");
    let doc_dir = mount_point.join("doc");
    let mut wal_index = 0;

    while let Ok(request) = db_request_rx.recv() {
        match request {
            DbThreadRequest::Read { key, tx } => {
                if let Some(cached) = cache.get(&key) {
                    _ = tx.send(Ok(cached.get()));
                    continue;
                }

                if let Some(file_access) = file_accesses.get_mut(&key) {
                    file_access.waiters.push(tx);
                    continue;
                }

                let file_access = FileAccess { waiters: vec![tx] };
                file_accesses.insert(key.clone(), file_access);

                let path = doc_dir.join(&key);
                let db_request_tx = db_request_tx.clone();

                thread::spawn(move || {
                    // TODO: Handle panics or hangs. I guess timeout is a good idea.

                    let result = read_file(&path);
                    _ = db_request_tx.send(DbThreadRequest::ReadResult { key, result });
                });
            }
            DbThreadRequest::ReadResult { key, result } => {
                let file_access = file_accesses.remove(&key).unwrap();
                file_access.waiters.into_iter().for_each(|tx| {
                    _ = tx.send(result.clone());
                });
                if let Ok(bytes) = result {
                    cache.insert(key, CachedItem::new(bytes));
                }
            }
            DbThreadRequest::Write { writes, tx } => {
                // WAL 먼저 작성하고, 캐시 업데이트하고, 파일 업데이트는 백그라운드에서.
                if writes
                    .iter()
                    .any(|write| file_accesses.contains_key(write.key()))
                {
                    let _ = tx.send(Err(TransactionError::LockFailed));
                    continue;
                }

                writes.iter().for_each(|write| {
                    assert!(file_accesses
                        .insert(write.key().to_string(), FileAccess { waiters: vec![] })
                        .is_none());
                });

                let cache_includes = writes
                    .into_iter()
                    .map(|write| {
                        if matches!(write, TransactionWrite::Update { .. }) {
                            let cached = cache.get(write.key()).map(|cached| cached.bytes.clone());
                            (write, cached)
                        } else {
                            (write, None)
                        }
                    })
                    .collect::<Vec<(TransactionWrite, Option<Option<Bytes>>)>>();

                let doc_dir = doc_dir.clone();
                let wal_path = wal_dir.join(wal_index.to_string());
                wal_index += 1;
                let db_request_tx = db_request_tx.clone();

                std::thread::spawn(move || {
                    let keys = cache_includes
                        .iter()
                        .map(|(write, _)| write.key().to_string())
                        .collect::<Vec<_>>();

                    let wal_writes: Result<Vec<WalWrite>> = cache_includes
                        .into_par_iter()
                        .map(|(write, cached)| match write {
                            TransactionWrite::Put { key, value } => Ok(WalWrite {
                                key,
                                value: Some(value),
                            }),
                            TransactionWrite::Delete { key } => Ok(WalWrite { key, value: None }),
                            TransactionWrite::Update { key, data_fn } => {
                                let bytes = {
                                    if let Some(bytes) = cached {
                                        bytes
                                    } else {
                                        read_file(doc_dir.join(&key))?
                                    }
                                };
                                let value = data_fn(bytes)?;

                                Ok(WalWrite { key, value })
                            }
                        })
                        .collect();

                    let result: std::result::Result<Vec<WalWrite>, TransactionError> = (|| {
                        let wal_writes = wal_writes?;
                        let wal_bytes = serialize_wal_writes(&wal_writes);

                        let mut wal_file = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(&wal_path)
                            .map_err(|err| TransactionError::IoError(err.kind()))?;
                        wal_file
                            .write_all(&wal_bytes)
                            .map_err(|err| TransactionError::IoError(err.kind()))?;
                        wal_file
                            .sync_all()
                            .map_err(|err| TransactionError::IoError(err.kind()))?;

                        Ok(wal_writes)
                    })(
                    );

                    _ = db_request_tx.send(match result {
                        Ok(wal_writes) => DbThreadRequest::WriteResultOk { wal_writes },
                        Err(error) => DbThreadRequest::WriteResultErr { keys, error },
                    });
                });
            }
            DbThreadRequest::WriteResultOk { wal_writes } => {
                for wal_write in wal_writes {
                    let file_access = file_accesses.remove(&wal_write.key).unwrap();
                    file_access.waiters.into_iter().for_each(|tx| {
                        _ = tx.send(Ok(wal_write.value.clone().map(Bytes::from)));
                    });
                    cache.insert(wal_write.key, CachedItem::new(wal_write.value));
                }
            }
            DbThreadRequest::WriteResultErr { keys, error } => {
                for key in keys {
                    let file_access = file_accesses.remove(&key).unwrap();
                    file_access.waiters.into_iter().for_each(|tx| {
                        _ = tx.send(Err(error.clone()));
                    });
                }
            }
        }
    }
}

fn read_file(path: impl AsRef<Path>) -> Result<Option<Bytes>> {
    match std::fs::read(path) {
        Ok(bytes) => Ok(Some(Bytes::from(bytes))),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(None),
            _ => Err(TransactionError::IoError(error.kind())),
        },
    }
}

struct CachedItem {
    bytes: Option<Bytes>,
    last_accessed_secs: AtomicU64,
}

impl CachedItem {
    pub fn new(bytes: Option<Bytes>) -> Self {
        Self {
            bytes,
            last_accessed_secs: AtomicU64::new(get_last_accessed_secs()),
        }
    }
    pub fn get(&self) -> Option<Bytes> {
        self.last_accessed_secs.store(
            get_last_accessed_secs(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.bytes.clone()
    }
}

fn get_last_accessed_secs() -> u64 {
    std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_secs()
}

pub(crate) enum DbThreadRequest<'a> {
    Read {
        key: String,
        tx: DataTx,
    },
    ReadResult {
        key: String,
        result: DataResult,
    },
    Write {
        writes: Vec<TransactionWrite<'a>>,
        tx: oneshot::Sender<Result<()>>,
    },
    WriteResultOk {
        wal_writes: Vec<WalWrite>,
    },
    WriteResultErr {
        keys: Vec<String>,
        error: NfsStoreError,
    },
}

struct FileAccess {
    waiters: Vec<DataTx>,
}
