//! # Wal File
//!
//! [Header][Body][Header][Body]...
//!

use super::*;
use crate::checksum;
use bytes::BufMut;
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    mem::MaybeUninit,
};

pub struct Wal {
    buf_writer: BufWriter<File>,
    dirty: bool,
}

impl Wal {
    pub(crate) fn open(path: std::path::PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)?;

        Ok(Self {
            dirty: file.metadata()?.len() != 0,
            buf_writer: BufWriter::new(file),
        })
    }

    fn file(&self) -> &File {
        self.buf_writer.get_ref()
    }

    pub(crate) fn flush(&mut self, file: &mut File) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let wal_file_len = self.file().metadata()?.len();
        if wal_file_len == 0 {
            return Ok(());
        }

        let mut reader = BufReader::new(self.file());
        reader.seek(SeekFrom::Start(0))?;

        while reader.stream_position()? < wal_file_len {
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
                    let root_node_offset = PageOffset::new(1);

                    let header =
                        Header::new(PageOffset::NULL, root_node_offset, PageOffset::new(2));

                    let root_node = LeafNode::new();

                    let mut bytes = Vec::with_capacity(size_of::<Header>() + size_of::<LeafNode>());
                    bytes.put_slice(header.as_slice());
                    bytes.put_slice(root_node.as_slice());

                    file.set_len(0)?;
                    file.write_all(&bytes)?;
                    file.sync_all()?;
                }
                // PutPage
                1 => {
                    let body = unsafe {
                        let mut body = MaybeUninit::<PutPage>::uninit();
                        reader.read_exact(std::slice::from_raw_parts_mut(
                            body.as_mut_ptr() as *mut u8,
                            header.body_length as usize,
                        ))?;
                        body.assume_init()
                    };

                    file.seek(body.page_offset.file_pos())?;
                    file.write_all(body.page.as_slice())?;
                }
                _ => unreachable!(),
            }
        }

        self.file().set_len(0)?;
        self.file().sync_all()?;
        self.dirty = false;

        Ok(())
    }

    pub(crate) fn write_init(&mut self) -> Result<()> {
        self.write_wal(Init)?;
        self.sync_all()?;
        Ok(())
    }

    pub(crate) fn update_pages(&mut self, pages: &BTreeMap<PageOffset, Page>) -> Result<()> {
        for (offset, page) in pages {
            let put_page = PutPage {
                page_offset: *offset,
                page: *page,
            };

            self.write_wal(put_page)?;
        }

        self.sync_all()?;

        Ok(())
    }
    fn write_wal<Body: WalBody>(&mut self, body: Body) -> Result<()> {
        self.dirty = true;

        let body_bytes = body.as_slice();
        let header = WalHeader {
            checksum: checksum(body_bytes),
            body_length: body_bytes.len() as u32,
            body_types: Body::body_types(),
        };

        self.buf_writer.write_all(header.as_slice())?;
        self.buf_writer.write_all(body_bytes)?;

        Ok(())
    }

    fn sync_all(&mut self) -> Result<()> {
        self.buf_writer.flush()?;
        self.file().sync_all()?;

        Ok(())
    }
}

#[repr(C)]
struct WalHeader {
    checksum: u64,
    body_length: u32,
    body_types: u8,
}
impl AsSlice for WalHeader {}

#[repr(C)]
struct Init;
impl AsSlice for Init {}
impl WalBody for Init {
    fn body_types() -> u8 {
        0
    }
}

#[repr(C)]
struct PutPage {
    page_offset: PageOffset,
    page: Page,
}
impl AsSlice for PutPage {}
impl WalBody for PutPage {
    fn body_types() -> u8 {
        1
    }
}

trait WalBody: AsSlice {
    fn body_types() -> u8;
}
