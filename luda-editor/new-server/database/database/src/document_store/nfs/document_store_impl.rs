use super::{NfsV4DocStore, TransactionWrite};
use crate::*;
use futures::TryStreamExt;

fn key(name: &str, id: bptree::id_set::Id) -> String {
    format!("{name}/{id}")
}

impl DocumentStore for NfsV4DocStore {
    async fn get(
        &self,
        name: document_store::DocName,
        id: bptree::id_set::Id,
    ) -> Result<Option<Bytes>> {
        Ok(self.read(key(name, id)).await?)
    }

    async fn put(
        &self,
        name: document_store::DocName,
        id: bptree::id_set::Id,
        value: &impl AsRef<[u8]>,
    ) -> Result<()> {
        Ok(self
            .write(vec![TransactionWrite::Put {
                key: key(name, id),
                value: Bytes::from(value.as_ref().to_vec()),
            }])
            .await?)
    }

    async fn delete(&self, name: document_store::DocName, id: bptree::id_set::Id) -> Result<()> {
        Ok(self
            .write(vec![TransactionWrite::Delete { key: key(name, id) }])
            .await?)
    }

    async fn create<'a, AsBytes: AsRef<[u8]>>(
        &'a self,
        name: document_store::DocName,
        id: bptree::id_set::Id,
        value_fn: impl FnOnce() -> AsBytes + Send + 'a,
    ) -> Result<()> {
        Ok(self
            .write(vec![TransactionWrite::Update {
                key: key(name, id),
                data_fn: Box::new(|bytes| {
                    if bytes.is_some() {
                        return Err(Box::new(Error::AlreadyExistsOnCreate));
                    }
                    Ok(Some(value_fn().as_ref().to_vec().into()))
                }),
            }])
            .await?)
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        todo!()
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
