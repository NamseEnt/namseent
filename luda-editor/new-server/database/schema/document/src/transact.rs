pub use arrayvec::ArrayVec;
use serializer::*;
use std::{borrow::Cow, time::Duration};

pub enum TransactItem<'a, AbortReason> {
    Put {
        name: &'static str,
        pk: Cow<'a, [u8]>,
        sk: Option<Cow<'a, [u8]>>,
        value: Vec<u8>,
        ttl: Option<Duration>,
    },
    Create {
        name: &'static str,
        pk: Cow<'a, [u8]>,
        sk: Option<Cow<'a, [u8]>>,
        value_fn: Option<Box<dyn 'a + Send + FnOnce() -> Result<Vec<u8>>>>,
        ttl: Option<Duration>,
    },
    Update {
        name: &'static str,
        pk: Cow<'a, [u8]>,
        sk: Option<Cow<'a, [u8]>>,
        update_fn: UpdateFn<'a, AbortReason>,
    },
    Delete {
        name: &'static str,
        pk: Cow<'a, [u8]>,
        sk: Option<Cow<'a, [u8]>>,
    },
}

impl<'a, AbortReason> std::fmt::Debug for TransactItem<'a, AbortReason> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactItem::Put {
                name,
                pk,
                sk,
                value,
                ttl,
            } => f
                .debug_struct("Put")
                .field("name", name)
                .field("pk", pk)
                .field("sk", sk)
                .field("value", value)
                .field("ttl", ttl)
                .finish(),
            TransactItem::Create {
                name,
                pk,
                sk,
                value_fn: _,
                ttl,
            } => f
                .debug_struct("Create")
                .field("name", name)
                .field("pk", pk)
                .field("sk", sk)
                .field("value_fn", &"...")
                .field("ttl", ttl)
                .finish(),
            TransactItem::Update {
                name,
                pk,
                sk,
                update_fn: _,
            } => f
                .debug_struct("Update")
                .field("name", name)
                .field("pk", pk)
                .field("sk", sk)
                .field("update_fn", &"...")
                .finish(),
            TransactItem::Delete { name, pk, sk } => f
                .debug_struct("Delete")
                .field("name", name)
                .field("pk", pk)
                .field("sk", sk)
                .finish(),
        }
    }
}

type UpdateFn<'a, AbortReason> =
    Option<Box<dyn 'a + Send + FnOnce(&mut Vec<u8>) -> Result<WantUpdate<AbortReason>>>>;

pub enum WantUpdate<AbortReason> {
    /// No changes but keeps the transaction
    No,
    Yes,
    Abort {
        reason: AbortReason,
    },
}
// impl<'a> AsRef<TransactItem<'a>> for TransactItem<'a> {
//     fn as_ref(&self) -> &TransactItem<'a> {
//         self
//     }
// }

pub type TransactItems<'a, AbortReason> = ArrayVec<TransactItem<'a, AbortReason>, 10>;

pub trait Transact<'a, AbortReason> {
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>>
    where
        Self: 'a;
}
impl<'a, AbortReason, T: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>>
    Transact<'a, AbortReason> for T
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        Ok(ArrayVec::from_iter([self.try_into()?]))
    }
}
impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2) = self;
        Ok(ArrayVec::from_iter([t1.try_into()?, t2.try_into()?]))
    }
}
impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
        ]))
    }
}
impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
        ]))
    }
}
impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T5: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4, T5)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4, t5) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
        ]))
    }
}
impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T5: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T6: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4, T5, T6)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4, t5, t6) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
        ]))
    }
}
impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T5: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T6: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T7: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4, T5, T6, T7)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4, t5, t6, t7) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
        ]))
    }
}

impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T5: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T6: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T7: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T8: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4, t5, t6, t7, t8) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
            t8.try_into()?,
        ]))
    }
}

impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T5: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T6: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T7: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T8: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T9: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4, t5, t6, t7, t8, t9) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
            t8.try_into()?,
            t9.try_into()?,
        ]))
    }
}

impl<
        'a,
        AbortReason,
        T1: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T2: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T3: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T4: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T5: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T6: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T7: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T8: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T9: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
        T10: TryInto<TransactItem<'a, AbortReason>, Error = SerErr>,
    > Transact<'a, AbortReason> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn try_into_transact_items(self) -> Result<ArrayVec<TransactItem<'a, AbortReason>, 10>> {
        let (t1, t2, t3, t4, t5, t6, t7, t8, t9, t10) = self;
        Ok(ArrayVec::from_iter([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
            t8.try_into()?,
            t9.try_into()?,
            t10.try_into()?,
        ]))
    }
}
