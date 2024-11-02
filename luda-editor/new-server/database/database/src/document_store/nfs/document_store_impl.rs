use super::NfsV4DocStore;
use crate::*;

impl DocumentStore for NfsV4DocStore {
    async fn get(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<document::ValueBuffer>> {
        todo!()
    }

    async fn get_with_expiration(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<(document::ValueBuffer, Option<std::time::SystemTime>)>> {
        todo!()
    }

    async fn query(&self, name: &'static str, pk: &[u8]) -> Result<Vec<document::ValueBuffer>> {
        todo!()
    }

    async fn put(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value: &impl AsRef<[u8]>,
        ttl: Option<std::time::Duration>,
    ) -> Result<()> {
        todo!()
    }

    async fn delete(&self, name: &'static str, pk: &[u8], sk: Option<&[u8]>) -> Result<()> {
        todo!()
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<std::time::Duration>,
    ) -> Result<()> {
        todo!()
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut document::TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        todo!()
    }
}
