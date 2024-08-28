use rkyv::{
    ser::{ScratchSpace, Serializer},
    string::{ArchivedString, StringResolver},
    vec::ArchivedVec,
    with::{ArchiveWith, SerializeWith},
    Archive, Archived, Fallible, Resolver, Serialize,
};
use std::ops::Deref;

pub struct StrVec;

impl<Str: Deref<Target = str>> ArchiveWith<&[Str]> for StrVec {
    type Archived = Archived<Vec<String>>;
    type Resolver = Resolver<Vec<String>>;

    unsafe fn resolve_with(
        slice: &&[Str],
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(slice.len(), pos, resolver, out);
    }
}

impl<Str, S> SerializeWith<&[Str], S> for StrVec
where
    Str: Deref<Target = str>,
    S: Fallible + Serializer + ScratchSpace,
{
    fn serialize_with(
        slice: &&[Str],
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        let ptr = slice.as_ptr().cast::<DerefWrap<Str>>();
        let len = slice.len();

        // SAFETY: `DerefWrap` is a transparent wrapper around `Str`.
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

        ArchivedVec::serialize_from_slice(slice, serializer)
    }
}

#[repr(transparent)]
struct DerefWrap<T: Deref<Target = str>>(T);

impl<T: Deref<Target = str>> Archive for DerefWrap<T> {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedString::resolve_from_str(&self.0, pos, resolver, out);
    }
}

impl<T, S> Serialize<S> for DerefWrap<T>
where
    T: Deref<Target = str>,
    S: Fallible + Serializer,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        ArchivedString::serialize_from_str(&self.0, serializer)
    }
}
