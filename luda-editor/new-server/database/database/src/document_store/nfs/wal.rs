//! # Write Ahead Log (WAL)
//!
//! ## file layout
//!
//! head: [u64: body checksum] [u32: body length]
//! body: [[u16: key length] [bytes: key] [u32: value length] [bytes: value]]...

use bytes::{Buf, Bytes};

const HEADER_SIZE: usize = size_of::<u64>() + size_of::<u32>();

fn crc() -> crc::Crc<u64> {
    crc::Crc::<u64>::new(&crc::CRC_64_REDIS)
}

pub(crate) fn serialize_wal_writes(wal_writes: &[WalWrite]) -> Vec<u8> {
    let body_length = wal_writes.iter().fold(0, |acc, wal_write| {
        acc + size_of::<u16>()
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

    let body_slice = &bytes[HEADER_SIZE..];
    let checksum = crc().checksum(body_slice);
    bytes[..size_of::<u64>()].copy_from_slice(&checksum.to_le_bytes());

    bytes
}

pub(crate) fn deserialize_wal_writes(
    mut bytes: Bytes,
) -> Result<Vec<WalWrite>, WalDeserializeError> {
    println!("bytes.len(): {}", bytes.len());
    if bytes.len() < HEADER_SIZE {
        return Err(WalDeserializeError::NotEnoughBytes);
    }

    let checksum = bytes.get_u64_le();
    let body_length = bytes.get_u32_le() as usize;
    println!("body_length: {}", body_length);

    if bytes.remaining() != body_length {
        return Err(WalDeserializeError::BodyLengthMismatch);
    }
    if crc().checksum(&bytes) != checksum {
        return Err(WalDeserializeError::ChecksumMismatch);
    }

    let mut wal_writes = Vec::new();
    while !bytes.is_empty() {
        let key_length = bytes.get_u16_le() as usize;
        let key = String::from_utf8_lossy(&bytes.split_to(key_length)).to_string();
        let value_length = bytes.get_u32_le() as usize;
        let value = if value_length == 0 {
            None
        } else {
            Some(bytes.copy_to_bytes(value_length))
        };
        wal_writes.push(WalWrite { key, value });
    }

    Ok(wal_writes)
}

#[derive(Debug)]
pub(crate) enum WalDeserializeError {
    ChecksumMismatch,
    BodyLengthMismatch,
    NotEnoughBytes,
}

impl std::fmt::Display for WalDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for WalDeserializeError {}

#[derive(Debug, PartialEq)]
pub(crate) struct WalWrite {
    pub(crate) key: String,
    pub(crate) value: Option<Bytes>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_wal_writes() {
        let wal_writes = vec![
            WalWrite {
                key: "key1".to_string(),
                value: Some(Bytes::from("value1")),
            },
            WalWrite {
                key: "key2".to_string(),
                value: None,
            },
        ];
        let bytes = serialize_wal_writes(&wal_writes);
        let deserialized_wal_writes = deserialize_wal_writes(Bytes::from(bytes)).unwrap();
        assert_eq!(wal_writes, deserialized_wal_writes);
    }
}
