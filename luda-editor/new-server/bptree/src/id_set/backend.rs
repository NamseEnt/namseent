use super::*;
use std::{path::Path, time::Duration};
use tokio::{sync::mpsc::Receiver, time::timeout};

pub struct Backend {
    file_read_fd: ReadFd,
    file_write_fd: WriteFd,
    wal: Wal,
    cache: PageCache,
    request_rx: Receiver<Request>,
}

impl Backend {
    pub fn open(
        path: impl AsRef<Path>,
        request_rx: Receiver<Request>,
        cache: PageCache,
    ) -> Result<()> {
        let path = path.as_ref();

        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        let (file_read_fd, mut file_write_fd) = split_file(file);

        let wal = Wal::open(path.with_extension("wal"), &mut file_write_fd)?;

        let this = Self {
            file_read_fd,
            wal,
            cache,
            request_rx,
            file_write_fd,
        };
        this.run();
        Ok(())
    }

    fn run(mut self) {
        tokio::spawn(async move {
            loop {
                let Some(mut request) = self.request_rx.recv().await else {
                    break;
                };

                let mut operator = Operator::new(self.cache.load(), self.file_read_fd.clone());

                let mut txs = Vec::<Tx>::new();

                let mut result;

                let start_time = tokio::time::Instant::now();

                loop {
                    match request {
                        Request::Insert { id, tx } => {
                            txs.push(Tx::Insert { tx });
                            result = operator.insert(id);
                        }
                        Request::Delete { id, tx } => {
                            txs.push(Tx::Delete { tx });
                            result = operator.delete(id);
                        }
                        Request::Contains { id, tx } => {
                            let mut contains = false;
                            let contains_result = operator.contains(id);
                            if let Ok(true) = contains_result {
                                contains = true;
                            }
                            let tx = Tx::Contains { tx, contains };
                            txs.push(tx);
                            result = contains_result.map(|_| ());
                        }
                    }

                    if result.is_err() {
                        break;
                    }

                    if start_time.elapsed() > Duration::from_millis(4) {
                        break;
                    }

                    match timeout(Duration::from_millis(1), self.request_rx.recv()).await {
                        Ok(Some(_request)) => {
                            request = _request;
                        }
                        Ok(None) => break,
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
                            return;
                        }
                    }

                    if result.is_ok() {
                        let mut new_pages = pages_read_from_file;
                        new_pages.append(&mut updated_pages);
                        let stale_tuples = self.cache.push(new_pages);
                        self.write_staled_pages(stale_tuples).await;
                    }
                }

                if result.is_err() {
                    eprintln!("Error: {:?}", result);
                }

                let result_to_send = match result.is_ok() {
                    true => Ok(()),
                    false => Err(()),
                };
                txs.into_iter().for_each(|tx| match tx {
                    Tx::Insert { tx } | Tx::Delete { tx } => {
                        _ = tx.send(result_to_send);
                    }
                    Tx::Contains { tx, contains } => {
                        if result_to_send.is_err() {
                            _ = tx.send(Err(()));
                        } else {
                            _ = tx.send(Ok(contains));
                        }
                    }
                });
            }
        });
    }

    /// Don't fsync
    async fn write_staled_pages(&mut self, stale_tuples: Vec<(PageOffset, Page)>) {
        if stale_tuples.is_empty() {
            return;
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
                return;
            }

            eprintln!("Error on writing staled pages: {:?}", result);
            tokio::time::sleep(sleep_time).await;
            sleep_time = (sleep_time * 2).max(Duration::from_secs(4));
        }

        unreachable!("Too many retrial on writing staled pages");
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
        contains: bool,
    },
}
