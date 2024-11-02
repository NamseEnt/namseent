//! # B+IdTree
//!
//! B+IdTree is a B+Tree implementation for storing 128bit Ids.
//! All node size is 4KB, which will be called a page.
//! Offset size is 4Byte, so the maximum item count is 268,435,456.
//!
//! u32::MAX will be used as a null.
//! Endian is little.
//!
//! ## File Structure
//!
//! ### Free Page Stack
//!
//! Linked List, storing free page's offset in the file.
//! - Next Node Offset: u32
//! - Length in this page: u32
//! - Free Page Offsets: [u32; 1022]
//!
//! ### Header
//! - Free Internal Node Stack Top Offset: u32
//! - Root Node Offset: u32

use super::crc;
use bytes::{Buf, BufMut, Bytes};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

pub struct BpIdTree {
    file: std::fs::File,
    wal_file: std::fs::File,
}

impl BpIdTree {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();

        let mut wal_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.with_extension("wal"))?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        if file.metadata()?.len() == 0 {
            init(&mut file, &mut wal_file)?
        };

        flush_wal(&mut file, &mut wal_file);

        Ok(Self { file, wal_file })
    }
}

/// # Wal File
/// - Header
///   - Body Checksum: u64
///   - Body Length: u32
///   - Body types: u8
/// - Body
///   - Init: type 0
///     - nothing
///
#[repr(C)]
enum WalBody {
    Init,
}

const WAL_HEADER_SIZE: usize =
    std::mem::size_of::<u64>() + std::mem::size_of::<u32>() + std::mem::size_of::<u8>();

impl WalBody {
    fn write(self, wal_file: &mut std::fs::File) -> std::io::Result<()> {
        let (body_types, body) = match self {
            Self::Init => (0u8, vec![]),
        };
        let body_checksum = crc().checksum(&body);
        let body_length = body.len() as u32;

        let wal_bytes = {
            let mut wal_bytes = Vec::with_capacity(WAL_HEADER_SIZE + body.len());
            wal_bytes.put_u64_le(body_checksum);
            wal_bytes.put_u32_le(body_length);
            wal_bytes.put_u8(body_types);
            wal_bytes.put_slice(&body);
            wal_bytes
        };

        wal_file.write_all(&wal_bytes)?;
        wal_file.sync_all()?;
        Ok(())
    }
    fn read(wal_file: &mut std::fs::File) -> std::io::Result<Option<Self>> {
        let mut bytes = {
            let mut bytes = vec![];
            wal_file.read_to_end(&mut bytes)?;
            Bytes::from(bytes)
        };

        if bytes.len() < WAL_HEADER_SIZE {
            return Ok(None);
        }

        let checksum = bytes.get_u64_le();
        let body_length = bytes.get_u32_le() as usize;
        let body_types = bytes.get_u8();

        if bytes.len() < body_length {
            return Ok(None);
        }

        let body = bytes.split_to(body_length);
        if crc().checksum(&body) != checksum {
            return Ok(None);
        }

        match body_types {
            0 => Ok(Some(Self::Init)),
            _ => unreachable!(),
        }
    }
}

fn init(file: &mut File, wal_file: &mut File) -> std::io::Result<()> {
    WalBody::Init.write(wal_file)?;

    let header = Header {
        free_page_stack_top_offset: Offset::NULL,
        root_node_offset: Offset::NULL,
    };
    file.write_all(unsafe {
        std::slice::from_raw_parts(
            &header as *const _ as *const u8,
            std::mem::size_of::<Header>(),
        )
    })?;
    file.sync_all()?;

    Ok(())
}

fn flush_wal(file: &mut File, wal_file: &mut File) -> std::io::Result<()> {
    let Some(wal_body) = WalBody::read(wal_file)? else {
        return Ok(());
    };
    match wal_body {
        WalBody::Init => init(file, wal_file)?,
    }
    wal_file.seek(SeekFrom::Start(0))?;
    wal_file.set_len(0)?;
    wal_file.sync_all()?;
    Ok(())
}

#[repr(C)]
struct Offset {
    value: u32,
}
impl Offset {
    const NULL: Self = Self { value: u32::MAX };
}

#[repr(C)]
struct Header {
    free_page_stack_top_offset: Offset,
    root_node_offset: Offset,
}

#[repr(C)]
struct FreePageStackNode {
    next_node_offset: Offset,
    length: u32,
    free_page_offsets: [u32; 1022],
}

#[repr(C)]
struct InternalNode {
    parent_offset: Offset,
    key_count: u32,
    ids: [u128; 203],
    child_offsets: [u32; 204],
    _padding: u32,
}

#[repr(C)]
struct LeafNode {
    parent_offset: Offset,
    id_count: u32,
    ids: [u128; 255],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_size() {
        assert_eq!(std::mem::size_of::<FreePageStackNode>(), 4096);
        assert_eq!(std::mem::size_of::<InternalNode>(), 4096);
        assert_eq!(std::mem::size_of::<LeafNode>(), 4096);
    }
}
