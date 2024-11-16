use super::*;
use bytes::{Buf, BufMut, Bytes};
use std::{os::fd::AsRawFd, path::Path};
use tokio::fs::{File, OpenOptions};

type Result<T> = std::io::Result<T>;

/**
 * # File layout
 * - Header
 *   - Data 1, 2 Headers
 *     - Offset: u64
 *     - Length: u64
 *     - Checksum: u64
 *     - Transaction ID: u128
 *     - Transaction Commit Checked: u8(bool)
 * - Data 1, 2: Bytes
 *
 * Data 1, 2 may not be right next to the header part.
 */

pub struct SimpleDocFile {
    file: File,
    file_id: u128,
    header1: Header,
    header2: Header,
    memory: Bytes,
    memory_before_commit: Bytes,
    trx_id_map: TrxIdMap,
}

impl SimpleDocFile {
    pub async fn open(
        dir_path: impl AsRef<Path>,
        filename: impl AsRef<Path>,
        file_id: u128,
        trx_id_map: TrxIdMap,
    ) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(dir_path.as_ref().join(&filename))
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

        let memory = read_last_value(&file, &mut header1, &mut header2, &trx_id_map, file_id)
            .await?
            .unwrap_or_else(Bytes::new);

        Ok(Self {
            file,
            file_id,
            header1,
            header2,
            memory,
            memory_before_commit: Bytes::new(),
            trx_id_map,
        })
    }

    pub fn get(&self) -> Bytes {
        self.memory.clone()
    }

    pub async fn put(&mut self, bytes: Bytes, trx_id: u128) -> Result<()> {
        let (header, other_side_header) = self.get_writable_header().await?;
        let offset = {
            if other_side_header.empty()
                || bytes.len() <= other_side_header.offset - Header::VALUE_START_OFFSET
            {
                Header::VALUE_START_OFFSET
            } else {
                other_side_header.offset + other_side_header.len
            }
        };

        header.offset = offset;
        header.len = bytes.len();
        header.checksum = checksum(&bytes);
        header.written_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        header.trx_id = trx_id;
        header.trx_commit_checked = false;

        let header_file_offset = header.file_offset();
        let header_bytes = Bytes::from(header.to_vec());
        pwrite(&self.file, header_file_offset, header_bytes).await?;
        pwrite(&self.file, offset, bytes.clone()).await?;
        self.file.set_len((offset + bytes.len()) as u64).await?;
        fsync(&self.file).await?;

        self.memory_before_commit = bytes;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.memory.is_empty()
    }

    pub async fn commit(&mut self, trx_id: u128) {
        self.memory = std::mem::take(&mut self.memory_before_commit);
        let header = if self.header1.trx_id == trx_id {
            &mut self.header1
        } else {
            &mut self.header2
        };
        header.trx_commit_checked = true;
        _ = header.update_trx_commit_checked(&self.file).await;
    }

    pub async fn rollback(&mut self, trx_id: u128) {
        self.memory_before_commit = Bytes::new();
        let header = if self.header1.trx_id == trx_id {
            &mut self.header1
        } else {
            &mut self.header2
        };
        header.offset = 0;
        _ = header.reset(&self.file).await;
    }

    async fn get_writable_header(&mut self) -> Result<(&mut Header, &Header)> {
        if self.header1.empty() {
            return Ok((&mut self.header1, &self.header2));
        } else if self.header2.empty() {
            return Ok((&mut self.header2, &self.header1));
        }

        let (early, later) = if self.header1.written_timestamp < self.header2.written_timestamp {
            (&mut self.header1, &mut self.header2)
        } else {
            (&mut self.header2, &mut self.header1)
        };

        if later.trx_commit_checked {
            Ok((early, later))
        } else if self
            .trx_id_map
            .check_trx_id(later.trx_id, self.file_id)
            .await
        {
            later.update_trx_commit_checked(&self.file).await?;
            Ok((early, later))
        } else {
            later.reset(&self.file).await?;
            Ok((later, early))
        }
    }
}

