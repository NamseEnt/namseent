mod leb128;
mod list;
mod map;
mod primitive;
mod set;

use bytes::Bytes;
use leb128::*;
pub use list::*;
pub use map::*;
pub use set::*;

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
