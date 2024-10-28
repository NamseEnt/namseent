//! # Map
//!
//! ## Memory Layout
//!
//! - tuple count: leb128
//! - tuple:
//!   - key byte len: leb128
//!   - key: bytes
//!   - value byte len: leb128
//!   - value: bytes

use crate::*;

#[derive(Debug, Clone)]
pub struct Map<K: Nsd + Eq, V: Nsd> {
    source: Bytes,
    source_exclude_indexes: Vec<usize>,
    extra: Vec<(K, V)>,
}

impl<K: Nsd + Eq, V: Nsd> Map<K, V> {
    pub fn new() -> Self {
        Self {
            source: Bytes::new(),
            source_exclude_indexes: Vec::new(),
            extra: Vec::new(),
        }
    }

    pub fn get(&self, key: K) -> Option<V> {
        if !self.source.is_empty() {
            let mut source = self.source.clone();
            let tuple_count = Leb128::read(&mut source);
            for _ in 0..tuple_count {
                let key_byte_len = Leb128::read(&mut source);
                let tuple_key = K::from_bytes(source.split_to(key_byte_len));

                let value_byte_len = Leb128::read(&mut source);
                let value = V::from_bytes(source.split_to(value_byte_len));

                if key == tuple_key {
                    return Some(value);
                }
            }
        }

        for (tuple_key, tuple_value) in self.extra.iter() {
            if tuple_key == &key {
                return Some(tuple_value.clone());
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        if !self.source.is_empty() {
            let index = self
                .iter_with_index()
                .find_map(|(i, k, _)| if k == key { Some(i) } else { None });

            if let Some(index) = index {
                self.source_exclude_indexes.push(index);
                self.extra.push((key, value));
                return;
            }
        }

        for (tuple_key, tuple_value) in self.extra.iter_mut() {
            if tuple_key == &key {
                *tuple_value = value;
                return;
            }
        }

        self.extra.push((key, value));
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> + '_ {
        self.iter_with_index().map(|(_, k, v)| (k, v))
    }

    fn iter_with_index(&self) -> impl Iterator<Item = (usize, K, V)> + '_ {
        MapSourceIter {
            bytes: self.source_without_tuple_count(),
            source_exclude_indexes: &self.source_exclude_indexes,
            index: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let mut source = self.source.clone();
        let tuple_count_in_source = Leb128::read(&mut source);
        tuple_count_in_source + self.extra.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn source_without_tuple_count(&self) -> Bytes {
        let mut source = self.source.clone();
        let _tuple_count = Leb128::read(&mut source);
        source
    }
}

impl<K: Nsd + Eq, V: Nsd> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Nsd + Eq, V: Nsd> Nsd for Map<K, V> {
    fn byte_len(&self) -> usize {
        leb128_byte_len(self.len())
            + self
                .iter()
                .chain(self.extra.iter().cloned())
                .map(|(k, v)| {
                    let key_byte_len = k.byte_len();
                    let value_byte_len = v.byte_len();
                    leb128_byte_len(key_byte_len)
                        + key_byte_len
                        + leb128_byte_len(value_byte_len)
                        + value_byte_len
                })
                .sum::<usize>()
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index = 0;

        index += Leb128::new(self.len()).write_on_bytes(&mut bytes[index..]);

        if !self.source.is_empty() {
            if self.source_exclude_indexes.is_empty() {
                let source_without_tuple_count = self.source_without_tuple_count();
                bytes[index..index + source_without_tuple_count.len()]
                    .copy_from_slice(&source_without_tuple_count);
                index += source_without_tuple_count.len();
            } else {
                // Note: This can be faster if we store the tuple index with the byte range.
                self.iter().for_each(|(k, v)| {
                    index += Leb128::new(k.byte_len()).write_on_bytes(&mut bytes[index..]);
                    index += k.write_on_bytes(&mut bytes[index..]);
                    index += Leb128::new(v.byte_len()).write_on_bytes(&mut bytes[index..]);
                    index += v.write_on_bytes(&mut bytes[index..]);
                });
            }
        }

        for (key, value) in self.extra.iter() {
            let key_bytes = key.to_bytes();
            let value_bytes = value.to_bytes();

            index += Leb128::new(key_bytes.len()).write_on_bytes(&mut bytes[index..]);

            bytes
                .get_mut(index..index + key_bytes.len())
                .unwrap()
                .copy_from_slice(&key_bytes);
            index += key_bytes.len();

            index += Leb128::new(value_bytes.len()).write_on_bytes(&mut bytes[index..]);

            bytes
                .get_mut(index..index + value_bytes.len())
                .unwrap()
                .copy_from_slice(&value_bytes);
            index += value_bytes.len();
        }

        index
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, FromBytesError>
    where
        Self: Sized,
    {
        Ok(Self {
            source: bytes,
            extra: Vec::new(),
            source_exclude_indexes: Vec::new(),
        })
    }
}

struct MapSourceIter<'a, K, V> {
    /// after tuple count
    bytes: Bytes,
    source_exclude_indexes: &'a [usize],
    index: usize,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: Nsd, V: Nsd> std::iter::Iterator for MapSourceIter<'_, K, V> {
    type Item = (usize, K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        let key_byte_len = Leb128::read(&mut self.bytes);
        let key_bytes = self.bytes.split_to(key_byte_len);

        let value_byte_len = Leb128::read(&mut self.bytes);
        let value_bytes = self.bytes.split_to(value_byte_len);

        if self.source_exclude_indexes.contains(&self.index) {
            self.index += 1;
            return self.next();
        }

        let key = K::from_bytes(key_bytes);
        let value = V::from_bytes(value_bytes);
        self.index += 1;

        Some((self.index, key, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum IetfLanguageTag {
        Ko,
        EnUs,
        Ja,
    }

    impl Nsd for IetfLanguageTag {
        fn byte_len(&self) -> usize {
            std::mem::size_of::<Self>()
        }

        fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
            bytes[0] = *self as u8;
            self.byte_len()
        }

        fn from_bytes(bytes: Bytes) -> Self
        where
            Self: Sized,
        {
            unsafe { std::mem::transmute(bytes[0]) }
        }
    }

    #[test]
    fn test_map() {
        #[derive(Debug, Clone)]
        struct SpeakerDoc {
            names: Map<IetfLanguageTag, String>,
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
                let names = Map::from_bytes(bytes);
                Self { names }
            }
        }
        let bytes = {
            let mut doc = SpeakerDoc { names: Map::new() };

            doc.names
                .insert(IetfLanguageTag::Ko, "안녕하세요".to_string());
            let option_value = doc.names.get(IetfLanguageTag::Ko);
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "안녕하세요");

            let option_value = doc.names.get(IetfLanguageTag::Ja);
            assert!(option_value.is_none());

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 19);

        let bytes = {
            let doc = SpeakerDoc::from_bytes(bytes);

            let option_value = doc.names.get(IetfLanguageTag::Ko);
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "안녕하세요");

            let option_value = doc.names.get(IetfLanguageTag::Ja);
            assert!(option_value.is_none());

            let mut doc = doc;

            doc.names.insert(IetfLanguageTag::EnUs, "Hello".to_string());

            let option_value = doc.names.get(IetfLanguageTag::EnUs);
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "Hello");

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 27);
    }
}
