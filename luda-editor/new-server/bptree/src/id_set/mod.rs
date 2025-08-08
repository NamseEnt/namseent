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
mod fd;
mod frontend;
mod operator;
mod pages;
mod wal;

use backend::*;
use cache::*;
use fd::*;
pub use frontend::*;
use operator::*;
use pages::*;
use thiserror::Error;
use tokio::sync::oneshot;
use wal::*;

pub type Id = u128;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Something broken, please close and reopen the IdSet")]
    Broken,
    #[error("Temporary error")]
    Temporary,
}
type Result<T> = std::result::Result<T, Error>;

enum FeBeRequest {
    Insert {
        id: Id,
        tx: oneshot::Sender<std::result::Result<(), ()>>,
    },
    Delete {
        id: Id,
        tx: oneshot::Sender<std::result::Result<(), ()>>,
    },
    Contains {
        id: Id,
        tx: oneshot::Sender<std::result::Result<bool, ()>>,
    },
    Next {
        exclusive_start_id: Option<Id>,
        tx: oneshot::Sender<std::result::Result<Option<Vec<Id>>, ()>>,
    },
    Close,
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::TryStreamExt;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_insert() {
        let path = std::env::temp_dir().join("id_map::id_set_test_insert");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path, 5000).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=1 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }

        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_and_delete() {
        let path = std::env::temp_dir().join("id_map::id_set_test_insert_and_delete");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path, 5000).await.unwrap();
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
        let path = std::env::temp_dir().join("id_map::id_set_test_insert_contains");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path, 5000).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }

        join_set.join_all().await;

        let mut join_set = JoinSet::new();

        for i in 1..=10000 {
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
        let path = std::env::temp_dir().join("id_map::id_set_test_insert_delete_contains");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path, 5000).await.unwrap();
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

    #[tokio::test]
    async fn test_insert_delete_contains_without_cache() {
        let path = std::env::temp_dir().join("id_map::test_insert_delete_contains_without_cache");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path, 0).await.unwrap();
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

    #[tokio::test]
    async fn test_insert_delete_contains_small_cache() {
        let path = std::env::temp_dir().join("id_map::test_insert_delete_contains_small_cache");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(path, 2).await.unwrap();
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

    #[tokio::test]
    async fn test_insert_turn_off_contains() {
        let path = std::env::temp_dir().join("id_map::test_insert_turn_off_contains");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(&path, 5000).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }
        join_set.join_all().await;

        assert!(set.try_close().await.is_ok());

        let set = IdSet::new(path, 5000).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=20000 {
            let set = set.clone();
            join_set.spawn(async move {
                let contains = set.contains(i as Id).await.unwrap();
                if i <= 10000 {
                    assert!(contains, "{i}");
                } else {
                    assert!(!contains, "{i}");
                }
            });
        }
        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_next_insert_next_delete_next() {
        let path = std::env::temp_dir().join("id_map::test_next_insert_next_delete_next");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(&path, 5000).await.unwrap();

        assert!(set.next(None).await.unwrap().is_none());
        assert!(set.next(Some(5)).await.unwrap().is_none());

        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }
        join_set.join_all().await;

        let mut all_ids = vec![];
        let mut exclusive_start_id = None;
        while let Some(ids) = set.next(exclusive_start_id).await.unwrap() {
            for i in 1..ids.len() {
                assert!(ids[i - 1] < ids[i]);
            }
            exclusive_start_id = ids.last().cloned();
            all_ids.extend(ids);
        }
        assert!(set.contains(1).await.unwrap());

        assert_eq!(all_ids.len(), 10000);
        for i in 1..=10000 {
            assert_eq!(all_ids[i - 1], i as Id);
        }
    }

    #[tokio::test]
    async fn test_stream() {
        let path = std::env::temp_dir().join("id_map::test_stream");
        if let Err(err) = std::fs::remove_file(&path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let wal_path = path.with_extension("wal");
        if let Err(err) = std::fs::remove_file(&wal_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        let shadow_path = path.with_extension("shadow");
        if let Err(err) = std::fs::remove_file(&shadow_path)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            panic!("{err:?}");
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let set = IdSet::new(&path, 5000).await.unwrap();

        assert!(set.stream().try_next().await.unwrap().is_none());

        let mut join_set = JoinSet::new();
        for i in 1..=10000 {
            let set = set.clone();
            join_set.spawn(async move { set.insert(i as Id).await });
        }
        join_set.join_all().await;

        let mut stream = set.stream();
        let mut all_ids = vec![];
        let mut index = 0;
        while let Some(id) = stream.try_next().await.unwrap() {
            assert_eq!(id, index as Id + 1);
            all_ids.push(id);
            index += 1;
        }

        assert_eq!(all_ids.len(), 10000);

        for i in 1..=10000 {
            assert_eq!(all_ids[i - 1], i as Id);
        }
    }
}
