//! # Wal File
//!
//! [Header][Body][Header][Body]...
//!

use super::*;
use crate::checksum;
use bytes::BufMut;
use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, ErrorKind, Read, Seek, SeekFrom, Write},
    mem::MaybeUninit,
    sync::Arc,
};

pub struct Wal {
    writer: BufWriter<Arc<File>>,
    reader: BufReader<Arc<File>>,
    read_offset: u64,
    write_offset: u64,
    written: u64,
}

impl Wal {
    pub(crate) fn open(path: std::path::PathBuf, file: &mut File) -> Result<Self, ExecuteError> {
        let wal_file = Arc::new(
            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(false)
                .open(path)?,
        );

        let wal_file_len = wal_file.metadata()?.len();

        let mut this = Self {
            read_offset: 0,
            write_offset: 0,
            written: 0,
            writer: BufWriter::new(wal_file.clone()),
            reader: BufReader::new(wal_file),
        };

        if wal_file_len == 0 {
            return Ok(this);
        }

        this.write_offset = wal_file_len;
        while this.write_offset > 0 {
            match this.execute_one(file) {
                Ok(_) => continue,
                Err(err) => {
                    if err.is_corrupted() {
                        this.reset()?;
                        break;
                    } else {
                        return Err(err);
                    }
                }
            };
        }

        Ok(this)
    }
    pub(crate) fn execute_one(&mut self, file: &mut File) -> Result<(), ExecuteError> {
        self.reset_if_need()?;

        if self.read_offset == self.write_offset {
            return Ok(());
        }

        self.reader.seek(SeekFrom::Start(self.read_offset))?;

        let header = unsafe {
            let mut header = MaybeUninit::<WalHeader>::uninit();
            self.reader.read_exact(std::slice::from_raw_parts_mut(
                header.as_mut_ptr() as *mut u8,
                size_of::<WalHeader>(),
            ))?;

            header.assume_init()
        };
        match header.body_types {
            // Init
            0 => {
                let root_node_offset = PageOffset::new(1);

                let header = Header::new(PageOffset::NULL, root_node_offset, PageOffset::new(2));

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
                    self.reader.read_exact(std::slice::from_raw_parts_mut(
                        body.as_mut_ptr() as *mut u8,
                        header.body_length as usize,
                    ))?;
                    body.assume_init()
                };

                let body_checksum = checksum(body.as_slice());
                let bad_checksum = body_checksum != header.checksum;
                if bad_checksum {
                    return Err(ExecuteError::Checksum {
                        expected: header.checksum,
                        actual: body_checksum,
                    });
                }

                file.seek(body.page_offset.file_pos())?;
                file.write_all(body.page.as_slice())?;
            }
            body_type => {
                return Err(ExecuteError::WrongBodyType { body_type });
            }
        }

        self.read_offset += size_of::<WalHeader>() as u64 + header.body_length as u64;

        if self.read_offset == self.write_offset {
            self.reset()?;
        }

        Ok(())
    }

    fn reset_if_need(&mut self) -> std::io::Result<()> {
        if self.read_offset == self.write_offset && self.read_offset != 0 {
            self.reset()?;
        }
        Ok(())
    }

    fn reset(&mut self) -> std::io::Result<()> {
        self.file().set_len(0)?;
        self.file().sync_all()?;

        self.read_offset = 0;
        self.write_offset = 0;
        Ok(())
    }

    pub(crate) fn write_init(&mut self) -> Result<()> {
        self.start_write()?;
        self.write_wal(Init)?;
        self.sync_all()?;
        Ok(())
    }

    pub(crate) fn update_pages(&mut self, pages: &BTreeMap<PageOffset, Page>) -> Result<()> {
        self.start_write()?;

        if pages.is_empty() {
            return Ok(());
        }

        for (offset, page) in pages {
            let put_page = PutPage {
                page_offset: *offset,
                page: page.clone(),
            };

            self.write_wal(put_page)?;
        }

        self.sync_all()?;

        Ok(())
    }

    fn file(&self) -> &File {
        self.writer.get_ref()
    }
    fn write_wal<Body: WalBody>(&mut self, body: Body) -> Result<()> {
        let body_bytes = body.as_slice();
        let header = WalHeader {
            checksum: checksum(body_bytes),
            body_length: body_bytes.len() as u32,
            body_types: Body::body_types(),
        };

        self.writer.write_all(header.as_slice())?;
        self.writer.write_all(body_bytes)?;

        self.written += size_of::<WalHeader>() as u64 + body_bytes.len() as u64;

        Ok(())
    }

    fn sync_all(&mut self) -> Result<()> {
        self.writer.flush()?;
        self.file().sync_all()?;
        self.write_offset += self.written;

        Ok(())
    }

    /// This must be called before writing to the file,
    /// because the wal file would be corrupted on previous write with error.
    fn start_write(&mut self) -> Result<()> {
        self.writer.seek(SeekFrom::Start(self.write_offset))?;

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum ExecuteError {
    Io(std::io::Error),
    #[allow(dead_code)]
    Checksum {
        expected: u64,
        actual: u64,
    },
    #[allow(dead_code)]
    WrongBodyType {
        body_type: u8,
    },
}
impl ExecuteError {
    pub(crate) fn is_corrupted(&self) -> bool {
        match self {
            ExecuteError::Io(error) => error.kind() == ErrorKind::UnexpectedEof,
            ExecuteError::Checksum { .. } => true,
            ExecuteError::WrongBodyType { .. } => true,
        }
    }
}
impl From<std::io::Error> for ExecuteError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ExecuteError {}

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

#[repr(C, align(1))]
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
