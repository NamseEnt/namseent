mod leb128;
mod map;

use bytes::Bytes;
use leb128::*;
pub use map::*;

pub trait VVV {
    fn byte_len(&self) -> usize;
    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize;
    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized;
    fn to_bytes(&self) -> Bytes {
        let mut bytes = vec![0u8; self.byte_len()];
        self.write_on_bytes(&mut bytes);
        Bytes::from(bytes)
    }
}

/// Memory layout:
/// - value: [u8; byte_len]
pub struct VStr {
    bytes: Bytes,
}

impl From<&str> for VStr {
    fn from(s: &str) -> Self {
        Self {
            bytes: Bytes::copy_from_slice(s.as_bytes()),
        }
    }
}

impl From<String> for VStr {
    fn from(s: String) -> Self {
        Self {
            bytes: Bytes::from(s.into_bytes()),
        }
    }
}

impl AsRef<str> for VStr {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes) }
    }
}

impl std::ops::Deref for VStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl VVV for VStr {
    fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        bytes
            .get_mut(0..self.bytes.len())
            .unwrap()
            .copy_from_slice(&self.bytes);
        self.bytes.len()
    }

    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized,
    {
        Self { bytes }
    }
}

pub struct VVec<T: VVV> {
    byte_len: usize,
    _t: T,
}

impl<T: VVV> VVV for VVec<T> {
    fn byte_len(&self) -> usize {
        self.byte_len
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        todo!()
    }

    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized,
    {
        todo!()
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
        ZhCn,
        ZhTw,
    }

    impl VVV for IetfLanguageTag {
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
    fn test() {
        struct SpeakerDoc {
            names: VMap<IetfLanguageTag, VStr>,
        }
        impl VVV for SpeakerDoc {
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
                let names = VMap::from_bytes(bytes);
                Self { names }
            }
        }
        let bytes = {
            let mut doc = SpeakerDoc { names: VMap::new() };

            doc.names.insert(IetfLanguageTag::Ko, "안녕하세요");
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

            doc.names.insert(IetfLanguageTag::EnUs, "Hello");

            let option_value = doc.names.get(IetfLanguageTag::EnUs);
            assert!(option_value.is_some());
            let value: &str = &option_value.unwrap();
            assert_eq!(value, "Hello");

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 27);
    }
}
