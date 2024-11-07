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
    fs::OpenOptions,
    io::ErrorKind,
    mem::MaybeUninit,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};
use tokio::sync::mpsc;

type Result<T> = std::result::Result<T, ExecuteError>;

pub struct Wal {
    wal_write_fd: WriteFd,
    read_offset: ReadOffset,
    write_offset: usize,
    written: usize,
    tx: mpsc::UnboundedSender<ExecutorRequest>,
}

impl Wal {
    pub(crate) fn open(path: std::path::PathBuf, mut file_write_fd: WriteFd) -> Result<Self> {
        let wal_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)?;

        let (wal_read_fd, wal_write_fd) = split_file(wal_file);

        let (tx, rx) = mpsc::unbounded_channel();

        let mut this = Self {
            wal_write_fd,
            read_offset: ReadOffset::new(),
            write_offset: 0,
            written: 0,
            tx,
        };

        if file_write_fd.len()? == 0 {
            this.write_init()?;
        }

        let wal_file_len = this.wal_write_fd.len()?;

        let mut read_offset: usize = 0;
        while read_offset < wal_file_len {
            match execute_one(&wal_read_fd, read_offset, &mut this.wal_write_fd) {
                Ok(new_read_offset) => {
                    read_offset = new_read_offset;
                }
                Err(err) => {
                    if err.is_corrupted() {
                        break;
                    } else {
                        return Err(err);
                    }
                }
            };
        }
        if wal_file_len > 0 {
            this.wal_write_fd.set_len(0)?;
            this.wal_write_fd.fsync()?;
        }

        Executor {
            wal_read_fd,
            file_write_fd,
            rx,
            read_offset: this.read_offset.clone(),
        }
        .start();

        Ok(this)
    }
    // pub(crate) fn execute_one(&mut self, file: &mut File) -> Result<()> {
    //     self.reset_if_need()?;

    //     if self.read_offset == self.write_offset {
    //         return Ok(());
    //     }

    //     self.reader.seek(SeekFrom::Start(self.read_offset))?;

    //     let header = unsafe {
    //         let mut header = MaybeUninit::<WalHeader>::uninit();
    //         self.reader.read_exact(std::slice::from_raw_parts_mut(
    //             header.as_mut_ptr() as *mut u8,
    //             size_of::<WalHeader>(),
    //         ))?;

    //         header.assume_init()
    //     };
    //     match header.body_types {
    //         // Init
    //         0 => {
    //             let root_node_offset = PageOffset::new(1);

    //             let header = Header::new(PageOffset::NULL, root_node_offset, PageOffset::new(2));

    //             let root_node = LeafNode::new();

    //             let mut bytes = Vec::with_capacity(size_of::<Header>() + size_of::<LeafNode>());
    //             bytes.put_slice(header.as_slice());
    //             bytes.put_slice(root_node.as_slice());

    //             file.set_len(0)?;
    //             file.write_all(&bytes)?;
    //             file.sync_all()?;
    //         }
    //         // PutPage
    //         1 => {
    //             let body = unsafe {
    //                 let mut body = MaybeUninit::<PutPage>::uninit();
    //                 self.reader.read_exact(std::slice::from_raw_parts_mut(
    //                     body.as_mut_ptr() as *mut u8,
    //                     header.body_length as usize,
    //                 ))?;
    //                 body.assume_init()
    //             };

    //             let body_checksum = checksum(body.as_slice());
    //             let bad_checksum = body_checksum != header.checksum;
    //             if bad_checksum {
    //                 return Err(ExecuteError::Checksum {
    //                     expected: header.checksum,
    //                     actual: body_checksum,
    //                 });
    //             }

    //             file.seek(body.page_offset.file_pos())?;
    //             file.write_all(body.page.as_slice())?;
    //         }
    //         body_type => {
    //             return Err(ExecuteError::WrongBodyType { body_type });
    //         }
    //     }

    //     self.read_offset += size_of::<WalHeader>() as u64 + header.body_length as u64;

    //     if self.read_offset == self.write_offset {
    //         self.reset()?;
    //     }

    //     Ok(())
    // }

    // fn reset_if_need(&mut self) -> std::io::Result<()> {
    //     if self.read_offset == self.write_offset && self.read_offset != 0 {
    //         self.reset()?;
    //     }
    //     Ok(())
    // }

    // fn reset(&mut self) -> std::io::Result<()> {
    //     self.file().set_len(0)?;
    //     self.file().sync_all()?;

    //     self.read_offset = 0;
    //     self.write_offset = 0;
    //     Ok(())
    // }

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

    fn write_wal<Body: WalBody>(&mut self, body: Body) -> Result<()> {
        let body_bytes = body.as_slice();
        let header = WalHeader {
            checksum: checksum(body_bytes),
            body_length: body_bytes.len() as u32,
            body_types: Body::body_types(),
        };

        let mut bytes = Vec::with_capacity(size_of::<WalHeader>() + body_bytes.len());
        bytes.put_slice(header.as_slice());
        bytes.put_slice(body_bytes);

        self.write(&bytes)?;

        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.wal_write_fd
            .write_exact(buf, self.write_offset + self.written)?;
        self.written += buf.len();

        Ok(())
    }

    fn sync_all(&mut self) -> Result<()> {
        self.wal_write_fd.fsync()?;
        self.tx
            .send(ExecutorRequest::Push {
                written: self.written,
            })
            .map_err(|_| ExecuteError::ExecutorDown)?;
        self.write_offset += self.written;
        self.written = 0;

        Ok(())
    }

    /// This must be called before writing to the file.
    fn start_write(&mut self) -> Result<()> {
        todo!();

        Ok(())
    }
}

