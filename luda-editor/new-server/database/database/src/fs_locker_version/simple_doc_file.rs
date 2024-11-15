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
    header1: Option<Header>,
    header2: Option<Header>,
    memory: Bytes,
    memory_before_commit: Bytes,
}

struct Header {
    offset: usize, // u64,
    len: usize,    // u64,
    checksum: u64,
    written_timestamp: u128,
    trx_id: u128,
    trx_commit_checked: bool,
}

impl Header {
    const HEADER_SIZE: usize = size_of::<u64>() * 3 + size_of::<u128>() * 2 + size_of::<u8>();
    const NULL_HEADER_BYTES: Bytes = Bytes::from_static(&[0; Self::HEADER_SIZE]);
    const TRX_COMMIT_CHECKED_OFFSET: usize = Self::HEADER_SIZE - size_of::<u8>();
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
    fn from_slice(mut slice: &[u8]) -> Self {
        Self {
            offset: slice.get_u64_le() as usize,
            len: slice.get_u64_le() as usize,
            checksum: slice.get_u64_le(),
            written_timestamp: slice.get_u128_le(),
            trx_id: slice.get_u128_le(),
            trx_commit_checked: slice.get_u8() != 0,
        }
    }
}

impl SimpleDocFile {
    pub async fn open(
        dir_path: impl AsRef<Path>,
        filename: impl AsRef<Path>,
        file_id: u128,
        trx_id_map: &TrxIdMap,
    ) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(dir_path.as_ref().join(&filename))
            .await?;

        let header1 = {
            let header1_bytes = pread(&file, 0, Header::HEADER_SIZE).await?;
            if header1_bytes.len() < Header::HEADER_SIZE {
                None
            } else {
                Some(Header::from_slice(&header1_bytes))
            }
        };
        let header2 = {
            let header2_bytes = pread(&file, Header::HEADER_SIZE, Header::HEADER_SIZE).await?;
            if header2_bytes.len() < Header::HEADER_SIZE {
                None
            } else {
                Some(Header::from_slice(&header2_bytes))
            }
        };

        let memory = match (&header1, &header2) {
            (None, None) => Bytes::new(),
            (None, Some(_)) => unreachable!(),
            (Some(header1), None) => {
                if header1.offset == 0 {
                    Bytes::new()
                } else if header1.trx_commit_checked {
                    pread(&file, header1.offset, header1.len).await?
                } else if trx_id_map.check_trx_id(header1.trx_id, file_id).await {
                    pwrite(
                        &file,
                        Header::TRX_COMMIT_CHECKED_OFFSET,
                        Bytes::from_static(&[1]),
                    )
                    .await?;
                    pread(&file, header1.offset, header1.len).await?
                } else {
                    pwrite(&file, 0, Header::NULL_HEADER_BYTES).await?;
                    Bytes::new()
                }
            }
            (Some(_), Some(_)) => todo!(),
        };

        Ok(Self {
            file,
            file_id,
            header1,
            header2,
            memory,
            memory_before_commit: Bytes::new(),
        })
    }

    pub fn get(&self) -> Bytes {
        self.memory.clone()
    }

    pub fn put(&mut self, bytes: Bytes, trx_id: u128) -> Result<()> {
        append_to_wal(&mut self.wal_file, &bytes, trx_id)?;
        self.memory_before_commit = bytes;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.memory.is_empty()
    }

    pub fn commit(&mut self) {
        self.memory = std::mem::take(&mut self.memory_before_commit);
    }

    pub fn rollback(&mut self) {
        self.memory_before_commit = Bytes::new();
    }
}

fn append_to_wal(wal_file: &mut File, bytes: &Bytes, trx_id: u128) -> Result<()> {
    todo!()
}

fn last_version_in_wal(wal_file: &mut File) -> Result<Option<Bytes>> {
    todo!()
}

/// Returned Bytes may not be the same length as len.
async fn pread(file: &File, offset: usize, len: usize) -> Result<Bytes> {
    let fd = file.as_raw_fd();
    tokio::task::spawn_blocking(move || {
        let mut buf = vec![0; len as usize];
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
