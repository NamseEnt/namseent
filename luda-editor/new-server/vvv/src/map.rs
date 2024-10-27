//! # VMap
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

pub struct VMap<K: VVV + Eq, V: VVV> {
    source: Bytes,
    source_exclude_indexes: Vec<usize>,
    extra: Vec<(Bytes, Bytes)>,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: VVV + Eq, V: VVV> VMap<K, V> {
    pub fn new() -> Self {
        Self {
            source: Bytes::new(),
            source_exclude_indexes: Vec::new(),
            extra: Vec::new(),
            _phantom: std::marker::PhantomData,
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

        for (key_bytes, value_bytes) in self.extra.iter() {
            let tuple_key = K::from_bytes(key_bytes.clone());
            if tuple_key == key {
                return Some(V::from_bytes(value_bytes.clone()));
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: impl Into<V>) {
        let value: V = value.into();
        if !self.source.is_empty() {
            let index = self
                .iter_with_index()
                .find_map(|(i, k, _)| if k == key { Some(i) } else { None });

            if let Some(index) = index {
                self.source_exclude_indexes.push(index);
                self.extra.push((key.to_bytes(), value.to_bytes()));
                return;
            }
        }

        for (key_bytes, value_bytes) in self.extra.iter_mut() {
            let tuple_key = K::from_bytes(key_bytes.clone());
            if tuple_key == key {
                *value_bytes = value.to_bytes();
                return;
            }
        }

        self.extra.push((key.to_bytes(), value.to_bytes()));
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> + '_ {
        self.iter_with_index().map(|(_, k, v)| (k, v))
    }

    fn iter_with_index(&self) -> impl Iterator<Item = (usize, K, V)> + '_ {
        SourceIter {
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

impl<K: VVV + Eq, V: VVV> Default for VMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: VVV + Eq, V: VVV> VVV for VMap<K, V> {
    fn byte_len(&self) -> usize {
        leb128_byte_len(self.len())
            + self
                .iter()
                .map(|(k, v)| {
                    leb128_byte_len(k.byte_len())
                        + k.byte_len()
                        + leb128_byte_len(v.byte_len())
                        + v.byte_len()
                })
                .sum::<usize>()
            + self
                .extra
                .iter()
                .map(|(k, v)| {
                    leb128_byte_len(k.len()) + k.len() + leb128_byte_len(v.len()) + v.len()
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
            index += Leb128::new(key.len()).write_on_bytes(&mut bytes[index..]);

            bytes
                .get_mut(index..index + key.len())
                .unwrap()
                .copy_from_slice(key);
            index += key.len();

            index += Leb128::new(value.len()).write_on_bytes(&mut bytes[index..]);

            bytes
                .get_mut(index..index + value.len())
                .unwrap()
                .copy_from_slice(value);
            index += value.len();
        }

        index
    }

    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized,
    {
        Self {
            source: bytes,
            extra: Vec::new(),
            source_exclude_indexes: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

struct SourceIter<'a, K, V> {
    /// after tuple count
    bytes: Bytes,
    source_exclude_indexes: &'a [usize],
    index: usize,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: VVV, V: VVV> std::iter::Iterator for SourceIter<'_, K, V> {
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
