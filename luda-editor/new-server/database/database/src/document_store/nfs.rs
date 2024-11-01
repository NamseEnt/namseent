use anyhow::{bail, Result};
use bytes::Bytes;
use dashmap::DashMap;
use nix::fcntl::Flock;
use std::{
    collections::HashMap,
    fs::*,
    path::PathBuf,
    sync::{atomic::AtomicU64, RwLock},
    thread,
};
use tokio::{
    sync::{oneshot, OnceCell},
    task::JoinSet,
};

struct NfsV4Db {
    mount_point: PathBuf,
    db_request_tx: tokio::sync::mpsc::Sender<DbThreadRequest>,
}

struct CachedItem {
    bytes: Bytes,
    last_accessed_secs: AtomicU64,
}

impl CachedItem {
    pub fn new(bytes: Bytes) -> Self {
        Self {
            bytes,
            last_accessed_secs: AtomicU64::new(get_last_accessed_secs()),
        }
    }
    pub fn get(&self) -> Bytes {
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

fn db_thread(mount_point: PathBuf) {
    let (db_request_tx, db_request_rx) = std::sync::mpsc::channel::<DbThreadRequest>();

    let mut cache = HashMap::<String, CachedItem>::new();
    let mut file_accesses = HashMap::<String, FileAccess>::new();

    while let Ok(request) = db_request_rx.recv() {
        match request {
            DbThreadRequest::Read { key, tx } => {
                if let Some(cached) = cache.get(&key) {
                    _ = tx.send(Ok(Some(cached.get())));
                    continue;
                }

                if let Some(file_access) = file_accesses.get_mut(&key) {
                    file_access.waiters.push(tx);
                    continue;
                }

                let file_access = FileAccess { waiters: vec![tx] };
                file_accesses.insert(key.clone(), file_access);

                let path = mount_point.join(&key);
                let db_request_tx = db_request_tx.clone();
                thread::spawn(move || {
                    let result = match std::fs::read(&path) {
                        Ok(bytes) => Ok(Some(Bytes::from(bytes))),
                        Err(error) => match error.kind() {
                            std::io::ErrorKind::NotFound => Ok(None),
                            _ => Err(error.kind()),
                        },
                    };
                    _ = db_request_tx.send(DbThreadRequest::ReadResult { key, result });
                });
            }
            DbThreadRequest::ReadResult { key, result } => {
                let file_access = file_accesses.remove(&key).unwrap();
                file_access.waiters.into_iter().for_each(|tx| {
                    _ = tx.send(result.clone());
                });
                if let Ok(Some(bytes)) = result {
                    cache.insert(key, CachedItem::new(bytes));
                }
            }
            DbThreadRequest::Write { tuples, tx } => {
                WAL 먼저 작성하고, 캐시 업데이트하고, 파일 업데이트는 백그라운드에서.
            },
        }
    }
}

type DataResult = Result<Option<Bytes>, std::io::ErrorKind>;
type DataTx = oneshot::Sender<DataResult>;

enum DbThreadRequest {
    Read {
        key: String,
        tx: DataTx,
    },
    ReadResult {
        key: String,
        result: DataResult,
    },
    Write {
        tuples: Vec<(String, Vec<u8>)>,
        tx: oneshot::Sender<Result<()>>,
    },
}

struct FileAccess {
    waiters: Vec<DataTx>,
}

impl NfsV4Db {
    /// Consistency: Read-After-Write
    async fn read(&self, key: &str) -> Result<Option<Bytes>> {
        let (tx, rx) = oneshot::channel();
        self.db_request_tx
            .send(DbThreadRequest::Read {
                key: key.to_string(),
                tx,
            })
            .await?;
        Ok(rx.await?.map_err(std::io::Error::from)?)
    }

    async fn write(&self, tuples: &[(&str, &[u8])]) -> Result<()> {
        // 아니지. lock을 얻고 WAL을 작성하고 업데이트를 수행해야해.
        // 근데 만약 WAL 작성 후 죽으면, 누군가는 WAL이 적용되도록 보장해줘야해.
        // 그렇다면 read하기에 앞서 WAL이 무조건 적용되어야하지.

        // let mut files = Vec::with_capacity(tuples.len());
        // for (key, _) in tuples {
        //     let file = self.open_and_lock(key, LockType::Write).await?.unwrap();
        //     files.push(file);
        // }

        // let mut join_set: JoinSet<Result<()>> = JoinSet::new();

        // for (mut file, (_, value)) in files.into_iter().zip(tuples.iter()) {
        //     join_set.spawn_blocking(move || {
        //         file.write_all(value)?;
        //         Ok(())
        //     });
        // }

        // join_set.await?;

        Ok(())
    }
}
