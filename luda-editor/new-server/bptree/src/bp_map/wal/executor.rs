use super::*;
use bytes::Buf;
use std::time::Duration;

pub(crate) struct Executor {
    wal_read_fd: ReadFd,
    shadow_write_fd: WriteFd,
    rx: mpsc::UnboundedReceiver<ExecutorRequest>,
    read_offset: ReadOffset,
    close_tx: oneshot::Sender<()>,
}

impl Executor {
    pub(crate) fn new(
        wal_read_fd: ReadFd,
        shadow_write_fd: WriteFd,
        rx: mpsc::UnboundedReceiver<ExecutorRequest>,
        read_offset: ReadOffset,
        close_tx: oneshot::Sender<()>,
    ) -> Self {
        Self {
            wal_read_fd,
            shadow_write_fd,
            rx,
            read_offset,
            close_tx,
        }
    }
    pub(crate) fn start(mut self) {
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
    pub(crate) async fn handle_push(&mut self, written: usize) {
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
                            "Error on execute wal record. error: {err:?} Retry after {sleep_time:?}"
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
pub(crate) async fn execute_one(
    wal_read_fd: &ReadFd,
    mut wal_read_offset: usize,
    file_write_fd: &mut WriteFd,
) -> Result<usize> {
    let header = {
        let bytes = wal_read_fd
            .read_exact(wal_read_offset, WAL_HEADER_SIZE)
            .await?;
        let header = WalHeader::from_slice(&bytes);
        wal_read_offset += bytes.len();
        header
    };

    match header.body_types {
        // Init
        0 => {
            let root_node_offset = PageOffset::new(1);

            let header = Header::new(PageOffset::NULL, root_node_offset, PageOffset::new(2));

            let root_node = LeafNode::new(PageOffset::NULL);

            let mut bytes = Vec::with_capacity(size_of::<Header>() + size_of::<LeafNode>());
            bytes.put_slice(&header.to_vec());
            bytes.put_slice(&root_node.to_vec());

            file_write_fd.set_len(0)?;
            file_write_fd.write_exact(&bytes, 0)?;
        }
        // PutPageBlocks
        1 => {
            let body = {
                let body_length: usize = header.body_length as usize;

                if !body_length.is_multiple_of(PutPageBlocks::CHUNK_SIZE) {
                    return Err(ExecuteError::WrongBodyLength { body_length }.into());
                }
                let body = wal_read_fd.read_exact(wal_read_offset, body_length).await?;
                wal_read_offset += body.len();
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

            // Below code assume that all body is OK by checksum.

            let mut body_reader = body.reader();
            let body_reader = body_reader.get_mut();
            while body_reader.remaining() > 0 {
                let page_offset = PageOffset::new(body_reader.get_u32_le());
                let page_count = body_reader.get_u8();

                let byte_len = page_count as usize * PAGE_LEN;

                let page_bytes = &body_reader[..byte_len];

                file_write_fd.write_exact(page_bytes, page_offset.file_offset())?;
                body_reader.advance(byte_len);
            }
        }
        body_type => {
            return Err(ExecuteError::WrongBodyType { body_type }.into());
        }
    }
    file_write_fd.fsync()?;

    Ok(wal_read_offset)
}

#[derive(Debug)]
pub(crate) enum ExecutorRequest {
    /// Push new wal record
    Push {
        written: usize,
    },
    /// Reset wal file
    Reset,
    Close,
}

#[derive(Debug, Error)]
pub(crate) enum ExecuteError {
    #[error("checksum error: expected={expected}, actual={actual}")]
    Checksum { expected: u64, actual: u64 },
    #[error("wrong body type: {body_type}")]
    WrongBodyType { body_type: u8 },
    #[error("wrong body length: {body_length}")]
    WrongBodyLength { body_length: usize },
}
