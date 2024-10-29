use crate::*;
use bytes::{Buf, Bytes};

pub fn leb128_byte_len(value: usize) -> usize {
    if value < 128 {
        1
    } else if value < 16384 {
        2
    } else if value < 2097152 {
        3
    } else if value < 268435456 {
        4
    } else {
        let mut i = 5;
        loop {
            if value < 1 << (7 * i) {
                return i;
            }
            i += 1;
        }
    }
}

pub fn read(bytes: &mut Bytes) -> Result<usize, FromBytesError> {
    let mut value = 0;
    let mut shift = 0;
    loop {
        if bytes.remaining() == 0 {
            return Err(FromBytesError::NotEnoughBytes);
        }
        let byte = bytes.get_u8();
        value |= ((byte & 0x7F) as usize) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Result::Ok(value)
}
pub fn write_on_bytes_usize(mut value: usize, mut dest: &mut [u8]) -> Result<(), FromBytesError> {
    loop {
        if dest.is_empty() {
            return Err(FromBytesError::NotEnoughBytes);
        }
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        dest[0] = byte;
        dest = &mut dest[1..];
        if value == 0 {
            break;
        }
    }
    Result::Ok(())
}
