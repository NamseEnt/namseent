//! # List
//!
//! ## Memory Layout
//!
//! - item count: leb128
//! - item:
//!   - item byte len: leb128
//!   - item: bytes

use crate::*;

#[derive(Debug, Clone)]
pub struct List<T: Nsd> {
    source: Bytes,
    source_exclude_indexes: Vec<usize>,
    extra: Vec<T>,
}
impl<T: Nsd> List<T> {
    pub fn new() -> Self {
        Self {
            source: Bytes::new(),
            source_exclude_indexes: Vec::new(),
            extra: Vec::new(),
        }
    }

    pub fn push(&mut self, value: impl Into<T>) {
        let value: T = value.into();
        self.extra.push(value);
    }

    pub fn last(&self) -> Option<T> {
        self.iter().last()
    }

    pub fn len(&self) -> usize {
        let mut len = self.extra.len();
        if !self.source.is_empty() {
            let mut source = self.source.clone();
            let item_count = Leb128::read(&mut source);
            len += item_count;
            len -= self.source_exclude_indexes.len();
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(value) = self.extra.pop() {
            return Some(value);
        }

        if self.source.is_empty() {
            return None;
        }

        self.iter_with_index().last().map(|(index, value)| {
            self.source_exclude_indexes.push(index);
            value
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.iter_with_index().map(|(_, t)| t)
    }

    fn iter_with_index(&self) -> impl Iterator<Item = (usize, T)> + '_ {
        ListSourceIter {
            bytes: self.source_without_tuple_count(),
            source_exclude_indexes: &self.source_exclude_indexes,
            index: ListSourceIterIndex::Source(0),
            extra: &self.extra,
            _phantom: std::marker::PhantomData,
        }
    }

    fn source_without_tuple_count(&self) -> Bytes {
        let mut source = self.source.clone();
        let _tuple_count = Leb128::read(&mut source);
        source
    }
}

impl<T: Nsd> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Nsd> Nsd for List<T> {
    fn byte_len(&self) -> usize {
        let mut byte_len = Leb128::new(self.len()).byte_len();

        self.iter().for_each(|item| {
            byte_len += Leb128::new(item.byte_len()).byte_len();
            byte_len += item.byte_len();
        });

        byte_len
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index = 0;
        index += Leb128::new(self.len()).write_on_bytes(bytes.get_mut(index..).unwrap());
        for item in self.extra.iter() {
            index += Leb128::new(item.byte_len()).write_on_bytes(bytes.get_mut(index..).unwrap());
            index += item.write_on_bytes(bytes.get_mut(index..).unwrap());
        }
        index
    }

    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized,
    {
        Self {
            source: bytes,
            source_exclude_indexes: Vec::new(),
            extra: Vec::new(),
        }
    }
}

struct ListSourceIter<'a, T> {
    /// after tuple count
    bytes: Bytes,
    source_exclude_indexes: &'a [usize],
    extra: &'a [T],
    index: ListSourceIterIndex,
    _phantom: std::marker::PhantomData<T>,
}

enum ListSourceIterIndex {
    Source(usize),
    Extra(usize),
}

impl<T: Nsd> std::iter::Iterator for ListSourceIter<'_, T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            ListSourceIterIndex::Source(index) => {
                if self.bytes.is_empty() {
                    self.index = ListSourceIterIndex::Extra(0);
                    return self.next();
                }

                let value_byte_len = Leb128::read(&mut self.bytes);
                let value_bytes = self.bytes.split_to(value_byte_len);

                if self.source_exclude_indexes.contains(&index) {
                    self.index = ListSourceIterIndex::Source(index + 1);
                    return self.next();
                }

                let value = T::from_bytes(value_bytes);
                self.index = ListSourceIterIndex::Source(index + 1);

                Some((index, value))
            }
            ListSourceIterIndex::Extra(index) => {
                if index >= self.extra.len() {
                    return None;
                }

                let value = &self.extra[index];
                self.index = ListSourceIterIndex::Extra(index + 1);

                Some((index, value.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        #[derive(Debug, Clone)]
        struct SpeakerDoc {
            names: List<VStr>,
        }
        impl Nsd for SpeakerDoc {
            fn byte_len(&self) -> usize {
                self.names.byte_len()
            }

            fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
                let mut index = 0;
                index += self.names.write_on_bytes(bytes.get_mut(index..).unwrap());
                index
            }

            fn from_bytes(bytes: Bytes) -> Self
            where
                Self: Sized,
            {
                let names = List::from_bytes(bytes);
                Self { names }
            }
        }
        let bytes = {
            let mut doc = SpeakerDoc { names: List::new() };

            let option_value = doc.names.last();
            assert!(option_value.is_none());

            doc.names.push("안녕하세요");

            let option_value = doc.names.last();
            assert!(option_value.is_some());

            let value: &str = &option_value.unwrap();
            assert_eq!(value, "안녕하세요");

            assert!(doc.names.len() == 1);

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 17);

        let bytes = {
            let doc = SpeakerDoc::from_bytes(bytes);

            let option_value = doc.names.last();
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "안녕하세요");

            let mut doc = doc;

            doc.names.pop();

            assert_eq!(doc.names.len(), 0);

            let option_value = doc.names.last();
            assert!(option_value.is_none());
            assert!(doc.names.is_empty());

            doc.names.push("Hello");

            let option_value = doc.names.last();
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "Hello");

            assert_eq!(doc.names.len(), 1);

            doc.names.push("World");

            let option_value = doc.names.last();
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "World");

            assert_eq!(doc.names.len(), 2);

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 13);
    }
}
