mod cache;
mod relation_file;
mod simple_doc_file;
mod trx_id_map;

use crate::*;
use cache::Cache;
use document_store::*;
use simple_doc_file::*;
use std::{
    collections::{hash_map, HashMap, VecDeque},
    path::PathBuf,
    sync::Mutex,
};
use tokio::sync::oneshot;
use trx_id_map::TrxIdMap;

type Id = u128;

pub struct FsLockerVersionDocStore {
    mount_point: std::path::PathBuf,
    trx_id_map: TrxIdMap,
    cache: Cache,
    key_queue: Mutex<HashMap<Key, KeyUsage>>,
}

enum KeyUsage {
    Using {
        wait_queue: VecDeque<oneshot::Sender<Option<SimpleDocFile>>>,
    },
    NotUsing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    name: DocName,
    id: Id,
}

impl DocumentStore for FsLockerVersionDocStore {
    async fn get(&self, name: DocName, id: Id) -> Result<Option<Bytes>> {
        let key = Key { name, id };
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached);
        }

        let rx = {
            let mut key_queue = self.key_queue.lock().unwrap();
            match key_queue.entry(key) {
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
            }
        };

        let mut file = if let Some(rx) = rx {
            rx.await.unwrap()
        } else {
            None
        };

        let result = self.do_get(key, &mut file).await;

        {
            let mut key_queue = self.key_queue.lock().unwrap();
            let key_usage = key_queue.get_mut(&key).unwrap();
            let KeyUsage::Using { wait_queue } = key_usage else {
                unreachable!()
            };
            let mut file = file;
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

        result
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        return todo!();
        // 1. Create a WAL (Write-Ahead Log) for each file,
        // 2. Generate a transaction ID,
        // 3. Append new values to the WAL of each file participating in the transaction, including the transaction ID,
        // 4. Finally, add the transaction ID to the success_transaction_id_tree.

        // Then, even if there’s a failure in the middle,
        // - Before applying the WAL, check if the transaction ID is in the success_transaction_id_tree.
        // If it isn’t, skip it. Wouldn’t that work?

        // let mut files = self.files.lock().await;

        // let trx_id = uuid::Uuid::new_v4().as_u128();

        // let keys = extract_file_keys(&transact_items);
        // self.make_sure_files(&mut files, &keys, &self.trx_id_map)
        //     .await?;

        // let ins = transact_items.into_iter().zip(keys).map(|(trx_item, key)| {
        //     let file_writer = files.get(&key).unwrap().1.get_writer();
        //     In {
        //         key,
        //         file_writer,
        //         trx_item,
        //     }
        // });

        // struct In<'a, AbortReason> {
        //     key: FileKey,
        //     file_writer: DocFileWriter,
        //     trx_item: TransactItem<'a, AbortReason>,
        // }

        // struct Out {
        //     key: FileKey,
        //     file_writer: DocFileWriter,
        // }

        // enum TrxError<AbortReason> {
        //     NotExistsOnUpdate,
        //     Abort(AbortReason),
        //     Io(std::io::Error),
        //     SerErr(SerErr),
        // }
        // impl<AbortReason> From<std::io::Error> for TrxError<AbortReason> {
        //     fn from(e: std::io::Error) -> Self {
        //         TrxError::Io(e)
        //     }
        // }
        // impl<AbortReason> From<SerErr> for TrxError<AbortReason> {
        //     fn from(e: SerErr) -> Self {
        //         TrxError::SerErr(e)
        //     }
        // }

        // async fn handle_trx_item<AbortReason>(
        //     trx_item: TransactItem<'_, AbortReason>,
        //     file_writer: &mut DocFileWriter,
        //     trx_id: u128,
        // ) -> std::result::Result<(), TrxError<AbortReason>> {
        //     match trx_item {
        //         TransactItem::Put { value, .. } => {
        //             let DocFileWriter::Simple(file_writer) = file_writer else {
        //                 unreachable!()
        //             };
        //             file_writer.put(Bytes::from(value.to_vec()), trx_id).await?;
        //             Ok(())
        //         }
        //         TransactItem::Create { mut value_fn, .. } => {
        //             let DocFileWriter::Simple(file_writer) = file_writer else {
        //                 unreachable!()
        //             };

        //             if file_writer.is_empty() {
        //                 let value = value_fn.take().unwrap()()?;
        //                 file_writer.put(Bytes::from(value), trx_id).await?;
        //             }
        //             Ok(())
        //         }
        //         TransactItem::Update { mut update_fn, .. } => {
        //             let DocFileWriter::Simple(file_writer) = file_writer else {
        //                 unreachable!()
        //             };

        //             let mut vec = file_writer.get().to_vec();
        //             if vec.is_empty() {
        //                 return Err(TrxError::NotExistsOnUpdate);
        //             }

        //             match update_fn.take().unwrap()(&mut vec)? {
        //                 WantUpdate::No => Ok(()),
        //                 WantUpdate::Yes => {
        //                     file_writer.put(vec.into(), trx_id).await?;
        //                     Ok(())
        //                 }
        //                 WantUpdate::Abort { reason } => Err(TrxError::Abort(reason)),
        //             }
        //         }
        //         TransactItem::Delete { .. } => {
        //             let DocFileWriter::Simple(file_writer) = file_writer else {
        //                 unreachable!()
        //             };

        //             if !file_writer.is_empty() {
        //                 file_writer.put(Bytes::new(), trx_id).await?;
        //             }
        //             Ok(())
        //         }
        //     }
        // }

        // let outs = futures::future::try_join_all(ins.map(
        //     |In {
        //          key,
        //          mut file_writer,
        //          trx_item,
        //      }| async move {
        //         handle_trx_item(trx_item, &mut file_writer, trx_id).await?;

        //         Ok(Out { key, file_writer })
        //     },
        // ))
        // .await?;

        // let file_ids = keys.iter().map(|key| key.id).collect::<Vec<_>>();
        // self.trx_id_map.insert(trx_id, file_ids).await;

        // for Out { key, file_writer } in outs {
        //     let (_time, file) = files.get_mut(&key).unwrap();
        //     file.commit(trx_id).await;
        // }

        // let mut maybe_aborted = MaybeAborted::No;

        // for (transact_item, key) in transact_items.into_iter().zip(&keys) {
        //     match transact_item {
        //         TransactItem::Put { value, .. } => {
        //             let (_, file) = files.get_mut(key).unwrap();
        //             let DocFileWriter::Simple(simple_doc_file) = file else {
        //                 unreachable!()
        //             };
        //             simple_doc_file
        //                 .put(Bytes::from(value.to_vec()), trx_id)
        //                 .await?;
        //         }
        //         TransactItem::Create { mut value_fn, .. } => {
        //             let (_, file) = files.get_mut(key).unwrap();
        //             let DocFile::Simple(simple_doc_file) = file else {
        //                 unreachable!()
        //             };

        //             if !simple_doc_file.is_empty() {
        //                 continue;
        //             }
        //             let value = value_fn.take().unwrap()()?;
        //             simple_doc_file.put(Bytes::from(value), trx_id).await?;
        //         }
        //         TransactItem::Update { mut update_fn, .. } => {
        //             let (_, file) = files.get_mut(key).unwrap();
        //             let DocFile::Simple(simple_doc_file) = file else {
        //                 unreachable!()
        //             };

        //             let mut vec = simple_doc_file.get().to_vec();
        //             if vec.is_empty() {
        //                 return Err(Error::NotExistsOnUpdate);
        //             }

        //             match update_fn.take().unwrap()(&mut vec)? {
        //                 WantUpdate::No => continue,
        //                 WantUpdate::Yes => simple_doc_file.put(vec.into(), trx_id).await?,
        //                 WantUpdate::Abort { reason } => {
        //                     maybe_aborted = MaybeAborted::Aborted { reason };
        //                     break;
        //                 }
        //             }
        //         }
        //         TransactItem::Delete { .. } => {
        //             let (_, file) = files.get_mut(key).unwrap();
        //             let DocFile::Simple(simple_doc_file) = file else {
        //                 unreachable!()
        //             };

        //             if simple_doc_file.is_empty() {
        //                 continue;
        //             }
        //             simple_doc_file.put(Bytes::new(), trx_id).await?;
        //         }
        //         TransactItem::InsertRelation { to_id, .. } => {
        //             let (_, file) = files.get_mut(key).unwrap();
        //             let DocFile::RelationFile(file) = file else {
        //                 unreachable!()
        //             };

        //             file.insert(to_id, trx_id)?;
        //         }
        //         TransactItem::RemoveRelation { to_id, .. } => {
        //             let (_, file) = files.get_mut(key).unwrap();
        //             let DocFile::RelationFile(file) = file else {
        //                 unreachable!()
        //             };

        //             file.remove(to_id, trx_id)?;
        //         }
        //     };
        // }

        // if let MaybeAborted::No = &maybe_aborted {
        //     let file_ids = keys.iter().map(|key| key.id).collect::<Vec<_>>();

        //     self.trx_id_map.insert(trx_id, file_ids).await;
        // };

        // let mut join_set = JoinSet::new();
        // for &key in &keys {
        //     let (instant, mut file) = files.remove(&key).unwrap();
        //     join_set.spawn(async move {
        //         file.commit(trx_id).await;
        //         (key, (instant, file))
        //     });
        // }
        // while let Some(entry) = join_set.join_next().await {
        //     let (key, value) = entry.unwrap();
        //     files.insert(key, value);
        // }

        // Ok(maybe_aborted)
    }

    // fn query(
    //     &self,
    //     from_name: document_store::DocName,
    //     id: Id,
    //     to_name: document_store::DocName,
    // ) -> impl futures::Stream<Item = Result<Id>> + 'static + Unpin {
    //     IdSet::stream(todo!()).map_err(|e| Error::from(e))
    // }
}

