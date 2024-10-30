use crate::*;
use bytes::Buf;
use std::ops::Deref;

/// # Str
///
/// ## Memory Layout
///
/// - string length: leb128
/// - content: bytes
#[derive(Debug, Clone)]
pub struct Str {
    inner: Bytes,
}

impl Nsd for Str {
    fn byte_len(&self) -> usize {
        let inner_len = self.inner.len();
        inner_len.byte_len() + inner_len
    }

    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()> {
        let mut dest_writer = DestWriter::new(dest);
        dest_writer.write(&self.inner.len())?;
        dest_writer.write_bytes(&self.inner)?;

        Ok(())
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let inner_len = leb128::read(bytes)?;
        if bytes.remaining() < inner_len {
            bail!(FromBytesError::NotEnoughBytes);
        }
        let inner = bytes.split_to(inner_len);
        Ok(Self { inner })
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Self {
            inner: Bytes::copy_from_slice(value.as_bytes()),
        }
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Str {}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        std::str::from_utf8(&self.inner).unwrap()
    }
}

pub trait ToStr {
    fn to_str(&self) -> Str;
}

impl ToStr for str {
    fn to_str(&self) -> Str {
        Str::from(self)
    }
}

impl ToStr for String {
    fn to_str(&self) -> Str {
        Str::from(self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str() {
        let value: Str = "abc".into();
        assert_eq!(4, value.byte_len());
        let mut bytes = value.to_bytes();
        let new_value = Str::from_bytes(&mut bytes).unwrap();
        assert_eq!(value, new_value);
    }
}
