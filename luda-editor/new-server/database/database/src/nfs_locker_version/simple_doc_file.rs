use bytes::Bytes;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

type Result<T> = std::io::Result<T>;

pub struct SimpleDocFile {
    main_file: std::fs::File,
    wal_file: std::fs::File,
    memory: Bytes,
}

impl SimpleDocFile {
    pub fn open(dir_path: impl AsRef<Path>, filename: String) -> Result<Self> {
        let mut main_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dir_path.as_ref().join(&filename).with_extension("main"))?;

        let mut wal_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dir_path.as_ref().join(&filename).with_extension("wal"))?;

        let memory = match last_version_in_wal(&mut wal_file)? {
            Some(last_version) => {
                main_file.write_all(&last_version)?;
                main_file.sync_all()?;
                wal_file.set_len(0)?;
                last_version
            }
            None => {
                let mut buf = vec![];
                main_file.read_to_end(&mut buf)?;
                Bytes::from(buf)
            }
        };

        let memory = Bytes::new();
        Ok(Self {
            main_file,
            wal_file,
            memory,
        })
    }

    pub fn get(&self) -> Bytes {
        self.memory.clone()
    }

    pub fn put(&mut self, bytes: Bytes, trx_id: u64) -> Result<()> {
        append_to_wal(&mut self.wal_file, &bytes, trx_id)?;
        self.memory = bytes;
        Ok(())
    }
}

fn append_to_wal(wal_file: &mut File, bytes: &Bytes, trx_id: u64) -> Result<()> {
    todo!()
}

fn last_version_in_wal(wal_file: &mut File) -> Result<Option<Bytes>> {
    todo!()
}
