//! # List
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
        let now = std::time::Instant::now();
        let source_count = leb128::read(bytes)?;
        println!("source_count elapsed: {:?}", now.elapsed());

        let now = std::time::Instant::now();
        let mut vec = Vec::with_capacity(source_count);
        println!("Vec::with_capacity elapsed: {:?}", now.elapsed());

        for _ in 0..source_count {
            let now = std::time::Instant::now();
            let value = T::from_bytes(bytes)?;
            vec.push(value);
            println!("T::from_bytes elapsed: {:?}", now.elapsed());
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut bytes = {
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
            let now = std::time::Instant::now();
            let doc = Vec::<String>::from_bytes(&mut bytes).unwrap();
            panic!("elapsed: {:?}", now.elapsed());
            println!("doc: {:?}", doc);

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
