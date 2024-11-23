mod cache;
mod simple_doc_file;
mod trx_id_map;

use crate::*;
use cache::Cache;
use document_store::*;
use futures::future::try_join_all;
use simple_doc_file::*;
use std::{
    collections::{hash_map, HashMap, VecDeque},
    sync::Mutex,
};
use tokio::sync::oneshot;
use trx_id_map::TrxIdMap;

type Id = u128;
type KeyQueue = Arc<Mutex<HashMap<Key, KeyUsage>>>;

pub struct FsStore {
    mount_point: std::path::PathBuf,
    trx_id_map: TrxIdMap,
    cache: Cache,
    key_queue: KeyQueue,
}

enum KeyUsage {
    Using {
        wait_queue: VecDeque<oneshot::Sender<Option<SimpleDocFile>>>,
    },
    NotUsing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Key {
    name: DocName,
    id: Id,
}

impl DocumentStore for FsStore {
    async fn get(&self, name: DocName, id: Id) -> Result<Option<Bytes>> {
        let key = Key { name, id };
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached);
        }

        let mut key_locker = KeyLocker::new(self.key_queue.clone(), key);
        let file = key_locker.wait_turn().await;

        // Maybe cache updated during waiting.
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached);
        }

        if file.is_none() {
            *file = Some(self.open_doc_file(key).await?);
        }

        let bytes = file.as_mut().unwrap().get();
        self.cache.push([(key, bytes.clone())]);

        Ok(bytes)
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        mut transact_items: TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        transact_items.sort_by_key(|a| get_trx_item_key(a));

        let trx_id = uuid::Uuid::new_v4().as_u128();

        let mut key_lockers = try_join_all(
            transact_items
                .iter()
                .map(|trx_item| get_trx_item_key(trx_item))
                .map(|key| async move {
                    let mut key_locker = KeyLocker::new(self.key_queue.clone(), key);
                    let file = key_locker.wait_turn().await;

                    if file.is_none() {
                        *file = Some(self.open_doc_file(key).await?);
                    }

                    std::io::Result::Ok(key_locker)
                }),
        )
        .await?;

        let result = try_join_all(key_lockers.iter_mut().zip(transact_items).map(
            |(key_locker, trx_item)| async move {
                let file = key_locker.file_mut().unwrap();
                handle_trx_item(trx_item, file, trx_id).await
            },
        ))
        .await;

        if let Err(trx_err) = result {
            return match trx_err {
                TrxError::AlreadyExistsOnCreate => Err(Error::AlreadyExistsOnCreate),
                TrxError::NotExistsOnUpdate => Err(Error::NotExistsOnUpdate),
                TrxError::Abort(abort_reason) => Ok(MaybeAborted::Aborted {
                    reason: abort_reason,
                }),
                TrxError::Io(error) => Err(Error::IoError(error)),
                TrxError::SerErr(my_serializer_error) => {
                    Err(Error::SerializationError(my_serializer_error))
                }
            };
        }

        self.trx_id_map
            .insert(trx_id, key_lockers.iter().map(|x| x.key.id).collect())
            .await;

        key_lockers
            .iter_mut()
            .for_each(|x| x.file_mut().unwrap().commit(trx_id));

        self.cache
            .push(key_lockers.iter().map(|x| (x.key, x.file().unwrap().get())));

        Ok(MaybeAborted::No)
    }
}

impl FsStore {
    pub async fn new(mount_point: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let mount_point = mount_point.as_ref();
        Ok(Self {
            mount_point: mount_point.to_path_buf(),
            trx_id_map: TrxIdMap::new(mount_point).await?,
            cache: Cache::new(512 * 1024 * 1024),
            key_queue: Default::default(),
        })
    }
    async fn open_doc_file(&self, key: Key) -> std::io::Result<SimpleDocFile> {
        SimpleDocFile::open(
            &self.mount_point,
            format!("{}/{}", key.name, key.id),
            key.id,
            self.trx_id_map.clone(),
        )
        .await
    }
}

struct KeyLocker {
    rx: Option<oneshot::Receiver<Option<SimpleDocFile>>>,
    key_queue: KeyQueue,
    key: Key,
    file: Option<SimpleDocFile>,
}

