//! # Wal File
//!
//! [Header][Body][Header][Body]...
//!
//!
//! # Shadow File
//!
//! WAL will be applied to the shadow file first.
//! If no more WAL records are available, WAL file will be truncated.
//! That makes WAL file smaller.
//! On the start, shadow file will be copied to the main file, and apply small WAL file.
//!
//! During the system, main file will be updated by cache staled pages.
//! But for the strong consistency and durability, WAL file will be used every start time.
//!

mod executor;

use super::*;
use crate::checksum;
use bytes::BufMut;
use executor::*;
use std::{
    collections::BTreeMap,
    io::ErrorKind,
    sync::{Arc, atomic::AtomicU64},
};
use tokio::{fs::OpenOptions, sync::mpsc};

type Result<T> = std::result::Result<T, WalError>;

pub struct Wal {
    wal_write_fd: WriteFd,
    read_offset: ReadOffset,
    /// flag to determined wal reader and writer are in the same context.
    /// This value will compared to read_offset's flag.
    ///
    /// If read_offset == write_offset, but the flags are different,
    /// that mean reader didn't start read data from the start of file.
    track_flag: bool,
    write_offset: usize,
    written: usize,
    tx: mpsc::UnboundedSender<ExecutorRequest>,
    executer_close_rx: oneshot::Receiver<()>,
}

impl Wal {
    pub(crate) async fn open(
        path: std::path::PathBuf,
        file_write_fd: &mut WriteFd,
    ) -> std::io::Result<Self> {
        let wal_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&path)
            .await?;

        let shadow_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path.with_extension("shadow"))
            .await?;

        let (wal_read_fd, wal_write_fd) = split_file(wal_file.into_std().await);
        let (shadow_read_fd, mut shadow_write_fd) = split_file(shadow_file.into_std().await);

        let (tx, rx) = mpsc::unbounded_channel();
        let (executer_close_tx, executer_close_rx) = oneshot::channel();

        let mut this = Self {
            wal_write_fd,
            read_offset: ReadOffset::new(),
            track_flag: true,
            write_offset: 0,
            written: 0,
            tx,
            executer_close_rx,
        };

        if file_write_fd.len()? == 0 {
            this.write_wal(Init)?;
            this.wal_write_fd.fsync()?;
        }

        let wal_file_len = this.wal_write_fd.len()?;

        let mut read_offset: usize = 0;
        while read_offset < wal_file_len {
            match execute_one(&wal_read_fd, read_offset, &mut shadow_write_fd).await {
                Ok(new_read_offset) => {
                    read_offset = new_read_offset;
                }
                Err(err) => {
                    if err.is_corrupted() {
                        break;
                    }
                    match err {
                        WalError::Executor(execute_error) => match execute_error {
                            ExecuteError::Checksum { .. } => unreachable!(),
                            ExecuteError::WrongBodyType { .. } => unreachable!(),
                            ExecuteError::WrongBodySize { .. } => unreachable!(),
                        },
                        WalError::Io(error) => {
                            return Err(error);
                        }
                        WalError::ExecutorDown => unreachable!(),
                    }
                }
            };
        }
        file_write_fd.copy_from(&shadow_read_fd)?;

        if wal_file_len > 0 {
            this.wal_write_fd.set_len(0)?;
            this.wal_write_fd.fsync()?;
            this.write_offset = 0;
            this.written = 0;
        }

        Executor::new(
            wal_read_fd,
            shadow_write_fd,
            rx,
            this.read_offset.clone(),
            executer_close_tx,
        )
        .start();

        Ok(this)
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

    fn write_wal<Body: WalBody>(&mut self, body: Body) -> std::io::Result<()> {
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

    fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
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
            .map_err(|_| WalError::ExecutorDown)?;
        self.write_offset += self.written;
        self.written = 0;

        Ok(())
    }

    /// This must be called before writing to the file.
    fn start_write(&mut self) -> Result<()> {
        let (read_offset, flag) = self.read_offset.get_with_flag();

        let reader_cached_writer = read_offset == self.write_offset && self.track_flag == flag;
        if reader_cached_writer {
            self.tx
                .send(ExecutorRequest::Reset)
                .map_err(|_| WalError::ExecutorDown)?;
            self.wal_write_fd.set_len(0)?;
            self.written = 0;
            self.write_offset = 0;
            self.track_flag = !self.track_flag;
        }

        Ok(())
    }

    pub(crate) async fn close(self) {
        _ = self.tx.send(ExecutorRequest::Close);
        _ = self.executer_close_rx.await;
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum WalError {
    #[error("Error on executor: {0}")]
    Executor(#[from] ExecuteError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("Executor is down")]
    ExecutorDown,
}

impl WalError {
    fn is_corrupted(&self) -> bool {
        match self {
            WalError::Executor(execute_error) => match execute_error {
                ExecuteError::Checksum { .. } => true,
                ExecuteError::WrongBodyType { .. } => true,
                ExecuteError::WrongBodySize { .. } => true,
            },
            WalError::Io(error) => error.kind() == ErrorKind::UnexpectedEof,
            WalError::ExecutorDown => false,
        }
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
    fn get_with_flag(&self) -> (usize, bool) {
        let value = self.inner.load(std::sync::atomic::Ordering::Relaxed);
        let offset = value & !(1 << 63);
        let flag = value & (1 << 63) != 0;
        (offset as usize, flag)
    }
    fn reset(&self) {
        let flag = self.inner.load(std::sync::atomic::Ordering::Relaxed) & (1 << 63);
        // 1xxxx -> 0xxxx
        // 0xxxx -> 1xxxx
        let toggled = flag ^ (1 << 63);
        self.inner
            .store(toggled, std::sync::atomic::Ordering::Relaxed);
    }
    fn set(&self, new_read_offset: usize) {
        let flag = self.inner.load(std::sync::atomic::Ordering::Relaxed) & (1 << 63);
        let new_value = new_read_offset as u64 | flag;
        self.inner
            .store(new_value, std::sync::atomic::Ordering::Relaxed);
    }
}

#[repr(C)]
#[derive(Debug)]
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
