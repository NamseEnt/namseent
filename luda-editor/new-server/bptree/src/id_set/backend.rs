use super::*;
use std::{path::Path, time::Duration};
use tokio::{sync::mpsc::Receiver, time::timeout};

pub struct Backend {
    file_read_fd: ReadFd,
    file_write_fd: WriteFd,
    wal: Wal,
    cache: PageCache,
    request_rx: Receiver<Request>,
    backend_close_tx: oneshot::Sender<()>,
}

impl Backend {
    pub async fn open(
        path: impl AsRef<Path>,
        request_rx: Receiver<Request>,
        cache: PageCache,
        backend_close_tx: oneshot::Sender<()>,
    ) -> Result<()> {
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

                let mut result = Ok(());

                let start_time = tokio::time::Instant::now();

                loop {
                    for request in requests.drain(..) {
                        match request {
                            Request::Insert { id, tx } => {
                                txs.push(Tx::Insert { tx });
                                result = operator.insert(id).await;
                            }
                            Request::Delete { id, tx } => {
                                txs.push(Tx::Delete { tx });
                                result = operator.delete(id).await;
                            }
                            Request::Contains { id, tx } => {
                                let contains_result = operator.contains(id).await;
                                let tx_result;
                                match contains_result {
                                    Ok(contains) => {
                                        tx_result = Ok(contains);
                                        result = Ok(());
                                    }
                                    Err(err) => {
                                        tx_result = Err(());
                                        result = Err(err);
                                    }
                                }
                                txs.push(Tx::Contains {
                                    tx,
                                    result: tx_result,
                                });
                            }
                            Request::Next {
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
                                        result = Err(err);
                                    }
                                };

                                txs.push(Tx::Next {
                                    tx,
                                    result: tx_result,
                                });
                            }
                            Request::Close => {
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

                    result = self.wal.update_pages(&updated_pages);

                    if let Err(err) = &result {
                        if let Some(ExecuteError::ExecutorDown) = err.downcast_ref::<ExecuteError>()
                        {
                            eprintln!("Executor down!");
                            break 'outer;
                        }
                    }

                    if result.is_ok() {
                        let mut new_pages = pages_read_from_file;
                        new_pages.append(&mut updated_pages);
                        let stale_tuples = self.cache.push(new_pages);
                        if let Err(err) = self.write_staled_pages(stale_tuples).await {
                            eprintln!("Error on writing staled pages: {:?}", err);
                            break 'outer;
                        }
                    }
                }

                if result.is_err() {
                    eprintln!("Error: {:?}", result);
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

    /// Don't fsync
    async fn write_staled_pages(&mut self, stale_tuples: Vec<(PageOffset, Page)>) -> Result<()> {
        if stale_tuples.is_empty() {
            return Ok(());
        }
        let mut sleep_time = Duration::from_millis(100);
        for _ in 0..=10 {
            let result: Result<()> = (|| {
                for (offset, page) in &stale_tuples {
                    self.file_write_fd
                        .write_exact(page.as_slice(), offset.file_offset())?;
                }
                Ok(())
            })();

            if result.is_ok() {
                return Ok(());
            }

            eprintln!("Error on writing staled pages: {:?}", result);
            tokio::time::sleep(sleep_time).await;
            sleep_time = (sleep_time * 2).max(Duration::from_secs(4));
        }

        anyhow::bail!("Too many retrial on writing staled pages");
    }
}

enum Tx {
    Insert {
        tx: oneshot::Sender<Result<(), ()>>,
    },
    Delete {
        tx: oneshot::Sender<Result<(), ()>>,
    },
    Contains {
        tx: oneshot::Sender<Result<bool, ()>>,
        result: Result<bool, ()>,
    },
    Next {
        tx: oneshot::Sender<Result<Option<Vec<Id>>, ()>>,
        result: Result<Option<Vec<Id>>, ()>,
    },
}
