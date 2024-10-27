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
        if self.extra.is_empty() {
            if self.source.is_empty() {
                None
            } else {
                let mut source = self.source.clone();
                let item_count = Leb128::read(&mut source);
                for index in 0..item_count {
                    let item_byte_len = Leb128::read(&mut source);
                    if index == item_count - 1 {
                        assert_eq!(source.len(), item_byte_len);
                        let item = T::from_bytes(source.split_to(item_byte_len));
                        return Some(item);
                    }
                }
                assert_eq!(item_count, 0);
                None
            }
        } else {
            Some(self.extra.last().unwrap().clone())
        }
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

    pub fn pop(&self) -> Option<T> {
        todo!()
    }
}

impl<T: Nsd> Nsd for List<T> {
    fn byte_len(&self) -> usize {
        let mut byte_len = Leb128::new(self.extra.len()).byte_len();
        for item in self.extra.iter() {
            byte_len += Leb128::new(item.byte_len()).byte_len();
            byte_len += item.byte_len();
        }
        byte_len
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index = 0;
        index += Leb128::new(self.extra.len()).write_on_bytes(bytes.get_mut(index..).unwrap());
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

        assert_eq!(bytes.len(), 23);
    }
}