/// Returned Bytes may not be the same length as len.
async fn pread(file: &File, offset: usize, len: usize) -> Result<Bytes> {
    let fd = file.as_raw_fd();
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

async fn pwrite(file: &File, offset: usize, bytes: Bytes) -> Result<()> {
    let fd = file.as_raw_fd();
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

async fn fsync(file: &File) -> Result<()> {
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

async fn read_last_value(
    file: &File,
    header1: &mut Header,
    header2: &mut Header,
    trx_id_map: &TrxIdMap,
    file_id: u128,
) -> Result<Option<Bytes>> {
    if header1.empty() && header2.empty() {
        return Ok(None);
    }

    if header1.empty() {
        return single_header_case(file, header2, trx_id_map, file_id).await;
    } else if header2.empty() {
        return single_header_case(file, header1, trx_id_map, file_id).await;
    }

    if header1.written_timestamp > header2.written_timestamp {
        if header1.trx_commit_checked {
            return Ok(Some(header1.read_value(file).await?));
        } else if trx_id_map.check_trx_id(header1.trx_id, file_id).await {
            header1.update_trx_commit_checked(file).await?;

            return Ok(Some(header1.read_value(file).await?));
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
        Ok(if header.empty() {
            None
        } else if header.trx_commit_checked {
            Some(header.read_value(file).await?)
        } else if trx_id_map.check_trx_id(header.trx_id, file_id).await {
            header.update_trx_commit_checked(file).await?;
            Some(header.read_value(file).await?)
        } else {
            header.reset(file).await?;

            None
        })
    }
}

#[derive(Debug)]
struct Header {
    header_index: HeaderIndex,
    offset: usize, // u64,
    len: usize,    // u64,
    checksum: u64,
    written_timestamp: u128,
    trx_id: u128,
    trx_commit_checked: bool,
}

impl Header {
    const HEADER_SIZE: usize = size_of::<u64>() * 3 + size_of::<u128>() * 2 + size_of::<u8>();
    const NULL_HEADER_SLICE: [u8; Self::HEADER_SIZE] = [0; Self::HEADER_SIZE];
    const NULL_HEADER_BYTES: Bytes = Bytes::from_static(&Self::NULL_HEADER_SLICE);
    const TRX_COMMIT_CHECKED_OFFSET: usize = Self::HEADER_SIZE - size_of::<u8>();
    const VALUE_START_OFFSET: usize = Self::HEADER_SIZE * 2;

    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::HEADER_SIZE);
        buf.put_u64_le(self.offset as u64);
        buf.put_u64_le(self.len as u64);
        buf.put_u64_le(self.checksum);
        buf.put_u128_le(self.written_timestamp);
        buf.put_u128_le(self.trx_id);
        buf.put_u8(self.trx_commit_checked as u8);
        buf
    }
    fn from_slice(mut slice: &[u8], header_index: HeaderIndex) -> Self {
        Self {
            header_index,
            offset: slice.get_u64_le() as usize,
            len: slice.get_u64_le() as usize,
            checksum: slice.get_u64_le(),
            written_timestamp: slice.get_u128_le(),
            trx_id: slice.get_u128_le(),
            trx_commit_checked: slice.get_u8() != 0,
        }
    }
    fn empty(&self) -> bool {
        self.offset == 0
    }
    fn file_offset(&self) -> usize {
        match self.header_index {
            HeaderIndex::First => 0,
            HeaderIndex::Second => Self::HEADER_SIZE,
        }
    }
    async fn update_trx_commit_checked(&mut self, file: &File) -> Result<()> {
        pwrite(
            file,
            self.file_offset() + Header::TRX_COMMIT_CHECKED_OFFSET,
            Bytes::from_static(&[1]),
        )
        .await?;
        self.trx_commit_checked = true;
        Ok(())
    }
    async fn read_value(&self, file: &File) -> Result<Bytes> {
        pread(file, self.offset, self.len).await
    }

    async fn reset(&mut self, file: &File) -> Result<()> {
        pwrite(file, self.file_offset(), Header::NULL_HEADER_BYTES).await?;
        self.offset = 0;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum HeaderIndex {
    First,
    Second,
}

fn checksum(values: &[u8]) -> u64 {
    crc::Crc::<u64>::new(&crc::CRC_64_REDIS).checksum(values)
}
