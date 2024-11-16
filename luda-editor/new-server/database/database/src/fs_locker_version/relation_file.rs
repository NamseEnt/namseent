use super::Id;
use std::{collections::BTreeSet, fs::File, io::Write, path::Path};

type Result<T> = std::io::Result<T>;

pub struct RelationFile {
    main_file: std::fs::File,
    wal_file: std::fs::File,
    set: Set,
    set_before_commit: Set,
}

impl RelationFile {
    pub fn open(dir_path: impl AsRef<Path>, filename: impl AsRef<Path>) -> Result<Self> {
        let mut main_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(dir_path.as_ref().join(&filename).with_extension("main"))?;

        let mut wal_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(dir_path.as_ref().join(&filename).with_extension("wal"))?;

        let set = match last_version_in_wal(&mut wal_file)? {
            Some(last_version) => {
                main_file.write_all(&last_version.to_vec())?;
                main_file.sync_all()?;
                wal_file.set_len(0)?;
                last_version
            }
            None => {
                todo!()
            }
        };

        Ok(Self {
            main_file,
            wal_file,
            set_before_commit: set.clone(),
            set,
        })
    }

    pub fn get(&self) -> &Set {
        &self.set
    }

    pub fn insert(&mut self, id: Id, trx_id: u128) -> Result<()> {
        append_to_wal(&mut self.wal_file, Job::Insert(id), trx_id)?;
        self.set_before_commit.insert(id);
        Ok(())
    }
    pub fn remove(&mut self, id: Id, trx_id: u128) -> Result<()> {
        append_to_wal(&mut self.wal_file, Job::Remove(id), trx_id)?;
        self.set_before_commit.remove(id);
        Ok(())
    }

    pub async fn commit(&mut self) {
        self.set = self.set_before_commit.clone();
    }

    pub async fn rollback(&mut self) {
        self.set_before_commit = self.set.clone();
    }
}

enum Job {
    Insert(Id),
    Remove(Id),
}

#[derive(Default, Clone)]
struct Set {
    inner: BTreeSet<Id>,
}
impl Set {
    fn to_vec(&self) -> Vec<u8> {
        self.inner
            .iter()
            .copied()
            .flat_map(|id| id.to_le_bytes())
            .collect()
    }
    fn insert(&mut self, id: u128) {
        self.inner.insert(id);
    }
    fn remove(&mut self, id: u128) {
        self.inner.remove(&id);
    }
}

fn append_to_wal(wal_file: &mut File, job: Job, trx_id: u128) -> Result<()> {
    todo!()
}

fn last_version_in_wal(wal_file: &mut File) -> Result<Option<Set>> {
    todo!()
}
