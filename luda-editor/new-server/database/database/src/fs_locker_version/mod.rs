mod simple_doc_file;

use crate::*;
use bptree::id_set::{Id, IdSet};
use document_store::*;
use simple_doc_file::*;
use std::{
    collections::HashMap,
    ops::DerefMut,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

pub struct FsLockerVersionDocStore {
    mount_point: std::path::PathBuf,
    files: tokio::sync::Mutex<HashMap<FileKey, (Instant, DocFile)>>,
    trx_id_tree: IdSet,
    next_trx_id: AtomicU64,
}

impl DocumentStore for FsLockerVersionDocStore {
    async fn get(&self, name: DocName, id: Id) -> Result<Option<Bytes>> {
        let key = FileKey {
            name,
            id,
            file_type: FileType::Simple,
        };
        let mut files = self.files.lock().await;

        self.make_sure_files(&mut files, &[&key]).await?;
        let (_, file) = files.get(&key).unwrap();

        let DocFile::Simple(file) = file else {
            unreachable!()
        };

        let bytes = file.get();

        Ok(if bytes.is_empty() { None } else { Some(bytes) })
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        // 1. Create a WAL (Write-Ahead Log) for each file,
        // 2. Generate a transaction ID,
        // 3. Append new values to the WAL of each file participating in the transaction, including the transaction ID,
        // 4. Finally, add the transaction ID to the success_transaction_id_tree.

        // Then, even if there’s a failure in the middle,
        // - Before applying the WAL, check if the transaction ID is in the success_transaction_id_tree.
        // If it isn’t, skip it. Wouldn’t that work?

        let mut files = self.files.lock().await;

        let trx_id = self.next_trx_id.fetch_add(1, Ordering::Relaxed);

        let keys = vec![];
        self.make_sure_files(&mut files, &keys).await?;

        let mut maybe_aborted = MaybeAborted::No;

        for transact_item in transact_items {
            match *transact_item {
                TransactItem::Put {
                    name,
                    id,
                    ref value,
                } => {
                    let key = FileKey {
                        name,
                        id,
                        file_type: FileType::Simple,
                    };
                    let (_, file) = files.get_mut(&key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };
                    simple_doc_file.put(Bytes::from(value.to_vec()), trx_id)?;
                }
                TransactItem::Create {
                    name,
                    id,
                    ref mut value_fn,
                } => {
                    let key = FileKey {
                        name,
                        id,
                        file_type: FileType::Simple,
                    };
                    let (_, file) = files.get_mut(&key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };

                    if !simple_doc_file.is_empty() {
                        continue;
                    }
                    let value = value_fn.take().unwrap()()?;
                    simple_doc_file.put(Bytes::from(value), trx_id)?;
                }
                TransactItem::Update {
                    name,
                    id,
                    ref mut update_fn,
                } => {
                    let key = FileKey {
                        name,
                        id,
                        file_type: FileType::Simple,
                    };
                    let (_, file) = files.get_mut(&key).unwrap();
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
                TransactItem::Delete { name, id } => {
                    let key = FileKey {
                        name,
                        id,
                        file_type: FileType::Simple,
                    };
                    let (_, file) = files.get_mut(&key).unwrap();
                    let DocFile::Simple(simple_doc_file) = file else {
                        unreachable!()
                    };

                    if simple_doc_file.is_empty() {
                        continue;
                    }
                    simple_doc_file.put(Bytes::new(), trx_id)?;
                }
                TransactItem::InsertRelation { name, id, to_id } => {
                    let key = FileKey {
                        name,
                        id,
                        file_type: FileType::IdSet,
                    };
                    let (_, file) = files.get_mut(&key).unwrap();
                    let DocFile::IdSet(id_set) = file else {
                        unreachable!()
                    };

                    if let Err(err) = id_set.insert(to_id).await {
                        match err {
                            bptree::id_set::Error::Broken => todo!(),
                            bptree::id_set::Error::Temporary => todo!(),
                        }
                    }
                }
                TransactItem::RemoveRelation { name, id } => {}
            };
        }

        if let MaybeAborted::No = &maybe_aborted {
            if let Err(err) = self.trx_id_tree.insert(trx_id as u128).await {
                match err {
                    bptree::id_set::Error::Broken => todo!(),
                    bptree::id_set::Error::Temporary => todo!(),
                }
            }
        };

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
    async fn close_old_files(&self, except_keys: &[&FileKey]) {
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
            if except_keys.contains(&&entry.0) {
                survived.push(entry);
            } else {
                entry.1 .1.close().await;
            }
        }

        *files = survived.into_iter().collect();
    }

    async fn make_sure_files(
        &self,
        files: &mut impl DerefMut<Target = HashMap<FileKey, (Instant, DocFile)>>,
        keys: &[&FileKey],
    ) -> std::io::Result<()> {
        self.close_old_files(keys);

        let now = Instant::now();
        for &&key in keys {
            if !files.contains_key(&key) {
                let doc_file = match key.file_type {
                    FileType::Simple => {
                        DocFile::Simple(SimpleDocFile::open(&self.mount_point, key.path())?)
                    }
                    FileType::IdSet => {
                        DocFile::IdSet(IdSet::new(self.mount_point.join(key.path()), 8).await?)
                    }
                };
                files.insert(key, (Instant::now(), doc_file));
            } else {
                let (last_access, _) = files.get_mut(&key).unwrap();
                *last_access = now;
            }
        }

        Ok(())
    }
}

enum DocFile {
    Simple(SimpleDocFile),
    IdSet(IdSet),
}

impl DocFile {
    async fn close(self) {
        match self {
            DocFile::Simple(_) => {}
            DocFile::IdSet(file) => file.try_close().await.unwrap(),
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
    IdSet,
}