impl KeyLocker {
    fn new(key_queue: KeyQueue, key: Key) -> Self {
        let rx = match key_queue.lock().unwrap().entry(key) {
            hash_map::Entry::Occupied(occupied_entry) => {
                let key_usage = occupied_entry.into_mut();
                match key_usage {
                    KeyUsage::Using { wait_queue } => {
                        let (tx, rx) = oneshot::channel();
                        wait_queue.push_back(tx);
                        Some(rx)
                    }
                    KeyUsage::NotUsing => {
                        *key_usage = KeyUsage::Using {
                            wait_queue: Default::default(),
                        };
                        None
                    }
                }
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(KeyUsage::Using {
                    wait_queue: Default::default(),
                });
                None
            }
        };

        Self {
            rx,
            key_queue,
            key,
            file: None,
        }
    }
    async fn wait_turn(&mut self) -> &mut Option<SimpleDocFile> {
        if let Some(rx) = &mut self.rx {
            match rx.await {
                Ok(file) => {
                    self.file = file;
                }
                Err(_) => {
                    self.file = None;
                }
            }
        }

        &mut self.file
    }
    fn file_mut(&mut self) -> Option<&mut SimpleDocFile> {
        self.file.as_mut()
    }

    fn file(&self) -> Option<&SimpleDocFile> {
        self.file.as_ref()
    }
}

impl Drop for KeyLocker {
    fn drop(&mut self) {
        let mut key_queue = self.key_queue.lock().unwrap();
        let key_usage = key_queue.get_mut(&self.key).unwrap();
        let KeyUsage::Using { wait_queue } = key_usage else {
            unreachable!()
        };
        let mut file = self.file.take();
        loop {
            let Some(tx) = wait_queue.pop_front() else {
                break;
            };
            match tx.send(file) {
                Ok(_) => break,
                Err(_file) => {
                    file = _file;
                    continue;
                }
            }
        }

        if wait_queue.is_empty() {
            *key_usage = KeyUsage::NotUsing;
        }
    }
}

enum TrxError<AbortReason> {
    AlreadyExistsOnCreate,
    NotExistsOnUpdate,
    Abort(AbortReason),
    Io(std::io::Error),
    SerErr(SerErr),
}

impl<AbortReason> From<std::io::Error> for TrxError<AbortReason> {
    fn from(e: std::io::Error) -> Self {
        TrxError::Io(e)
    }
}
impl<AbortReason> From<SerErr> for TrxError<AbortReason> {
    fn from(e: SerErr) -> Self {
        TrxError::SerErr(e)
    }
}
async fn handle_trx_item<'a, AbortReason>(
    trx_item: TransactItem<'a, AbortReason>,
    file: &mut SimpleDocFile,
    trx_id: u128,
) -> std::result::Result<(), TrxError<AbortReason>> {
    match trx_item {
        TransactItem::Put { value, .. } => {
            file.put(value, trx_id).await?;
            Ok(())
        }
        TransactItem::Create { mut value_fn, .. } => {
            if file.get().is_some() {
                return Err(TrxError::AlreadyExistsOnCreate);
            }
            let value = value_fn.take().unwrap()()?;
            file.put(value, trx_id).await?;
            Ok(())
        }
        TransactItem::Update { mut update_fn, .. } => {
            let Some(bytes) = file.get() else {
                return Err(TrxError::NotExistsOnUpdate);
            };
            let mut vec = bytes.to_vec();
            let result = update_fn.take().unwrap()(&mut vec)?;
            match result {
                WantUpdate::No => Ok(()),
                WantUpdate::Yes => {
                    file.put(vec, trx_id).await?;
                    Ok(())
                }
                WantUpdate::Abort { reason } => Err(TrxError::Abort(reason)),
            }
        }
        TransactItem::Delete { .. } => {
            file.delete(trx_id).await?;
            Ok(())
        }
    }
}

fn get_trx_item_key<AbortReason>(trx_item: &TransactItem<'_, AbortReason>) -> Key {
    match *trx_item {
        TransactItem::Put { name, id, .. }
        | TransactItem::Create { name, id, .. }
        | TransactItem::Update { name, id, .. }
        | TransactItem::Delete { name, id } => Key { name, id },
    }
}
