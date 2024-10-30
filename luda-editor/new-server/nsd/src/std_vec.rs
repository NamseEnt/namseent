//! # Vec
//!
//! ## Memory Layout
//!
//! - item count: leb128
//! - item:
//!   - item: bytes

use crate::*;

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
            dest_writer.write(item)?;
        }
        Ok(())
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let source_count = leb128::read(bytes)?;

        let mut vec = Vec::with_capacity(source_count);

        for _ in 0..source_count {
            let value = T::from_bytes(bytes)?;
            vec.push(value);
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let mut bytes = {
            let mut doc = Vec::<Str>::new();

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

        assert_eq!(bytes[..], [1, 5, b'a', b'b', b'c', b'd', b'e']);

        let bytes = {
            let doc = Vec::<Str>::from_bytes(&mut bytes).unwrap();

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

        assert_eq!(bytes.len(), 13);
    }
}
