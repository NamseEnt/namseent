//! # BpMap
//!
//! BpMap is a B+Tree implementation for storing 128bit Key and Variable Size Value.
//! Maximum value bytes limit is 1MB.
//!
//! Page size is 4KB.
//! Page Offset size is 32 bit.
//! So, BpMap can store 2^32 pages, and 2^40 entries.
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
use bytes::Bytes;
use cache::*;
use fd::*;
pub use frontend::*;
use operator::*;
use pages::*;
use thiserror::Error;
use tokio::sync::oneshot;
use wal::*;

pub type Key = u128;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Something broken, please close and reopen the BpMap")]
    Broken,
    #[error("Temporary error")]
    Temporary,
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Entry {
    pub key: Key,
    pub value: Bytes,
}

enum FeBeRequest {
    Insert {
        key: Key,
        value: Bytes,
        tx: oneshot::Sender<std::result::Result<(), ()>>,
    },
    Delete {
        key: Key,
        tx: oneshot::Sender<std::result::Result<(), ()>>,
    },
    Contains {
        key: Key,
        tx: oneshot::Sender<std::result::Result<bool, ()>>,
    },
    // Next {
    //     exclusive_start_key: Option<Key>,
    //     tx: oneshot::Sender<std::result::Result<Option<Vec<Entry>>, ()>>,
    // },
    Close,
    Get {
        key: Key,
        tx: oneshot::Sender<std::result::Result<Option<Bytes>, ()>>,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::TryStreamExt;
    use tokio::task::JoinSet;

    const TEST_COUNT: u32 = 10000;

    #[tokio::test]
    async fn test_insert() {
        let path = std::env::temp_dir().join("bp_map_test_insert");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        let shadow_path = path.with_extension("shadow");
        if shadow_path.exists() {
            std::fs::remove_file(&shadow_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let map = BpMap::new(path, TEST_COUNT as usize / 2).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set
                .spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
        }

        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_and_delete() {
        let path = std::env::temp_dir().join("bp_map_test_insert_and_delete");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        let shadow_path = path.with_extension("shadow");
        if shadow_path.exists() {
            std::fs::remove_file(&shadow_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let map = BpMap::new(path, TEST_COUNT as usize / 2).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set
                .spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
        }

        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set.spawn(async move { map.delete(i as Key).await });
        }

        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_contains() {
        let path = std::env::temp_dir().join("bp_map_test_insert_contains");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        let shadow_path = path.with_extension("shadow");
        if shadow_path.exists() {
            std::fs::remove_file(&shadow_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let map = BpMap::new(path, TEST_COUNT as usize / 2).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set
                .spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
        }

        join_set.join_all().await;

        let mut join_set = JoinSet::new();

        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set.spawn(async move {
                let contains = map.contains(i as Key).await.unwrap();
                assert!(contains, "{}", i);
            });
        }
        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_delete_contains() {
        let path = std::env::temp_dir().join("bp_map_test_insert_delete_contains");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        let shadow_path = path.with_extension("shadow");
        if shadow_path.exists() {
            std::fs::remove_file(&shadow_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let map = BpMap::new(path, TEST_COUNT as usize / 2).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set
                .spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
        }

        for i in TEST_COUNT / 2..=TEST_COUNT {
            let map = map.clone();
            join_set.spawn(async move { map.delete(i as Key).await });
        }

        join_set.join_all().await;

        let mut join_set = JoinSet::new();

        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set.spawn(async move {
                let contains = map.contains(i as Key).await.unwrap();
                if i < TEST_COUNT / 2 {
                    assert!(contains, "{}", i);
                } else {
                    assert!(!contains);
                }
            });
        }
        join_set.join_all().await;
    }

    #[tokio::test]
    async fn test_insert_delete_get() {
        let path = std::env::temp_dir().join("bp_map_test_insert_delete_get");
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let wal_path = path.with_extension("wal");
        if wal_path.exists() {
            std::fs::remove_file(&wal_path).unwrap();
        }
        let shadow_path = path.with_extension("shadow");
        if shadow_path.exists() {
            std::fs::remove_file(&shadow_path).unwrap();
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let map = BpMap::new(path, TEST_COUNT as usize / 2).await.unwrap();
        let mut join_set = JoinSet::new();
        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set
                .spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
        }

        for i in TEST_COUNT / 2..=TEST_COUNT {
            let map = map.clone();
            join_set.spawn(async move { map.delete(i as Key).await });
        }

        join_set.join_all().await;

        let mut join_set = JoinSet::new();

        for i in 1..=TEST_COUNT {
            let map = map.clone();
            join_set.spawn(async move {
                let bytes = map.get(i as Key).await.unwrap();
                if i < TEST_COUNT / 2 {
                    let bytes = bytes.unwrap();

                    assert_eq!(bytes.as_ref(), i.to_le_bytes(), "{}", i);
                } else {
                    assert!(bytes.is_none());
                }
            });
        }
        join_set.join_all().await;
    }

    //     #[tokio::test]
    //     async fn test_insert_delete_contains_without_cache() {
    //         let path = std::env::temp_dir().join("test_insert_delete_contains_without_cache");
    //         if path.exists() {
    //             std::fs::remove_file(&path).unwrap();
    //         }
    //         let wal_path = path.with_extension("wal");
    //         if wal_path.exists() {
    //             std::fs::remove_file(&wal_path).unwrap();
    //         }
    //         let shadow_path = path.with_extension("shadow");
    //         if shadow_path.exists() {
    //             std::fs::remove_file(&shadow_path).unwrap();
    //         }
    //         std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    //         let map = BpMap::new(path, 0).await.unwrap();
    //         let mut join_set = JoinSet::new();
    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
    //         }

    //         for i in as TEST_COUNusize T/2..=10000 {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.delete(i as Key).await });
    //         }

    //         join_set.join_all().await;

    //         let mut join_set = JoinSet::new();

    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move {
    //                 let contains = map.contains(i as Key).await.unwrap();
    //                 if i < as TEST_COUNusize T/2 {
    //                     assert!(contains, "{}", i);
    //                 } else {
    //                     assert!(!contains);
    //                 }
    //             });
    //         }
    //         join_set.join_all().await;
    //     }

    //     #[tokio::test]
    //     async fn test_insert_delete_contains_small_cache() {
    //         let path = std::env::temp_dir().join("test_insert_delete_contains_small_cache");
    //         if path.exists() {
    //             std::fs::remove_file(&path).unwrap();
    //         }
    //         let wal_path = path.with_extension("wal");
    //         if wal_path.exists() {
    //             std::fs::remove_file(&wal_path).unwrap();
    //         }
    //         let shadow_path = path.with_extension("shadow");
    //         if shadow_path.exists() {
    //             std::fs::remove_file(&shadow_path).unwrap();
    //         }
    //         std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    //         let map = BpMap::new(path, 2).await.unwrap();
    //         let mut join_set = JoinSet::new();
    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
    //         }

    //         for i in as TEST_COUNusize T/2..=10000 {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.delete(i as Key).await });
    //         }

    //         join_set.join_all().await;

    //         let mut join_set = JoinSet::new();

    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move {
    //                 let contains = map.contains(i as Key).await.unwrap();
    //                 if i < as TEST_COUNusize T/2 {
    //                     assert!(contains, "{}", i);
    //                 } else {
    //                     assert!(!contains);
    //                 }
    //             });
    //         }
    //         join_set.join_all().await;
    //     }

    //     #[tokio::test]
    //     async fn test_insert_turn_off_contains() {
    //         let path = std::env::temp_dir().join("test_insert_turn_off_contains");
    //         if path.exists() {
    //             std::fs::remove_file(&path).unwrap();
    //         }
    //         let wal_path = path.with_extension("wal");
    //         if wal_path.exists() {
    //             std::fs::remove_file(&wal_path).unwrap();
    //         }
    //         let shadow_path = path.with_extension("shadow");
    //         if shadow_path.exists() {
    //             std::fs::remove_file(&shadow_path).unwrap();
    //         }
    //         std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    //         let map = BpMap::new(&path, as TEST_COUNusize T/2).await.unwrap();
    //         let mut join_set = JoinSet::new();
    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
    //         }
    //         join_set.join_all().await;

    //         assert!(map.try_close().await.is_ok());

    //         let map = BpMap::new(path, as TEST_COUNusize T/2).await.unwrap();
    //         let mut join_set = JoinSet::new();
    //         for i in 1..=20000 {
    //             let map = map.clone();
    //             join_set.spawn(async move {
    //                 let contains = map.contains(i as Key).await.unwrap();
    //                 if i <= 10000 {
    //                     assert!(contains, "{i}");
    //                 } else {
    //                     assert!(!contains, "{i}");
    //                 }
    //             });
    //         }
    //         join_set.join_all().await;
    //     }

    //     #[tokio::test]
    //     async fn test_next_insert_next_delete_next() {
    //         let path = std::env::temp_dir().join("test_next_insert_next_delete_next");
    //         if path.exists() {
    //             std::fs::remove_file(&path).unwrap();
    //         }
    //         let wal_path = path.with_extension("wal");
    //         if wal_path.exists() {
    //             std::fs::remove_file(&wal_path).unwrap();
    //         }
    //         let shadow_path = path.with_extension("shadow");
    //         if shadow_path.exists() {
    //             std::fs::remove_file(&shadow_path).unwrap();
    //         }
    //         std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    //         let map = BpMap::new(&path, as TEST_COUNusize T/2).await.unwrap();

    //         assert!(map.next(None).await.unwrap().is_none());
    //         assert!(map.next(Some(5)).await.unwrap().is_none());

    //         let mut join_set = JoinSet::new();
    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
    //         }
    //         join_set.join_all().await;

    //         let mut all_keys = vec![];
    //         let mut exclusive_start_key = None;
    //         while let Some(keys) = map.next(exclusive_start_key).await.unwrap() {
    //             for i in 1..keys.len() {
    //                 assert!(keys[i - 1] < keys[i]);
    //             }
    //             exclusive_start_key = keys.last().cloned();
    //             all_keys.extend(keys);
    //         }
    //         assert!(map.contains(1).await.unwrap());

    //         assert_eq!(all_keys.len(), 10000);
    //         for i in 1..=TEST_COUNT {
    //             assert_eq!(all_keys[i - 1], i as Key);
    //         }
    //     }

    //     #[tokio::test]
    //     async fn test_stream() {
    //         let path = std::env::temp_dir().join("test_stream");
    //         if path.exists() {
    //             std::fs::remove_file(&path).unwrap();
    //         }
    //         let wal_path = path.with_extension("wal");
    //         if wal_path.exists() {
    //             std::fs::remove_file(&wal_path).unwrap();
    //         }
    //         let shadow_path = path.with_extension("shadow");
    //         if shadow_path.exists() {
    //             std::fs::remove_file(&shadow_path).unwrap();
    //         }
    //         std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    //         let map = BpMap::new(&path, as TEST_COUNusize T/2).await.unwrap();

    //         assert!(map.stream().try_next().await.unwrap().is_none());

    //         let mut join_set = JoinSet::new();
    //         for i in 1..=TEST_COUNT {
    //             let map = map.clone();
    //             join_set.spawn(async move { map.insert(i as Key, i.to_le_bytes().to_vec().into()).await });
    //         }
    //         join_set.join_all().await;

    //         let mut stream = map.stream();
    //         let mut all_keys = vec![];
    //         let mut index = 0;
    //         while let Some(key) = stream.try_next().await.unwrap() {
    //             assert_eq!(key, index as Key + 1);
    //             all_keys.push(key);
    //             index += 1;
    //         }

    //         assert_eq!(all_keys.len(), 10000);

    //         for i in 1..=TEST_COUNT {
    //             assert_eq!(all_keys[i - 1], i as Key);
    //         }
    //     }
}
