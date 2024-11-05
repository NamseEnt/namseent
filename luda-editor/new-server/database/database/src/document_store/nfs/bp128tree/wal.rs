//! # Wal File
//!
//! [Header][Body][Header][Body]...
//!

use super::{super::crc, *};
use bytes::BufMut;
use libc::c_int;
use std::{
    collections::BTreeMap,
    ffi::CString,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, Write},
    os::unix::fs::OpenOptionsExt,
};

pub struct Wal {
    // buf_writer: BufWriter<File>,
    file: File,
    dirty: bool,
}

impl Wal {
    pub(crate) fn open(path: std::path::PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .custom_flags(libc::O_DIRECT)
            .open(path)?;

        Ok(Self {
            dirty: file.metadata()?.len() != 0,
            // buf_writer: BufWriter::new(file),
            file,
        })
    }

    fn file(&self) -> &File {
        // self.buf_writer.get_ref()
        &self.file
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

                    let header = Header {
                        free_page_stack_top_page_offset: PageOffset::NULL,
                        root_node_offset,
                        next_page_offset: PageOffset::new(2),
                        _padding: [0; 1021],
                    };

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
        println!("write_init");
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
            checksum: crc().checksum(body_bytes),
            body_length: body_bytes.len() as u32,
            body_types: Body::body_types(),
        };

        // store header and body in a multiple of 512 bytes
        let header_size = size_of::<WalHeader>();
        let body_size = body_bytes.len();
        let padding_size = (512 - ((header_size + body_size) % 512)) % 512;
        let bytes_size = header_size + body_size + padding_size;
        assert_eq!(bytes_size % 512, 0);
        let mut bytes = Vec::with_capacity(bytes_size);
        bytes.put_slice(header.as_slice());
        bytes.put_slice(body_bytes);
        bytes.resize(bytes_size, 0);

        println!("bytes.len(): {}", bytes.len());
        // self.file.write_all(&bytes)?;
        let mut bytes = [0u8; 1024];
        let align_offset = bytes.as_ptr().align_offset(512);
        println!("align_offset: {}", align_offset);
        let bytes_aligned_512 = bytes.get_mut(align_offset..align_offset + 512).unwrap();
        println!("bytes_aligned_512.len(): {}", bytes_aligned_512.len());
        assert_eq!(bytes_aligned_512.as_ptr().align_offset(512), 0);
        let written = self.file.write(bytes_aligned_512)?;

        println!("written: {}", written);
        Ok(())
    }

    fn sync_all(&mut self) -> Result<()> {
        // self.file.flush()?;
        // let now = std::time::Instant::now();
        // self.file().sync_all()?;
        // println!("sync_all: {:?}", now.elapsed());

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
