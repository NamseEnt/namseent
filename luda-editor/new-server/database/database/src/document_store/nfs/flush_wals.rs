use super::deserialize_wal_writes;
use bytes::Bytes;
use rayon::prelude::*;
use std::{io::Write, path::Path};

pub(crate) fn flush_wals(wal_dir: &Path, doc_dir: &Path) {
    for entry in std::fs::read_dir(wal_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let wal_bytes = Bytes::from(std::fs::read(&path).unwrap());
        match deserialize_wal_writes(wal_bytes.clone()) {
            Ok(wal_writes) => {
                wal_writes.into_par_iter().for_each(|wal_write| {
                    let path = doc_dir.join(&wal_write.key);
                    match wal_write.value {
                        Some(value) => {
                            let mut file = std::fs::OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(&path)
                                .unwrap();
                            file.write_all(&value).unwrap();
                            file.sync_all().unwrap();
                        }
                        None => {
                            std::fs::remove_file(&path).unwrap();
                        }
                    }
                });
            }
            Err(err) => {
                eprintln!("Error deserializing WAL: {:?}, bytes: {:?}", err, wal_bytes);
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}
