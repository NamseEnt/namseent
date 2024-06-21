use std::borrow::Cow;

pub struct RingBuffer {
    buffer: Box<[u8]>,
    buffer_index: usize,
    read_count: usize,
}
impl RingBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size].into_boxed_slice(),
            buffer_index: 0,
            read_count: 0,
        }
    }
    pub fn size(&self) -> usize {
        self.buffer.len()
    }
    pub fn ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }
    pub fn take_read_count(&mut self) -> usize {
        let read_count = self.read_count;
        self.read_count = 0;
        read_count
    }
    pub fn read_bytes(&mut self, byte_length: usize) -> Cow<'_, [u8]> {
        self.read_count += byte_length;
        if self.buffer_index + byte_length <= self.buffer.len() {
            let slice = &self.buffer[self.buffer_index..self.buffer_index + byte_length];
            self.buffer_index += byte_length;
            Cow::Borrowed(slice)
        } else {
            let mut vec = vec![0; byte_length];
            vec.copy_from_slice(&self.buffer[self.buffer_index..]);
            self.buffer_index = byte_length - (self.buffer.len() - self.buffer_index);
            Cow::from(vec)
        }
    }
    pub fn read_u8(&mut self) -> u8 {
        self.read_bytes(1)[0]
    }
    pub fn read_u16(&mut self) -> u16 {
        let bytes = self.read_bytes(2);
        u16::from_le_bytes([bytes[0], bytes[1]])
    }
    pub fn read_u32(&mut self) -> u32 {
        let bytes = self.read_bytes(4);
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}