/// Use MSB as a flag to indicate the reset version
#[derive(Debug, Clone)]
struct ReadOffset {
    inner: Arc<AtomicU64>,
}

impl ReadOffset {
    fn new() -> Self {
        Self {
            inner: Arc::new(AtomicU64::new(0)),
        }
    }
    fn get(&self) -> usize {
        let value = self.inner.load(std::sync::atomic::Ordering::Relaxed);
        let offset = value & !(1 << 63);
        offset as usize
    }
    fn flag(&self) -> bool {
        self.inner.load(std::sync::atomic::Ordering::Relaxed) & (1 << 63) != 0
    }
    fn reset(&self) {
        let flag = self.inner.load(std::sync::atomic::Ordering::Relaxed) & (1 << 63);
        // 1xxxx -> 0xxxx
        // 0xxxx -> 1xxxx
        let toggled = flag ^ 1 << 63;
        self.inner
            .store(toggled, std::sync::atomic::Ordering::Relaxed);
    }
    fn add(&self, written: usize) {
        self.inner
            .fetch_add(written as u64, std::sync::atomic::Ordering::Relaxed);
    }
}

struct Executor {
    wal_read_fd: ReadFd,
    file_write_fd: WriteFd,
    rx: mpsc::UnboundedReceiver<ExecutorRequest>,
    read_offset: ReadOffset,
}

impl Executor {
    fn start(mut self) {
        tokio::spawn(async move {
            while let Some(request) = self.rx.recv().await {
                match request {
                    ExecutorRequest::Push { written } => {
                        self.handle_push(written).await;
                    }
                    ExecutorRequest::Reset => {
                        self.read_offset.reset();
                    }
                }
            }
        });
    }
    async fn handle_push(&mut self, written: usize) {
        let mut sleep_time = Duration::from_millis(100);
        for _ in 0..=10 {
            match execute_one(
                &self.wal_read_fd,
                self.read_offset.get(),
                &mut self.file_write_fd,
            ) {
                Ok(new_read_offset) => {
                    assert_eq!(written, new_read_offset - self.read_offset.get());
                    self.read_offset.add(written);
                    break;
                }
                Err(err) => {
                    if err.is_corrupted() {
                        unreachable!("wal file is corrupted");
                    }

                    eprintln!(
                        "Error on execute wal record. error: {:?} Retry after {:?}",
                        err, sleep_time
                    );
                    tokio::time::sleep(sleep_time).await;
                    sleep_time = (sleep_time * 2).max(Duration::from_secs(4));
                }
            }
        }

        unreachable!("Too many retrial on writing staled pages");
    }
}

fn start_executor(
    wal_read_fd: ReadFd,
    file_write_fd: WriteFd,
    rx: mpsc::Receiver<ExecutorRequest>,
    read_offset: ReadOffset,
) {
}

/// # Return
/// The next read offset.
///
/// This function returns next read offset on successful execution
/// because it would be failed in the middle of the execution.
fn execute_one(
    wal_read_fd: &ReadFd,
    mut wal_read_offset: usize,
    file_write_fd: &mut WriteFd,
) -> Result<usize> {
    let header = unsafe {
        let mut header = MaybeUninit::<WalHeader>::uninit();
        let buf =
            std::slice::from_raw_parts_mut(header.as_mut_ptr() as *mut u8, size_of::<WalHeader>());

        wal_read_fd.read_exact(buf, wal_read_offset)?;
        wal_read_offset += buf.len();

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

            file_write_fd.set_len(0)?;
            file_write_fd.write_exact(&bytes, 0)?;
            file_write_fd.fsync()?;
        }
        // PutPage
        1 => {
            let body = unsafe {
                let mut body = MaybeUninit::<PutPage>::uninit();
                wal_read_fd.read_exact(
                    std::slice::from_raw_parts_mut(
                        body.as_mut_ptr() as *mut u8,
                        header.body_length as usize,
                    ),
                    wal_read_offset,
                )?;
                wal_read_offset += header.body_length as usize;
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

            file_write_fd.write_exact(body.page.as_slice(), body.page_offset.file_offset())?;
        }
        body_type => {
            return Err(ExecuteError::WrongBodyType { body_type });
        }
    }

    Ok(wal_read_offset)
}

enum ExecutorRequest {
    /// Push new wal record
    Push { written: usize },
    /// Reset wal file
    Reset,
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
    ExecutorDown,
}
impl ExecuteError {
    pub(crate) fn is_corrupted(&self) -> bool {
        match self {
            ExecuteError::Io(error) => error.kind() == ErrorKind::UnexpectedEof,
            ExecuteError::Checksum { .. } => true,
            ExecuteError::WrongBodyType { .. } => true,
            ExecuteError::ExecutorDown => false,
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
