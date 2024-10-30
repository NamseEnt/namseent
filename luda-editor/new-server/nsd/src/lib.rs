mod leb128;
#[cfg(feature = "vec")]
mod std_vec;
// mod map;
mod list;
mod primitive;
mod str;
// mod set;

use anyhow::*;
use bytes::{Bytes, BytesMut};
use leb128::*;
pub use list::*;
pub use str::*;
// pub use map::*;
// pub use set::*;

pub trait Nsd: Clone {
    fn byte_len(&self) -> usize;
    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()>;
    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Bytes {
        let byte_len = self.byte_len();
        let mut bytes = BytesMut::with_capacity(byte_len);
        bytes.resize(byte_len, 0);
        self.write_on_bytes(&mut bytes).unwrap();
        bytes.freeze()
    }
}

#[derive(Debug)]
pub enum FromBytesError {
    NotEnoughBytes,
}

impl std::fmt::Display for FromBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not enough bytes")
    }
}

impl std::error::Error for FromBytesError {}

struct DestWriter<'a> {
    dest: &'a mut [u8],
    index: usize,
}
impl<'a> DestWriter<'a> {
    fn new(dest: &'a mut [u8]) -> Self {
        Self { dest, index: 0 }
    }

    fn write(&mut self, nsd: &impl Nsd) -> Result<()> {
        let byte_len = nsd.byte_len();
        if self.index + byte_len > self.dest.len() {
            bail!(FromBytesError::NotEnoughBytes);
        }
        nsd.write_on_bytes(&mut self.dest[self.index..self.index + byte_len])?;
        self.index += byte_len;
        Result::Ok(())
    }
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if self.index + bytes.len() > self.dest.len() {
            bail!(FromBytesError::NotEnoughBytes);
        }
        self.dest[self.index..self.index + bytes.len()].copy_from_slice(bytes);
        self.index += bytes.len();
        Result::Ok(())
    }
    fn written_len(&self) -> usize {
        self.index
    }
}
