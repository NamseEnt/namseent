use super::*;
use std::{path::Path, time::Duration};
use tokio::{sync::mpsc::Receiver, time::timeout};

type Result<T> = std::result::Result<T, BackendError>;

pub struct Backend {
    file_read_fd: ReadFd,
    file_write_fd: WriteFd,
    wal: Wal,
    cache: PageCache,
    request_rx: Receiver<FeBeRequest>,
    backend_close_tx: oneshot::Sender<()>,
}

impl Backend {
    pub async fn open(
        path: impl AsRef<Path>,
        request_rx: Receiver<FeBeRequest>,
        cache: PageCache,
        backend_close_tx: oneshot::Sender<()>,
    ) -> std::io::Result<()> {
        let path = path.as_ref();

        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)
            .await?;

        let (file_read_fd, mut file_write_fd) = split_file(file.into_std().await);

        let wal = Wal::open(path.with_extension("wal"), &mut file_write_fd).await?;

        let this = Self {
            file_read_fd,
            wal,
            cache,
            request_rx,
            file_write_fd,
            backend_close_tx,
        };
        this.run();
        Ok(())
    }

    fn run(mut self) {
        tokio::spawn(async move {
            let mut close_requested = false;
            'outer: while !close_requested {
                const LIMIT: usize = 64;
                let mut requests = Vec::with_capacity(LIMIT);

                if self.request_rx.recv_many(&mut requests, LIMIT).await == 0 {
                    break 'outer;
                };

                let mut operator = Operator::new(self.cache.load_full(), self.file_read_fd.clone());

                let mut txs = Vec::<Tx>::new();

                let mut result: Result<()> = Ok(());

                let start_time = tokio::time::Instant::now();

                loop {
                    for request in requests.drain(..) {
                        match request {
                            FeBeRequest::Insert { id, tx } => {
                                txs.push(Tx::Insert { tx });
                                result = operator.insert(id).await.map_err(BackendError::from);
                            }
                            FeBeRequest::Delete { id, tx } => {
                                txs.push(Tx::Delete { tx });
                                result = operator.delete(id).await.map_err(BackendError::from);
                            }
                            FeBeRequest::Contains { id, tx } => {
                                let contains_result = operator.contains(id).await;
                                let tx_result;
                                match contains_result {
                                    Ok(contains) => {
                                        tx_result = Ok(contains);
                                        result = Ok(());
                                    }
                                    Err(err) => {
                                        tx_result = Err(());
                                        result = Err(err.into());
                                    }
                                }
                                txs.push(Tx::Contains {
                                    tx,
                                    result: tx_result,
                                });
                            }
                            FeBeRequest::Next {
                                exclusive_start_id,
                                tx,
                            } => {
                                let next_result = operator.next(exclusive_start_id).await;
                                let tx_result;
                                match next_result {
                                    Ok(ids) => {
                                        tx_result = Ok(ids);
                                        result = Ok(());
                                    }
                                    Err(err) => {
                                        tx_result = Err(());
                                        result = Err(err.into());
                                    }
                                };

                                txs.push(Tx::Next {
                                    tx,
                                    result: tx_result,
                                });
                            }
                            FeBeRequest::Close => {
                                close_requested = true;
                            }
                        }
                    }

                    if close_requested {
                        break;
                    }

                    if result.is_err() {
                        break;
                    }

                    if start_time.elapsed() > Duration::from_millis(4) {
                        break;
                    }

                    match timeout(
                        Duration::from_millis(1),
                        self.request_rx.recv_many(&mut requests, LIMIT),
                    )
                    .await
                    {
                        Ok(recv) => {
                            if recv == 0 {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }

                if result.is_ok() {
                    let Done {
                        mut updated_pages,
                        pages_read_from_file,
                    } = operator.done();

                    result = self
                        .wal
                        .update_pages(&updated_pages)
                        .map_err(BackendError::from);

                    if let Err(BackendError::Wal(WalError::ExecutorDown)) = &result {
                        eprintln!("Executor down!");
                        break 'outer;
                    }

                    if result.is_ok() {
                        let mut new_pages = pages_read_from_file;
                        new_pages.append(&mut updated_pages);
                        let stale_tuples = self.cache.push(new_pages);
                        if self.write_staled_pages(stale_tuples).await.is_err() {
                            break 'outer;
                        }
                    }
                }

                if result.is_err() {
                    eprintln!("Error: {result:?}");
                }

                let no_error = result.is_ok();

                txs.into_iter().for_each(|tx| match tx {
                    Tx::Insert { tx } | Tx::Delete { tx } => {
                        _ = tx.send(if no_error { Ok(()) } else { Err(()) });
                    }
                    Tx::Contains { tx, result } => {
                        _ = tx.send(if no_error {
                            assert!(result.is_ok());
                            result
                        } else {
                            Err(())
                        });
                    }
                    Tx::Next { tx, result } => {
                        _ = tx.send(if no_error {
                            assert!(result.is_ok());
                            result
                        } else {
                            Err(())
                        });
                    }
                });
            }

            _ = self.wal.close().await;
            _ = self.backend_close_tx.send(());
        });
    }

    /// NOTE: Don't fsync here.
    async fn write_staled_pages(
        &mut self,
        stale_tuples: Vec<(PageOffset, Page)>,
    ) -> std::result::Result<(), ()> {
        if stale_tuples.is_empty() {
            return Ok(());
        }
        let mut sleep_time = Duration::from_millis(100);
        for _ in 0..=10 {
            let result: std::io::Result<()> = (|| {
                for (offset, page) in &stale_tuples {
                    self.file_write_fd
                        .write_exact(page.as_slice(), offset.file_offset())?;
                }
                Ok(())
            })();

            if result.is_ok() {
                return Ok(());
            }

            eprintln!("Error on writing staled pages: {result:?}");
            tokio::time::sleep(sleep_time).await;
            sleep_time = (sleep_time * 2).max(Duration::from_secs(4));
        }

        Err(())
    }
}

#[derive(Debug, Error)]
enum BackendError {
    #[error("Error on wal: {0}")]
    Wal(#[from] WalError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

enum Tx {
    Insert {
        tx: oneshot::Sender<std::result::Result<(), ()>>,
    },
    Delete {
        tx: oneshot::Sender<std::result::Result<(), ()>>,
    },
    Contains {
        tx: oneshot::Sender<std::result::Result<bool, ()>>,
        result: std::result::Result<bool, ()>,
    },
    Next {
        tx: oneshot::Sender<std::result::Result<Option<Vec<Id>>, ()>>,
        result: std::result::Result<Option<Vec<Id>>, ()>,
    },
}
