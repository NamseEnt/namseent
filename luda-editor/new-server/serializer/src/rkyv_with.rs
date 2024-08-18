use rkyv::{
    ser::Serializer,
    string::{ArchivedString, StringResolver},
    with::{ArchiveWith, SerializeWith},
};

pub struct StrAsString;

impl<'a> ArchiveWith<&'a str> for StrAsString {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    unsafe fn resolve_with(
        field: &&'a str,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedString::resolve_from_str(field, pos, resolver, out);
    }
}

impl<'a, S: Serializer + ?Sized> SerializeWith<&'a str, S> for StrAsString {
    #[inline]
    fn serialize_with(field: &&'a str, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(field, serializer)
    }
}
