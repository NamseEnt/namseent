//! # List
//!
//! ## Memory Layout
//!
//! - item count: leb128
//! - item:
//!   - item byte len: leb128
//!   - item: bytes

use crate::*;
// use std::ops::{Deref, DerefMut};

// #[derive(Debug, Clone, Default)]
// pub struct List<T: Nsd> {
//     inner: Vec<T>,
// }

// impl<T: Nsd> List<T> {
//     pub fn new() -> Self {
//         Self { inner: Vec::new() }
//     }

//     pub fn with_capacity(capacity: usize) -> Self {
//         Self {
//             inner: Vec::with_capacity(capacity),
//         }
//     }
// }

// impl<T: Nsd> Deref for List<T> {
//     type Target = Vec<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl<T: Nsd> DerefMut for List<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

// impl<T: Nsd> From<Vec<T>> for List<T> {
//     fn from(inner: Vec<T>) -> Self {
//         Self { inner }
//     }
// }

// impl<T: Nsd> From<List<T>> for Vec<T> {
//     fn from(list: List<T>) -> Self {
//         list.inner
//     }
// }

// impl<T: Nsd> Nsd for List<T> {
//     fn byte_len(&self) -> usize {
//         let mut byte_len = Leb128::new(self.len()).byte_len();

//         self.iter().for_each(|item| {
//             let item_byte_len = item.byte_len();
//             byte_len += item_byte_len;
//         });

//         byte_len
//     }

//     fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
//         let mut index = 0;
//         index += Leb128::new(self.len()).write_on_bytes(bytes.get_mut(index..).unwrap());
//         for item in self.extra.iter() {
//             index += item.write_on_bytes(bytes.get_mut(index..).unwrap());
//         }
//         index
//     }

//     fn from_bytes(mut bytes: Bytes) -> Result<Self, FromBytesError>
//     where
//         Self: Sized,
//     {
//         let source_count = Leb128::read(&mut bytes);
//         let mut inner = Vec::with_capacity(source_count);

//         for _ in 0..source_count {
//             let value_byte_len = Leb128::read(&mut bytes);
//             let value_bytes = bytes.split_to(value_byte_len);
//             let value = T::from_bytes(value_bytes)?;
//             inner.push(value);
//         }

//         Ok(Self { inner })
//     }
// }

// struct ListSourceIter<'a, T> {
//     /// after tuple count
//     bytes: Bytes,
//     source_exclude_indexes: &'a [usize],
//     index: usize,
//     _phantom: std::marker::PhantomData<T>,
// }

// impl<T: Nsd> std::iter::Iterator for ListSourceIter<'_, T> {
//     type Item = (usize, T);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.bytes.is_empty() {
//             return None;
//         }

//         let value_byte_len = Leb128::read(&mut self.bytes);
//         println!("value_byte_len: {}", value_byte_len);
//         let value_bytes = self.bytes.split_to(value_byte_len);
//         println!("value_bytes.len(): {}", value_bytes.len());

//         if self.source_exclude_indexes.contains(&self.index) {
//             self.index += 1;
//             return self.next();
//         }

//         let value = T::from_bytes(value_bytes);
//         self.index += 1;

//         Some((self.index - 1, value))
//     }
// }

impl<T: Nsd> Nsd for Vec<T> {
    fn byte_len(&self) -> usize {
        let mut byte_len = self.len().byte_len();

        self.iter().for_each(|item| {
            let item_byte_len = item.byte_len();
            byte_len += item_byte_len;
        });

        byte_len
    }

    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()> {
        let mut dest_writer = DestWriter::new(dest);
        dest_writer.write(&self.len())?;
        for item in self.iter() {
            dest_writer.write(&item.byte_len())?;
            dest_writer.write(item)?;
        }
        Ok(())
    }

    fn from_bytes(mut bytes: Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let source_count = leb128::read(&mut bytes)?;
        let mut vec = Vec::with_capacity(source_count);

        for _ in 0..source_count {
            let value_byte_len = leb128::read(&mut bytes)?;
            let value_bytes = bytes.split_to(value_byte_len);
            let value = T::from_bytes(value_bytes)?;
            vec.push(value);
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let bytes = {
            let mut doc = Vec::<String>::new();

            let option_value = doc.last();
            assert!(option_value.is_none());

            doc.push("abcde".to_string());

            let option_value = doc.last();
            assert!(option_value.is_some());

            let value: &str = option_value.unwrap();
            assert_eq!(value, "abcde");

            assert!(doc.len() == 1);

            doc.to_bytes()
        };

        assert_eq!(bytes[..], [1, 5, b'a', b'b', b'c', b'd', b'e']);

        let bytes = {
            let doc = Vec::<String>::from_bytes(bytes).unwrap();
            println!("doc: {:?}", doc);

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = option_value.unwrap();
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
            let value: &str = option_value.unwrap();
            assert_eq!(value, "Hello");

            assert_eq!(doc.len(), 1);

            doc.push("World".to_string());

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = option_value.unwrap();
            assert_eq!(value, "World");

            assert_eq!(doc.len(), 2);

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 13);
    }
}