impl FsLockerVersionDocStore {
    async fn do_get(&self, key: Key, file: &mut Option<SimpleDocFile>) -> Result<Option<Bytes>> {
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached);
        }

        if file.is_none() {
            *file = Some(
                SimpleDocFile::open(
                    &self.mount_point,
                    format!("{}/{}", key.name, key.id),
                    key.id,
                    self.trx_id_map.clone(),
                )
                .await?,
            );
        }

        let bytes = file.as_mut().unwrap().get();
        self.cache.put(key, bytes.clone());

        Ok(bytes)
    }
    // async fn close_old_files(&self, except_keys: &[FileKey]) {
    //     const SOFT_MAX_FILES_LIMIT: usize = 20000;
    //     let mut files = self.files.lock().await;

    //     if files.len() <= SOFT_MAX_FILES_LIMIT {
    //         return;
    //     }

    //     let overflow = files.len() - SOFT_MAX_FILES_LIMIT;

    //     let mut vec: Vec<_> = std::mem::take(&mut *files).into_iter().collect();
    //     vec.sort_by_key(|(_, (last_access, _))| *last_access);

    //     let mut survived = vec.split_off(overflow);

    //     for entry in vec {
    //         if except_keys.contains(&entry.0) {
    //             survived.push(entry);
    //         }
    //     }

    //     *files = survived.into_iter().collect();
    // }

    // async fn make_sure_files(
    //     &self,
    //     files: &mut impl DerefMut<Target = HashMap<FileKey, (Instant, DocFile)>>,
    //     keys: &[FileKey],
    //     trx_id_map: &TrxIdMap,
    // ) -> std::io::Result<()> {
    //     self.close_old_files(keys).await;

    //     let now = Instant::now();
    //     for key in keys {
    //         if !files.contains_key(key) {
    //             let doc_file = match key.file_type {
    //                 FileType::Simple => DocFile::Simple(
    //                     SimpleDocFile::open(
    //                         &self.mount_point,
    //                         key.path(),
    //                         key.id,
    //                         trx_id_map.clone(),
    //                     )
    //                     .await?,
    //                 ),
    //                 FileType::RelationFile => {
    //                     DocFile::RelationFile(RelationFile::open(&self.mount_point, key.path())?)
    //                 }
    //             };
    //             files.insert(*key, (Instant::now(), doc_file));
    //         } else {
    //             let (last_access, _) = files.get_mut(key).unwrap();
    //             *last_access = now;
    //         }
    //     }

    //     Ok(())
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FileKey {
    name: DocName,
    id: Id,
    file_type: FileType,
}

impl FileKey {
    fn path(&self) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.name, self.id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FileType {
    Simple,
    RelationFile,
}

fn extract_file_keys<T>(items: &TransactItems<'_, T>) -> Vec<FileKey> {
    items
        .iter()
        .map(|item| match item {
            TransactItem::Put { name, id, .. }
            | TransactItem::Create { name, id, .. }
            | TransactItem::Update { name, id, .. }
            | TransactItem::Delete { name, id } => FileKey {
                name,
                id: *id,
                file_type: FileType::Simple,
            },
        })
        .collect()
}
