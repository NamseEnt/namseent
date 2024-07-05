use rkyv::{
    ser::{serializers::AllocSerializer, ScratchSpace, Serializer},
    with::UnixTimestampError,
    Fallible, Serialize,
};
use std::error::Error;

pub fn serialize<T>(
    value: &T,
) -> Result<Vec<u8>, <MySerializer<AllocSerializer<1024>> as Fallible>::Error>
where
    T: Serialize<MySerializer<AllocSerializer<1024>>>,
{
    // rkyv::to_bytes(value)
    let mut serializer = MySerializer::default();
    serializer.serialize_value(value)?;
    Ok(serializer
        .into_inner()
        .into_serializer()
        .into_inner()
        .to_vec())
}

pub struct MySerializer<S> {
    inner: S,
}

impl<S> MySerializer<S> {
    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S: Fallible> Fallible for MySerializer<S> {
    type Error = MySerializerError<S::Error>;
}

// Our Serializer impl just forwards everything down to the inner serializer.
impl<S: Serializer> Serializer for MySerializer<S> {
    #[inline]
    fn pos(&self) -> usize {
        self.inner.pos()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.write(bytes).map_err(MySerializerError::Inner)
    }
}

impl<S: ScratchSpace> ScratchSpace for MySerializer<S> {
    unsafe fn push_scratch(
        &mut self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, Self::Error> {
        self.inner
            .push_scratch(layout)
            .map_err(MySerializerError::Inner)
    }

    unsafe fn pop_scratch(
        &mut self,
        ptr: std::ptr::NonNull<u8>,
        layout: std::alloc::Layout,
    ) -> Result<(), Self::Error> {
        self.inner
            .pop_scratch(ptr, layout)
            .map_err(MySerializerError::Inner)
    }
}

impl<S: Default> Default for MySerializer<S> {
    fn default() -> Self {
        Self {
            inner: S::default(),
        }
    }
}

#[derive(Debug)]
pub enum MySerializerError<E> {
    Inner(E),
    TimeBeforeUnixEpoch,
}

impl<E> From<UnixTimestampError> for MySerializerError<E> {
    fn from(_: UnixTimestampError) -> Self {
        Self::TimeBeforeUnixEpoch
    }
}

impl<E: Error> std::fmt::Display for MySerializerError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MySerializerError::Inner(err) => write!(f, "{}", err),
            MySerializerError::TimeBeforeUnixEpoch => write!(f, "Time before Unix epoch"),
        }
    }
}

impl<E: Error> Error for MySerializerError<E> {}
