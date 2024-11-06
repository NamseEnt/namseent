use super::*;
use std::{
    fs::File,
    io::{Read, Seek},
    mem::MaybeUninit,
    path::Path,
    time::Duration,
};
use tokio::{sync::mpsc::Receiver, time::timeout};

pub struct Backend {
    file: File,
    wal: Wal,
    cache: PageCache,
}

impl Backend {
    pub fn open(
        path: impl AsRef<Path>,
        request_rx: Receiver<Request>,
        cache: PageCache,
    ) -> Result<()> {
        let path = path.as_ref();

        let mut wal = Wal::open(path.with_extension("wal"))?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        wal.flush(&mut file)?;

        if file.metadata()?.len() == 0 {
            wal.write_init()?;
            wal.flush(&mut file)?;
        }

        let this = Self { file, wal, cache };
        this.run(request_rx);
        Ok(())
    }

    fn run(mut self, mut request_rx: Receiver<Request>) {
        tokio::spawn(async move {
            loop {
                let Some(mut request) = request_rx.recv().await else {
                    break;
                };
                let mut operator = Operator::new(self.cache.load(), &mut self.file);

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

                    match timeout(Duration::from_millis(1), request_rx.recv()).await {
                        Ok(Some(_request)) => {
                            request = _request;
                        }
                        Ok(None) => break,
                        Err(_) => break,
                    }
                }

                if result.is_ok() {
                    let done = operator.done();
                    result = self.wal.update_pages(&done.updated_pages);
                    if result.is_ok() {
                        let mut new_cache = self.cache.clone_inner();
                        for (page_offset, page) in done.pages_read_from_file {
                            new_cache.insert(page_offset, page);
                        }
                        for (page_offset, page) in done.updated_pages {
                            new_cache.insert(page_offset, page);
                        }
                        self.cache.store(new_cache);
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
}

pub(crate) fn read_page_from_file(file: &mut File, page_offset: PageOffset) -> Result<Page> {
    file.seek(page_offset.file_pos())?;

    let page = unsafe {
        let mut page = MaybeUninit::<Page>::uninit();
        file.read_exact(std::slice::from_raw_parts_mut(
            page.as_mut_ptr() as *mut u8,
            std::mem::size_of::<Page>(),
        ))?;
        page.assume_init()
    };
    Ok(page)
}
