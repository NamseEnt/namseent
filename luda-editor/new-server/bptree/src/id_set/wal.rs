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

use super::*;
use crate::checksum;
use bytes::BufMut;
use std::{
    collections::BTreeMap,
    fmt::Display,
    io::ErrorKind,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};
use tokio::{fs::OpenOptions, sync::mpsc};

type Result<T> = anyhow::Result<T>;

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
    ) -> Result<Self> {
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
                    return Err(err);
                }
            };
        }
        if wal_file_len > 0 {
            file_write_fd.copy_from(&shadow_read_fd)?;

            this.wal_write_fd.set_len(0)?;
            this.wal_write_fd.fsync()?;
            this.write_offset = 0;
            this.written = 0;
        }

        Executor {
            wal_read_fd,
            shadow_write_fd,
            rx,
            read_offset: this.read_offset.clone(),
            close_tx: executer_close_tx,
        }
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
        let (read_offset, flag) = self.read_offset.get_with_flag();

        let reader_cached_writer = read_offset == self.write_offset && self.track_flag == flag;
        if reader_cached_writer {
            self.tx
                .send(ExecutorRequest::Reset)
                .map_err(|_| ExecuteError::ExecutorDown)?;
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
        let toggled = flag ^ 1 << 63;
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

struct Executor {
    wal_read_fd: ReadFd,
    shadow_write_fd: WriteFd,
    rx: mpsc::UnboundedReceiver<ExecutorRequest>,
    read_offset: ReadOffset,
    close_tx: oneshot::Sender<()>,
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
                    ExecutorRequest::Close => {
                        let _ = self.close_tx.send(());
                        return;
                    }
                }
            }
        });
    }
    async fn handle_push(&mut self, written: usize) {
        let mut sleep_time = Duration::from_millis(100);
        let mut read_count = 0;

        while read_count < written {
            let mut success = false;

            for _ in 0..=10 {
                match execute_one(
                    &self.wal_read_fd,
                    self.read_offset.get(),
                    &mut self.shadow_write_fd,
                )
                .await
                {
                    Ok(new_read_offset) => {
                        read_count += new_read_offset - self.read_offset.get();
                        self.read_offset.set(new_read_offset);
                        success = true;
                        break;
                    }
                    Err(err) => {
                        if err.is_corrupted() {
                            unreachable!("wal file is corrupted: {:?}", err);
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

            if !success {
                unreachable!("Too many retrial on writing staled pages");
            }
        }

        assert_eq!(written, read_count);
    }
}

/// # Return
/// The next read offset.
///
/// This function returns next read offset on successful execution
/// because it would be failed in the middle of the execution.
async fn execute_one(
    wal_read_fd: &ReadFd,
    mut wal_read_offset: usize,
    file_write_fd: &mut WriteFd,
) -> Result<usize> {
    let header = {
        let size = size_of::<WalHeader>();
        let header = wal_read_fd.read_init::<WalHeader>(wal_read_offset).await?;
        wal_read_offset += size;
        header
    };

    match header.body_types {
        // Init
        0 => {
            let root_node_offset = PageOffset::new(1);

            let header = Header::new(PageOffset::NULL, root_node_offset, PageOffset::new(2));

            let root_node = LeafNode::new(PageOffset::NULL, PageOffset::NULL);

            let mut bytes = Vec::with_capacity(size_of::<Header>() + size_of::<LeafNode>());
            bytes.put_slice(header.as_slice());
            bytes.put_slice(root_node.as_slice());

            file_write_fd.set_len(0)?;
            file_write_fd.write_exact(&bytes, 0)?;
        }
        // PutPage
        1 => {
            let body = {
                let body_length = header.body_length as usize;
                if body_length != size_of::<PutPage>() {
                    return Err(ExecuteError::WrongBodySize {
                        expected: size_of::<PutPage>(),
                        actual: body_length,
                    }
                    .into());
                }
                let body = wal_read_fd.read_init::<PutPage>(wal_read_offset).await?;
                wal_read_offset += body_length;
                body
            };

            let body_checksum = checksum(body.as_slice());
            let bad_checksum = body_checksum != header.checksum;
            if bad_checksum {
                return Err(ExecuteError::Checksum {
                    expected: header.checksum,
                    actual: body_checksum,
                }
                .into());
            }

            file_write_fd.write_exact(body.page.as_slice(), body.page_offset.file_offset())?;
        }
        body_type => {
            return Err(ExecuteError::WrongBodyType { body_type }.into());
        }
    }
    file_write_fd.fsync()?;

    Ok(wal_read_offset)
}

#[derive(Debug)]
enum ExecutorRequest {
    /// Push new wal record
    Push {
        written: usize,
    },
    /// Reset wal file
    Reset,
    Close,
}

#[derive(Debug)]
pub(crate) enum ExecuteError {
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
    #[allow(dead_code)]
    WrongBodySize {
        expected: usize,
        actual: usize,
    },
}

trait CorruptionCheck {
    fn is_corrupted(&self) -> bool;
}
impl CorruptionCheck for anyhow::Error {
    fn is_corrupted(&self) -> bool {
        if let Some(err) = self.downcast_ref::<ExecuteError>() {
            match err {
                ExecuteError::Checksum { .. } => true,
                ExecuteError::WrongBodyType { .. } => true,
                ExecuteError::ExecutorDown => false,
                ExecuteError::WrongBodySize { .. } => true,
            }
        } else if let Some(err) = self.downcast_ref::<std::io::Error>() {
            err.kind() == ErrorKind::UnexpectedEof
        } else {
            false
        }
    }
}
impl Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ExecuteError {}

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
