//! # List
//!
//! ## Memory Layout
//!
//! - item count: leb128
//! - item:
//!   - bytes length: leb128
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

    pub fn push(&mut self, value: T) {
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

        self.iter_source().last().map(|(index, value)| {
            self.source_exclude_indexes.push(index);
            value
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.iter_source()
            .map(|(_, t)| t)
            .chain(self.extra.iter().cloned())
    }

    fn iter_source(&self) -> impl Iterator<Item = (usize, T)> + '_ {
        ListSourceIter {
            bytes: self.source_without_tuple_count(),
            source_exclude_indexes: &self.source_exclude_indexes,
            index: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    fn source_without_tuple_count(&self) -> Bytes {
        let mut source = self.source.clone();
        let _tuple_count = Leb128::read(&mut source);
        source
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: Eq,
    {
        self.iter().any(|v| &v == value)
    }

    pub fn retain(&mut self, f: impl Fn(&T) -> bool)
    where
        T: Eq,
    {
        self.extra.retain(|v| f(v));

        let indexes = self
            .iter_source()
            .filter(|(_, v)| f(v))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        self.source_exclude_indexes.extend(indexes);
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
            let item_byte_len = item.byte_len();
            byte_len += Leb128::new(item_byte_len).byte_len();
            byte_len += item_byte_len;
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
    index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Nsd> std::iter::Iterator for ListSourceIter<'_, T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        let value_byte_len = Leb128::read(&mut self.bytes);
        println!("value_byte_len: {}", value_byte_len);
        let value_bytes = self.bytes.split_to(value_byte_len);
        println!("value_bytes.len(): {}", value_bytes.len());

        if self.source_exclude_indexes.contains(&self.index) {
            self.index += 1;
            return self.next();
        }

        let value = T::from_bytes(value_bytes);
        self.index += 1;

        Some((self.index - 1, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let bytes = {
            let mut doc = List::<String>::new();

            let option_value = doc.last();
            assert!(option_value.is_none());

            doc.push("abcde".to_string());

            let option_value = doc.last();
            assert!(option_value.is_some());

            let value: &str = &option_value.unwrap();
            assert_eq!(value, "abcde");

            assert!(doc.len() == 1);

            doc.to_bytes()
        };

        assert_eq!(bytes[..], [1, 6, 5, b'a', b'b', b'c', b'd', b'e']);

        let bytes = {
            let doc = List::<String>::from_bytes(bytes);
            println!("doc: {:?}", doc);

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "안녕하세요");

            let mut doc = doc;

            doc.pop();

            assert_eq!(doc.len(), 0);

            let option_value = doc.last();
            assert!(option_value.is_none());
            assert!(doc.is_empty());

            doc.push("Hello".to_string());

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "Hello");

            assert_eq!(doc.len(), 1);

            doc.push("World".to_string());

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "World");

            assert_eq!(doc.len(), 2);

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 13);
    }
}
