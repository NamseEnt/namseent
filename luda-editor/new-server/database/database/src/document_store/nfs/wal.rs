//! # Write Ahead Log (WAL)
//!
//! ## file layout
//!
//! head: [u64: body checksum] [u32: body length]
//! body: [[u16: key length] [bytes: key] [u32: value length] [bytes: value]]...

use bytes::Bytes;

pub(crate) fn serialize_wal_writes(wal_writes: &[WalWrite]) -> Vec<u8> {
    let body_length = wal_writes.iter().fold(0, |acc, wal_write| {
        acc + size_of::<u64>()
            + wal_write.key.bytes().len()
            + size_of::<u32>()
            + wal_write.value.as_ref().map_or(0, |v| v.len())
    });
    let mut bytes = Vec::with_capacity(body_length + size_of::<u64>() + size_of::<u32>());
    bytes.extend_from_slice(&0_u64.to_le_bytes()); // temporary checksum value
    bytes.extend_from_slice(&(body_length as u32).to_le_bytes());

    for wal_write in wal_writes {
        assert!(wal_write.key.bytes().len() <= u16::MAX as usize);
        bytes.extend_from_slice(&(wal_write.key.bytes().len() as u16).to_le_bytes());
        bytes.extend_from_slice(wal_write.key.as_bytes());
        if let Some(value) = &wal_write.value {
            assert!(value.len() <= u32::MAX as usize);
            bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
            bytes.extend_from_slice(value);
        } else {
            bytes.extend_from_slice(&0_u32.to_le_bytes());
        }
    }

    let crc = crc::Crc::<u64>::new(&crc::CRC_64_REDIS);
    let header_size = size_of::<u64>() + size_of::<u32>();
    let body_slice = &bytes[header_size..];
    let checksum = crc.checksum(body_slice);
    bytes[..size_of::<u64>()].copy_from_slice(&checksum.to_le_bytes());

    bytes
}

pub(crate) struct WalWrite {
    pub(crate) key: String,
    pub(crate) value: Option<Bytes>,
}
