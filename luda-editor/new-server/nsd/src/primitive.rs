use crate::*;

impl Nsd for usize {
    fn byte_len(&self) -> usize {
        leb128_byte_len(*self)
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        Leb128::new(*self).write_on_bytes(bytes)
    }

    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized,
    {
        let mut bytes = bytes;
        Leb128::read(&mut bytes)
    }
}

impl Nsd for String {
    fn byte_len(&self) -> usize {
        let bytes: &[u8] = self.as_bytes();
        leb128_byte_len(bytes.len()) + bytes.len()
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        let str_bytes = self.as_bytes();
        let mut index = 0;
        index += Leb128::new(str_bytes.len()).write_on_bytes(bytes.get_mut(index..).unwrap());
        bytes[index..index + str_bytes.len()].copy_from_slice(str_bytes);
        index += str_bytes.len();
        index
    }

    fn from_bytes(bytes: Bytes) -> Self
    where
        Self: Sized,
    {
        let mut bytes = bytes;
        let len = Leb128::read(&mut bytes);
        assert_eq!(len, bytes.len());
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primitive() {
        let value = 123_usize;
        assert_eq!(1, value.byte_len());
        let bytes = value.to_bytes();
        let new_value = usize::from_bytes(bytes);
        assert_eq!(value, new_value);

        let value = "hello world".to_string();
        assert_eq!(value.len() + 1, value.byte_len());
        let bytes = value.to_bytes();
        assert_eq!(bytes[0], value.len() as u8);
        assert_eq!(bytes[1], b"h"[0]);
        let new_value = String::from_bytes(bytes);
        assert_eq!(value, new_value);
    }
}
