use crate::*;
use bptree::id_set::*;

pub type DocName = &'static str;

#[allow(async_fn_in_trait)]
pub trait DocumentStore {
    async fn get(&self, name: DocName, id: Id) -> Result<Option<Bytes>>;
    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>>;
}
