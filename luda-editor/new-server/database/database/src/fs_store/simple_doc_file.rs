use super::*;
use bytes::{Buf, BufMut, Bytes};
use futures::future::try_join3;
use std::{os::fd::AsRawFd, path::Path};
use tokio::fs::{File, OpenOptions};

type Result<T> = std::io::Result<T>;

/**
 * # File layout
 * - Header
 *   - Data 1, 2 Headers
 *     - Offset: u64
 *     - Length: u64
 *       - u64::MAX means empty
 *     - Transaction ID: u128
 * - Data 1, 2: Bytes
 *
 * Data 1, 2 may not be right next to the header part.
 */
pub struct SimpleDocFile {
    file: File,
    file_id: u128,
    header1: Header,
    header2: Header,
    memory: Option<Bytes>,
    memory_before_commit: Option<Bytes>,
    trx_id_map: TrxIdMap,
}

impl SimpleDocFile {
    pub async fn open(
        dir_path: impl AsRef<Path>,
        filename: impl AsRef<Path>,
        file_id: u128,
        trx_id_map: TrxIdMap,
    ) -> Result<Self> {
        let dir_path = dir_path.as_ref();
        let file_name = filename.as_ref();
        let file_path = dir_path.join(file_name);
        tokio::fs::create_dir_all(&file_path.parent().unwrap()).await?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(file_path)
            .await?;

        let mut header1 = {
            let bytes = pread(&file, 0, Header::HEADER_SIZE).await?;
            Header::from_slice(
                if bytes.len() < Header::HEADER_SIZE {
                    &Header::NULL_HEADER_SLICE
                } else {
                    &bytes
                },
                HeaderIndex::First,
            )
        };
        let mut header2 = {
            let bytes = pread(&file, Header::HEADER_SIZE, Header::HEADER_SIZE).await?;
            Header::from_slice(
                if bytes.len() < Header::HEADER_SIZE {
                    &Header::NULL_HEADER_SLICE
                } else {
                    &bytes
                },
                HeaderIndex::Second,
            )
        };

        let memory =
            read_last_value(&file, &mut header1, &mut header2, &trx_id_map, file_id).await?;

        Ok(Self {
            file,
            file_id,
            header1,
            header2,
            memory,
            memory_before_commit: Default::default(),
            trx_id_map,
        })
    }

    pub fn get(&self) -> Option<Bytes> {
        self.memory.clone()
    }

    /// This method is not applied to the file until `commit` is called.
    pub async fn put(&mut self, value: Vec<u8>, trx_id: u128) -> Result<()> {
        let bytes = Bytes::from(value);
        self.write_to_file(Some(bytes.clone()), trx_id).await?;
        self.memory_before_commit = Some(bytes);
        Ok(())
    }

    pub async fn delete(&mut self, trx_id: u128) -> Result<()> {
        self.write_to_file(None, trx_id).await?;
        self.memory_before_commit = None;
        Ok(())
    }

    pub fn commit(&mut self, trx_id: u128) {
        self.memory = std::mem::take(&mut self.memory_before_commit);
        let header = if self.header1.trx_id == trx_id {
            &mut self.header1
        } else {
            &mut self.header2
        };
        header.trx_commit_checked = true;
    }

    async fn determine_headers(&mut self) -> (&mut Header, &Header) {
        if self.header1.null() {
            return (&mut self.header1, &self.header2);
        } else if self.header2.null() {
            return (&mut self.header2, &self.header1);
        }

        let (early, later) = if self.header1.written_timestamp < self.header2.written_timestamp {
            (&mut self.header1, &mut self.header2)
        } else {
            (&mut self.header2, &mut self.header1)
        };

        if later.trx_commit_checked
            || self
                .trx_id_map
                .check_trx_id(later.trx_id, self.file_id)
                .await
        {
            (early, later)
        } else {
            (later, early)
        }
    }

    async fn write_to_file(&mut self, bytes: Option<Bytes>, trx_id: u128) -> Result<()> {
        let fd = self.file.as_raw_fd();
        let real_bytes_len = bytes.as_ref().map_or(0, |b| b.len());

        let (header, other_header) = self.determine_headers().await;
        let offset = {
            if other_header.null()
                || real_bytes_len <= other_header.offset - Header::VALUE_START_OFFSET
            {
                Header::VALUE_START_OFFSET
            } else {
                other_header.offset + other_header.len
            }
        };

        header.offset = offset;
        header.len = bytes.as_ref().map_or(u64::MAX as usize, |b| b.len());
        header.written_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        header.trx_id = trx_id;
        header.trx_commit_checked = false;

        let header_file_offset = header.file_offset();
        let header_bytes = Bytes::from(header.to_vec());

        try_join3(
            pwrite(&fd, header_file_offset, header_bytes),
            async {
                if let Some(bytes) = bytes {
                    pwrite(&fd, offset, bytes).await?;
                }
                Ok(())
            },
            async {
                if other_header.offset < header.offset {
                    file_set_len(&fd, (offset + real_bytes_len) as u64).await?;
                }
                Ok(())
            },
        )
        .await?;
        fsync(&fd).await?;

        Ok(())
    }
}

