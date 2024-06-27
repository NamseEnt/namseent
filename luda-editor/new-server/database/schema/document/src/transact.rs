use crate::*;
use std::time::Duration;

pub type SerErr = rkyv::ser::serializers::CompositeSerializerError<
    std::convert::Infallible,
    rkyv::ser::serializers::AllocScratchError,
    rkyv::ser::serializers::SharedSerializeMapError,
>;

pub enum TransactItem {
    Put {
        key: String,
        value: Vec<u8>,
        ttl: Option<Duration>,
    },
    Create {
        key: String,
        value: Vec<u8>,
        ttl: Option<Duration>,
    },
    Delete {
        key: String,
    },
}

pub trait Transact {
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr>;
}
impl<T: TryInto<TransactItem, Error = SerErr>> Transact for T {
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        Ok([self.try_into()?])
    }
}
impl<T1: TryInto<TransactItem, Error = SerErr>, T2: TryInto<TransactItem, Error = SerErr>> Transact
    for (T1, T2)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2) = self;
        Ok([t1.try_into()?, t2.try_into()?])
    }
}
impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3) = self;
        Ok([t1.try_into()?, t2.try_into()?, t3.try_into()?])
    }
}
impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4) = self;
        Ok([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
        ])
    }
}
impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
        T5: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4, T5)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4, t5) = self;
        Ok([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
        ])
    }
}
impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
        T5: TryInto<TransactItem, Error = SerErr>,
        T6: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4, T5, T6)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4, t5, t6) = self;
        Ok([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
        ])
    }
}
impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
        T5: TryInto<TransactItem, Error = SerErr>,
        T6: TryInto<TransactItem, Error = SerErr>,
        T7: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4, T5, T6, T7)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4, t5, t6, t7) = self;
        Ok([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
        ])
    }
}

impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
        T5: TryInto<TransactItem, Error = SerErr>,
        T6: TryInto<TransactItem, Error = SerErr>,
        T7: TryInto<TransactItem, Error = SerErr>,
        T8: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4, t5, t6, t7, t8) = self;
        Ok([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
            t8.try_into()?,
        ])
    }
}

impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
        T5: TryInto<TransactItem, Error = SerErr>,
        T6: TryInto<TransactItem, Error = SerErr>,
        T7: TryInto<TransactItem, Error = SerErr>,
        T8: TryInto<TransactItem, Error = SerErr>,
        T9: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4, t5, t6, t7, t8, t9) = self;
        Ok([
            t1.try_into()?,
            t2.try_into()?,
            t3.try_into()?,
            t4.try_into()?,
            t5.try_into()?,
            t6.try_into()?,
            t7.try_into()?,
            t8.try_into()?,
            t9.try_into()?,
        ])
    }
}

impl<
        T1: TryInto<TransactItem, Error = SerErr>,
        T2: TryInto<TransactItem, Error = SerErr>,
        T3: TryInto<TransactItem, Error = SerErr>,
        T4: TryInto<TransactItem, Error = SerErr>,
        T5: TryInto<TransactItem, Error = SerErr>,
        T6: TryInto<TransactItem, Error = SerErr>,
        T7: TryInto<TransactItem, Error = SerErr>,
        T8: TryInto<TransactItem, Error = SerErr>,
        T9: TryInto<TransactItem, Error = SerErr>,
        T10: TryInto<TransactItem, Error = SerErr>,
    > Transact for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn try_into_transact_items(self) -> Result<impl IntoIterator<Item = TransactItem>, SerErr> {
        let (t1, t2, t3, t4, t5, t6, t7, t8, t9, t10) = self;
        Ok([
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
        ])
    }
}
