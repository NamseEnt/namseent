use super::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem::MaybeUninit,
    path::Path,
    time::Duration,
};
use tokio::{sync::mpsc::Receiver, time::timeout};

pub struct Backend {
    file: File,
    wal: Wal,
    header: Header,
    // TODO: Remove nodes cache for memory usage.
    pages: HashMap<PageOffset, Page>,
}

impl Backend {
    pub fn open(path: impl AsRef<Path>, request_rx: Receiver<Request>) -> Result<()> {
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

        let this = Self::read_from_file(file, wal)?;
        this.run(request_rx);
        Ok(())
    }
    pub fn delete(&mut self, id: Id) -> Result<()> {
        todo!()
    }
    pub fn iter(&self) -> Result<impl Iterator<Item = Id>> {
        // TODO
        Ok(std::iter::empty())
    }
    fn apply_operator_done(&mut self, done: operator::Done) -> Result<()> {
        self.wal.update_pages(&done.updated_pages)?;
        if let Some(header) = done.updated_header {
            self.header = header;
        }
        for (page_offset, page) in done.pages_read_from_file {
            self.pages.insert(page_offset, page);
        }
        for (page_offset, page) in done.updated_pages {
            self.pages.insert(page_offset, page);
        }
        Ok(())
    }

    fn read_from_file(mut file: File, wal: Wal) -> Result<Self> {
        let header = unsafe {
            let mut header = MaybeUninit::<Header>::uninit();
            file.seek(SeekFrom::Start(0))?;
            file.read_exact(std::slice::from_raw_parts_mut(
                header.as_mut_ptr() as *mut u8,
                std::mem::size_of::<Header>(),
            ))?;
            header.assume_init()
        };

        Ok(Self {
            file,
            wal,
            header,
            pages: HashMap::new(),
        })
    }

    fn run(mut self, mut request_rx: Receiver<Request>) {
        tokio::spawn(async move {
            loop {
                let Some(mut request) = request_rx.recv().await else {
                    break;
                };
                let mut operator = Operator::new(&self.header, &self.pages, &mut self.file);
                let mut txs = Vec::new();

                // false positive warning
                #[allow(unused_assignments)]
                let mut result = None;

                let start_time = tokio::time::Instant::now();

                loop {
                    match request {
                        Request::Insert { id, tx } => {
                            txs.push(tx);
                            result = Some(operator.insert(id));
                            if result.as_ref().unwrap().is_err() {
                                break;
                            }
                        }
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

                let mut result = result.unwrap();

                if result.is_ok() {
                    let done = operator.done();
                    result = self.wal.update_pages(&done.updated_pages);
                    if result.is_ok() {
                        if let Some(updated_header) = done.updated_header {
                            self.header = updated_header;
                        }
                        for (page_offset, page) in done.pages_read_from_file {
                            self.pages.insert(page_offset, page);
                        }
                        for (page_offset, page) in done.updated_pages {
                            self.pages.insert(page_offset, page);
                        }
                    }
                }

                if result.is_err() {
                    eprintln!("Error: {:?}", result);
                }

                let result_to_send = match result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                };
                txs.into_iter().for_each(|tx| {
                    let _ = tx.send(result_to_send);
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
