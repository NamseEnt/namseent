mod simple_doc_file;

use crate::*;
use document_store::*;
use futures::TryStreamExt;
use simple_doc_file::*;
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

fn key(name: &str, id: bptree::id_set::Id) -> String {
    format!("{name}/{id}")
}

struct NfsLockerVersionDocStore {
    mount_point: std::path::PathBuf,
    files: tokio::sync::Mutex<HashMap<String, (Instant, SimpleDocFile)>>,
    trx_id_tree: bptree::id_set::IdSet,
    next_trx_id: AtomicU64,
}

impl DocumentStore for NfsLockerVersionDocStore {
    async fn get(&self, name: DocName, id: bptree::id_set::Id) -> Result<Option<Bytes>> {
        let key = key(name, id);
        let mut files = self.files.lock().await;

        self.make_sure_files(&mut files, &[&key])?;
        let (_, file) = files.get(&key).unwrap();

        let bytes = file.get();

        Ok(if bytes.is_empty() { None } else { Some(bytes) })
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        let mut files = self.files.lock().await;

        let trx_id = self.next_trx_id.fetch_add(1, Ordering::Relaxed);

        let keys = vec![];
        self.make_sure_files(&mut files, &keys)?;

        let maybe_aborted = MaybeAborted::No;

        for transact_item in transact_items {
            let key = String::new(); // TODO
            let (_, file) = files.get_mut(&key).unwrap();

            let new_bytes = match transact_item {
                TransactItem::Put {
                    name,
                    pk,
                    sk,
                    value,
                    ttl,
                } => todo!(),
                TransactItem::Create {
                    name,
                    pk,
                    sk,
                    value_fn,
                    ttl,
                } => todo!(),
                TransactItem::Update {
                    name,
                    pk,
                    sk,
                    update_fn,
                } => todo!(),
                TransactItem::Delete { name, pk, sk } => todo!(),
            };

            file.put(new_bytes, trx_id)?;
        }

        self.trx_id_tree.insert(trx_id as u128).await?;

        // 1. 파일별로 wal을 만들고,
        // 2. transaction id를 하나 만들고,
        // 3. transaction에 참가하는 파일들의 wal에 새 값을 append하는데, transaction id도 같이 적고
        // 4. 마지막으로 transaction id를 success_transaction_id_tree에 추가한다.

        // 그러면 중간에 실패해도,
        // - wal를 적용하기 전에 success_transaction_id_tree에 transacion id가 있는지 확인 후 없으면 패스하면 되니까 되지 않을까?

        Ok(maybe_aborted)
    }

    fn query(
        &self,
        from_name: document_store::DocName,
        id: bptree::id_set::Id,
        to_name: document_store::DocName,
    ) -> impl futures::Stream<Item = Result<bptree::id_set::Id>> + 'static + Unpin {
        bptree::id_set::IdSet::stream(todo!()).map_err(|e| Error::from(e))
    }
}

impl NfsLockerVersionDocStore {
    async fn close_old_files(&self, except_keys: &[&String]) {
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
            }
        }

        *files = survived.into_iter().collect();
    }

    fn make_sure_files(
        &self,
        files: &mut impl DerefMut<Target = HashMap<String, (Instant, SimpleDocFile)>>,
        keys: &[&String],
    ) -> std::io::Result<()> {
        self.close_old_files(keys);

        let now = Instant::now();
        for &key in keys {
            if !files.contains_key(key) {
                files.insert(
                    key.to_string(),
                    (
                        Instant::now(),
                        SimpleDocFile::open(&self.mount_point, key.to_string())?,
                    ),
                );
            }

            let (last_access, _) = files.get_mut(key).unwrap();
            *last_access = now;
        }

        Ok(())
    }
}