/// Returned Bytes may not be the same length as len.
async fn pread(fd: &impl AsRawFd, offset: usize, len: usize) -> Result<Bytes> {
    let fd = fd.as_raw_fd();
    tokio::task::spawn_blocking(move || {
        let mut buf = vec![0; len];
        let mut read_offset = 0;
        while read_offset < len {
            let read = unsafe {
                libc::pread(
                    fd,
                    buf.as_mut_ptr().add(read_offset) as *mut _,
                    len,
                    (offset + read_offset) as i64,
                )
            };
            if read == -1 {
                return Err(std::io::Error::last_os_error());
            }
            if read == 0 {
                break;
            }
            read_offset += read as usize;
        }
        Ok(Bytes::from(buf).split_to(len))
    })
    .await?
}

async fn pwrite(fd: &impl AsRawFd, offset: usize, bytes: Bytes) -> Result<()> {
    let fd = fd.as_raw_fd();
    tokio::task::spawn_blocking(move || {
        let mut write_offset = 0;
        while write_offset < bytes.len() {
            let written = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_ptr().add(write_offset) as *const _,
                    bytes.len(),
                    (offset + write_offset) as i64,
                )
            };
            if written == -1 {
                return Err(std::io::Error::last_os_error());
            }
            if written == 0 {
                break;
            }
            write_offset += written as usize;
        }
        Ok(())
    })
    .await?
}

async fn fsync(file: &impl AsRawFd) -> Result<()> {
    let fd = file.as_raw_fd();
    tokio::task::spawn_blocking(move || {
        let ret = unsafe { libc::fsync(fd) };
        if ret == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    })
    .await?
}

async fn file_set_len(file: &impl AsRawFd, len: u64) -> Result<()> {
    let fd = file.as_raw_fd();
    tokio::task::spawn_blocking(move || {
        let ret = unsafe { libc::ftruncate(fd, len as i64) };
        if ret == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    })
    .await?
}

async fn read_last_value(
    file: &File,
    header1: &mut Header,
    header2: &mut Header,
    trx_id_map: &TrxIdMap,
    file_id: u128,
) -> Result<Option<Bytes>> {
    if header1.null() && header2.null() {
        return Ok(None);
    }

    if header1.null() {
        return single_header_case(file, header2, trx_id_map, file_id).await;
    } else if header2.null() {
        return single_header_case(file, header1, trx_id_map, file_id).await;
    }

    if header1.written_timestamp > header2.written_timestamp {
        if header1.trx_commit_checked || trx_id_map.check_trx_id(header1.trx_id, file_id).await {
            return header1.read_value(file).await;
        } else {
            header1.reset(file).await?;
        }
    }
    return single_header_case(file, header2, trx_id_map, file_id).await;

    async fn single_header_case(
        file: &File,
        header: &mut Header,
        trx_id_map: &TrxIdMap,
        file_id: u128,
    ) -> Result<Option<Bytes>> {
        Ok(if header.null() {
            None
        } else if header.trx_commit_checked || trx_id_map.check_trx_id(header.trx_id, file_id).await
        {
            header.read_value(file).await?
        } else {
            header.reset(file).await?;

            None
        })
    }
}

#[derive(Debug, Clone)]
struct Header {
    header_index: HeaderIndex,
    offset: usize, // u64,
    len: usize,    // u64,
    written_timestamp: u128,
    trx_id: u128,
    /// in-memory flag
    trx_commit_checked: bool,
}

impl Header {
    const HEADER_SIZE: usize = size_of::<u64>() * 2 + size_of::<u128>() * 2;
    const NULL_HEADER_SLICE: [u8; Self::HEADER_SIZE] = [0; Self::HEADER_SIZE];
    const NULL_HEADER_BYTES: Bytes = Bytes::from_static(&Self::NULL_HEADER_SLICE);
    const VALUE_START_OFFSET: usize = Self::HEADER_SIZE * 2;

    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::HEADER_SIZE);
        buf.put_u64_le(self.offset as u64);
        buf.put_u64_le(self.len as u64);
        buf.put_u128_le(self.written_timestamp);
        buf.put_u128_le(self.trx_id);
        buf
    }
    fn from_slice(mut slice: &[u8], header_index: HeaderIndex) -> Self {
        Self {
            header_index,
            offset: slice.get_u64_le() as usize,
            len: slice.get_u64_le() as usize,
            written_timestamp: slice.get_u128_le(),
            trx_id: slice.get_u128_le(),
            trx_commit_checked: false,
        }
    }
    fn null(&self) -> bool {
        self.offset == 0
    }
    fn file_offset(&self) -> usize {
        match self.header_index {
            HeaderIndex::First => 0,
            HeaderIndex::Second => Self::HEADER_SIZE,
        }
    }
    async fn read_value(&self, fd: &impl AsRawFd) -> Result<Option<Bytes>> {
        if self.len == u64::MAX as usize {
            return Ok(None);
        }
        pread(fd, self.offset, self.len).await.map(Some)
    }
    async fn reset(&mut self, fd: &impl AsRawFd) -> Result<()> {
        pwrite(fd, self.file_offset(), Header::NULL_HEADER_BYTES).await?;
        self.offset = 0;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum HeaderIndex {
    First,
    Second,
}
