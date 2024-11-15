mod relation_file;
mod simple_doc_file;
mod trx_id_map;

use crate::*;
use document_store::*;
use relation_file::RelationFile;
use simple_doc_file::*;
use std::{collections::HashMap, ops::DerefMut, path::PathBuf, time::Instant};
use trx_id_map::TrxIdMap;

type Id = u128;

pub struct FsLockerVersionDocStore {
    mount_point: std::path::PathBuf,
    files: tokio::sync::Mutex<HashMap<FileKey, (Instant, DocFile)>>,
    trx_id_map: TrxIdMap,
}

impl DocumentStore for FsLockerVersionDocStore {
    async fn get(&self, name: DocName, id: Id) -> Result<Option<Bytes>> {
        let key = FileKey {
            name,
            id,
            file_type: FileType::Simple,
        };
        let mut files = self.files.lock().await;

        self.make_sure_files(&mut files, &[key], &self.trx_id_map)
            .await?;
        let (_, file) = files.get(&key).unwrap();

        let DocFile::Simple(file) = file else {
            unreachable!()
        };

        let bytes = file.get();

        Ok(if bytes.is_empty() { None } else { Some(bytes) })
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        // 1. Create a WAL (Write-Ahead Log) for each file,
        // 2. Generate a transaction ID,
        // 3. Append new values to the WAL of each file participating in the transaction, including the transaction ID,
        // 4. Finally, add the transaction ID to the success_transaction_id_tree.

        // Then, even if there’s a failure in the middle,
        // - Before applying the WAL, check if the transaction ID is in the success_transaction_id_tree.
        // If it isn’t, skip it. Wouldn’t that work?

        let mut files = self.files.lock().await;

        let trx_id = uuid::Uuid::now_v7().as_u128();

        let keys = extract_file_keys(&transact_items);
        self.make_sure_files(&mut files, &keys, &self.trx_id_map)
            .await?;

        let mut maybe_aborted = MaybeAborted::No;

        keys.iter().for_each(|key| {
            let (_, file) = files.get_mut(key).unwrap();
            file.rollback();
        });

        for (transact_item, key) in transact_items.into_iter().zip(&keys) {
            match transact_item {
                TransactItem::Put { value, .. } => {
                    let (_, file) = files.get_mut(key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };
                    simple_doc_file.put(Bytes::from(value.to_vec()), trx_id)?;
                }
                TransactItem::Create { mut value_fn, .. } => {
                    let (_, file) = files.get_mut(key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };

                    if !simple_doc_file.is_empty() {
                        continue;
                    }
                    let value = value_fn.take().unwrap()()?;
                    simple_doc_file.put(Bytes::from(value), trx_id)?;
                }
                TransactItem::Update { mut update_fn, .. } => {
                    let (_, file) = files.get_mut(key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };

                    let mut vec = simple_doc_file.get().to_vec();
                    if vec.is_empty() {
                        return Err(Error::NotExistsOnUpdate);
                    }

                    match update_fn.take().unwrap()(&mut vec)? {
                        WantUpdate::No => continue,
                        WantUpdate::Yes => simple_doc_file.put(vec.into(), trx_id)?,
                        WantUpdate::Abort { reason } => {
                            maybe_aborted = MaybeAborted::Aborted { reason };
                            break;
                        }
                    }
                }
                TransactItem::Delete { .. } => {
                    let (_, file) = files.get_mut(key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };

                    if simple_doc_file.is_empty() {
                        continue;
                    }
                    simple_doc_file.put(Bytes::new(), trx_id)?;
                }
                TransactItem::InsertRelation { to_id, .. } => {
                    let (_, file) = files.get_mut(key).unwrap();
                    let DocFile::RelationFile(file) = file else {
                        unreachable!()
                    };

                    file.insert(to_id, trx_id)?;
                }
                TransactItem::RemoveRelation { to_id, .. } => {
                    let (_, file) = files.get_mut(key).unwrap();
                    let DocFile::RelationFile(file) = file else {
                        unreachable!()
                    };

                    file.remove(to_id, trx_id)?;
                }
            };
        }

        if let MaybeAborted::No = &maybe_aborted {
            let file_ids = keys.iter().map(|key| key.id).collect::<Vec<_>>();

            self.trx_id_map.insert(trx_id, file_ids).await;
        };

        for key in keys {
            let (_, file) = files.get_mut(&key).unwrap();
            file.commit();
        }

        Ok(maybe_aborted)
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
    async fn close_old_files(&self, except_keys: &[FileKey]) {
        const SOFT_MAX_FILES_LIMIT: usize = 20000;
        let mut files = self.files.lock().await;

        if files.len() <= SOFT_MAX_FILES_LIMIT {
            return;
        }

        let overflow = files.len() - SOFT_MAX_FILES_LIMIT;

        let mut vec: Vec<_> = std::mem::take(&mut *files).into_iter().collect();
        vec.sort_by_key(|(_, (last_access, _))| *last_access);

        let mut survived = vec.split_off(overflow);

        for entry in vec {
            if except_keys.contains(&entry.0) {
                survived.push(entry);
            }
        }

        *files = survived.into_iter().collect();
    }

    async fn make_sure_files(
        &self,
        files: &mut impl DerefMut<Target = HashMap<FileKey, (Instant, DocFile)>>,
        keys: &[FileKey],
        trx_id_map: &TrxIdMap,
    ) -> std::io::Result<()> {
        self.close_old_files(keys).await;

        let now = Instant::now();
        for key in keys {
            if !files.contains_key(key) {
                let doc_file = match key.file_type {
                    FileType::Simple => DocFile::Simple(
                        SimpleDocFile::open(&self.mount_point, key.path(), key.id, trx_id_map)
                            .await?,
                    ),
                    FileType::RelationFile => {
                        DocFile::RelationFile(RelationFile::open(&self.mount_point, key.path())?)
                    }
                };
                files.insert(*key, (Instant::now(), doc_file));
            } else {
                let (last_access, _) = files.get_mut(key).unwrap();
                *last_access = now;
            }
        }

        Ok(())
    }
}

enum DocFile {
    Simple(SimpleDocFile),
    RelationFile(RelationFile),
}

impl DocFile {
    fn commit(&mut self) {
        match self {
            DocFile::Simple(file) => file.commit(),
            DocFile::RelationFile(file) => file.commit(),
        }
    }

    fn rollback(&mut self) {
        match self {
            DocFile::Simple(file) => file.rollback(),
            DocFile::RelationFile(file) => file.rollback(),
        }
    }
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
            TransactItem::InsertRelation { name, id, .. }
            | TransactItem::RemoveRelation { name, id, .. } => FileKey {
                name,
                id: *id,
                file_type: FileType::RelationFile,
            },
        })
        .collect()
}
