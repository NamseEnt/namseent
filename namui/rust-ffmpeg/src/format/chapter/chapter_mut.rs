use super::Chapter;
use ffi::*;
use format::context::common::Context;
use std::ops::Deref;
use {Dictionary, DictionaryMut, Rational};

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct ChapterMut {
    context: Context,
    index: usize,
    immutable: Chapter,
}

impl ChapterMut {
    pub unsafe fn wrap(context: Context, index: usize) -> ChapterMut {
        ChapterMut {
            context: context.clone(),
            index,

            immutable: Chapter::wrap(context, index),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVChapter {
        *(*self.context.as_mut_ptr()).chapters.add(self.index)
    }
}

impl ChapterMut {
    pub fn set_id(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).id = value as _;
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_start(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).start = value;
        }
    }

    pub fn set_end(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).end = value;
        }
    }

    pub fn set_metadata<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        // dictionary.set() allocates the AVDictionary the first time a key/value is inserted
        // so we want to update the metadata dictionary afterwards
        unsafe {
            let mut dictionary = Dictionary::own(self.metadata().as_mut_ptr());
            dictionary.set(key.as_ref(), value.as_ref());
            (*self.as_mut_ptr()).metadata = dictionary.disown();
        }
    }

    pub fn metadata(&mut self) -> DictionaryMut {
        unsafe { DictionaryMut::wrap((*self.as_mut_ptr()).metadata) }
    }
}

impl Deref for ChapterMut {
    type Target = Chapter;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
