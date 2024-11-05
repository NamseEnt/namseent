//! # IdSet
//!
//! IdSet is a B+Tree implementation for storing 128bit Ids.
//! This is a Set, so it only stores unique ids.
//!
//! Page size is 4KB.
//! Page Offset size is 32 bit.
//! So, IdSet can store 2^32 pages, and 2^40 ids.
//!
//! # Warnings
//!
//! Current version only supports single file ownership.
//!

mod backend;
mod frontend;
mod operator;
mod pages;
mod wal;

use anyhow::Result;
use backend::*;
pub use frontend::*;
use operator::*;
use pages::*;
use tokio::sync::oneshot;
use wal::*;

pub type Id = u128;

enum Request {
    Insert {
        id: u128,
        tx: oneshot::Sender<Result<(), ()>>,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_insert() {
        let path = std::env::temp_dir().join("id_set_test_insert");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let tree = IdSet::new(path).unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let tree = tree.clone();
            join_set.spawn(async move { tree.insert(i as Id).await });
        }

        while let Some(result) = join_set.join_next().await {
            result.unwrap().unwrap();
        }
    }
}
