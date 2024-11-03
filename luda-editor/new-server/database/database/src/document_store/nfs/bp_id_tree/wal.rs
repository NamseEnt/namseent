//! # Wal File
//!
//! [Header][Body][Header][Body]...
//!
//! - Header
//!   - Body Checksum: u64
//!   - Body Length: u32
//!   - Body types: u8
//!
//! - Init(0) Body
//!   - nothing
//!
//! - InsertToLeafNode(1) Body
//!   - NodeIndex: PageIndex
//!   - Id: u128

use super::{super::crc, *};
use bytes::BufMut;
use std::{
    fs::File,
    io::{BufReader, Read, Result, Seek, Write},
};

pub struct Wal {
    file: File,
    dirty: bool,
}

impl Wal {
    pub(crate) fn open(path: std::path::PathBuf) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        Ok(Self { file, dirty: true })
    }

    pub(crate) fn flush(&mut self, file: &mut File) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let wal_file_len = self.file.metadata()?.len();
        let mut reader = BufReader::new(&mut self.file);

        while wal_file_len > reader.stream_position()? {
            let header = unsafe {
                let mut header = MaybeUninit::<WalHeader>::uninit();
                reader.read_exact(std::slice::from_raw_parts_mut(
                    header.as_mut_ptr() as *mut u8,
                    size_of::<WalHeader>(),
                ))?;
                header.assume_init()
            };

            match header.body_types {
                // Init
                0 => {
                    let root_node_index =
                        PageIndex::without_node_type_msb(NonZeroU32::new(1).unwrap());

                    let header = Header {
                        free_page_stack_top_page_index: PageIndex::NULL,
                        root_node_index,
                        padding: [0; 1022],
                    };

                    let root_node = LeafNode::new(PageIndex::NULL);

                    let mut bytes = Vec::with_capacity(size_of::<Header>() + size_of::<LeafNode>());
                    bytes.put_slice(header.as_slice());
                    bytes.put_slice(root_node.as_slice());

                    file.set_len(0)?;
                    file.write_all(&bytes)?;
                    file.sync_all()?;
                }
                // InsertToLeafNode
                1 => {
                    let body = unsafe {
                        let mut body = MaybeUninit::<InsertToLeafNodeBody>::uninit();
                        reader.read_exact(std::slice::from_raw_parts_mut(
                            body.as_mut_ptr() as *mut u8,
                            header.body_length as usize,
                        ))?;
                        body.assume_init()
                    };

                    let mut node = read_node_from_file(file, body.node_index)?.into_leaf_node();
                    node.insert(body.id);
                    write_node_to_file(file, body.node_index, node.into_node())?;
                }
                _ => unreachable!(),
            }
        }

        if wal_file_len > 0 {
            self.file.set_len(0)?;
            self.file.sync_all()?;
        }
        self.dirty = false;

        Ok(())
    }

    pub(crate) fn write_init(&mut self) -> Result<()> {
        self.dirty = true;

        let body = [];
        let header = WalHeader {
            checksum: crc().checksum(&body),
            body_length: body.len() as u32,
            body_types: 0u8,
        };

        self.file.write_all(header.as_slice())?;
        self.file.sync_all()?;
        Ok(())
    }

    pub(crate) fn write_insert_to_leaf_node(
        &mut self,
        node_index: PageIndex,
        id: u128,
    ) -> Result<()> {
        self.dirty = true;

        let body = InsertToLeafNodeBody { node_index, id };
        let body_bytes = body.as_slice();
        let header = WalHeader {
            checksum: crc().checksum(body_bytes),
            body_length: body_bytes.len() as u32,
            body_types: 1u8,
        };

        let mut bytes = [0u8; size_of::<WalHeader>() + size_of::<InsertToLeafNodeBody>()];
        {
            let mut bytes = bytes.as_mut();
            bytes.put_slice(header.as_slice());
            bytes.put_slice(body_bytes);
        }

        self.file.write_all(&bytes)?;
        self.file.sync_all()?;
        Ok(())
    }
}

fn write_node_to_file(file: &mut File, node_index: PageIndex, node: Node) -> Result<()> {
    file.seek(node_index.file_pos())?;

    file.write_all(node.as_slice())?;
    file.sync_all()?;

    Ok(())
}

#[repr(C)]
struct WalHeader {
    checksum: u64,
    body_length: u32,
    body_types: u8,
}
impl AsSlice for WalHeader {}

#[repr(C)]
struct InsertToLeafNodeBody {
    node_index: PageIndex,
    id: u128,
}
impl AsSlice for InsertToLeafNodeBody {}
