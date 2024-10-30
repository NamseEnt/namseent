//! # List
//!
//! ## Memory Layout
//!
//! - header
//!     - body bytes length: leb128
//! - body
//!     - item count: leb128
//!     - items:
//!         - item: bytes

use crate::*;
use std::{
    ops::{Deref, DerefMut},
    sync::OnceLock,
};

#[derive(Debug, Clone)]
pub struct List<T> {
    body: Bytes,
    items: OnceLock<Vec<T>>,
}

impl<T: Nsd> List<T> {
    pub fn new() -> Self {
        Self {
            body: Bytes::new(),
            items: OnceLock::from(Vec::new()),
        }
    }
    fn items(&self) -> &Vec<T> {
        self.items.get_or_init(|| {
            let mut items = Vec::new();
            let mut bytes = self.body.clone();
            let count = leb128::read(&mut bytes).unwrap();
            for _ in 0..count {
                let item = T::from_bytes(&mut bytes).unwrap();
                items.push(item);
            }
            items
        })
    }

    fn items_mut(&mut self) -> &mut Vec<T> {
        _ = self.items();
        self.items.get_mut().unwrap()
    }
}

impl<T: Nsd> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Nsd> Deref for List<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        self.items()
    }
}

impl<T: Nsd> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.items_mut()
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            body: Bytes::new(),
            items: OnceLock::from(vec),
        }
    }
}

impl<T: Nsd> Nsd for List<T> {
    fn byte_len(&self) -> usize {
        let mut body_byte_len = self.len().byte_len();
        self.iter().for_each(|item| {
            body_byte_len += item.byte_len();
        });

        let header_byte_len = body_byte_len.byte_len();

        header_byte_len + body_byte_len
    }

    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()> {
        let mut dest_writer = DestWriter::new(dest);

        // write body first
        dest_writer.write(&self.len())?;
        for item in self.iter() {
            dest_writer.write(item)?;
        }

        let body_bytes_len = dest_writer.written_len();
        let header_size = body_bytes_len.byte_len();

        // move body
        dest.copy_within(0..body_bytes_len, header_size);

        // write header
        let mut dest_writer = DestWriter::new(dest);
        dest_writer.write(&body_bytes_len)?;

        Ok(())
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let body_bytes_len = leb128::read(bytes)?;

        Ok(Self {
            body: bytes.split_to(body_bytes_len),
            items: OnceLock::new(),
        })
    }
}

#[macro_export]
macro_rules! list {
    () => (
        List::new()
    );
    ($elem:expr; $n:expr) => (
        List::from(std::vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        List::from(<[_]>::into_vec(
            // This rustc_box is not required, but it produces a dramatic improvement in compile
            // time when constructing arrays with many elements.
            Box::new([$($x),+])
        ))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut bytes = {
            let mut doc = List::<Str>::new();

            let option_value = doc.last();
            assert!(option_value.is_none());

            doc.push("abcde".to_str());

            let option_value = doc.last();
            assert!(option_value.is_some());

            let value: &str = option_value.unwrap();
            assert_eq!(value, "abcde");

            assert!(doc.len() == 1);

            doc.to_bytes()
        };

        assert_eq!(bytes[..], [7, 1, 5, b'a', b'b', b'c', b'd', b'e']);

        let bytes = {
            let doc = List::<Str>::from_bytes(&mut bytes).unwrap();

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = option_value.unwrap();
            assert_eq!(value, "abcde");

            let mut doc = doc;

            doc.pop();

            assert_eq!(doc.len(), 0);

            let option_value = doc.last();
            assert!(option_value.is_none());
            assert!(doc.is_empty());

            doc.push("Hello".to_str());

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = option_value.unwrap();
            assert_eq!(value, "Hello");

            assert_eq!(doc.len(), 1);

            doc.push("World".to_str());

            let option_value = doc.last();
            assert!(option_value.is_some());
            let value: &str = option_value.unwrap();
            assert_eq!(value, "World");

            assert_eq!(doc.len(), 2);

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 14);
    }
}
