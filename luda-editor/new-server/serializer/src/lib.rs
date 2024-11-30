// pub mod rkyv_with;

use rkyv::{
    //     de::deserializers::{SharedDeserializeMap, SharedDeserializeMapError},
    //     ser::{serializers::AllocSerializer, ScratchSpace, Serializer},
    //     with::UnixTimestampError,
    Archive,
    Deserialize,
    Serialize,
};
use std::error::Error;

// pub type SerErr = MySerializerError<
//     rkyv::ser::serializers::CompositeSerializerError<
//         std::convert::Infallible,
//         rkyv::ser::serializers::AllocScratchError,
//         rkyv::ser::serializers::SharedSerializeMapError,
//     >,
// >;

pub type SerErr = rkyv::rancor::Error;
pub type Result<T> = std::result::Result<T, SerErr>;
// pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
// where
//     T: Serialize<MySerializer<AllocSerializer<1024>>>,
// {
//     let mut serializer = MySerializer::default();
//     serializer.serialize_value(value)?;
//     Ok(serializer
//         .into_inner()
//         .into_serializer()
//         .into_inner()
//         .to_vec())
// }

// pub fn deserialize<T>(bytes: &[u8]) -> Result<T>
// where
//     T: Archive,
//     T::Archived: Deserialize<T, rkyv::rancor::Error>,
// {
//     Ok(unsafe { rkyv::from_bytes_unchecked(bytes)? })
// }

// pub trait DeserializeInfallible<T>: rkyv::Deserialize<T, rkyv::Infallible>
// where
//     T: rkyv::Archive,
// {
//     fn deserialize(&self) -> T {
//         rkyv::Deserialize::deserialize(self, &mut rkyv::Infallible).unwrap()
//     }
// }

// impl<T> DeserializeInfallible<T> for rkyv::Archived<T>
// where
//     T: rkyv::Archive,
//     <T as Archive>::Archived: Deserialize<T, rkyv::Infallible>,
// {
// }

// pub struct MySerializer<S> {
//     inner: S,
// }

// impl<S> MySerializer<S> {
//     pub fn into_inner(self) -> S {
//         self.inner
//     }
// }

// impl<S: Fallible> Fallible for MySerializer<S> {
//     type Error = MySerializerError<S::Error>;
// }

// // Our Serializer impl just forwards everything down to the inner serializer.
// impl<S: Serializer> Serializer for MySerializer<S> {
//     #[inline]
//     fn pos(&self) -> usize {
//         self.inner.pos()
//     }

//     #[inline]
//     fn write(&mut self, bytes: &[u8]) -> std::result::Result<(), Self::Error> {
//         self.inner
//             .write(bytes)
//             .map_err(MySerializerError::Serialize)
//     }
// }

// impl<S: ScratchSpace> ScratchSpace for MySerializer<S> {
//     unsafe fn push_scratch(
//         &mut self,
//         layout: std::alloc::Layout,
//     ) -> std::result::Result<std::ptr::NonNull<[u8]>, Self::Error> {
//         self.inner
//             .push_scratch(layout)
//             .map_err(MySerializerError::Serialize)
//     }

//     unsafe fn pop_scratch(
//         &mut self,
//         ptr: std::ptr::NonNull<u8>,
//         layout: std::alloc::Layout,
//     ) -> std::result::Result<(), Self::Error> {
//         self.inner
//             .pop_scratch(ptr, layout)
//             .map_err(MySerializerError::Serialize)
//     }
// }

// impl<S: Default> Default for MySerializer<S> {
//     fn default() -> Self {
//         Self {
//             inner: S::default(),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum MySerializerError<E> {
//     Serialize(E),
//     Deserialize(SharedDeserializeMapError),
//     TimeBeforeUnixEpoch,
// }

// impl<E> From<UnixTimestampError> for MySerializerError<E> {
//     fn from(_: UnixTimestampError) -> Self {
//         Self::TimeBeforeUnixEpoch
//     }
// }

// impl<E> From<SharedDeserializeMapError> for MySerializerError<E> {
//     fn from(err: SharedDeserializeMapError) -> Self {
//         Self::Deserialize(err)
//     }
// }

// impl<E: Error> std::fmt::Display for MySerializerError<E> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             MySerializerError::Serialize(err) => write!(f, "{}", err),
//             MySerializerError::Deserialize(err) => write!(f, "{}", err),
//             MySerializerError::TimeBeforeUnixEpoch => write!(f, "Time before Unix epoch"),
//         }
//     }
// }

// impl<E: Error> Error for MySerializerError<E> {}
