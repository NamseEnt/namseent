use crate::*;
use bytes::Buf;

impl Nsd for usize {
    fn byte_len(&self) -> usize {
        leb128_byte_len(*self)
    }

    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()> {
        Ok(leb128::write_on_bytes_usize(*self, dest)?)
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(leb128::read(bytes)?)
    }
}

impl Nsd for String {
    fn byte_len(&self) -> usize {
        let bytes: &[u8] = self.as_bytes();
        leb128_byte_len(bytes.len()) + bytes.len()
    }

    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()> {
        let str_bytes = self.as_bytes();

        let mut dest_writer = DestWriter::new(dest);
        dest_writer.write(&str_bytes.len())?;
        dest_writer.write_bytes(str_bytes)?;

        Ok(())
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let byte_len = leb128::read(bytes)?;
        println!("byte_len: {}", byte_len);
        if bytes.remaining() < byte_len {
            bail!(FromBytesError::NotEnoughBytes);
        }
        Ok(String::from_utf8(bytes.split_to(byte_len).to_vec())?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primitive() {
        let value = 123_usize;
        assert_eq!(1, value.byte_len());
        let mut bytes = value.to_bytes();
        let new_value = usize::from_bytes(&mut bytes).unwrap();
        assert_eq!(value, new_value);

        let value = "hello world".to_string();
        assert_eq!(value.len() + 1, value.byte_len());
        let mut bytes = value.to_bytes();
        assert_eq!(bytes[0], value.len() as u8);
        assert_eq!(bytes[1], b"h"[0]);
        let new_value = String::from_bytes(&mut bytes).unwrap();
        assert_eq!(value, new_value);
    }
}
