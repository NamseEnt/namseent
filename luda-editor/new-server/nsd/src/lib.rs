mod leb128;
mod list;
mod map;

use bytes::Bytes;
use leb128::*;
pub use list::*;
pub use map::*;

pub trait Nsd: Clone {
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
#[derive(Debug, Clone)]
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

impl Nsd for VStr {
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
