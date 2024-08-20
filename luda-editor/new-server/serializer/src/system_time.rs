// use crate::*;
// use namui_type::*;
// use rkyv::with::{ArchiveWith, UnixTimestamp};

// impl ArchiveWith<SystemTime> for UnixTimestamp {
//     type Archived = ArchivedDuration;
//     type Resolver = ();

//     #[inline]
//     unsafe fn resolve_with(
//         field: &SystemTime,
//         pos: usize,
//         resolver: Self::Resolver,
//         out: *mut Self::Archived,
//     ) {
//         // We already checked the duration during serialize_with
//         let duration = field.duration_since(UNIX_EPOCH).unwrap();
//         Archive::resolve(&duration, pos, resolver, out);
//     }
// }

// impl<S: Fallible + ?Sized> SerializeWith<SystemTime, S> for UnixTimestamp
// where
//     S::Error: From<UnixTimestampError>,
// {
//     fn serialize_with(field: &SystemTime, _: &mut S) -> Result<Self::Resolver, S::Error> {
//         field
//             .duration_since(UNIX_EPOCH)
//             .map_err(|_| UnixTimestampError::TimeBeforeUnixEpoch)?;
//         Ok(())
//     }
// }

// impl<D: Fallible + ?Sized> DeserializeWith<ArchivedDuration, SystemTime, D> for UnixTimestamp {
//     fn deserialize_with(field: &ArchivedDuration, _: &mut D) -> Result<SystemTime, D::Error> {
//         Ok(UNIX_EPOCH + (*field).into())
//     }
// }
