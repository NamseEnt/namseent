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
mod cache;
mod frontend;
mod operator;
mod pages;
mod wal;

use anyhow::Result;
use backend::*;
use cache::*;
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
    Delete {
        id: u128,
        tx: oneshot::Sender<Result<(), ()>>,
    },
    Contains {
        id: u128,
        tx: oneshot::Sender<Result<bool, ()>>,
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

        let set = IdSet::new(path).unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }

        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_and_delete() {
        let path = std::env::temp_dir().join("id_set_test_insert_and_delete");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path).unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }

        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.delete(i as Id).await });
        }

        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_contains() {
        let path = std::env::temp_dir().join("id_set_test_insert_contains");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path).unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=300 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }

        join_set.join_all().await;

        let mut join_set = JoinSet::new();

        for i in 1..=300 {
            let set = set.clone();
            join_set.spawn(async move {
                let contains = set.contains(i as Id).await.unwrap();
                assert!(contains, "{}", i);
            });
        }
        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_delete_contains() {
        let path = std::env::temp_dir().join("id_set_test_insert_delete_contains");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path).unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }

        for i in 5000..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.delete(i as Id).await });
        }

        join_set.join_all().await;

        let mut join_set = JoinSet::new();

        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move {
                let contains = set.contains(i as Id).await.unwrap();
                if i < 5000 {
                    assert!(contains, "{}", i);
                } else {
                    assert!(!contains);
                }
            });
        }
        join_set.join_all().await;
    }
}
