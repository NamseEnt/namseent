use crate::*;
use bptree::id_set::*;

pub type DocName = &'static str;

#[allow(async_fn_in_trait)]
pub trait DocumentStore {
    async fn get(&self, name: DocName, id: Id) -> Result<Option<Bytes>>;
    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>>;
    fn query(
        &self,
        from_name: DocName,
        id: Id,
        to_name: DocName,
    ) -> impl futures::Stream<Item = Result<Id>> + 'static + Unpin;
}
