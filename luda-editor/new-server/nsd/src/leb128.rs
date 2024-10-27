use bytes::{Buf, Bytes};

pub struct Leb128 {
    inner: [u8; 8],
    value: usize,
    byte_len: usize,
}

impl Leb128 {
    pub fn new(value: usize) -> Self {
        let mut inner = [0u8; 8];
        let mut byte_len = 0;

        {
            let mut value = value;
            loop {
                let mut byte = (value & 0x7F) as u8;
                value >>= 7;
                if value != 0 {
                    byte |= 0x80;
                }
                inner[byte_len] = byte;
                byte_len += 1;
                if value == 0 {
                    break;
                }
            }
        }

        Self {
            inner,
            value,
            byte_len,
        }
    }
    pub fn value(&self) -> usize {
        self.value
    }

    pub fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index = 0;
        for i in 0..self.byte_len {
            bytes[index] = self.inner[i];
            index += 1;
        }
        index
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn read(bytes: &mut Bytes) -> usize {
        let mut value = 0;
        let mut byte_len = 0;
        for i in 0..bytes.len() {
            let byte = bytes[i];
            value |= ((byte & 0x7F) as usize) << (7 * i);
            byte_len += 1;
            if byte & 0x80 == 0 {
                break;
            }
        }
        bytes.advance(byte_len);
        value
    }
}

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
